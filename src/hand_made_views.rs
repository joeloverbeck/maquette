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

fn read_view(record: &toml::Table, view: &str, turned: bool) -> Result<(Reading, String)> {
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

    // A position the head covers in the turned drawing cannot be seen there. The skull's own
    // points are never under the skull, so they are kept.
    let seen = |name: &str| -> Option<[f64; 2]> {
        recorded
            .get(name)
            .filter(|point| !(turned && point.covered_by_head))
            .map(|point| point.position)
    };
    let midway = |first: &str, second: &str| -> Option<[f64; 2]> {
        let (first, second) = (seen(first)?, seen(second)?);
        Some([(first[0] + second[0]) / 2.0, (first[1] + second[1]) / 2.0])
    };

    let mut reading = Reading {
        turn_degrees: if turned {
            THREE_QUARTER_TURN_DEGREES
        } else {
            0.0
        },
        ..Reading::default()
    };
    let mut put = |name: String, position: Option<[f64; 2]>| {
        if let Some(position) = position {
            reading.points.insert(name, position);
        }
    };

    for name in ON_CENTRE_LINE {
        put(hyphens(name), seen(name));
    }
    for side in SIDES {
        for (stem, point) in PAIRED {
            let name = format!("{stem}_{side}_{point}");
            put(hyphens(&name), seen(&name));
        }
        for point in EAR {
            let name = format!("ear_{side}_{point}");
            put(hyphens(&name), seen(&name));
        }

        let lid = |point: &str| format!("lid_line_{side}_{point}");
        put(
            format!("lid-line-{side}-inner-end"),
            midway(&lid("inner_lower"), &lid("inner_upper")),
        );
        put(
            format!("lid-line-{side}-crest"),
            midway(&lid("lower_mid"), &lid("upper_mid")),
        );
        put(
            format!("lid-line-{side}-outer-end"),
            midway(&lid("outer_lower"), &lid("outer_tip")),
        );
        for end in ["inner", "outer"] {
            put(
                format!("brow-line-{side}-{end}-end"),
                midway(
                    &format!("brow_{side}_{end}_top"),
                    &format!("brow_{side}_{end}_bottom"),
                ),
            );
        }

        // The face outline: both the cheek point and the jaw-line point in the front view;
        // the cheek point on the far side and the jaw-line point on the near side when turned.
        let outline = |point: &str| seen(&format!("face_outline_{side}_{point}"));
        let cheek = !turned || side == FAR;
        let jaw_line = !turned || side != FAR;
        let mut levels = vec![("top".to_string(), outline("node_1"))];
        for level in LEVELS {
            levels.push((
                format!("at-{level}-level"),
                outline(&format!("at_{level}_level")),
            ));
        }
        for (level, position) in levels {
            // The near side's first node in the turned view is where its drawn outline
            // starts, at the nose level: it is no top end.
            if turned && side != FAR && level == "top" {
                continue;
            }
            if cheek {
                put(format!("face-outline-{side}-{level}"), position);
            }
            if jaw_line {
                put(format!("jaw-line-{side}-{level}"), position);
            }
        }
        let jaw_node = if turned && side == FAR {
            "node_5"
        } else {
            "node_3"
        };
        put(format!("face-outline-{side}-jaw"), outline(jaw_node));
    }

    Ok((reading, sha256))
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
    let (front, front_sha256) = read_view(&record, "front", false)?;
    let (three_quarter, three_quarter_sha256) = read_view(&record, "three_quarter", true)?;
    Ok(HandMade {
        front,
        three_quarter,
        what,
        front_sha256,
        three_quarter_sha256,
    })
}
