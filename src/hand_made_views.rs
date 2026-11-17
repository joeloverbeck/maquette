//! Reads the record of a hand-made armature's views as the landmarks the list of proportions
//! asks for.
//!
//! The record is `data/saffi/hand-made-views.toml`: positions taken from two hand-made
//! drawings, a front view and a three-quarter view. It was written before the list of
//! proportions existed, so it names what the drawings hold, not what the head model reads.
//! This module is the one place that knows how the two meet, and [`RULES`] states it.
//! It also writes the set of proportions worked out from the record, with a note of the record
//! and the rules it was read by. It serves that one record: a second hand-made armature would
//! earn its own reading.

use std::collections::BTreeMap;

use crate::hand_made_drawings::SKULL;
use crate::head_model::{self, Reading};
use crate::number::toml as toml_number;
use crate::proportions;
use crate::toml_text::{quoted, quoted_lines};
use crate::views::View;
use crate::{Error, Result};

/// How the record's positions are read as landmarks. Written into every set of proportions
/// worked out from a record.
pub const RULES: &[&str] = &[
    "A landmark the list of proportions asks for under the record's own name is that position, with hyphens for underscores.",
    "A lid line is the middle of the drawn lid band: its inner end is midway between `inner_lower` and `inner_upper`, its crest midway between `lower_mid` and `upper_mid`, its outer end midway between `outer_lower` and `outer_tip`.",
    "A brow line is the middle of the drawn brow band: each end is midway between the band's top and bottom corner there.",
    "The face outline's top end is the record's first node of that side, and its jaw point is the node next to the chin: `node_3` in the front view and, in the three-quarter view, `node_5` on the far side and `node_3` on the near side.",
    "Where the record gives a crossing of the face outline with a drawn level line, it stands for the crossing at the eye line, the nose level or the mouth level. The drawn lines are 4 units, half a unit and nothing away from those with the S2 underdrawings' lids, and the first is about 6 units away with the final drawings' lids, which lower the eye line. The outline moves sideways by less than half a unit between them.",
    "In the front view a point of the face outline is both the cheek point and the jaw-line point of its level. In the three-quarter view it is the cheek point on the far side, the character's left, and the jaw-line point on the near side.",
    "A position that the head itself covers in the three-quarter drawing is left out, because it cannot be seen there: the far ear's two base points and its inner extent. Hair that covers a position is ignored, because an armature has no hair. The skull is kept: it is a construction, drawn under everything.",
    "The three-quarter view is taken as turned by the pinned turn, toward the viewer's right.",
    "Not read: pupils, the line under the nose mark, the nose mark's top, the nodes of an outline that mark no landmark of the list, the ends of the drawn level lines and the drawn centre line. The armature builds its level lines and its centre line by rule.",
];

/// The two views of a record, read as landmarks, with what identifies the drawings.
#[derive(Debug, Clone)]
pub struct HandMade {
    pub front: Reading,
    pub three_quarter: Reading,
    /// What the record says the drawings are.
    pub what: String,
    pub front_sha256: String,
    pub three_quarter_sha256: String,
    /// Where the front view's landmarks stand in the record.
    pub front_record: Provenance,
    /// Where the three-quarter view's landmarks stand in the record.
    pub three_quarter_record: Provenance,
    /// The record of the final drawings, when one was given: its landmarks replace the
    /// record's landmarks of the same name in the same view.
    pub final_drawings: Option<FinalDrawings>,
}

/// What a record of final drawings says of itself.
#[derive(Debug, Clone)]
pub struct FinalDrawings {
    /// The path the record was named by.
    pub path: String,
    /// What it says the drawings are.
    pub what: String,
    /// The rule by which its landmarks replace the other record's.
    pub rule: String,
    pub front_sha256: String,
    pub three_quarter_sha256: String,
}

/// How one view's landmarks stand in the record, under the record's own names.
#[derive(Debug, Clone, Default)]
pub struct Provenance {
    /// For each landmark, seen or covered, the record's names behind it: one, or the two it
    /// is midway between.
    pub read_from: BTreeMap<String, Vec<String>>,
    /// Every name the record gives the view.
    pub recorded: Vec<String>,
    /// The names among those whose position is the final drawing's, not the record's own.
    pub from_final_drawings: Vec<String>,
    /// Where the record puts each of those names, in the drawing's own coordinates, and
    /// whether the head covers it there.
    pub positions: BTreeMap<String, RecordedPosition>,
    /// The landmarks the reading leaves out because the head covers them in the drawing, and
    /// where the record puts each. No proportion is taken from them; they can still be laid
    /// beside a generated position.
    pub covered_by_head: BTreeMap<String, [f64; 2]>,
}

/// One position the record gives a view, under the record's own name.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RecordedPosition {
    pub at: [f64; 2],
    /// The head covers it in the turned drawing, so it cannot be seen there.
    pub covered_by_head: bool,
}

/// What a hand-made position says about the head, and so what it can be compared in.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Marks {
    /// A place on the head: both its sideways position and its height.
    Position,
    /// How high or how low a round form or an envelope reaches: its height only.
    Height,
    /// How far to a side a round form or an envelope reaches: its sideways position only.
    Sideways,
}

/// The rule behind [`marks`], in words.
pub const MARKS_RULE: &str = "The top, the bottom and the side extents of the skull, of an iris and of the hair say how far a round form or an envelope reaches, not where a feature is. The record takes each where the drawn outline is level or upright, and along the outline that place marks nothing: the hair's is a hairstyle's, and an iris is cut by the lid. So a top or a bottom of those three is compared in height only, and a side extent sideways only. Where the face outline crosses the eye line, the nose level or the mouth level, the record takes the crossing at the height of the drawn level line, which need not be that level's: the height says where the line was drawn and nothing of the outline, so such a crossing is compared sideways only. Every other landmark is compared as a position, by the straight distance.";

/// What the hand-made position of a landmark marks.
pub fn marks(landmark: &str) -> Marks {
    let round_form = ["skull-", "iris-", "hair-"]
        .iter()
        .any(|form| landmark.starts_with(form));
    let outline_at_a_level = ["face-outline-", "jaw-line-"]
        .iter()
        .any(|outline| landmark.starts_with(outline))
        && landmark.ends_with("-level");
    if outline_at_a_level {
        Marks::Sideways
    } else if !round_form {
        Marks::Position
    } else if landmark.ends_with("-top") || landmark.ends_with("-bottom") {
        Marks::Height
    } else if landmark.ends_with("-extent") {
        Marks::Sideways
    } else {
        Marks::Position
    }
}

struct Recorded {
    position: [f64; 2],
    covered_by_head: bool,
}

const SIDES: [&str; 2] = ["left", "right"];
/// The far side of a head turned toward the viewer's right.
const FAR: &str = "left";

/// Landmarks kept under the record's own name, for each side.
const PAIRED: [(&str, &str); 14] = [
    ("eye", "outer_corner"),
    ("eye", "inner_corner"),
    ("eye", "opening_top"),
    ("eye", "opening_bottom"),
    ("iris", "centre"),
    ("iris", "top"),
    ("iris", "bottom"),
    ("iris", "outer_extent"),
    ("iris", "inner_extent"),
    ("nose_mark", "corner"),
    ("mouth_line", "end"),
    ("hair", "extent"),
    ("shoulder", "point"),
    ("neck", "base"),
];

const EAR: [&str; 5] = [
    "base_outer",
    "base_inner",
    "top",
    "outer_extent",
    "inner_extent",
];

const ON_CENTRE_LINE: [&str; 9] = [
    "chin",
    "hair_top",
    "skull_centre",
    "skull_top",
    "skull_bottom",
    "skull_left_extent",
    "skull_right_extent",
    "nose_mark_bottom",
    "mouth_line_centre",
];

const LEVELS: [&str; 3] = ["eye", "nose", "mouth"];

fn hyphens(name: &str) -> String {
    name.replace('_', "-")
}

/// One view of the record, read.
struct ViewRead {
    reading: Reading,
    sha256: String,
    /// The digest of the view's final drawing, when a record of the final drawings is given.
    final_sha256: Option<String>,
    provenance: Provenance,
}

/// The table of the record that holds a view.
fn record_key(view: View) -> &'static str {
    match view {
        View::Front => "front",
        View::ThreeQuarter => "three_quarter",
        // No hand-made side view exists; a record that had one would name it so.
        View::Side => "side",
    }
}

/// One of the two files a reading is made from, as it was parsed: the path it was named by,
/// what a message calls it, and its tables. A fault is reported under the path of the file
/// it is in.
struct RecordFile<'a> {
    path: &'a str,
    what: &'static str,
    table: toml::Table,
}

impl<'a> RecordFile<'a> {
    fn parse(path: &'a str, what: &'static str, text: &str) -> Result<RecordFile<'a>> {
        let table = text
            .parse()
            .map_err(|error| Error::new(format!("{path}: {what} is not valid TOML: {error}")))?;
        Ok(RecordFile { path, what, table })
    }

    /// A fault in this file, in words that follow what a message calls it.
    fn fault(&self, found: impl std::fmt::Display) -> Error {
        Error::new(format!("{}: {}{found}", self.path, self.what))
    }

    /// A text the file gives under a key of one of its tables.
    fn said(&self, section: &str, key: &str) -> Result<String> {
        self.table
            .get(section)
            .and_then(|section| section.get(key))
            .and_then(toml::Value::as_str)
            .map(str::to_string)
            .ok_or_else(|| self.fault(format!(" has no text `{key}` in [{section}]")))
    }

    /// A view's table, with its digest and its landmarks.
    fn view_section(&self, view: &str) -> Result<(String, &toml::Table)> {
        let section = self
            .table
            .get("views")
            .and_then(|views| views.get(view))
            .and_then(toml::Value::as_table)
            .ok_or_else(|| self.fault(format!(" has no [views.{view}]")))?;
        let sha256 = section
            .get("sha256")
            .and_then(toml::Value::as_str)
            .ok_or_else(|| self.fault(format!(" has no sha256 in [views.{view}]")))?
            .to_string();
        let landmarks = section
            .get("landmarks")
            .and_then(toml::Value::as_table)
            .ok_or_else(|| self.fault(format!(" has no [views.{view}.landmarks]")))?;
        Ok((sha256, landmarks))
    }
}

fn read_view(
    record: &RecordFile,
    final_drawings: Option<&RecordFile>,
    view: View,
) -> Result<ViewRead> {
    let (turned, turn_degrees) = (view != View::Front, view.turn_degrees());
    // The messages below name the view as the record does.
    let view = record_key(view);
    let (sha256, landmarks) = record.view_section(view)?;
    let final_landmarks = match final_drawings {
        Some(final_drawings) => Some(final_drawings.view_section(view)?),
        None => None,
    };

    // A landmark of the final drawing replaces the record's landmark of the same name.
    let mut recorded: BTreeMap<&str, Recorded> = BTreeMap::new();
    let mut from_final_drawings: Vec<String> = Vec::new();
    let both = landmarks.iter().map(|entry| (entry, false)).chain(
        final_landmarks
            .iter()
            .flat_map(|(_, landmarks)| landmarks.iter().map(|entry| (entry, true))),
    );
    for ((name, entry), is_final) in both {
        // The file the entry stands in: the one a fault in it is reported under.
        let file = match final_drawings {
            Some(final_drawings) if is_final => final_drawings,
            _ => record,
        };
        let coordinate = |key: &str| -> Result<f64> {
            match entry.get(key) {
                Some(toml::Value::Float(number)) => Ok(*number),
                Some(toml::Value::Integer(number)) => Ok(*number as f64),
                _ => Err(file.fault(format!(
                    " gives `{name}` no number `{key}` in its {view} view"
                ))),
            }
        };
        let covered_by_head = entry
            .get("under")
            .and_then(toml::Value::as_array)
            .is_some_and(|shapes| shapes.iter().any(|shape| shape.as_str() == Some(SKULL)));
        if is_final {
            from_final_drawings.push(name.clone());
        }
        recorded.insert(
            name.as_str(),
            Recorded {
                position: [coordinate("x")?, coordinate("y")?],
                covered_by_head,
            },
        );
    }

    let recorded_at = |name: &str| recorded.get(name).map(|point| point.position);
    // A position the head covers in the turned drawing cannot be seen there. The skull's own
    // points are never under the skull, so they are kept.
    let covered = |name: &str| {
        turned
            && recorded
                .get(name)
                .is_some_and(|point| point.covered_by_head)
    };

    let mut reading = Reading {
        turn_degrees,
        ..Reading::default()
    };
    let mut read_from: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut covered_landmarks: BTreeMap<String, [f64; 2]> = BTreeMap::new();
    // A landmark stands where the record puts it, or midway between two of the record's
    // positions. It is read as seen only when every position behind it can be seen.
    let mut put = |name: String, from: &[&str]| {
        let position = match from {
            [only] => recorded_at(only),
            [first, second] => match (recorded_at(first), recorded_at(second)) {
                (Some(first), Some(second)) => {
                    Some([(first[0] + second[0]) / 2.0, (first[1] + second[1]) / 2.0])
                }
                _ => None,
            },
            _ => None,
        };
        if let Some(position) = position {
            let sources = from.iter().map(|from| from.to_string()).collect();
            read_from.insert(name.clone(), sources);
            if from.iter().any(|from| covered(from)) {
                covered_landmarks.insert(name, position);
            } else {
                reading.points.insert(name, position);
            }
        }
    };

    for name in ON_CENTRE_LINE {
        put(hyphens(name), &[name]);
    }
    for side in SIDES {
        for (stem, point) in PAIRED {
            let name = format!("{stem}_{side}_{point}");
            put(hyphens(&name), &[&name]);
        }
        for point in EAR {
            let name = format!("ear_{side}_{point}");
            put(hyphens(&name), &[&name]);
        }

        let lid = |point: &str| format!("lid_line_{side}_{point}");
        put(
            format!("lid-line-{side}-inner-end"),
            &[&lid("inner_lower"), &lid("inner_upper")],
        );
        put(
            format!("lid-line-{side}-crest"),
            &[&lid("lower_mid"), &lid("upper_mid")],
        );
        put(
            format!("lid-line-{side}-outer-end"),
            &[&lid("outer_lower"), &lid("outer_tip")],
        );
        for end in ["inner", "outer"] {
            put(
                format!("brow-line-{side}-{end}-end"),
                &[
                    &format!("brow_{side}_{end}_top"),
                    &format!("brow_{side}_{end}_bottom"),
                ],
            );
        }

        // The face outline: both the cheek point and the jaw-line point in the front view;
        // the cheek point on the far side and the jaw-line point on the near side when turned.
        let outline = |point: &str| format!("face_outline_{side}_{point}");
        let cheek = !turned || side == FAR;
        let jaw_line = !turned || side != FAR;
        let mut levels = vec![("top".to_string(), outline("node_1"))];
        for level in LEVELS {
            levels.push((
                format!("at-{level}-level"),
                outline(&format!("at_{level}_level")),
            ));
        }
        for (level, from) in levels {
            // The near side's first node in the turned view is where its drawn outline
            // starts, at the nose level: it is no top end.
            if turned && side != FAR && level == "top" {
                continue;
            }
            if cheek {
                put(format!("face-outline-{side}-{level}"), &[&from]);
            }
            if jaw_line {
                put(format!("jaw-line-{side}-{level}"), &[&from]);
            }
        }
        let jaw_node = if turned && side == FAR {
            "node_5"
        } else {
            "node_3"
        };
        put(format!("face-outline-{side}-jaw"), &[&outline(jaw_node)]);
    }

    let provenance = Provenance {
        read_from,
        recorded: recorded.keys().map(|name| name.to_string()).collect(),
        from_final_drawings,
        positions: recorded
            .keys()
            .map(|name| {
                let position = RecordedPosition {
                    at: recorded[name].position,
                    covered_by_head: covered(name),
                };
                (name.to_string(), position)
            })
            .collect(),
        covered_by_head: covered_landmarks,
    };
    Ok(ViewRead {
        reading,
        sha256,
        final_sha256: final_landmarks.map(|(sha256, _)| sha256),
        provenance,
    })
}

/// The record of the final drawings, as it is given beside a record of hand-made views: the
/// path it was named by, and its text.
#[derive(Debug, Clone, Copy)]
pub struct FinalRecord<'a> {
    pub path: &'a str,
    pub text: &'a str,
}

/// Reads the text of a record of hand-made views, which `path` names. Given the record of
/// the final drawings as well, each of its landmarks replaces the record's landmark of the
/// same name in the same view. A fault is reported under the path of the record it is in.
pub fn read(path: &str, text: &str, final_record: Option<FinalRecord>) -> Result<HandMade> {
    let record = RecordFile::parse(path, "the record", text)?;
    let what = record.said("source", "what")?;
    let final_file = match final_record {
        Some(given) => Some(RecordFile::parse(
            given.path,
            "the record of the final drawings",
            given.text,
        )?),
        None => None,
    };
    let front = read_view(&record, final_file.as_ref(), View::Front)?;
    let three_quarter = read_view(&record, final_file.as_ref(), View::ThreeQuarter)?;
    let final_drawings = match &final_file {
        Some(file) => Some(FinalDrawings {
            path: file.path.to_string(),
            what: file.said("source", "what")?,
            rule: file.said("supersedes", "rule")?,
            front_sha256: front.final_sha256.clone().unwrap_or_default(),
            three_quarter_sha256: three_quarter.final_sha256.clone().unwrap_or_default(),
        }),
        None => None,
    };
    Ok(HandMade {
        front: front.reading,
        three_quarter: three_quarter.reading,
        what,
        front_sha256: front.sha256,
        three_quarter_sha256: three_quarter.sha256,
        front_record: front.provenance,
        three_quarter_record: three_quarter.provenance,
        final_drawings,
    })
}

impl HandMade {
    /// The set of proportions worked out from the record, as the text of its file: the set
    /// under a `[source]` table that names the record, the rules it was read by, and where
    /// each hand-made view puts the unit, the eye line and the turn axis. `record_path` is the
    /// path the record was named by, which the file gives as it is.
    ///
    /// Given a share, the set also carries the named correction `lower_face_set_back`, worked
    /// out from its own depths at that share of the chin's full amount, and the file says so.
    pub fn proportions_file(
        &self,
        record_path: &str,
        lower_face_set_back: Option<f64>,
    ) -> Result<String> {
        let mut set = head_model::proportions_from_views(&self.front, &self.three_quarter)?;
        if let Some(share) = lower_face_set_back {
            set = head_model::with_lower_face_set_back(set, share)?;
        }
        let mut opening = String::from(
            "\
# A set of proportions worked out from the record of a hand-made armature's views,
# by `maquette proportions-from-hand-made-views`. It was not typed in by hand: the tests check
# that this file is what the command writes from the record.
#
# These are the proportions of a comparison case. They never enter a proportion table.
#
# `maquette list-proportions` defines each proportion. `maquette draw` reads this file as it
# reads any set of proportions.

",
        );
        opening.push_str("[source]\n");
        opening.push_str(&format!("record = {}\n", quoted(record_path)));
        opening.push_str(&format!("what = {}\n", quoted(&self.what)));
        opening.push_str(&format!("front_sha256 = {}\n", quoted(&self.front_sha256)));
        opening.push_str(&format!(
            "three_quarter_sha256 = {}\n",
            quoted(&self.three_quarter_sha256)
        ));
        if let Some(final_drawings) = &self.final_drawings {
            opening.push_str(
                "# The record of the final drawings, whose landmarks replace the record's of the same name.\n",
            );
            opening.push_str(&format!(
                "final_drawings = {}\n",
                quoted(&final_drawings.path)
            ));
            opening.push_str(&format!(
                "final_drawings_what = {}\n",
                quoted(&final_drawings.what)
            ));
            opening.push_str(&format!(
                "final_drawings_rule = {}\n",
                quoted(&final_drawings.rule)
            ));
            opening.push_str(&format!(
                "final_front_sha256 = {}\n",
                quoted(&final_drawings.front_sha256)
            ));
            opening.push_str(&format!(
                "final_three_quarter_sha256 = {}\n",
                quoted(&final_drawings.three_quarter_sha256)
            ));
        }
        opening.push_str(&format!(
            "turn_degrees = {}\n",
            toml_number(self.three_quarter.turn_degrees, 3)
        ));
        opening.push_str(&format!(
            "taken_by = {}\n",
            quoted(
                "The head model's inverse direction: landmark positions read from the two views, back into proportions."
            )
        ));
        opening.push_str("# How the record's positions were read as the landmarks the list of proportions asks for.\n");
        opening.push_str(&format!("reading_rules = {}\n", quoted_lines(RULES)));
        opening.push_str(
            "# What was done where the two views disagree, or one does not show a point.\n",
        );
        opening.push_str(&format!(
            "disagreement_rules = {}\n",
            quoted_lines(proportions::list()?.disagreement_rules())
        ));
        if let Some(share) = lower_face_set_back {
            opening.push_str("# The share of the chin's full amount at which the named correction `lower_face_set_back`, at the\n");
            opening.push_str("# end of this file, was worked out from this set's own depths. The list of proportions states the\n");
            opening.push_str("# rule of each amount. The drawings give no side view, so no amount was measured on them.\n");
            opening.push_str(&format!(
                "lower_face_set_back_share = {}\n",
                toml_number(share, 6)
            ));
        }
        opening.push_str("\n# Where each hand-made view puts the unit, the eye line and the upright axis the head turns about,\n");
        opening.push_str("# in the record's own units. `maquette draw` with these lays a generated view over the hand-made one.\n");
        opening.push_str(&frame_lines(&self.front, View::Front)?);
        opening.push('\n');
        opening.push_str(&frame_lines(&self.three_quarter, View::ThreeQuarter)?);
        set.write(&opening)
    }
}

/// Where a hand-made view puts the unit, the eye line and the turn axis, in its own units.
fn frame_lines(reading: &Reading, view: View) -> Result<String> {
    let frame = reading.frame(view)?;
    let table = record_key(view);
    let axis = reading
        .points
        .get("skull-centre")
        .ok_or_else(|| Error::new(format!("the record's {table} view has no skull centre")))?;
    Ok(format!(
        "[source.{table}]\nunit = {}\neye_line_y = {}\nturn_axis_x = {}\n",
        toml_number(frame.unit, 3),
        toml_number(frame.eye_line_y, 3),
        toml_number(axis[0], 3),
    ))
}
