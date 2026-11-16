//! The generator, through the command line: a set of proportions in, armature files out.
//!
//! The set of proportions used here is made up (`tests/fixtures/made-up-proportions.toml`).
//! Nothing here pins the bytes of an armature: none is accepted yet.

mod support;

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use support::{LandmarkFile, MADE_UP, assert_close, draw, maquette, path_text, scratch, text};

const VIEWS: [&str; 2] = ["front", "three-quarter"];

/// The guides issue #3 lists, under the names the SVG carries.
const GUIDES: [&str; 22] = [
    "skull",
    "face-outline",
    "centre-line",
    "eye-level-line",
    "nose-level-line",
    "mouth-level-line",
    "eye-opening-left",
    "eye-opening-right",
    "lid-line-left",
    "lid-line-right",
    "iris-left",
    "iris-right",
    "brow-line-left",
    "brow-line-right",
    "nose-mark",
    "mouth-line",
    "ear-left",
    "ear-right",
    "neck-left",
    "neck-right",
    "hair-volume",
    "shoulder-line",
];

/// The groups that hold guides, and the one layer.
const GROUPS: [&str; 4] = ["armature", "eye-left", "eye-right", "neck"];

/// The three-quarter turn pinned from Saffi's hand-made view: 29 degrees, where her drawings
/// support 27.9 to 29.4 (`data/saffi/views-compared.toml`).
const TURN_DEGREES: f64 = 29.0;

fn landmark_file(folder: &Path, view: &str) -> LandmarkFile {
    LandmarkFile::read(&folder.join(format!("{view}.landmarks.toml")))
}

fn svg_text(folder: &Path, view: &str) -> String {
    text(&folder.join(format!("{view}.svg")))
}

/// Numbers of a path's `d`, a polyline's `points` or any attribute, split on spaces and commas.
fn numbers(list: &str) -> Vec<f64> {
    list.split([' ', ','])
        .filter(|piece| !piece.is_empty())
        .map(|piece| {
            piece
                .parse()
                .unwrap_or_else(|_| panic!("`{piece}` is not a number"))
        })
        .collect()
}

fn attribute(node: roxmltree::Node, name: &str) -> f64 {
    node.attribute(name)
        .unwrap_or_else(|| panic!("no attribute {name}"))
        .parse()
        .unwrap_or_else(|_| panic!("{name} is not a number"))
}

/// The commands of a path's `d`, each with its numbers.
fn path_commands(d: &str) -> Vec<(char, Vec<f64>)> {
    let mut commands: Vec<(char, String)> = Vec::new();
    for character in d.chars() {
        if character.is_ascii_alphabetic() {
            commands.push((character, String::new()));
        } else {
            commands
                .last_mut()
                .expect("a path starts with a command")
                .1
                .push(character);
        }
    }
    commands
        .into_iter()
        .map(|(command, list)| (command, numbers(&list)))
        .collect()
}

/// The anchors of an SVG element, as VectorCraft reads them back.
fn anchors_of(node: roxmltree::Node) -> Vec<[f64; 2]> {
    match node.tag_name().name() {
        "line" => vec![
            [attribute(node, "x1"), attribute(node, "y1")],
            [attribute(node, "x2"), attribute(node, "y2")],
        ],
        "polyline" => numbers(node.attribute("points").expect("points"))
            .chunks(2)
            .map(|pair| [pair[0], pair[1]])
            .collect(),
        "circle" | "ellipse" => {
            let (cx, cy) = (attribute(node, "cx"), attribute(node, "cy"));
            let (rx, ry) = if node.tag_name().name() == "circle" {
                (attribute(node, "r"), attribute(node, "r"))
            } else {
                (attribute(node, "rx"), attribute(node, "ry"))
            };
            // Right, bottom, left, top: the contract's order for a circle or an ellipse.
            vec![[cx + rx, cy], [cx, cy + ry], [cx - rx, cy], [cx, cy - ry]]
        }
        "path" => path_commands(node.attribute("d").expect("d"))
            .into_iter()
            .filter(|(command, _)| *command != 'Z')
            .map(|(_, values)| [values[values.len() - 2], values[values.len() - 1]])
            .collect(),
        other => panic!("{other} is not a guide's element"),
    }
}

#[test]
fn one_command_writes_an_svg_and_a_landmark_file_for_each_view() {
    let folder = draw("writes-both-views", MADE_UP, &[]);
    for view in VIEWS {
        let svg = svg_text(&folder, view);
        assert!(svg.contains("<svg"), "{view}.svg is not an SVG");
        let file = landmark_file(&folder, view);
        assert_eq!(file.table["view"].as_str(), Some(view));
        assert_eq!(
            file.table["svg"].as_str(),
            Some(format!("{view}.svg").as_str())
        );
    }
    let written: BTreeSet<String> = std::fs::read_dir(&folder)
        .expect("the folder")
        .map(|entry| {
            entry
                .expect("an entry")
                .file_name()
                .into_string()
                .expect("a name")
        })
        .collect();
    let expected: BTreeSet<String> = [
        "front.svg",
        "front.landmarks.toml",
        "three-quarter.svg",
        "three-quarter.landmarks.toml",
    ]
    .iter()
    .map(|name| name.to_string())
    .collect();
    assert_eq!(written, expected);
}

#[test]
fn the_three_quarter_view_is_turned_as_far_as_the_pinned_turn() {
    let folder = draw("pinned-turn", MADE_UP, &[]);
    let front = landmark_file(&folder, "front");
    let turned = landmark_file(&folder, "three-quarter");
    assert_eq!(support::number(&front.table["turn_degrees"]), 0.0);
    assert_eq!(support::number(&turned.table["turn_degrees"]), TURN_DEGREES);
}

#[test]
fn the_same_input_written_twice_gives_the_same_bytes() {
    let first = draw("same-bytes-first", MADE_UP, &[]);
    let second = draw("same-bytes-second", MADE_UP, &[]);
    for view in VIEWS {
        for name in [format!("{view}.svg"), format!("{view}.landmarks.toml")] {
            let one = std::fs::read(first.join(&name)).expect("the first file");
            let two = std::fs::read(second.join(&name)).expect("the second file");
            assert!(!one.is_empty(), "{name} is empty");
            assert_eq!(one, two, "{name} differs between two runs");
        }
    }
}

#[test]
fn every_landmark_has_the_same_height_in_both_views() {
    let folder = draw(
        "same-heights",
        MADE_UP,
        &["--unit", "173.3", "--eye-line-y", "412.7"],
    );
    let front = landmark_file(&folder, "front");
    let turned = landmark_file(&folder, "three-quarter");
    assert!(front.landmarks.len() > 50, "expected the whole head");
    assert_eq!(
        front.landmarks.keys().collect::<Vec<_>>(),
        turned.landmarks.keys().collect::<Vec<_>>(),
        "the two views do not show the same landmarks"
    );
    for (name, point) in &front.landmarks {
        assert_eq!(
            point.y, turned.landmarks[name].y,
            "{name} is not at one height in both views"
        );
    }
}

#[test]
fn a_landmarks_sideways_position_when_turned_is_the_one_its_front_position_and_depth_give() {
    let folder = draw(
        "front-and-depth",
        MADE_UP,
        &[
            "--unit",
            "173.3",
            "--front-axis-x",
            "481.25",
            "--three-quarter-axis-x",
            "507.5",
        ],
    );
    let front = landmark_file(&folder, "front");
    let turned = landmark_file(&folder, "three-quarter");
    let (sin, cos) = TURN_DEGREES.to_radians().sin_cos();
    let (front_axis, turned_axis) = (front.frame("turn_axis_x"), turned.frame("turn_axis_x"));
    assert_eq!(front_axis, 481.25);
    assert_eq!(turned_axis, 507.5);
    for (name, point) in &front.landmarks {
        let depth = point.depth.unwrap_or_else(|| panic!("{name} has no depth"));
        let shown = turned.landmarks[name];
        assert_eq!(shown.depth, Some(depth), "{name} has two depths");
        // A level head turned about an upright axis, drawn flat: offset times the cosine of
        // the turn plus depth times the sine. Each of the three numbers is rounded to 0.001.
        let expected = turned_axis + (point.x - front_axis) * cos + depth * sin;
        assert_close(shown.x, expected, 0.0015, name);
    }
}

#[test]
fn positions_follow_from_the_proportions_as_the_list_defines_them() {
    // Worked by hand from the made-up set, at the default placement: one unit is 200, the eye
    // line is at y = 400 and the turn axis at x = 500 in both views.
    let folder = draw("worked-by-hand", MADE_UP, &[]);
    let front = landmark_file(&folder, "front");
    let turned = landmark_file(&folder, "three-quarter");
    let (sin, cos) = TURN_DEGREES.to_radians().sin_cos();
    let unit = 200.0;
    let skull_centre_depth = -0.6375;

    // The chin is one unit below the eye line, on the centre line, at chin_depth = 0.1875.
    let chin = front.point("chin");
    assert_eq!((chin.x, chin.y), (500.0, 600.0));
    let chin_turned = turned.point("chin");
    assert_eq!(chin_turned.y, 600.0);
    assert_close(
        chin_turned.x,
        500.0 + unit * (0.1875 - skull_centre_depth) * sin,
        0.001,
        "the chin when turned",
    );

    // The outer corner of the left eye: offset 0.6125, drop -0.0121, and half the wrap
    // (0.1375) behind depth zero.
    let corner = front.point("eye-left-outer-corner");
    assert_close(corner.x, 500.0 + unit * 0.6125, 0.001, "the corner's x");
    assert_close(corner.y, 400.0 - unit * 0.0121, 0.001, "the corner's y");
    let corner_turned = turned.point("eye-left-outer-corner");
    assert_close(
        corner_turned.x,
        500.0 + unit * (0.6125 * cos + (-0.1375 / 2.0 - skull_centre_depth) * sin),
        0.001,
        "the corner when turned",
    );
    // Its twin on the character's right is on the viewer's left.
    let twin = turned.point("eye-right-outer-corner");
    assert_close(
        twin.x,
        500.0 + unit * (-0.6125 * cos + (-0.1375 / 2.0 - skull_centre_depth) * sin),
        0.001,
        "the right corner when turned",
    );

    // The skull is the same circle in both views: radius 0.9379, centre 0.3623 above the eye line.
    for file in [&front, &turned] {
        let skull = &file.guides["skull"].table;
        assert_close(
            support::number(&skull["r"]),
            unit * 0.9379,
            0.001,
            "the skull's radius",
        );
        let centre = file.point("skull-centre");
        assert_close(centre.x, 500.0, 1e-9, "the skull centre's x");
        assert_close(
            centre.y,
            400.0 - unit * 0.3623,
            0.001,
            "the skull centre's y",
        );
    }

    // Head height is hair top to chin: hair_top_rise (1.3127) plus the unit.
    assert_close(
        front.frame("head_height"),
        unit * 2.3127,
        0.001,
        "head height",
    );
    assert_eq!(front.frame("unit"), unit);

    // The eye level line is on the eye line, and the iris narrows with the turn while its height stays.
    for file in [&front, &turned] {
        for end in &file.guides["eye-level-line"].passes_through {
            assert_eq!(file.point(end).y, 400.0);
        }
    }
    let iris_front = &front.guides["iris-left"].table;
    let iris_turned = &turned.guides["iris-left"].table;
    assert_close(
        support::number(&iris_front["rx"]),
        unit * 0.2379 / 2.0,
        0.001,
        "iris rx",
    );
    assert_close(
        support::number(&iris_turned["rx"]),
        unit * 0.2379 / 2.0 * cos,
        0.001,
        "iris rx when turned",
    );
    assert_eq!(iris_front["ry"], iris_turned["ry"]);
}

#[test]
fn the_front_view_is_symmetric_left_to_right() {
    let folder = draw("front-symmetry", MADE_UP, &["--unit", "173.3"]);
    let front = landmark_file(&folder, "front");
    let axis = front.frame("turn_axis_x");
    let mut pairs = 0;
    let all = front.landmarks.iter().chain(front.view_points.iter());
    for (name, point) in all {
        if name.contains("-left-") {
            let twin = front.point(&name.replace("-left-", "-right-"));
            assert_close(point.x - axis, axis - twin.x, 1e-9, name);
            assert_eq!(point.y, twin.y, "{name} and its twin differ in height");
            assert!(point.x > axis, "{name} is not on the viewer's right");
            pairs += 1;
        } else if !name.contains("-right-") {
            assert_close(point.x, axis, 1e-9, name);
        }
    }
    assert!(pairs > 30, "expected the head's pairs, found {pairs}");
}

#[test]
fn left_and_right_in_a_name_are_the_characters_own_in_both_views() {
    let folder = draw("own-left-and-right", MADE_UP, &[]);
    for view in VIEWS {
        let file = landmark_file(&folder, view);
        for (name, point) in &file.landmarks {
            if name.contains("-left-") {
                let twin = file.landmarks[&name.replace("-left-", "-right-")];
                assert!(
                    point.x > twin.x,
                    "{view}: {name} is not toward the viewer's right of its twin"
                );
            }
        }
    }
}

#[test]
fn every_guide_of_the_inventory_appears_once_under_its_name_in_each_view() {
    let folder = draw("inventory", MADE_UP, &["--unit", "173.3"]);
    let guides: BTreeSet<&str> = GUIDES.into_iter().collect();
    let groups: BTreeSet<&str> = GROUPS.into_iter().collect();
    for view in VIEWS {
        let svg = svg_text(&folder, view);
        let document = roxmltree::Document::parse(&svg).expect("the SVG is XML");
        let mut ids: BTreeMap<&str, usize> = BTreeMap::new();
        for node in document.descendants().filter(roxmltree::Node::is_element) {
            if let Some(id) = node.attribute("id") {
                *ids.entry(id).or_default() += 1;
            }
        }
        for (id, count) in &ids {
            assert_eq!(*count, 1, "{view}: the id {id} is used {count} times");
        }
        let found: BTreeSet<&str> = ids.keys().copied().collect();
        let expected: BTreeSet<&str> = guides.union(&groups).copied().collect();
        assert_eq!(
            found, expected,
            "{view}: the SVG's ids are not the inventory"
        );

        // The landmark file names the same guides, the same groups and the same layer.
        let file = landmark_file(&folder, view);
        let listed: BTreeSet<&str> = file.guides.keys().map(String::as_str).collect();
        assert_eq!(listed, guides, "{view}: the landmark file's guides");
        let structure = file.table["structure"].as_table().expect("[structure]");
        let mut named: BTreeSet<&str> = structure["groups"]
            .as_table()
            .expect("[structure.groups]")
            .keys()
            .map(String::as_str)
            .collect();
        named.insert(structure["layer"].as_str().expect("a layer"));
        assert_eq!(named, groups, "{view}: the landmark file's groups");
    }
}

#[test]
fn every_guide_passes_through_the_points_the_landmark_file_names() {
    let folder = draw("guides-match-landmarks", MADE_UP, &["--unit", "173.3"]);
    for view in VIEWS {
        let svg = svg_text(&folder, view);
        let document = roxmltree::Document::parse(&svg).expect("the SVG is XML");
        let file = landmark_file(&folder, view);
        let structure = file.table["structure"].as_table().expect("[structure]");
        for (name, guide) in &file.guides {
            let node = document
                .descendants()
                .find(|node| node.attribute("id") == Some(name.as_str()))
                .unwrap_or_else(|| panic!("{view}: no element {name}"));
            assert_eq!(node.tag_name().name(), guide.element, "{view}: {name}");

            let mut expected: Vec<[f64; 2]> = guide
                .passes_through
                .iter()
                .map(|anchor| {
                    let point = file.point(anchor);
                    [point.x, point.y]
                })
                .collect();
            // A closed path comes back to its first anchor.
            if guide.table.get("closed").and_then(toml::Value::as_bool) == Some(true) {
                expected.push(expected[0]);
            }
            let drawn = anchors_of(node);
            assert_eq!(
                drawn.len(),
                expected.len(),
                "{view}: {name} has other anchors"
            );
            for (index, (drawn, expected)) in drawn.iter().zip(&expected).enumerate() {
                let what = format!("{view}: anchor {index} of {name}");
                assert_close(drawn[0], expected[0], 1e-9, &what);
                assert_close(drawn[1], expected[1], 1e-9, &what);
            }

            // A circle or an ellipse is centred on the landmark the file names.
            if let Some(centre) = guide.table.get("centre") {
                let centre = file.point(centre.as_str().expect("a name"));
                assert_eq!(attribute(node, "cx"), centre.x, "{view}: {name}");
                assert_eq!(attribute(node, "cy"), centre.y, "{view}: {name}");
            }

            // The guide sits where the file's structure says: in its group, or in the layer.
            let parent = node
                .parent()
                .expect("a parent")
                .attribute("id")
                .expect("an id");
            let group = structure["groups"]
                .as_table()
                .expect("[structure.groups]")
                .iter()
                .find(|(_, members)| {
                    members
                        .as_array()
                        .expect("a list")
                        .iter()
                        .any(|member| member.as_str() == Some(name.as_str()))
                })
                .map(|(group, _)| group.as_str());
            let layer = structure["layer"].as_str().expect("a layer");
            assert_eq!(parent, group.unwrap_or(layer), "{view}: where {name} sits");
        }
    }
}

#[test]
fn names_are_unique_within_a_view() {
    let folder = draw("unique-names", MADE_UP, &[]);
    for view in VIEWS {
        let file = landmark_file(&folder, view);
        let mut names = BTreeSet::new();
        let structure = file.table["structure"].as_table().expect("[structure]");
        let groups = structure["groups"].as_table().expect("[structure.groups]");
        let all = file
            .landmarks
            .keys()
            .chain(file.view_points.keys())
            .chain(file.guides.keys())
            .chain(groups.keys());
        for name in all {
            assert!(
                names.insert(name.clone()),
                "{view}: {name} names two things"
            );
        }
    }
}

/// The SVG contract of `docs/foundation/02-first-table.md`, "What the armature uses".
#[test]
fn the_svg_stays_inside_the_contracts_subset() {
    let folder = draw("contract-subset", MADE_UP, &["--unit", "173.3"]);
    let allowed_attributes: BTreeSet<&str> = [
        "id",
        "d",
        "x1",
        "y1",
        "x2",
        "y2",
        "points",
        "cx",
        "cy",
        "r",
        "rx",
        "ry",
        "fill",
        "stroke",
        "stroke-width",
        "stroke-linecap",
        "stroke-linejoin",
        "stroke-dasharray",
    ]
    .into_iter()
    .collect();
    let plain_number = |value: &str| {
        !value.is_empty()
            && value
                .chars()
                .all(|character| character.is_ascii_digit() || character == '.')
            && value.parse::<f64>().is_ok()
    };

    for view in VIEWS {
        let svg = svg_text(&folder, view);
        let document = roxmltree::Document::parse(&svg).expect("the SVG is XML");
        let root = document.root_element();

        // Size: plain numbers equal to a viewBox that starts at 0 0.
        assert_eq!(root.tag_name().name(), "svg");
        let (width, height) = (
            root.attribute("width").expect("width"),
            root.attribute("height").expect("height"),
        );
        assert!(
            plain_number(width) && plain_number(height),
            "{view}: a size with a unit"
        );
        assert_eq!(
            root.attribute("viewBox"),
            Some(format!("0 0 {width} {height}").as_str())
        );

        // Structure: every top-level element is a `g` with an id.
        for child in root.children().filter(roxmltree::Node::is_element) {
            assert_eq!(
                child.tag_name().name(),
                "g",
                "{view}: a top-level element is no group"
            );
            assert!(
                child.attribute("id").is_some(),
                "{view}: a layer without an id"
            );
        }

        for node in root.descendants().filter(roxmltree::Node::is_element) {
            if node == root {
                continue;
            }
            let tag = node.tag_name().name();
            assert!(
                ["g", "path", "line", "polyline", "circle", "ellipse"].contains(&tag),
                "{view}: <{tag}> is outside the contract"
            );

            // Names: the id alone, lower-case letters, digits and hyphens, starting with a letter.
            let id = node
                .attribute("id")
                .unwrap_or_else(|| panic!("{view}: <{tag}> has no id"));
            assert!(
                id.starts_with(|first: char| first.is_ascii_lowercase())
                    && id.chars().all(|character| {
                        character.is_ascii_lowercase()
                            || character.is_ascii_digit()
                            || character == '-'
                    }),
                "{view}: the id `{id}` is outside the contract"
            );
            for attribute in node.attributes() {
                assert!(
                    attribute.namespace().is_none()
                        && allowed_attributes.contains(attribute.name()),
                    "{view}: {id} has the attribute `{}`, which the contract does not use",
                    attribute.name()
                );
            }

            if tag == "g" {
                // Groups nest, each with at least one guide.
                assert!(
                    node.descendants()
                        .any(|inner| inner.is_element() && inner.tag_name().name() != "g"),
                    "{view}: the group {id} holds no guide"
                );
                continue;
            }

            // Paint: a stroke with fill="none", on the element or a group above it, and nothing hidden.
            let inherited = |name: &str| {
                node.ancestors()
                    .find_map(|ancestor| ancestor.attribute(name))
                    .unwrap_or_else(|| panic!("{view}: {id} has no {name}"))
            };
            assert_eq!(inherited("fill"), "none", "{view}: {id} is filled");
            assert_ne!(inherited("stroke"), "none", "{view}: {id} has no stroke");
            let stroke_width: f64 = inherited("stroke-width").parse().expect("a stroke width");
            assert!(stroke_width > 0.0, "{view}: {id} has a stroke of no width");

            // Guides: a path is one subpath of M, L, C and Z.
            if tag == "path" {
                let commands = path_commands(node.attribute("d").expect("d"));
                for (index, (command, values)) in commands.iter().enumerate() {
                    let expected_numbers = match command {
                        'M' => {
                            assert_eq!(index, 0, "{view}: {id} has a second subpath");
                            2
                        }
                        'L' => 2,
                        'C' => 6,
                        'Z' => {
                            assert_eq!(index, commands.len() - 1, "{view}: {id} goes on after Z");
                            0
                        }
                        other => panic!("{view}: {id} uses the path command {other}"),
                    };
                    assert_eq!(
                        values.len(),
                        expected_numbers,
                        "{view}: {id}, command {command}"
                    );
                    assert!(values.iter().all(|value| value.is_finite()));
                }
                assert_eq!(commands[0].0, 'M', "{view}: {id} does not start with M");
            }
            // A circle of no radius or a path of no length would be dropped.
            for radius in ["r", "rx", "ry"] {
                if node.has_attribute(radius) {
                    assert!(
                        attribute(node, radius) > 0.0,
                        "{view}: {id} has no {radius}"
                    );
                }
            }
        }
    }
}

#[test]
fn the_proportions_taken_back_from_the_landmark_files_equal_the_ones_put_in() {
    let folder = draw(
        "taken-back",
        MADE_UP,
        &[
            "--unit",
            "173.3",
            "--eye-line-y",
            "412.7",
            "--front-axis-x",
            "481.25",
            "--three-quarter-axis-x",
            "507.5",
        ],
    );
    let taken_back = folder.join("taken-back.toml");
    support::maquette_ok(&[
        "proportions-from-landmarks",
        "--front",
        path_text(&folder.join("front.landmarks.toml")),
        "--three-quarter",
        path_text(&folder.join("three-quarter.landmarks.toml")),
        "--out",
        path_text(&taken_back),
    ]);
    let put_in = support::proportions(&support::repository().join(MADE_UP));
    let came_back = support::proportions(&taken_back);
    assert_eq!(
        put_in.keys().collect::<Vec<_>>(),
        came_back.keys().collect::<Vec<_>>(),
        "the names differ"
    );
    assert!(put_in.len() > 40);
    // Positions are written to a thousandth of an SVG unit, which is what a proportion can
    // be taken back to: 0.0001 of a unit of 173.3.
    for (name, value) in &put_in {
        assert_close(came_back[name], *value, 1e-4, name);
    }
}

#[test]
fn the_placement_moves_the_head_and_leaves_the_proportions_alone() {
    let folder = draw(
        "placement",
        MADE_UP,
        &[
            "--unit",
            "150",
            "--eye-line-y",
            "300",
            "--front-axis-x",
            "420",
        ],
    );
    let front = landmark_file(&folder, "front");
    assert_eq!(front.frame("unit"), 150.0);
    assert_eq!(front.frame("eye_line_y"), 300.0);
    assert_eq!(front.frame("turn_axis_x"), 420.0);
    let chin = front.point("chin");
    assert_eq!((chin.x, chin.y), (420.0, 450.0));
}

#[test]
fn a_set_that_lacks_a_proportion_or_has_a_stray_one_is_refused_in_plain_words() {
    let folder = scratch("refused-sets");
    let whole = text(&support::repository().join(MADE_UP));

    let lacking = folder.join("lacking.toml");
    std::fs::write(&lacking, whole.replace("chin_depth = 0.1875\n", "")).expect("written");
    let output = maquette(&[
        "draw",
        "--proportions",
        path_text(&lacking),
        "--out",
        path_text(&folder.join("out")),
    ]);
    assert!(!output.status.success());
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(
        message.contains("lacks") && message.contains("chin_depth"),
        "{message}"
    );
    assert!(
        !folder.join("out").exists(),
        "files were written for a refused set"
    );

    let stray = folder.join("stray.toml");
    std::fs::write(&stray, format!("{whole}pupil_size = 0.1\n")).expect("written");
    let output = maquette(&[
        "draw",
        "--proportions",
        path_text(&stray),
        "--out",
        path_text(&folder.join("out")),
    ]);
    assert!(!output.status.success());
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains("pupil_size"), "{message}");

    let no_width = folder.join("no-width.toml");
    std::fs::write(
        &no_width,
        whole.replace("skull_radius = 0.9379", "skull_radius = 0.0"),
    )
    .expect("written");
    let output = maquette(&[
        "draw",
        "--proportions",
        path_text(&no_width),
        "--out",
        path_text(&folder.join("out")),
    ]);
    assert!(!output.status.success());
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains("skull_radius"), "{message}");
}
