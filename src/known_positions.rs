//! Scores a reading against a drawing whose true positions are on record, and sets two sets
//! of proportions against each other.
//!
//! An agent's reading is an artificial evaluation: it is trusted only as far as it has
//! matched drawings whose positions are known. This module hides how a reading and such a
//! record meet: which recorded position answers to which landmark of a reading, what a
//! reading's error is measured in, and which positions are kept apart because they cannot be
//! seen in the picture. The record is one of hand-made views, such as
//! `data/saffi/hand-made-views.toml`, read as [`hand_made_views`] reads it.
//!
//! It reports and judges nothing. What a score shows holds for the drawings scored and the
//! instructions followed, and no further.

use std::collections::BTreeMap;

use crate::differences::THRESHOLD_PERCENT;
use crate::hand_made_views::{self, Marks, Provenance};
use crate::head_model;
use crate::measuring::compared_in;
use crate::number::{median, toml as toml_number};
use crate::proportions::{self, Kind};
use crate::readings::{Faces, REFERENCE_FIELDS, Record, list_names};
use crate::toml_text::{position, quoted, quoted_list};
use crate::views::View;
use crate::{Error, Result};

/// The rules of the score, by name, as the file states them.
pub const RULES: &[(&str, &str)] = &[
    (
        "known_position",
        "A landmark of the reading is set against the record's position that the reading rules of the record give the same name: a lid line and a brow line are the middle of the drawn band, an iris centre is not asked of a reader, and in a turned view the near side's outline, which a reader places under the face outline's name, is the record's jaw line.",
    ),
    (
        "one_picture",
        "The record gives positions in the units of the drawing's file, and the reading in pixels of the picture it read. They are compared as they are: the picture is taken to be a render of that drawing at one pixel to a unit, which nothing here checks.",
    ),
    (
        "one_direction",
        "The top and the bottom of an iris and the hair's top are scored in height only, a side extent of the hair, an iris or an ear sideways only, and a crossing of the face outline with a level sideways only: the record takes a crossing at the height of the drawn level line. Every other landmark is scored by the straight distance.",
    ),
    (
        "yardstick",
        "Head height is the record's own in the view that is scored, from the hair's top to the chin. An error is above 1% when its distance is more than a hundredth of it.",
    ),
    (
        "covered",
        "A position that a filled shape covers in the drawing cannot be seen in a picture of it: the record says so with `under`. Such a position scores nothing. Where the reader placed it all the same, the distance is listed apart; where the reader stated it as not seen, that is listed as rightly left out.",
    ),
    (
        "kinds",
        "A landmark's kind is the feature it belongs to, as the list a reading asks for gives it. The mean, the worst and the count above 1% are given for each kind, over the landmarks that are placed, can be seen and have a known position.",
    ),
];

/// One landmark of a reading set against its known position.
#[derive(Debug, Clone)]
pub struct Scored {
    pub landmark: String,
    pub kind: &'static str,
    pub marks: Marks,
    pub read: [f64; 2],
    pub known: [f64; 2],
    pub distance: f64,
    pub percent_of_head_height: f64,
    /// The shapes that cover the known position in the drawing. Empty when it can be seen.
    pub under: Vec<String>,
}

/// A reading scored against known positions.
#[derive(Debug, Clone)]
pub struct Score {
    pub reading: String,
    pub record: String,
    pub what: String,
    pub record_sha256: String,
    pub reader: String,
    pub picture: String,
    pub picture_sha256: String,
    pub instructions: String,
    pub aids: String,
    pub view: View,
    pub head_height: f64,
    /// Placed, visible and known: worst first.
    pub scored: Vec<Scored>,
    /// Placed though the drawing covers the position: worst first.
    pub placed_though_covered: Vec<Scored>,
    pub not_seen_though_visible: Vec<String>,
    pub not_seen_and_covered: Vec<String>,
    pub not_seen_and_unknown: Vec<String>,
    /// Placed landmarks the record gives no position for.
    pub no_known_position: Vec<String>,
    /// Known, visible positions of landmarks the view asks for that the reading neither
    /// places nor states as not seen.
    pub not_read: Vec<String>,
}

fn record_key(view: View) -> &'static str {
    match view {
        View::Front => "front",
        View::ThreeQuarter => "three_quarter",
        View::Side => "side",
    }
}

/// The shapes the record says cover each of its positions in a view.
fn covers(record: &toml::Table, view: View) -> BTreeMap<String, Vec<String>> {
    let landmarks = record
        .get("views")
        .and_then(|views| views.get(record_key(view)))
        .and_then(|view| view.get("landmarks"))
        .and_then(toml::Value::as_table);
    let mut out = BTreeMap::new();
    for (name, entry) in landmarks.into_iter().flatten() {
        let under: Vec<String> = entry
            .get("under")
            .and_then(toml::Value::as_array)
            .map(|shapes| {
                shapes
                    .iter()
                    .filter_map(|shape| shape.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        out.insert(name.clone(), under);
    }
    out
}

/// The name the record's reading gives a landmark of a reader's: the list's name for it. A
/// record of hand-made views is of a head that faces the viewer's right.
fn known_name(view: View, landmark: &str) -> String {
    list_names(view, landmark).swap_remove(0)
}

/// One view of a record of known positions.
struct Known {
    view: View,
    what: String,
    sha256: String,
    /// Hair top to chin, in the record's units.
    head_height: f64,
    points: BTreeMap<String, [f64; 2]>,
    provenance: Provenance,
    covers: BTreeMap<String, Vec<String>>,
}

impl Known {
    fn read(record_path: &str, record_text: &str, view: View) -> Result<Known> {
        // A fault in the record is reported under the record's own path.
        let hand_made = hand_made_views::read(record_path, record_text, None)?;
        let table: toml::Table = record_text
            .parse()
            .map_err(|error| Error::new(format!("{record_path}: not valid TOML: {error}")))?;
        let (known, provenance, sha256) = match view {
            View::Front => (
                hand_made.front,
                hand_made.front_record,
                hand_made.front_sha256,
            ),
            View::ThreeQuarter => (
                hand_made.three_quarter,
                hand_made.three_quarter_record,
                hand_made.three_quarter_sha256,
            ),
            View::Side => {
                return Err(Error::new(
                    "no hand-made side view exists, so a side view has no known positions",
                ));
            }
        };
        let head_height = match (known.points.get("chin"), known.points.get("hair-top")) {
            (Some(chin), Some(hair_top)) if chin[1] > hair_top[1] => chin[1] - hair_top[1],
            _ => {
                return Err(Error::new(format!(
                    "{record_path}: the record's {} view has no hair top above a chin, so there is no head height",
                    view.name()
                )));
            }
        };
        Ok(Known {
            view,
            what: hand_made.what,
            sha256,
            head_height,
            points: known.points,
            provenance,
            covers: covers(&table, view),
        })
    }

    /// A landmark's known position, under a reader's name for it, and the shapes that cover
    /// it in the drawing.
    fn at(&self, landmark: &str) -> Option<([f64; 2], Vec<String>)> {
        let name = known_name(self.view, landmark);
        let at = self
            .points
            .get(&name)
            .or_else(|| self.provenance.covered_by_head.get(&name))?;
        let mut under: Vec<String> = self
            .provenance
            .read_from
            .get(&name)
            .into_iter()
            .flatten()
            .flat_map(|recorded| self.covers.get(recorded).cloned().unwrap_or_default())
            .collect();
        under.sort();
        under.dedup();
        Some((*at, under))
    }
}

/// The record's own positions written as a reading: every landmark a reader is asked for
/// that the record gives and nothing covers, placed where the record puts it, and the rest
/// stated as not seen. It is not a reading. It is what a reader who placed every visible
/// point exactly would write, so that what the measuring commands make of it can be told
/// from what a reader's error adds.
pub fn as_reading(
    record_path: &str,
    record_text: &str,
    view: View,
    picture: &crate::readings::Picture,
) -> Result<String> {
    let known = Known::read(record_path, record_text, view)?;
    let faces = (view != View::Front).then_some(Faces::ViewersRight);
    let said = "the record's, not a reader's";
    let mut out = String::from(
        "\
# Not a reading: the positions of a record of known positions, written as a reading by
# `maquette known-positions-as-reading`. Nobody read them from a picture. It shows what the
# measuring commands make of exact positions under a reader's names.
",
    );
    out.push_str(&format!(
        "instructions = {}\n",
        quoted(crate::readings::instructions_revision()?)
    ));
    out.push_str(&format!(
        "\n[reader]\nname = \"the record of known positions\"\naids = {}\n",
        quoted(&format!("none: copied from {record_path}"))
    ));
    out.push_str("\n[image]\n");
    out.push_str(&format!("file = {}\n", quoted(&picture.file)));
    out.push_str(&format!("sha256 = {}\n", quoted(&picture.sha256)));
    out.push_str(&format!(
        "width = {}\nheight = {}\n",
        picture.width, picture.height
    ));
    out.push_str("\n[reference]\n");
    // Every field a reading gives of its reference: what the record is, and the rest as said.
    for (field, _) in REFERENCE_FIELDS {
        let value = if *field == "work" { &known.what } else { said };
        out.push_str(&format!("{field} = {}\n", quoted(value)));
    }
    out.push_str("\n[judgement]\n");
    out.push_str(&format!("view = {}\n", quoted(view.name())));
    out.push_str(&format!("view_why = {}\n", quoted(said)));
    if let Some(faces) = faces {
        out.push_str(&format!("faces = {}\n", quoted(faces.words())));
        out.push_str(&format!("turn_why = {}\n", quoted(said)));
    }
    out.push_str(&format!(
        "turn_degrees = {}\n",
        toml_number(view.turn_degrees(), 3)
    ));
    out.push_str(&format!(
        "level = true\nlevel_why = {}\nset_aside = false\nset_aside_why = \"\"\n",
        quoted(said)
    ));
    let mut not_seen = String::new();
    out.push_str("\n[landmarks]\n");
    for asked in crate::readings::asked(view, faces) {
        match known.at(&asked.name) {
            Some((at, under)) if under.is_empty() => {
                out.push_str(&format!("{} = {}\n", asked.name, position(at)));
            }
            Some((_, under)) => not_seen.push_str(&format!(
                "{} = {}\n",
                asked.name,
                quoted(&format!(
                    "covered in the drawing by {}",
                    under.join(" and ")
                ))
            )),
            None => not_seen.push_str(&format!(
                "{} = \"the record gives no position\"\n",
                asked.name
            )),
        }
    }
    out.push_str("\n[not_seen]\n");
    out.push_str(&not_seen);
    Ok(out)
}

/// Scores a reading against the record of a drawing with known positions.
pub fn score(
    reading_path: &str,
    reading: &Record,
    record_path: &str,
    record_text: &str,
) -> Result<Score> {
    if reading.faces == Some(Faces::ViewersLeft) {
        return Err(Error::new(
            "the reading is of a head that faces the viewer's left, and a record of hand-made views is of a head that faces the viewer's right",
        ));
    }
    let known = Known::read(record_path, record_text, reading.view)?;
    let head_height = known.head_height;
    let known_at = |landmark: &str| known.at(landmark);

    let asked = reading.asked();
    let kind_of = |landmark: &str| -> &'static str {
        asked
            .iter()
            .find(|asked| asked.name == landmark)
            .map(|asked| asked.kind)
            .unwrap_or("not asked for")
    };
    let mut score = Score {
        reading: reading_path.to_string(),
        record: record_path.to_string(),
        what: known.what.clone(),
        record_sha256: known.sha256.clone(),
        reader: reading.reader.clone(),
        picture: reading.picture.file.clone(),
        picture_sha256: reading.picture.sha256.clone(),
        instructions: reading.instructions.clone(),
        aids: reading.aids.clone(),
        view: reading.view,
        head_height,
        scored: Vec::new(),
        placed_though_covered: Vec::new(),
        not_seen_though_visible: Vec::new(),
        not_seen_and_covered: Vec::new(),
        not_seen_and_unknown: Vec::new(),
        no_known_position: Vec::new(),
        not_read: Vec::new(),
    };
    for (landmark, read) in &reading.placed {
        let Some((known, under)) = known_at(landmark) else {
            score.no_known_position.push(landmark.clone());
            continue;
        };
        let marks = compared_in(landmark);
        let distance = marks.distance(*read, known);
        let scored = Scored {
            landmark: landmark.clone(),
            kind: kind_of(landmark),
            marks,
            read: *read,
            known,
            distance,
            percent_of_head_height: distance / head_height * 100.0,
            under,
        };
        if scored.under.is_empty() {
            score.scored.push(scored);
        } else {
            score.placed_though_covered.push(scored);
        }
    }
    for landmark in reading.not_seen.keys() {
        match known_at(landmark) {
            Some((_, under)) if under.is_empty() => {
                score.not_seen_though_visible.push(landmark.clone());
            }
            Some(_) => score.not_seen_and_covered.push(landmark.clone()),
            None => score.not_seen_and_unknown.push(landmark.clone()),
        }
    }
    for asked in &asked {
        let read =
            reading.placed.contains_key(&asked.name) || reading.not_seen.contains_key(&asked.name);
        if !read && known_at(&asked.name).is_some_and(|(_, under)| under.is_empty()) {
            score.not_read.push(asked.name.clone());
        }
    }
    let worst_first = |a: &Scored, b: &Scored| {
        b.distance
            .total_cmp(&a.distance)
            .then_with(|| a.landmark.cmp(&b.landmark))
    };
    score.scored.sort_by(worst_first);
    score.placed_though_covered.sort_by(worst_first);
    Ok(score)
}

/// The figures of a group of scored landmarks.
struct Figures {
    count: usize,
    mean: f64,
    median: f64,
    ninetieth: f64,
    worst: f64,
    worst_landmark: String,
    above_1: usize,
    above_2: usize,
}

fn figures(scored: &[&Scored], head_height: f64) -> Option<Figures> {
    // Worst first, as given.
    let worst = scored.first()?;
    let mut shares: Vec<f64> = scored
        .iter()
        .map(|scored| scored.percent_of_head_height)
        .collect();
    shares.sort_by(f64::total_cmp);
    let count = shares.len();
    // The smallest share that at least nine in ten are at or under.
    let ninetieth = shares[((count * 9).div_ceil(10)).clamp(1, count) - 1];
    let above = |percent: f64| {
        scored
            .iter()
            .filter(|scored| scored.distance * 100.0 > percent * head_height)
            .count()
    };
    Some(Figures {
        count,
        mean: shares.iter().sum::<f64>() / count as f64,
        median: median(&shares),
        ninetieth,
        worst: worst.percent_of_head_height,
        worst_landmark: worst.landmark.clone(),
        above_1: above(THRESHOLD_PERCENT),
        above_2: above(2.0 * THRESHOLD_PERCENT),
    })
}

fn figure_lines(figures: &Figures) -> String {
    let mut out = String::new();
    out.push_str(&format!("scored = {}\n", figures.count));
    for (name, value) in [
        ("mean", figures.mean),
        ("median", figures.median),
        ("ninetieth_percentile", figures.ninetieth),
        ("worst", figures.worst),
    ] {
        out.push_str(&format!(
            "{name}_percent_of_head_height = {}\n",
            toml_number(value, 3)
        ));
    }
    out.push_str(&format!("worst = {}\n", quoted(&figures.worst_landmark)));
    out.push_str(&format!("above_1_percent = {}\n", figures.above_1));
    out.push_str(&format!("above_2_percent = {}\n", figures.above_2));
    out
}

impl Score {
    /// The kinds of landmark that were scored, in the order a reading asks for them.
    fn kinds(&self) -> Vec<&'static str> {
        let mut kinds: Vec<&'static str> = Vec::new();
        for asked in crate::readings::asked(self.view, Some(Faces::ViewersRight)) {
            let scored = self.scored.iter().any(|scored| scored.kind == asked.kind);
            if scored && !kinds.contains(&asked.kind) {
                kinds.push(asked.kind);
            }
        }
        kinds
    }

    fn of_kind(&self, kind: &str) -> Vec<&Scored> {
        self.scored
            .iter()
            .filter(|scored| scored.kind == kind)
            .collect()
    }

    /// The score as the text of its file.
    pub fn file(&self) -> String {
        let mut out = String::from(
            "\
# A reading scored against a drawing whose true positions are on record, by
# `maquette score-reading`: how far the reader put each landmark from its known position, as a
# share of head height, by kind of landmark.
# What it shows holds for this drawing and these reading instructions, and no further.
# It is worked out from a reading and is never committed.
",
        );
        out.push_str("\n[reading]\n");
        out.push_str(&format!("file = {}\n", quoted(&self.reading)));
        out.push_str(&format!("reader = {}\n", quoted(&self.reader)));
        out.push_str(&format!("instructions = {}\n", quoted(&self.instructions)));
        out.push_str(&format!("aids = {}\n", quoted(&self.aids)));
        out.push_str(&format!("view = {}\n", quoted(self.view.name())));
        out.push_str(&format!("picture = {}\n", quoted(&self.picture)));
        out.push_str(&format!(
            "picture_sha256 = {}\n",
            quoted(&self.picture_sha256)
        ));
        out.push_str("\n[known_positions]\n");
        out.push_str(&format!("record = {}\n", quoted(&self.record)));
        out.push_str(&format!("what = {}\n", quoted(&self.what)));
        out.push_str(&format!(
            "drawing_sha256 = {}\n",
            quoted(&self.record_sha256)
        ));
        out.push_str("\n[yardstick]\n");
        out.push_str(&format!(
            "head_height = {}  # the record's, in this view\n",
            toml_number(self.head_height, 3)
        ));

        out.push_str("\n# Over every landmark that is placed, can be seen in the drawing and has a known position.\n[summary]\n");
        let all: Vec<&Scored> = self.scored.iter().collect();
        match figures(&all, self.head_height) {
            Some(figures) => out.push_str(&figure_lines(&figures)),
            None => out.push_str("scored = 0\n"),
        }
        out.push_str(&format!(
            "placed_though_covered = {}\n",
            self.placed_though_covered.len()
        ));
        out.push_str(&format!(
            "not_seen_though_nothing_covers_it = {}\n",
            self.not_seen_though_visible.len()
        ));
        out.push_str(&format!(
            "neither_placed_nor_stated_as_not_seen = {}\n",
            self.not_read.len()
        ));

        out.push_str("\n# By kind of landmark.\n");
        for kind in self.kinds() {
            let of_kind = self.of_kind(kind);
            let Some(figures) = figures(&of_kind, self.head_height) else {
                continue;
            };
            out.push_str("[[kind]]\n");
            out.push_str(&format!("kind = {}\n", quoted(kind)));
            out.push_str(&figure_lines(&figures));
        }

        out.push_str("\n# What the reader stated as not seen.\n[not_seen]\n");
        out.push_str(&format!(
            "though_nothing_covers_it = {}\n",
            quoted_list(&self.not_seen_though_visible)
        ));
        out.push_str(&format!(
            "and_covered_in_the_drawing = {}  # rightly left out\n",
            quoted_list(&self.not_seen_and_covered)
        ));
        out.push_str(&format!(
            "and_not_in_the_record = {}\n",
            quoted_list(&self.not_seen_and_unknown)
        ));
        out.push_str(&format!(
            "\n[not_scored]\n# Placed, and the record gives no position.\nno_known_position = {}\n",
            quoted_list(&self.no_known_position)
        ));
        out.push_str(&format!(
            "# Known and visible, and the reading neither places it nor states it as not seen.\nneither_placed_nor_stated_as_not_seen = {}\n",
            quoted_list(&self.not_read)
        ));

        for (list, entries, heading) in [
            (
                "landmark",
                &self.scored,
                "Every scored landmark, worst first.",
            ),
            (
                "placed_though_covered",
                &self.placed_though_covered,
                "Placed though a shape covers the position in the drawing. These score nothing.",
            ),
        ] {
            if !entries.is_empty() {
                out.push_str(&format!("\n# {heading}\n"));
            }
            for scored in entries {
                out.push_str(&format!("[[{list}]]\n"));
                out.push_str(&format!("landmark = {}\n", quoted(&scored.landmark)));
                out.push_str(&format!("kind = {}\n", quoted(scored.kind)));
                out.push_str(&format!("scored_in = {}\n", quoted(scored.marks.word())));
                out.push_str(&format!("read = {}\n", position(scored.read)));
                out.push_str(&format!("known = {}\n", position(scored.known)));
                if !scored.under.is_empty() {
                    out.push_str(&format!("under = {}\n", quoted_list(&scored.under)));
                }
                out.push_str(&format!("distance = {}\n", toml_number(scored.distance, 3)));
                out.push_str(&format!(
                    "percent_of_head_height = {}\n",
                    toml_number(scored.percent_of_head_height, 3)
                ));
            }
        }
        out.push_str("\n# The rules behind these numbers.\n[rules]\n");
        for (name, rule) in RULES {
            out.push_str(&format!("{name} = {}\n", quoted(rule)));
        }
        out
    }

    /// The score in a few lines, for a person.
    pub fn summary(&self) -> String {
        let plain = crate::number::plain;
        let mut out = format!(
            "The {} reading of `{}` by {}, against {}.\n",
            self.view.name(),
            self.picture,
            self.reader,
            self.record
        );
        let all: Vec<&Scored> = self.scored.iter().collect();
        let Some(overall) = figures(&all, self.head_height) else {
            out.push_str("No placed landmark has a known position that can be seen.\n");
            return out;
        };
        out.push_str(&format!(
            "{} landmarks scored: mean {}% of head height, median {}%, 90th percentile {}%, worst {}% ({}); {} above 1%, {} above 2%.\n",
            overall.count,
            plain(overall.mean, 3),
            plain(overall.median, 3),
            plain(overall.ninetieth, 3),
            plain(overall.worst, 3),
            overall.worst_landmark,
            overall.above_1,
            overall.above_2
        ));
        for kind in self.kinds() {
            if let Some(figures) = figures(&self.of_kind(kind), self.head_height) {
                out.push_str(&format!(
                    "  {kind}: {} scored, mean {}%, worst {}% ({}), {} above 1%\n",
                    figures.count,
                    plain(figures.mean, 3),
                    plain(figures.worst, 3),
                    figures.worst_landmark,
                    figures.above_1
                ));
            }
        }
        for (what, names) in [
            (
                "Stated as not seen though nothing covers it",
                &self.not_seen_though_visible,
            ),
            (
                "Stated as not seen, and covered in the drawing",
                &self.not_seen_and_covered,
            ),
            ("Placed, with no known position", &self.no_known_position),
            ("Neither placed nor stated as not seen", &self.not_read),
        ] {
            if !names.is_empty() {
                out.push_str(&format!("{what}: {}.\n", names.join(", ")));
            }
        }
        if !self.placed_though_covered.is_empty() {
            let names: Vec<String> = self
                .placed_though_covered
                .iter()
                .map(|scored| {
                    format!(
                        "{} ({}%)",
                        scored.landmark,
                        plain(scored.percent_of_head_height, 3)
                    )
                })
                .collect();
            out.push_str(&format!(
                "Placed though covered in the drawing, and not scored: {}.\n",
                names.join(", ")
            ));
        }
        out
    }
}

// ---------------------------------------------------------------- two sets of proportions

/// The `[proportions]` of a file, which need not be a whole set.
fn values_of(path: &str, text: &str) -> Result<BTreeMap<String, f64>> {
    let table: toml::Table = text
        .parse()
        .map_err(|error| Error::new(format!("{path}: not valid TOML: {error}")))?;
    let entries = table
        .get("proportions")
        .and_then(toml::Value::as_table)
        .ok_or_else(|| Error::new(format!("{path} has no [proportions] table")))?;
    let mut values = BTreeMap::new();
    for (name, value) in entries {
        let number = match value {
            toml::Value::Float(number) => *number,
            toml::Value::Integer(number) => *number as f64,
            _ => {
                return Err(Error::new(format!(
                    "{path}: the proportion `{name}` is not a number"
                )));
            }
        };
        values.insert(name.clone(), number);
    }
    Ok(values)
}

/// What a proportion is, by its name: a height, a width across the head or a depth.
fn dimension(name: &str) -> &'static str {
    if name.contains("_depth") {
        "depths"
    } else if name.ends_with("_rise") || name.ends_with("_drop") || name.ends_with("_height") {
        "heights"
    } else {
        "widths"
    }
}

/// Sets two files of proportions against each other, proportion by proportion, as text. Each
/// difference is the first minus the second, in units, and as a share of the second's head
/// height. A file that lacks some proportions is compared in the ones it has.
pub fn compare_proportions(
    first_path: &str,
    first_text: &str,
    second_path: &str,
    second_text: &str,
) -> Result<String> {
    let first = values_of(first_path, first_text)?;
    let second = values_of(second_path, second_text)?;
    let head_height = second
        .get("hair_top_rise")
        .or(first.get("hair_top_rise"))
        .map(|rise| head_model::head_height(*rise))
        .ok_or_else(|| {
            Error::new("neither file gives `hair_top_rise`, so there is no head height to state a share of")
        })?;
    let list = proportions::list()?;

    struct Line {
        name: String,
        group: &'static str,
        convention: bool,
        first: f64,
        second: f64,
    }
    let mut lines = Vec::new();
    let (mut only_first, mut only_second) = (Vec::new(), Vec::new());
    for proportion in &list.proportions {
        match (first.get(&proportion.name), second.get(&proportion.name)) {
            (Some(first), Some(second)) => {
                let convention = proportion.kind == Kind::Convention;
                lines.push(Line {
                    name: proportion.name.clone(),
                    group: if convention {
                        "conventions"
                    } else {
                        dimension(&proportion.name)
                    },
                    convention,
                    first: *first,
                    second: *second,
                });
            }
            (Some(_), None) => only_first.push(proportion.name.clone()),
            (None, Some(_)) => only_second.push(proportion.name.clone()),
            (None, None) => {}
        }
    }
    let share = |line: &Line| (line.first - line.second).abs() / head_height * 100.0;
    let above =
        |line: &Line| (line.first - line.second).abs() * 100.0 > THRESHOLD_PERCENT * head_height;

    let mut out = String::from(
        "\
# Two sets of proportions set against each other, by `maquette compare-proportions`.
# A difference is the first minus the second, in units, eye line to chin. Its share is of
# the second's head height, hair top to chin. A depth taken through a turn near 30 degrees
# carries twice the reading error of a position, so depths are summed up apart.
",
    );
    out.push_str(&format!("first = {}\n", quoted(first_path)));
    out.push_str(&format!("second = {}\n", quoted(second_path)));
    out.push_str(&format!("head_height = {}\n", toml_number(head_height, 6)));
    out.push_str(&format!("compared = {}\n", lines.len()));
    out.push_str(&format!("only_in_first = {}\n", quoted_list(&only_first)));
    out.push_str(&format!("only_in_second = {}\n", quoted_list(&only_second)));
    for group in ["heights", "widths", "depths", "conventions"] {
        let of_group: Vec<&Line> = lines.iter().filter(|line| line.group == group).collect();
        let Some(largest) = of_group.iter().max_by(|a, b| share(a).total_cmp(&share(b))) else {
            continue;
        };
        out.push_str(&format!("\n[summary.{group}]\n"));
        out.push_str(&format!("compared = {}\n", of_group.len()));
        out.push_str(&format!(
            "mean_percent_of_head_height = {}\n",
            toml_number(
                of_group.iter().map(|line| share(line)).sum::<f64>() / of_group.len() as f64,
                4
            )
        ));
        out.push_str(&format!(
            "largest_percent_of_head_height = {}\n",
            toml_number(share(largest), 4)
        ));
        out.push_str(&format!("largest = {}\n", quoted(&largest.name)));
        out.push_str(&format!(
            "above_1_percent = {}\n",
            of_group.iter().filter(|line| above(line)).count()
        ));
    }
    out.push_str("\n# Every proportion both files give, in the list's order.\n");
    for line in &lines {
        out.push_str("[[proportion]]\n");
        out.push_str(&format!("name = {}\n", quoted(&line.name)));
        out.push_str(&format!(
            "kind = {}\n",
            quoted(if line.convention {
                "convention"
            } else {
                "measured"
            })
        ));
        out.push_str(&format!("first = {}\n", toml_number(line.first, 6)));
        out.push_str(&format!("second = {}\n", toml_number(line.second, 6)));
        out.push_str(&format!(
            "difference = {}\n",
            toml_number(line.first - line.second, 6)
        ));
        out.push_str(&format!(
            "percent_of_head_height = {}\n",
            toml_number(share(line), 4)
        ));
        out.push_str(&format!("above_1_percent = {}\n", above(line)));
    }
    Ok(out)
}
