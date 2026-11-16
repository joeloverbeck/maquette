//! The hand-made views drawn over a generated armature, for a review sheet: for each view, an
//! SVG in the generated view's own units that marks every position the record gives and
//! numbers every difference above 1% of head height, with the numbers the differences file
//! gives them.
//!
//! The comparison decides everything: which differences there are, their numbers and the
//! lay-over. This module only undoes the lay-over, so that a record position stands where it
//! falls on the generated SVG, and decides how the marks look and where a number stands.
//! Placed at the same place and size as the generated SVG, the overlay lies over it exactly.

use crate::armature::{self, File};
use crate::differences::{self, Differences, Input, ViewDifferences};
use crate::hand_made_views::{self, Provenance};
use crate::number::{plain, plain3};
use crate::{Error, Result};

/// The hand-made positions: a warm colour the generated guides never use.
const HAND_MADE: &str = "#d9622b";
/// The line between a generated and a hand-made position, and a number's leader.
const APART: &str = "#b4511f";

/// How the marks are sized, in units: eye line to chin. Sizing by the unit keeps them the
/// same on the page wherever and however large the view is drawn.
const MARK_RADIUS: f64 = 0.025;
const COVERED_RADIUS: f64 = 0.045;
const BADGE_RADIUS: f64 = 0.085;
const BADGE_FONT: f64 = 0.095;
/// How far a number first stands from its hand-made position, and how far it moves out each
/// time it would touch another number or leave the view.
const BADGE_DISTANCE: f64 = 0.24;
const BADGE_STEP: f64 = 0.04;
const BADGE_GAP: f64 = 0.02;
const BADGE_TRIES: usize = 60;

/// The file name of a view's overlay.
pub fn file_name(view: &str) -> String {
    format!("{view}.hand-made-overlay.svg")
}

/// Writes the overlays of a generated front view and three-quarter view: one SVG for each,
/// in that view's units, over the record of a hand-made armature's views.
pub fn draw(front: Input, three_quarter: Input, record: Input) -> Result<Vec<File>> {
    let found = differences::compare(front, three_quarter, record)?;
    let hand_made = hand_made_views::read(record.text)
        .map_err(|error| Error::new(format!("{}: {error}", record.path)))?;
    Ok(vec![
        view_overlay(&found, &found.front, front, &hand_made.front_record)?,
        view_overlay(
            &found,
            &found.three_quarter,
            three_quarter,
            &hand_made.three_quarter_record,
        )?,
    ])
}

/// The size the landmark file gives its view.
fn view_size(text: &str) -> Result<[f64; 2]> {
    let table: toml::Table = text
        .parse()
        .map_err(|error| Error::new(format!("not valid TOML: {error}")))?;
    let number = |key: &str| -> Result<f64> {
        match table.get("frame").and_then(|frame| frame.get(key)) {
            Some(toml::Value::Float(number)) => Ok(*number),
            Some(toml::Value::Integer(number)) => Ok(*number as f64),
            _ => Err(Error::new(format!("its [frame] gives no number `{key}`"))),
        }
    };
    Ok([number("width")?, number("height")?])
}

fn view_overlay(
    found: &Differences,
    view: &ViewDifferences,
    file: Input,
    provenance: &Provenance,
) -> Result<File> {
    let in_file = |error: Error| Error::new(format!("{}: {error}", file.path));
    let [width, height] = view_size(file.text).map_err(in_file)?;
    let generated = armature::read_landmark_file(file.text).map_err(in_file)?;
    let lay_over = &view.lay_over;
    let unit = lay_over.generated.unit;
    // A record position, brought back into the view's units.
    let back = |at: [f64; 2]| {
        [
            (at[0] - lay_over.shift_x) / lay_over.scale,
            (at[1] - lay_over.shift_y) / lay_over.scale,
        ]
    };
    let centre = generated
        .points
        .get("skull-centre")
        .copied()
        .ok_or_else(|| Error::new(format!("{}: no `skull-centre`", file.path)))?;

    let mut svg = String::new();
    svg.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\">\n",
        w = plain3(width),
        h = plain3(height)
    ));
    svg.push_str(&format!(
        "  <!-- The hand-made {view} view of {record}, over the generated view of {file}, written by\n       `maquette overlay-hand-made-views`. Placed at the same place and size as that view's SVG,\n       it lies over it exactly. A record position x, y stands here at\n       (x - {sx}) / {scale}, (y - {sy}) / {scale}: the comparison's lay-over, undone.\n       Filled dots: every position the record gives the view. A dashed ring: a position the\n       head covers in the hand-made drawing. A numbered line: a difference above 1% of head\n       height, from the generated position to the hand-made one, numbered as the\n       differences file numbers it. -->\n",
        view = view.view,
        record = found.record,
        file = file.path,
        sx = plain3(lay_over.shift_x),
        sy = plain3(lay_over.shift_y),
        scale = plain(lay_over.scale, 6),
    ));

    svg.push_str(&format!(
        "  <g id=\"hand-made-landmarks\" fill=\"{HAND_MADE}\" stroke=\"none\">\n"
    ));
    for (name, position) in &provenance.positions {
        let [x, y] = back(position.at);
        if position.covered_by_head {
            svg.push_str(&format!(
                "    <circle id=\"hand-made-{name}\" class=\"covered\" cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"{HAND_MADE}\" stroke-width=\"{}\" stroke-dasharray=\"{} {}\"/>\n",
                plain3(x),
                plain3(y),
                plain3(COVERED_RADIUS * unit),
                plain3(0.0125 * unit),
                plain3(0.02 * unit),
                plain3(0.015 * unit),
            ));
        } else {
            svg.push_str(&format!(
                "    <circle id=\"hand-made-{name}\" cx=\"{}\" cy=\"{}\" r=\"{}\"/>\n",
                plain3(x),
                plain3(y),
                plain3(MARK_RADIUS * unit)
            ));
        }
    }
    svg.push_str("  </g>\n");

    svg.push_str("  <g id=\"differences\">\n");
    let radius = BADGE_RADIUS * unit;
    let mut badges: Vec<[f64; 2]> = Vec::new();
    for difference in &view.above {
        let label = difference
            .number
            .ok_or_else(|| Error::new("a difference above 1% has no number"))?;
        let from = generated
            .points
            .get(&difference.landmark)
            .copied()
            .ok_or_else(|| {
                Error::new(format!(
                    "{}: no `{}`, which the comparison lists",
                    file.path, difference.landmark
                ))
            })?;
        let to = back(difference.hand_made);
        let badge = place_badge(to, centre, radius, unit, [width, height], &badges);
        badges.push(badge);
        let stroke = plain3(0.01 * unit);
        svg.push_str(&format!(
            "    <g id=\"difference-{label}\" class=\"difference\">\n"
        ));
        svg.push_str(&format!(
            "      <line class=\"leader\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{APART}\" stroke-width=\"{stroke}\"/>\n",
            plain3(to[0]),
            plain3(to[1]),
            plain3(badge[0]),
            plain3(badge[1]),
        ));
        svg.push_str(&format!(
            "      <line class=\"apart\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{APART}\" stroke-width=\"{}\" stroke-linecap=\"round\"/>\n",
            plain3(from[0]),
            plain3(from[1]),
            plain3(to[0]),
            plain3(to[1]),
            plain3(0.02 * unit),
        ));
        svg.push_str(&format!(
            "      <circle class=\"badge\" cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{HAND_MADE}\" stroke=\"#ffffff\" stroke-width=\"{stroke}\"/>\n",
            plain3(badge[0]),
            plain3(badge[1]),
            plain3(radius),
        ));
        let font = BADGE_FONT * unit;
        svg.push_str(&format!(
            "      <text class=\"number\" x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"Inter\" font-weight=\"700\" font-size=\"{}\" fill=\"#ffffff\">{label}</text>\n",
            plain3(badge[0]),
            plain3(badge[1] + 0.35 * font),
            plain3(font),
        ));
        svg.push_str("    </g>\n");
    }
    svg.push_str("  </g>\n</svg>\n");

    Ok(File {
        name: file_name(view.view),
        text: svg,
    })
}

/// Where a number stands: out from the skull's centre beyond its hand-made position, moved
/// further out until it touches no number placed before it and stays inside the view.
fn place_badge(
    at: [f64; 2],
    centre: [f64; 2],
    radius: f64,
    unit: f64,
    size: [f64; 2],
    placed: &[[f64; 2]],
) -> [f64; 2] {
    let (dx, dy) = (at[0] - centre[0], at[1] - centre[1]);
    let length = dx.hypot(dy);
    let direction = if length > 0.0 {
        [dx / length, dy / length]
    } else {
        [0.0, -1.0]
    };
    let inside = |p: [f64; 2]| {
        [
            p[0].clamp(radius, size[0] - radius),
            p[1].clamp(radius, size[1] - radius),
        ]
    };
    let clear = |p: [f64; 2]| {
        placed.iter().all(|other| {
            (p[0] - other[0]).hypot(p[1] - other[1]) >= 2.0 * radius + BADGE_GAP * unit
        })
    };
    let mut fallback = None;
    for step in 0..BADGE_TRIES {
        let distance = (BADGE_DISTANCE + BADGE_STEP * step as f64) * unit;
        let candidate = inside([
            at[0] + direction[0] * distance,
            at[1] + direction[1] * distance,
        ]);
        if clear(candidate) {
            return candidate;
        }
        fallback.get_or_insert(candidate);
        // Out along the line and held inside the view, try beside it too.
        let side = [-direction[1], direction[0]];
        for sign in [1.0, -1.0] {
            let beside = inside([
                candidate[0] + sign * side[0] * 2.2 * radius,
                candidate[1] + sign * side[1] * 2.2 * radius,
            ]);
            if clear(beside) {
                return beside;
            }
        }
    }
    fallback.unwrap_or(at)
}
