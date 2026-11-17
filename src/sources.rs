//! Where the numbers of a drawn head come from, marked on its views for a review sheet: for
//! each view, an SVG in the view's own units with a dot on every point a guide passes through,
//! coloured by the source of the proportions that put the point there.
//!
//! This module hides which proportions a point of a view rests on, what a point is called
//! when its proportions come from more than one source, and how the dots look. The sources
//! themselves are the set's own: the `[sources]` of a set that `maquette set-from-table`
//! wrote. Placed at the same place and size as the view's SVG, the overlay lies over it
//! exactly. It marks and judges nothing.

use std::collections::BTreeMap;

use crate::armature::File;
use crate::head_model;
use crate::number::plain3;
use crate::proportions;
use crate::table::Source;
use crate::toml_text::words;
use crate::views::View;
use crate::{Error, Result};

/// The kinds of source, from the least supported to the most, each with the id of its group
/// in the overlay and its colour. A point takes the least supported kind among the
/// proportions that place it in the view: a point is no better measured than its weakest
/// number.
pub const KINDS: [(&str, &str, &str); 3] = [
    ("borrowed", "borrowed", "#d9622b"),
    ("a thin average", "thin-averages", "#d6a400"),
    ("the table", "from-the-table", "#1f9d55"),
];

/// The radius of a dot, in units: eye line to chin.
const DOT_RADIUS: f64 = 0.028;

/// The file name of a view's overlay.
pub fn file_name(view: View) -> String {
    format!("{}.sources-overlay.svg", view.name())
}

/// The proportions a view point rests on, by the start of its name: behind its sideways
/// place in a front view, behind its height, and behind its depth. A view point is a point of
/// a form that is no landmark: the hair's extents, the skull's, the iris's, the neck's base
/// and the ends of the shoulder line as a side view draws it. Each rests on its form's own
/// proportions. The points where a neck line starts and where a level line meets the outline
/// are read off other guides, and rest on nothing of their own: they get no dot.
const VIEW_POINTS: &[(&str, [&[&str]; 3])] = &[
    (
        "hair-",
        [
            &["hair_half_width"],
            &[],
            &["hair_half_depth", "hair_centre_depth"],
        ],
    ),
    (
        "skull-",
        [
            &["skull_radius"],
            &["skull_radius", "skull_centre_rise"],
            &["skull_radius", "skull_centre_depth"],
        ],
    ),
    (
        "iris-",
        [
            &["iris_centre_offset", "iris_width"],
            &["iris_centre_drop", "iris_height"],
            &["iris_depth"],
        ],
    ),
    (
        "shoulder-line-",
        [
            &["shoulder_half_width"],
            &["shoulder_drop"],
            &["shoulder_depth"],
        ],
    ),
];

/// The neck's base points, which every view names its own way: `neck-left-base`,
/// `neck-back-base`.
const NECK_BASE: [&[&str]; 3] = [&["neck_half_width"], &["neck_base_drop"], &["neck_depth"]];

/// The proportions a point of a view rests on, coordinate by coordinate, or none where it
/// rests on nothing of its own.
fn rests_on(point: &str) -> Option<[Vec<&'static str>; 3]> {
    if let Some(on) = head_model::rests_on(point) {
        return Some(on);
    }
    let of_form = if point.starts_with("neck-") && point.ends_with("-base") {
        Some(&NECK_BASE)
    } else {
        VIEW_POINTS
            .iter()
            .find(|(start, _)| point.starts_with(start))
            .map(|(_, on)| on)
    };
    of_form.map(|on| on.map(|names| names.to_vec()))
}

/// The dots of one view, with how many there are of each kind.
#[derive(Debug, Clone)]
pub struct Overlay {
    pub view: View,
    pub file: File,
    /// The number of dots of each kind, in the order of [`KINDS`].
    pub counts: [usize; 3],
}

fn number(table: &toml::Table, key: &str) -> Option<f64> {
    match table.get(key) {
        Some(toml::Value::Float(number)) => Some(*number),
        Some(toml::Value::Integer(number)) => Some(*number as f64),
        _ => None,
    }
}

/// The source of every proportion of the list, as a set's `[sources]` states it: the index of
/// its kind in [`KINDS`].
fn sources_of(path: &str, text: &str) -> Result<BTreeMap<String, usize>> {
    let file: toml::Table = text
        .parse()
        .map_err(|error| Error::new(format!("{path}: not valid TOML: {error}")))?;
    let stated = file
        .get("sources")
        .and_then(toml::Value::as_table)
        .ok_or_else(|| {
            Error::new(format!(
                "{path} has no [sources] table: it does not say where its proportions come from. `maquette set-from-table` writes a set that does"
            ))
        })?;
    let mut sources = BTreeMap::new();
    for proportion in &proportions::list()?.proportions {
        let said = words(stated, &proportion.name);
        // A proportion left out of the set places no point, so no dot stands for it.
        if proportion.optional() && Source::kind_of(&said) == Some("left out") {
            continue;
        }
        let kind = Source::kind_of(&said)
            .and_then(|kind| KINDS.iter().position(|(known, _, _)| *known == kind))
            .ok_or_else(|| {
                Error::new(format!(
                    "{path}: the source of `{}` is \"{said}\", which is not the table, a thin average or borrowed",
                    proportion.name
                ))
            })?;
        sources.insert(proportion.name.clone(), kind);
    }
    Ok(sources)
}

/// Draws the overlay of one view: `set` is the set of proportions the view was drawn from,
/// with its `[sources]`, and `landmarks` the view's landmark file.
pub fn overlay(
    set_path: &str,
    set_text: &str,
    landmarks_path: &str,
    landmarks_text: &str,
) -> Result<Overlay> {
    let sources = sources_of(set_path, set_text)?;
    let in_file = |why: String| Error::new(format!("{landmarks_path}: {why}"));
    let file: toml::Table = landmarks_text
        .parse()
        .map_err(|error| in_file(format!("not valid TOML: {error}")))?;
    let view =
        crate::readings::view_named(&words(&file, "view")).map_err(|error| in_file(error.0))?;
    let frame = file
        .get("frame")
        .and_then(toml::Value::as_table)
        .ok_or_else(|| in_file("it has no [frame]".to_string()))?;
    let of_frame = |key: &str| {
        number(frame, key).ok_or_else(|| in_file(format!("its [frame] gives no number `{key}`")))
    };
    let (width, height, unit) = (of_frame("width")?, of_frame("height")?, of_frame("unit")?);
    // Which coordinates the view shows: a front view no depth, a side view no offset.
    let shown: &[usize] = match view {
        View::Front => &[0, 1],
        View::ThreeQuarter => &[0, 1, 2],
        View::Side => &[1, 2],
    };

    // Every point a guide passes through, where the file places it.
    let mut placed: BTreeMap<&str, [f64; 2]> = BTreeMap::new();
    for key in ["landmarks", "view_points"] {
        for (name, point) in file
            .get(key)
            .and_then(toml::Value::as_table)
            .into_iter()
            .flatten()
        {
            if let Some(point) = point.as_table()
                && let (Some(x), Some(y)) = (number(point, "x"), number(point, "y"))
            {
                placed.insert(name.as_str(), [x, y]);
            }
        }
    }
    // One dot at a place: where two points stand at one place, the less supported kind.
    let mut dots: BTreeMap<(i64, i64), (usize, [f64; 2])> = BTreeMap::new();
    for guide in file
        .get("guides")
        .and_then(toml::Value::as_table)
        .into_iter()
        .flatten()
        .filter_map(|(_, guide)| guide.as_table())
    {
        for name in guide
            .get("passes_through")
            .and_then(toml::Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(toml::Value::as_str)
        {
            let (Some(at), Some(on)) = (placed.get(name), rests_on(name)) else {
                continue;
            };
            let Some(kind) = shown
                .iter()
                .flat_map(|coordinate| on[*coordinate].iter())
                .filter_map(|proportion| sources.get(*proportion))
                .min()
            else {
                continue;
            };
            let place = (
                (at[0] * 1000.0).round() as i64,
                (at[1] * 1000.0).round() as i64,
            );
            let dot = dots.entry(place).or_insert((*kind, *at));
            dot.0 = dot.0.min(*kind);
        }
    }

    let mut counts = [0; 3];
    let mut svg = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\">\n  <g id=\"sources\" stroke=\"#ffffff\" stroke-width=\"{}\">\n",
        plain3(DOT_RADIUS * unit / 4.0),
        w = plain3(width),
        h = plain3(height),
    );
    for (index, (_, group, colour)) in KINDS.iter().enumerate() {
        svg.push_str(&format!("    <g id=\"{group}\" fill=\"{colour}\">\n"));
        for (kind, at) in dots.values() {
            if *kind == index {
                counts[index] += 1;
                svg.push_str(&format!(
                    "      <circle cx=\"{}\" cy=\"{}\" r=\"{}\"/>\n",
                    plain3(at[0]),
                    plain3(at[1]),
                    plain3(DOT_RADIUS * unit)
                ));
            }
        }
        svg.push_str("    </g>\n");
    }
    svg.push_str("  </g>\n</svg>\n");
    Ok(Overlay {
        view,
        file: File {
            name: file_name(view),
            text: svg,
        },
        counts,
    })
}

impl Overlay {
    /// How many dots of each kind the view has, in a line for a person.
    pub fn summary(&self) -> String {
        format!(
            "{}: {} points rest on the table's middle values, {} on thin averages, {} on borrowed numbers.\n",
            self.view.name(),
            self.counts[2],
            self.counts[1],
            self.counts[0]
        )
    }
}
