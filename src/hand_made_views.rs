//! Reads the record of a hand-made armature's views as the landmarks the list of proportions
//! asks for.
//!
//! The record is `data/saffi/hand-made-views.toml`: positions taken from two hand-made
//! drawings, a front view and a three-quarter view. It was written before the list of
//! proportions existed, so it names what the drawings hold, not what the head model reads.
//! This module is the one place that knows how the two meet, and [`RULES`] states it.
//! It serves that one record: a second hand-made armature would earn its own reading.

use std::collections::BTreeMap;

use crate::head_model::Reading;
use crate::views::THREE_QUARTER_TURN_DEGREES;
use crate::{Error, Result};

/// How the record's positions are read as landmarks. Written into every set of proportions
/// worked out from a record.
pub const RULES: &[&str] = &[
    "A landmark the list of proportions asks for under the record's own name is that position, with hyphens for underscores.",
    "A lid line is the middle of the drawn lid band: its inner end is midway between `inner_lower` and `inner_upper`, its crest midway between `lower_mid` and `upper_mid`, its outer end midway between `outer_lower` and `outer_tip`.",
    "A brow line is the middle of the drawn brow band: each end is midway between the band's top and bottom corner there.",
    "The face outline's top end is the record's first node of that side, and its jaw point is the node next to the chin: `node_3` in the front view and, in the three-quarter view, `node_5` on the far side and `node_3` on the near side.",
    "Where the record gives a crossing of the face outline with a drawn level line, it stands for the crossing at the eye line, the nose level or the mouth level. The drawn lines are 4 units, half a unit and nothing away from those, and the outline moves sideways by less than half a unit between them.",
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
}

/// How one view's landmarks stand in the record, under the record's own names.
#[derive(Debug, Clone, Default)]
pub struct Provenance {
    /// For each landmark, seen or covered, the record's names behind it: one, or the two it
    /// is midway between.
    pub read_from: BTreeMap<String, Vec<String>>,
    /// Every name the record gives the view.
    pub recorded: Vec<String>,
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
pub const MARKS_RULE: &str = "The top, the bottom and the side extents of the skull, of an iris and of the hair say how far a round form or an envelope reaches, not where a feature is. The record takes each where the drawn outline is level or upright, and along the outline that place marks nothing: the hair's is a hairstyle's, and an iris is cut by the lid. So a top or a bottom of those three is compared in height only, and a side extent sideways only. Every other landmark is compared as a position, by the straight distance.";

/// What the hand-made position of a landmark marks.
pub fn marks(landmark: &str) -> Marks {
    let round_form = ["skull-", "iris-", "hair-"]
        .iter()
        .any(|form| landmark.starts_with(form));
    if !round_form {
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
    provenance: Provenance,
}

fn read_view(record: &toml::Table, view: &str, turned: bool) -> Result<ViewRead> {
    let section = record
        .get("views")
        .and_then(|views| views.get(view))
        .and_then(toml::Value::as_table)
        .ok_or_else(|| Error::new(format!("the record has no [views.{view}]")))?;
    let sha256 = section
        .get("sha256")
        .and_then(toml::Value::as_str)
        .ok_or_else(|| Error::new(format!("the record's [views.{view}] has no sha256")))?
        .to_string();
    let landmarks = section
        .get("landmarks")
        .and_then(toml::Value::as_table)
        .ok_or_else(|| Error::new(format!("the record has no [views.{view}.landmarks]")))?;

    let mut recorded: BTreeMap<&str, Recorded> = BTreeMap::new();
    for (name, entry) in landmarks {
        let coordinate = |key: &str| -> Result<f64> {
            match entry.get(key) {
                Some(toml::Value::Float(number)) => Ok(*number),
                Some(toml::Value::Integer(number)) => Ok(*number as f64),
                _ => Err(Error::new(format!(
                    "the record's {view} view gives `{name}` no number `{key}`"
                ))),
            }
        };
        let covered_by_head = entry
            .get("under")
            .and_then(toml::Value::as_array)
            .is_some_and(|shapes| {
                shapes
                    .iter()
                    .any(|shape| shape.as_str() == Some("face/cranium"))
            });
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
        turn_degrees: if turned {
            THREE_QUARTER_TURN_DEGREES
        } else {
            0.0
        },
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
        provenance,
    })
}

/// Reads the text of a record of hand-made views.
pub fn read(text: &str) -> Result<HandMade> {
    let record: toml::Table = text
        .parse()
        .map_err(|error| Error::new(format!("the record is not valid TOML: {error}")))?;
    let what = record
        .get("source")
        .and_then(|source| source.get("what"))
        .and_then(toml::Value::as_str)
        .ok_or_else(|| Error::new("the record's [source] does not say what the drawings are"))?
        .to_string();
    let front = read_view(&record, "front", false)?;
    let three_quarter = read_view(&record, "three_quarter", true)?;
    Ok(HandMade {
        front: front.reading,
        three_quarter: three_quarter.reading,
        what,
        front_sha256: front.sha256,
        three_quarter_sha256: three_quarter.sha256,
        front_record: front.provenance,
        three_quarter_record: three_quarter.provenance,
    })
}
