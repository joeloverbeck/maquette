//! Writes an armature: for each view an SVG of named guides and a landmark file beside it.
//!
//! The SVG uses only what the SVG contract in `docs/foundation/02-first-table.md` shows to
//! survive in VectorCraft: plain `width` and `height` equal to a `viewBox` from `0 0`; names
//! in `id` alone; `path` of one subpath of `M`, `L`, `C` and `Z`, `line`, `polyline`, `circle`
//! and `ellipse`; a stroke with `fill="none"`; one top-level `g`, and nested `g`s that each
//! hold a guide. No transform, no arc, nothing hidden.
//!
//! The landmark file is TOML. Its readers are the agent that paints over the armature, the
//! oracle that compares VectorCraft's read-back with it, and the tests; TOML is what a person
//! and an agent read without a tool, it carries comments that say what a number is, and the
//! repository's other data is already TOML, so one reader serves all of it. Every position in
//! it is the same rounded number the SVG carries. The same input gives the same bytes.

use crate::guides::{self, Guide, Guides, Paint, Shape, Stretch};
use crate::head_model::{self, Reading};
use crate::number::{plain, toml as toml_number};
use crate::proportions::Set;
use crate::views::{self, Placement, Seen, View};
use crate::{Error, Result};

/// One file of an armature: its name and its text.
#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub name: String,
    pub text: String,
}

/// Draws a set of proportions: the SVG and the landmark file of every view.
pub fn draw(set: &Set, placement: &Placement) -> Result<Vec<File>> {
    let placement = placement.checked()?;
    let head = head_model::head(set)?;
    let mut files = Vec::new();
    for view in View::ALL {
        let seen = views::show(&head, view, &placement);
        let built = guides::build(&head, &seen);
        let svg_name = format!("{}.svg", view.name());
        files.push(File {
            name: svg_name.clone(),
            text: svg(&seen, &built),
        });
        files.push(File {
            name: format!("{}.landmarks.toml", view.name()),
            text: landmark_file(&seen, &built, &svg_name),
        });
    }
    Ok(files)
}

fn number(value: f64) -> String {
    plain(value, 3)
}

fn paint_attributes(paint: Paint) -> &'static str {
    match paint {
        Paint::Form => "",
        Paint::Heavy => " stroke-width=\"4\"",
        Paint::Construction => " stroke=\"#2f6fdd\" stroke-width=\"1.5\"",
        Paint::Envelope => " stroke=\"#8d93a6\" stroke-dasharray=\"10 7\"",
    }
}

fn element(guide: &Guide, built: &Guides) -> String {
    let id = &guide.name;
    let paint = paint_attributes(guide.paint);
    let at = |name: &String| built.position(name);
    match &guide.shape {
        Shape::Circle { centre, r } => {
            let [cx, cy] = at(centre);
            format!(
                "<circle id=\"{id}\" cx=\"{}\" cy=\"{}\" r=\"{}\"{paint}/>",
                number(cx),
                number(cy),
                number(*r)
            )
        }
        Shape::Ellipse { centre, rx, ry } => {
            let [cx, cy] = at(centre);
            format!(
                "<ellipse id=\"{id}\" cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\"{paint}/>",
                number(cx),
                number(cy),
                number(*rx),
                number(*ry)
            )
        }
        Shape::Line => {
            let ([x1, y1], [x2, y2]) = (at(&guide.anchors[0]), at(&guide.anchors[1]));
            format!(
                "<line id=\"{id}\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"{paint}/>",
                number(x1),
                number(y1),
                number(x2),
                number(y2)
            )
        }
        Shape::Polyline => {
            let points: Vec<String> = guide
                .anchors
                .iter()
                .map(|name| {
                    let [x, y] = at(name);
                    format!("{},{}", number(x), number(y))
                })
                .collect();
            format!(
                "<polyline id=\"{id}\" points=\"{}\"{paint}/>",
                points.join(" ")
            )
        }
        Shape::Path { stretches, closed } => {
            let [x, y] = at(&guide.anchors[0]);
            let mut d = format!("M{} {}", number(x), number(y));
            for (index, stretch) in stretches.iter().enumerate() {
                let [x, y] = at(&guide.anchors[(index + 1) % guide.anchors.len()]);
                match stretch {
                    Stretch::Straight => d.push_str(&format!(" L{} {}", number(x), number(y))),
                    Stretch::Curve { leave, arrive } => d.push_str(&format!(
                        " C{} {} {} {} {} {}",
                        number(leave[0]),
                        number(leave[1]),
                        number(arrive[0]),
                        number(arrive[1]),
                        number(x),
                        number(y)
                    )),
                }
            }
            if *closed {
                d.push_str(" Z");
            }
            format!("<path id=\"{id}\" d=\"{d}\"{paint}/>")
        }
    }
}

/// The top-level children of the layer, in drawing order: a guide, or a group with its guides.
fn layout(built: &Guides) -> Vec<(Option<&'static str>, Vec<&Guide>)> {
    let mut children: Vec<(Option<&'static str>, Vec<&Guide>)> = Vec::new();
    for guide in &built.guides {
        match guide.group {
            Some(group) => match children.iter_mut().find(|(name, _)| *name == Some(group)) {
                Some((_, members)) => members.push(guide),
                None => children.push((Some(group), vec![guide])),
            },
            None => children.push((None, vec![guide])),
        }
    }
    children
}

fn svg(seen: &Seen, built: &Guides) -> String {
    let (width, height) = (number(seen.width), number(seen.height));
    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\">\n"
    ));
    out.push_str(&format!(
        "  <g id=\"{}\" fill=\"none\" stroke=\"#2a2f3d\" stroke-width=\"2.5\" stroke-linecap=\"round\" stroke-linejoin=\"round\">\n",
        guides::LAYER
    ));
    for (group, members) in layout(built) {
        match group {
            Some(group) => {
                out.push_str(&format!("    <g id=\"{group}\">\n"));
                for guide in members {
                    out.push_str(&format!("      {}\n", element(guide, built)));
                }
                out.push_str("    </g>\n");
            }
            None => out.push_str(&format!("    {}\n", element(members[0], built))),
        }
    }
    out.push_str("  </g>\n</svg>\n");
    out
}

fn quoted_list(names: &[String]) -> String {
    let quoted: Vec<String> = names.iter().map(|name| format!("\"{name}\"")).collect();
    format!("[{}]", quoted.join(", "))
}

fn landmark_file(seen: &Seen, built: &Guides, svg_name: &str) -> String {
    let position = |value: f64| toml_number(value, 3);
    let mut out = String::new();
    out.push_str(
        "# Landmark file of one view of an armature, written by maquette beside the view's SVG.\n",
    );
    out.push_str("# Positions are in the SVG's own units: x grows toward the viewer's right, y grows downward.\n");
    out.push_str("# Left and right in a name are the character's own.\n\n");
    out.push_str(&format!("view = \"{}\"\n", seen.view.name()));
    out.push_str(&format!("svg = \"{svg_name}\"\n"));
    out.push_str("# How far the head is turned toward the viewer's right, so the character's right is the near side.\n");
    out.push_str(&format!(
        "turn_degrees = {}\n",
        toml_number(seen.turn_degrees, 3)
    ));

    out.push_str("\n[frame]\n");
    out.push_str(&format!("width = {}\n", position(seen.width)));
    out.push_str(&format!("height = {}\n", position(seen.height)));
    out.push_str(&format!(
        "unit = {}  # eye line to chin: what every proportion is divided by\n",
        position(seen.unit)
    ));
    out.push_str(&format!(
        "head_height = {}  # hair top to chin: differences are stated as shares of it\n",
        position(seen.head_height)
    ));
    out.push_str(&format!("eye_line_y = {}\n", position(seen.eye_line_y)));
    out.push_str(&format!(
        "turn_axis_x = {}  # the upright axis the head turns about, through the skull's centre\n",
        position(seen.turn_axis_x)
    ));

    out.push_str("\n# Landmarks: points in space. Every view shows the same landmarks, each at the same height.\n");
    out.push_str("# `depth` is how far the point stands in front of the turn axis, the same in every view:\n");
    out.push_str("# x is turn_axis_x plus the front view's offset from its axis times the cosine of the turn,\n");
    out.push_str("# plus depth times the sine of the turn.\n");
    out.push_str("[landmarks]\n");
    for landmark in &seen.landmarks {
        out.push_str(&format!(
            "{} = {{ x = {}, y = {}, depth = {} }}\n",
            landmark.name,
            position(landmark.x),
            position(landmark.y),
            position(landmark.depth)
        ));
    }

    out.push_str("\n# View points: points of this view only, such as the side of a round form or where two guides cross.\n");
    out.push_str("# Under the same name another view shows a different point of the head.\n");
    out.push_str("[view_points]\n");
    for (name, [x, y]) in &built.view_points {
        out.push_str(&format!(
            "{name} = {{ x = {}, y = {} }}\n",
            position(*x),
            position(*y)
        ));
    }

    out.push_str("\n# The SVG's structure: one layer, its children in drawing order, and the guides inside each group.\n");
    out.push_str("[structure]\n");
    out.push_str(&format!("layer = \"{}\"\n", guides::LAYER));
    let children = layout(built);
    let order: Vec<String> = children
        .iter()
        .map(|(group, members)| group.map_or_else(|| members[0].name.clone(), str::to_string))
        .collect();
    out.push_str(&format!("children = {}\n", quoted_list(&order)));
    out.push_str("\n[structure.groups]\n");
    for (group, members) in &children {
        if let Some(group) = group {
            let names: Vec<String> = members.iter().map(|guide| guide.name.clone()).collect();
            out.push_str(&format!("{group} = {}\n", quoted_list(&names)));
        }
    }

    out.push_str("\n# Guides: each is the SVG element whose id is its name. `anchors` names the points its outline\n");
    out.push_str("# passes through, in order. A circle or an ellipse has no anchor at its centre: its anchors are its\n");
    out.push_str(
        "# right, bottom, left and top points, which is how VectorCraft reads one back.\n",
    );
    for guide in &built.guides {
        out.push_str(&format!("\n[guides.{}]\n", guide.name));
        match &guide.shape {
            Shape::Circle { centre, r } => {
                out.push_str("element = \"circle\"\n");
                out.push_str(&format!("centre = \"{centre}\"\n"));
                out.push_str(&format!("r = {}\n", position(*r)));
            }
            Shape::Ellipse { centre, rx, ry } => {
                out.push_str("element = \"ellipse\"\n");
                out.push_str(&format!("centre = \"{centre}\"\n"));
                out.push_str(&format!("rx = {}\n", position(*rx)));
                out.push_str(&format!("ry = {}\n", position(*ry)));
            }
            Shape::Line => out.push_str("element = \"line\"\n"),
            Shape::Polyline => out.push_str("element = \"polyline\"\n"),
            Shape::Path { closed, .. } => {
                out.push_str("element = \"path\"\n");
                out.push_str(&format!("closed = {closed}\n"));
            }
        }
        out.push_str(&format!("anchors = {}\n", quoted_list(&guide.anchors)));
    }
    out
}

/// Reads the positions of a landmark file: its landmarks and its view points, and its turn.
pub fn read_landmark_file(text: &str) -> Result<Reading> {
    let table: toml::Table = text
        .parse()
        .map_err(|error| Error::new(format!("the landmark file is not valid TOML: {error}")))?;
    let coordinate = |name: &str, entry: &toml::Table, key: &str| -> Result<f64> {
        match entry.get(key) {
            Some(toml::Value::Float(number)) => Ok(*number),
            Some(toml::Value::Integer(number)) => Ok(*number as f64),
            _ => Err(Error::new(format!(
                "the landmark file gives `{name}` no number `{key}`"
            ))),
        }
    };
    let mut reading = Reading {
        turn_degrees: match table.get("turn_degrees") {
            Some(toml::Value::Float(number)) => *number,
            Some(toml::Value::Integer(number)) => *number as f64,
            _ => return Err(Error::new("the landmark file has no number `turn_degrees`")),
        },
        ..Reading::default()
    };
    if !table.contains_key("landmarks") {
        return Err(Error::new("the landmark file has no [landmarks] table"));
    }
    for key in ["landmarks", "view_points"] {
        let Some(points) = table.get(key) else {
            continue;
        };
        let points = points
            .as_table()
            .ok_or_else(|| Error::new(format!("the landmark file's `{key}` is not a table")))?;
        for (name, entry) in points {
            let entry = entry.as_table().ok_or_else(|| {
                Error::new(format!("the landmark file's `{name}` is not a position"))
            })?;
            let position = [coordinate(name, entry, "x")?, coordinate(name, entry, "y")?];
            if reading.points.insert(name.clone(), position).is_some() {
                return Err(Error::new(format!(
                    "the landmark file names `{name}` twice"
                )));
            }
        }
    }
    Ok(reading)
}
