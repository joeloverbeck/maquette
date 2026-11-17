//! The generator, through the command line: a set of proportions in, armature files out.
//!
//! The set of proportions used here is made up (`tests/fixtures/made-up-proportions.toml`).
//! Nothing here pins the bytes of an armature: a made-up set has no accepted one.
//! `tests/saffi.rs` pins Saffi's.

mod support;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use support::{LandmarkFile, MADE_UP, assert_close, draw, maquette, path_text, scratch, text};

const VIEWS: [&str; 3] = ["front", "three-quarter", "side"];

/// The guides issue #3 lists, under the names the SVG carries, as the front view and the
/// three-quarter view show them.
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

/// The guides the side view shows. The head hides the eye, the brow and the ear of the
/// character's left there. The centre line is the front of the face outline, so it is not
/// drawn a second time. The neck's two lines are its front and its back, which are nobody's
/// left and right.
const SIDE_GUIDES: [&str; 16] = [
    "skull",
    "face-outline",
    "eye-level-line",
    "nose-level-line",
    "mouth-level-line",
    "eye-opening-right",
    "lid-line-right",
    "iris-right",
    "brow-line-right",
    "nose-mark",
    "mouth-line",
    "ear-right",
    "neck-front",
    "neck-back",
    "hair-volume",
    "shoulder-line",
];

const SIDE_GROUPS: [&str; 3] = ["armature", "eye-right", "neck"];

fn guides_of(view: &str) -> BTreeSet<&'static str> {
    if view == "side" {
        SIDE_GUIDES.into_iter().collect()
    } else {
        GUIDES.into_iter().collect()
    }
}

fn groups_of(view: &str) -> BTreeSet<&'static str> {
    if view == "side" {
        SIDE_GROUPS.into_iter().collect()
    } else {
        GROUPS.into_iter().collect()
    }
}

/// The three-quarter turn pinned from Saffi's hand-made view: 29 degrees, where her drawings
/// support 27.9 to 29.4 (`data/saffi/views-compared.toml`).
const TURN_DEGREES: f64 = 29.0;

/// The side view is the same turn carried to a full profile.
const SIDE_TURN_DEGREES: f64 = 90.0;

/// The sine and the cosine of a view's turn. The front and the side view are exact.
fn sin_cos(turn_degrees: f64) -> (f64, f64) {
    if turn_degrees == 0.0 {
        (0.0, 1.0)
    } else if turn_degrees == 90.0 {
        (1.0, 0.0)
    } else {
        turn_degrees.to_radians().sin_cos()
    }
}

fn landmark_file(folder: &Path, view: &str) -> LandmarkFile {
    LandmarkFile::read(&folder.join(format!("{view}.landmarks.toml")))
}

/// The named correction `lower_face_set_back`, to add after the made-up set: the chin set
/// back by 0.0625 of a unit and the mouth by 0.0375. Both amounts are made up, like the set.
const MADE_UP_SET_BACK: &str =
    "\n[corrections.lower_face_set_back]\nchin = 0.0625\nmouth = 0.0375\n";

/// Writes the made-up set with `more` added after it, and draws it into a folder of its own.
fn draw_made_up_with(name: &str, more: &str, placement: &[&str]) -> PathBuf {
    let set = scratch(&format!("{name}-set")).join("proportions.toml");
    let made_up = text(&support::repository().join(MADE_UP));
    std::fs::write(&set, format!("{made_up}{more}")).expect("the set is written");
    draw(name, path_text(&set), placement)
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
fn one_command_writes_an_svg_and_a_landmark_file_for_each_of_the_three_views() {
    let folder = draw("writes-three-views", MADE_UP, &[]);
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
        "side.svg",
        "side.landmarks.toml",
    ]
    .iter()
    .map(|name| name.to_string())
    .collect();
    assert_eq!(written, expected);
}

#[test]
fn the_three_quarter_view_is_turned_as_far_as_the_pinned_turn_and_the_side_view_to_90_degrees() {
    let folder = draw("pinned-turn", MADE_UP, &[]);
    let front = landmark_file(&folder, "front");
    let turned = landmark_file(&folder, "three-quarter");
    let side = landmark_file(&folder, "side");
    assert_eq!(support::number(&front.table["turn_degrees"]), 0.0);
    assert_eq!(support::number(&turned.table["turn_degrees"]), TURN_DEGREES);
    assert_eq!(
        support::number(&side.table["turn_degrees"]),
        SIDE_TURN_DEGREES
    );
}

#[test]
fn the_side_view_faces_the_viewers_right() {
    // The same turn as the three-quarter view, carried on: what stands in front of the turn
    // axis shows toward the viewer's right of it, and what stands behind toward the left.
    let folder = draw("side-faces-right", MADE_UP, &[]);
    let side = landmark_file(&folder, "side");
    let axis = side.frame("turn_axis_x");
    for in_front in ["chin", "nose-mark-bottom", "mouth-line-centre"] {
        assert!(
            side.point(in_front).x > axis,
            "{in_front} is not toward the viewer's right"
        );
    }
    // The made-up neck and shoulders stand behind the skull's centre.
    for behind in ["shoulder-right-point", "shoulder-left-point"] {
        assert!(
            side.point(behind).x < axis,
            "{behind} is not toward the viewer's left"
        );
    }
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
fn every_landmark_has_the_same_height_in_all_three_views() {
    let folder = draw(
        "same-heights",
        MADE_UP,
        &["--unit", "173.3", "--eye-line-y", "412.7"],
    );
    let front = landmark_file(&folder, "front");
    assert!(front.landmarks.len() > 50, "expected the whole head");
    for view in ["three-quarter", "side"] {
        let turned = landmark_file(&folder, view);
        assert_eq!(
            front.landmarks.keys().collect::<Vec<_>>(),
            turned.landmarks.keys().collect::<Vec<_>>(),
            "the front view and the {view} view do not show the same landmarks"
        );
        for (name, point) in &front.landmarks {
            assert_eq!(
                point.y, turned.landmarks[name].y,
                "{name} is not at one height in the front view and the {view} view"
            );
        }
    }
}

/// The landmarks the named correction `lower_face_set_back` moves in the side view, under the
/// amount that moves each: the chin with the jaw point beside it, and the mouth line's centre
/// with its two ends.
const SET_BACK: [(&str, [&str; 3]); 2] = [
    (
        "chin",
        ["chin", "face-outline-right-jaw", "face-outline-left-jaw"],
    ),
    (
        "mouth",
        [
            "mouth-line-centre",
            "mouth-line-right-end",
            "mouth-line-left-end",
        ],
    ),
];

#[test]
fn a_landmarks_sideways_position_in_the_side_view_is_its_depth_less_the_named_correction() {
    let placement = ["--unit", "173.3", "--side-axis-x", "455.5"];
    // The made-up set carries no correction, so nothing is set back; with the named
    // correction added, the chin is set back by 0.0625 of a unit of 173.3 and the mouth by
    // 0.0375, and the landmarks that go with each by the same.
    let flat = draw("side-is-depth", MADE_UP, &placement);
    let corrected = draw_made_up_with("side-is-depth-less", MADE_UP_SET_BACK, &placement);
    for (folder, amounts) in [
        (&flat, None),
        (&corrected, Some([173.3 * 0.0625, 173.3 * 0.0375])),
    ] {
        let side = landmark_file(folder, "side");
        let axis = side.frame("turn_axis_x");
        assert_eq!(axis, 455.5);
        let axis_depth = side.frame("turn_axis_depth");
        assert!(side.landmarks.len() > 50, "expected the whole head");
        let mut set_back = 0;
        for (name, point) in &side.landmarks {
            let depth = point.depth.unwrap_or_else(|| panic!("{name} has no depth"));
            let amount = amounts.and_then(|amounts| {
                let index = SET_BACK
                    .iter()
                    .position(|(_, moved)| moved.contains(&name.as_str()))?;
                Some(amounts[index])
            });
            // How far the point stands in front of the turn axis, less the correction's
            // amount where one moves it, and nothing of its offset. Each number is rounded
            // to 0.001.
            assert_close(
                point.x,
                axis + (depth - axis_depth) - amount.unwrap_or(0.0),
                0.002,
                name,
            );
            // Only a landmark the correction moves says where it is drawn, and it is drawn
            // at its depth less the amount.
            match amount {
                Some(amount) => {
                    let drawn_at = point
                        .drawn_at_depth
                        .unwrap_or_else(|| panic!("{name} does not say where it is drawn"));
                    assert_close(drawn_at, depth - amount, 0.0015, name);
                    set_back += 1;
                }
                None => assert_eq!(point.drawn_at_depth, None, "{name} is not moved"),
            }
        }
        assert_eq!(set_back, if amounts.is_some() { 6 } else { 0 });
        // The two points of a left and right pair stand at one place, set back or not.
        let mut pairs = 0;
        for (name, point) in &side.landmarks {
            if name.contains("-left-") {
                let twin = side.landmarks[&name.replace("-left-", "-right-")];
                assert_eq!((point.x, point.y), (twin.x, twin.y), "{name} and its twin");
                pairs += 1;
            }
        }
        assert!(pairs > 20, "expected the head's pairs, found {pairs}");
    }
}

#[test]
fn the_named_correction_changes_the_side_view_and_no_other() {
    let flat = draw("no-correction", MADE_UP, &[]);
    let corrected = draw_made_up_with("with-correction", MADE_UP_SET_BACK, &[]);
    // The front view and the three-quarter view are the same bytes with it and without it.
    for view in ["front", "three-quarter"] {
        for name in [format!("{view}.svg"), format!("{view}.landmarks.toml")] {
            let without = std::fs::read(flat.join(&name)).expect("the file without");
            let with = std::fs::read(corrected.join(&name)).expect("the file with");
            assert!(without == with, "the correction changed {name}");
        }
        assert!(
            !landmark_file(&corrected, view)
                .table
                .contains_key("corrections"),
            "the {view} view says it carries a correction"
        );
    }
    // A set that carries no correction says nothing of one in its side view.
    assert!(
        !landmark_file(&flat, "side")
            .table
            .contains_key("corrections")
    );
    // The side view's landmark file names the correction, with each amount in the SVG's own
    // units, 200 to a unit, and the landmarks it moves.
    let side = landmark_file(&corrected, "side");
    let carried = side.table["corrections"]
        .as_table()
        .expect("a [corrections] table");
    assert_eq!(
        carried.keys().collect::<Vec<_>>(),
        ["lower_face_set_back"],
        "the corrections the side view carries"
    );
    let amounts = carried["lower_face_set_back"]
        .as_table()
        .expect("its amounts");
    assert_eq!(amounts.len(), SET_BACK.len());
    for ((amount, moved), expected) in SET_BACK.iter().zip([12.5, 7.5]) {
        let entry = amounts[*amount].as_table().expect("an amount");
        assert_eq!(support::number(&entry["amount"]), expected, "{amount}");
        let landmarks: Vec<&str> = entry["landmarks"]
            .as_array()
            .expect("the landmarks it moves")
            .iter()
            .map(|name| name.as_str().expect("a name"))
            .collect();
        assert_eq!(landmarks, moved, "{amount}");
    }
    // Every landmark keeps its height and its depth in space.
    let flat_side = landmark_file(&flat, "side");
    for (name, point) in &side.landmarks {
        let before = flat_side.landmarks[name];
        assert_eq!((point.y, point.depth), (before.y, before.depth), "{name}");
    }
}

#[test]
fn a_named_correction_sets_the_lower_face_back_in_the_side_view() {
    // Worked by hand from the made-up set, at the default placement: a point at a depth stands
    // at x = 500 + 200 * (depth + 0.6375) in the side view. The correction sets the chin back
    // by 0.0625 and the mouth by 0.0375.
    let folder = draw_made_up_with("side-set-back", MADE_UP_SET_BACK, &[]);
    let side = landmark_file(&folder, "side");
    let at = |depth: f64| 500.0 + 200.0 * (depth + 0.6375);
    // Each: its depth in space, in units, and the amount it is set back by.
    for (name, depth, set_back) in [
        // chin_depth 0.1875, and jaw_depth 0.1621 for the jaw point that goes with the chin.
        ("chin", 0.1875, 0.0625),
        ("face-outline-right-jaw", 0.1621, 0.0625),
        ("face-outline-left-jaw", 0.1621, 0.0625),
        // mouth_centre_depth 0.2873, and mouth_end_depth 0.2129 for the ends that go with it.
        ("mouth-line-centre", 0.2873, 0.0375),
        ("mouth-line-right-end", 0.2129, 0.0375),
        ("mouth-line-left-end", 0.2129, 0.0375),
    ] {
        let point = side.point(name);
        // The landmark file still gives the point's depth in space,
        assert_close(
            point.depth.expect("a depth"),
            200.0 * depth,
            0.0011,
            &format!("the depth of {name}"),
        );
        // and beside it the depth the correction draws it at, which is where it stands.
        assert_close(
            point
                .drawn_at_depth
                .unwrap_or_else(|| panic!("{name} does not say where the correction draws it")),
            200.0 * (depth - set_back),
            0.0011,
            &format!("where {name} is drawn"),
        );
        assert_close(
            point.x,
            at(depth - set_back),
            0.0011,
            &format!("the x of {name}"),
        );
    }
    // The nose mark is above what the correction moves: nose_mark_bottom_depth 0.3377.
    let nose = side.point("nose-mark-bottom");
    assert_close(
        nose.x,
        at(0.3377),
        0.0011,
        "the x of the nose mark's bottom",
    );
    assert_eq!(nose.drawn_at_depth, None);
}

#[test]
fn the_correction_moves_no_landmark_above_the_mouth_and_bends_the_outline_beside_what_it_moves() {
    // The list of proportions says what the correction changes besides the landmarks it
    // moves. This holds the drawing to that statement, on the made-up set.
    let flat = draw("correction-reach-flat", MADE_UP, &[]);
    let corrected = draw_made_up_with("correction-reach", MADE_UP_SET_BACK, &[]);
    let (before, after) = (
        landmark_file(&flat, "side"),
        landmark_file(&corrected, "side"),
    );

    // Landmarks: only the six the correction names stand elsewhere.
    let moved: BTreeSet<&str> = after
        .landmarks
        .iter()
        .filter(|(name, point)| {
            (point.x, point.y) != (before.landmarks[*name].x, before.landmarks[*name].y)
        })
        .map(|(name, _)| name.as_str())
        .collect();
    let named: BTreeSet<&str> = SET_BACK
        .iter()
        .flat_map(|(_, landmarks)| landmarks.iter().copied())
        .collect();
    assert_eq!(moved, named, "the landmarks that stand elsewhere");

    // View points: two that are read off the face outline shift with its bend, though both
    // are above the mouth. Where the nose level line meets the outline's front goes forward,
    // and where the neck's front line starts goes up.
    let shifted: BTreeSet<&str> = after
        .view_points
        .iter()
        .filter(|(name, point)| **point != before.view_points[*name])
        .map(|(name, _)| name.as_str())
        .collect();
    assert_eq!(
        shifted,
        BTreeSet::from(["face-outline-front-at-nose-level", "neck-front-top"]),
        "the view points that stand elsewhere"
    );
    let mouth_level = after.point("mouth-line-centre").y;
    for name in &shifted {
        assert!(
            after.point(name).y < mouth_level,
            "{name} is not above the mouth"
        );
    }
    let (nose_before, nose_after) = (
        before.point("face-outline-front-at-nose-level"),
        after.point("face-outline-front-at-nose-level"),
    );
    assert!(nose_after.x > nose_before.x && nose_after.y == nose_before.y);
    let (neck_before, neck_after) = (
        before.point("neck-front-top"),
        after.point("neck-front-top"),
    );
    assert!(neck_after.y < neck_before.y && neck_after.x == neck_before.x);

    // Guides: the five built on a moved landmark or on one of those two view points.
    let (svg_before, svg_after) = (svg_text(&flat, "side"), svg_text(&corrected, "side"));
    let elements = |svg: &str| -> BTreeMap<String, String> {
        svg.lines()
            .filter(|line| !line.trim_start().starts_with("<g") && line.contains(" id=\""))
            .map(|line| {
                let id = line.split(" id=\"").nth(1).expect("an id");
                let id = id.split('"').next().expect("the id's end");
                (id.to_string(), line.trim().to_string())
            })
            .collect()
    };
    let (guides_before, guides_after) = (elements(&svg_before), elements(&svg_after));
    assert_eq!(guides_before.len(), SIDE_GUIDES.len());
    let redrawn: BTreeSet<&str> = guides_after
        .iter()
        .filter(|(name, element)| guides_before[*name] != **element)
        .map(|(name, _)| name.as_str())
        .collect();
    assert_eq!(
        redrawn,
        BTreeSet::from([
            "face-outline",
            "mouth-level-line",
            "mouth-line",
            "neck-front",
            "nose-level-line",
        ]),
        "the guides that are drawn differently"
    );

    // The face outline between the eye line and the nose mark's bottom, where no landmark is
    // moved: it leaves the eye line as before and arrives at the nose mark from elsewhere,
    // because a smooth curve's direction at a point follows the point's two neighbours, and
    // the nose mark's lower neighbour is the mouth.
    let stretch_to_nose = |svg: &str| -> Vec<f64> {
        let document = roxmltree::Document::parse(svg).expect("the SVG is XML");
        let outline = document
            .descendants()
            .find(|node| node.attribute("id") == Some("face-outline"))
            .expect("the face outline");
        let commands = path_commands(outline.attribute("d").expect("d"));
        // M at the skull's front, then a curve to the eye line, then the curve to the nose mark.
        let (command, values) = &commands[2];
        assert_eq!(*command, 'C');
        values.clone()
    };
    let (stretch_before, stretch_after) =
        (stretch_to_nose(&svg_before), stretch_to_nose(&svg_after));
    let nose = after.point("nose-mark-bottom");
    assert_eq!(
        stretch_after[4..],
        [nose.x, nose.y],
        "the stretch ends at the nose mark"
    );
    assert_eq!(
        stretch_after[4..],
        stretch_before[4..],
        "the nose mark stands where it did"
    );
    assert_eq!(
        stretch_after[..2],
        stretch_before[..2],
        "it leaves the eye line as before"
    );
    assert_ne!(
        stretch_after[2..4],
        stretch_before[2..4],
        "it arrives at the nose mark as before"
    );
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
            "--side-axis-x",
            "455.5",
        ],
    );
    let front = landmark_file(&folder, "front");
    let front_axis = front.frame("turn_axis_x");
    assert_eq!(front_axis, 481.25);
    for (view, turn, axis) in [
        ("three-quarter", TURN_DEGREES, 507.5),
        ("side", SIDE_TURN_DEGREES, 455.5),
    ] {
        let turned = landmark_file(&folder, view);
        let (sin, cos) = sin_cos(turn);
        let turned_axis = turned.frame("turn_axis_x");
        assert_eq!(turned_axis, axis, "{view}");
        // Depth is measured from the list's zero, the eyes; the turn axis stands at its own
        // depth, the same in every view.
        let axis_depth = turned.frame("turn_axis_depth");
        assert_eq!(front.frame("turn_axis_depth"), axis_depth, "{view}");
        for (name, point) in &front.landmarks {
            let depth = point.depth.unwrap_or_else(|| panic!("{name} has no depth"));
            let shown = turned.landmarks[name];
            assert_eq!(shown.depth, Some(depth), "{view}: {name} has two depths");
            // A level head turned about an upright axis, drawn flat: offset times the cosine
            // of the turn plus the depth in front of the axis times the sine. Each of the
            // four numbers is rounded to 0.001.
            let expected = turned_axis + (point.x - front_axis) * cos + (depth - axis_depth) * sin;
            assert_close(shown.x, expected, 0.0015, &format!("{view}: {name}"));
        }
    }
}

#[test]
fn depth_zero_is_the_midpoint_of_the_eye_openings_and_the_turn_axis_stands_at_the_skull_centre() {
    // Worked by hand from the made-up set at the default unit of 200: the eye openings'
    // centres are at depth zero, as `proportions/list.toml` defines it, and the skull's
    // centre, which the head turns about, at skull_centre_depth = -0.6375, so -127.5.
    let folder = draw("depth-zero", MADE_UP, &[]);
    for view in VIEWS {
        let file = landmark_file(&folder, view);
        for side in ["left", "right"] {
            let name = format!("eye-{side}-opening-centre");
            assert_eq!(file.point(&name).depth, Some(0.0), "{view}: {name}");
        }
        assert_eq!(file.frame("turn_axis_depth"), -127.5, "{view}");
        assert_eq!(file.point("skull-centre").depth, Some(-127.5), "{view}");
        // The chin is at chin_depth = 0.1875 in front of the eyes.
        assert_eq!(file.point("chin").depth, Some(37.5), "{view}");
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
fn the_side_views_guides_are_built_as_the_list_states_for_that_view() {
    // Worked by hand from the made-up set, at the default placement: one unit is 200, the eye
    // line is at y = 400 and the turn axis at x = 500, at skull_centre_depth = -0.6375. So a
    // point at a depth stands at x = 500 + 200 * (depth + 0.6375).
    let folder = draw("side-constructions", MADE_UP, &[]);
    let side = landmark_file(&folder, "side");
    let at = |depth: f64| 500.0 + 200.0 * (depth + 0.6375);
    let through = |guide: &str| -> Vec<&str> {
        side.guides[guide]
            .passes_through
            .iter()
            .map(String::as_str)
            .collect()
    };
    let stands = |name: &str, x: f64, y: f64| {
        let point = side.point(name);
        assert_close(point.x, x, 0.0011, &format!("the x of {name}"));
        assert_close(point.y, y, 0.0011, &format!("the y of {name}"));
    };

    // The face outline runs through the centre-line landmarks, from the skull's front down to
    // the chin, and then back and up through the jaw-line points, which are the side of the
    // head. The jaw point beside the chin belongs to the front of the face and is not on it.
    assert_eq!(
        through("face-outline"),
        [
            "skull-front",
            "centre-line-at-eye-level",
            "nose-mark-bottom",
            "mouth-line-centre",
            "chin",
            "jaw-line-right-at-mouth-level",
            "jaw-line-right-at-nose-level",
            "jaw-line-right-at-eye-level",
            "jaw-line-right-top",
        ]
    );
    // skull_radius 0.9379 in front of the skull's centre, at its height (rise 0.3623).
    stands("skull-front", 500.0 + 187.58, 400.0 - 72.46);
    // chin_depth 0.1875; nose_mark_bottom_depth 0.3377 at drop 0.4629;
    // jaw_line_depth_at_mouth_level -0.3881 at mouth_centre_drop 0.6871.
    stands("chin", at(0.1875), 600.0);
    stands("nose-mark-bottom", at(0.3377), 400.0 + 92.58);
    stands("jaw-line-right-at-mouth-level", at(-0.3881), 400.0 + 137.42);

    // The skull is the same circle, and its front is the landmark the face outline starts at.
    assert_eq!(side.guides["skull"].element, "circle");
    assert_eq!(
        through("skull"),
        ["skull-front", "skull-bottom", "skull-back", "skull-top"]
    );
    stands("skull-back", 500.0 - 187.58, 400.0 - 72.46);

    // The level lines span the face outline, from the jaw line at the back to its front.
    assert_eq!(
        through("eye-level-line"),
        ["jaw-line-right-at-eye-level", "centre-line-at-eye-level"]
    );
    assert_eq!(
        through("mouth-level-line"),
        ["jaw-line-right-at-mouth-level", "mouth-line-centre"]
    );
    assert_eq!(
        through("nose-level-line"),
        [
            "jaw-line-right-at-nose-level",
            "face-outline-front-at-nose-level"
        ]
    );
    // The centre line meets the eye line on the skull: sqrt(0.9379^2 - 0.3623^2) = 0.865098
    // in front of the skull's centre.
    stands("centre-line-at-eye-level", 500.0 + 200.0 * 0.865098, 400.0);
    // The nose level is midway between the nose mark's corners (0.3873) and its bottom
    // (0.4629): 0.4251. The outline's front there lies between the eye line and the nose
    // mark's bottom, so no further back than the first and no further forward than the second.
    let crossing = side.point("face-outline-front-at-nose-level");
    assert_close(crossing.y, 400.0 + 85.02, 0.0011, "the nose level");
    assert!(
        crossing.x > side.point("centre-line-at-eye-level").x
            && crossing.x < side.point("nose-mark-bottom").x,
        "the face outline's front at the nose level is at x = {}",
        crossing.x
    );

    // The side eye is the same opening at its depths: the inner corner half the wrap (0.1375)
    // in front of depth zero and the outer corner half behind.
    assert_eq!(
        through("eye-opening-right"),
        [
            "eye-right-outer-corner",
            "eye-right-opening-top",
            "eye-right-inner-corner",
            "eye-right-opening-bottom",
        ]
    );
    stands(
        "eye-right-inner-corner",
        at(0.06875),
        400.0 + 200.0 * 0.0379,
    );
    stands(
        "eye-right-outer-corner",
        at(-0.06875),
        400.0 - 200.0 * 0.0121,
    );

    // The iris is a flat disc facing forward, so the side view shows it edge on: an upright
    // line at iris_depth -0.0377, from its top to its bottom, iris_height 0.2121 about
    // iris_centre_drop 0.0123.
    assert_eq!(side.guides["iris-right"].element, "line");
    assert_eq!(
        through("iris-right"),
        ["iris-right-top", "iris-right-bottom"]
    );
    stands("iris-right-top", at(-0.0377), 400.0 + 2.46 - 21.21);
    stands("iris-right-bottom", at(-0.0377), 400.0 + 2.46 + 21.21);

    // The nose mark and the mouth line: each from its near end to the centre line.
    assert_eq!(side.guides["nose-mark"].element, "line");
    assert_eq!(
        through("nose-mark"),
        ["nose-mark-right-corner", "nose-mark-bottom"]
    );
    assert_eq!(
        through("mouth-line"),
        ["mouth-line-right-end", "mouth-line-centre"]
    );

    // The ear is the same lobe at its depths.
    assert_eq!(
        through("ear-right"),
        [
            "ear-right-base-outer",
            "ear-right-outer-extent",
            "ear-right-top",
            "ear-right-inner-extent",
            "ear-right-base-inner",
        ]
    );
    stands("ear-right-base-outer", at(-0.7621), 400.0 - 187.46);
    stands("ear-right-base-inner", at(-0.5877), 400.0 - 197.42);

    // The neck is the same upright column: neck_half_width 0.4373 to each side of neck_depth
    // -0.8879, down to neck_base_drop 1.1121.
    assert_eq!(through("neck-back"), ["neck-back-top", "neck-back-base"]);
    assert_eq!(through("neck-front"), ["neck-front-top", "neck-front-base"]);
    stands("neck-back-base", at(-0.8879 - 0.4373), 400.0 + 222.42);
    stands("neck-front-base", at(-0.8879 + 0.4373), 400.0 + 222.42);
    assert_eq!(
        side.point("neck-back-top").x,
        side.point("neck-back-base").x
    );
    assert_eq!(
        side.point("neck-front-top").x,
        side.point("neck-front-base").x
    );
    assert!(side.point("neck-front-top").y < side.point("neck-front-base").y);

    // The shoulder line is seen end on, at the one point both shoulder points stand at:
    // shoulder_depth -0.9129, shoulder_drop 1.8373. It is drawn as a level tick through that
    // point, half the neck's width to each side.
    assert_eq!(
        through("shoulder-line"),
        ["shoulder-line-back-end", "shoulder-line-front-end"]
    );
    stands("shoulder-right-point", at(-0.9129), 400.0 + 367.46);
    stands(
        "shoulder-line-back-end",
        at(-0.9129) - 87.46,
        400.0 + 367.46,
    );
    stands(
        "shoulder-line-front-end",
        at(-0.9129) + 87.46,
        400.0 + 367.46,
    );

    // The hair volume: the same dome, hair_half_depth 1.1629 to each side of hair_centre_depth
    // -0.7113 and hair_top_rise 1.3127 high, down to the height of the skull's centre. Behind
    // the head its side runs on down to the chin's height; in front it stops, so that it does
    // not cross the face.
    assert_eq!(
        through("hair-volume"),
        [
            "hair-back-end",
            "hair-back-extent",
            "hair-top",
            "hair-front-extent",
        ]
    );
    stands("hair-top", at(-0.7113), 400.0 - 262.54);
    stands("hair-back-end", at(-0.7113) - 232.58, 600.0);
    stands("hair-back-extent", at(-0.7113) - 232.58, 400.0 - 72.46);
    stands("hair-front-extent", at(-0.7113) + 232.58, 400.0 - 72.46);
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
fn left_and_right_in_a_name_are_the_characters_own_in_the_front_and_three_quarter_views() {
    // In the side view the two points of a pair stand at one place, which
    // `a_landmarks_sideways_position_in_the_side_view_is_its_depth` checks.
    let folder = draw("own-left-and-right", MADE_UP, &[]);
    for view in ["front", "three-quarter"] {
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
    for view in VIEWS {
        let (guides, groups) = (guides_of(view), groups_of(view));
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
    let placement = ["--unit", "173.3"];
    guides_pass_through_their_points(&draw("guides-match-landmarks", MADE_UP, &placement));
    // With the named correction, a guide is built on its landmarks where the view draws them.
    guides_pass_through_their_points(&draw_made_up_with(
        "guides-match-set-back",
        MADE_UP_SET_BACK,
        &placement,
    ));
}

fn guides_pass_through_their_points(folder: &Path) {
    for view in VIEWS {
        let svg = svg_text(folder, view);
        let document = roxmltree::Document::parse(&svg).expect("the SVG is XML");
        let file = landmark_file(folder, view);
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

/// The corrections a set of proportions carries, as its file gives them: each amount in units.
fn corrections_of(path: &Path) -> BTreeMap<String, BTreeMap<String, f64>> {
    let table: toml::Table = text(path).parse().expect("the set of proportions is TOML");
    let Some(corrections) = table.get("corrections") else {
        return BTreeMap::new();
    };
    corrections
        .as_table()
        .expect("a [corrections] table")
        .iter()
        .map(|(name, amounts)| {
            let amounts = amounts
                .as_table()
                .expect("a correction's amounts")
                .iter()
                .map(|(amount, value)| (amount.clone(), support::number(value)))
                .collect();
            (name.clone(), amounts)
        })
        .collect()
}

/// Takes the landmark files of a drawn folder back into a set of proportions, with the side
/// view's file when `with_side` is set, and returns the set's path.
fn take_back(folder: &Path, with_side: bool) -> PathBuf {
    let taken_back = folder.join(if with_side {
        "taken-back-with-side.toml"
    } else {
        "taken-back.toml"
    });
    let (front, three_quarter, side) = (
        folder.join("front.landmarks.toml"),
        folder.join("three-quarter.landmarks.toml"),
        folder.join("side.landmarks.toml"),
    );
    let mut arguments = vec![
        "proportions-from-landmarks",
        "--front",
        path_text(&front),
        "--three-quarter",
        path_text(&three_quarter),
        "--out",
        path_text(&taken_back),
    ];
    if with_side {
        arguments.extend(["--side", path_text(&side)]);
    }
    support::maquette_ok(&arguments);
    taken_back
}

#[test]
fn a_correction_the_side_view_carries_is_taken_back_with_the_proportions() {
    // The made-up set with its made-up correction: the chin set back by 0.0625 of a unit and
    // the mouth by 0.0375. At a unit of 173.3 the side view's landmark file gives them as
    // 10.831 and 6.499 SVG units, which is 0.0625 and 0.0375 again to within 0.00001.
    let placement = ["--unit", "173.3", "--eye-line-y", "412.7"];
    let folder = draw_made_up_with("taken-back-correction", MADE_UP_SET_BACK, &placement);
    let came_back = take_back(&folder, true);
    let corrections = corrections_of(&came_back);
    assert_eq!(
        corrections.keys().collect::<Vec<_>>(),
        ["lower_face_set_back"],
        "the corrections taken back"
    );
    let amounts = &corrections["lower_face_set_back"];
    assert_eq!(amounts.keys().collect::<Vec<_>>(), ["chin", "mouth"]);
    assert_close(amounts["chin"], 0.0625, 1e-4, "the chin's amount");
    assert_close(amounts["mouth"], 0.0375, 1e-4, "the mouth's amount");
    // The proportions come back as they do without the side view.
    let put_in = support::proportions(&support::repository().join(MADE_UP));
    for (name, value) in &put_in {
        assert_close(support::proportions(&came_back)[name], *value, 1e-4, name);
    }

    // Drawn again, the set taken back puts the chin and the mouth where the first drawing
    // has them in the side view: to within 0.05 SVG unit, because each proportion came back
    // to 0.0001 of a unit of 173.3 and a sideways position there is the sum of two.
    let again = draw(
        "taken-back-correction-again",
        path_text(&came_back),
        &placement,
    );
    let (first, second) = (
        landmark_file(&folder, "side"),
        landmark_file(&again, "side"),
    );
    for name in ["chin", "mouth-line-centre", "nose-mark-bottom"] {
        assert_close(
            second.point(name).x,
            first.point(name).x,
            0.05,
            &format!("the x of {name} drawn again"),
        );
    }
    assert!(second.table.contains_key("corrections"));
}

#[test]
fn a_side_view_that_carries_no_correction_gives_none_back() {
    let folder = draw("taken-back-flat", MADE_UP, &[]);
    let came_back = take_back(&folder, true);
    assert!(corrections_of(&came_back).is_empty());
    assert!(
        !text(&came_back).contains("not taken back"),
        "the side view was given, and the file says a correction was not taken back"
    );
}

#[test]
fn without_the_side_view_the_set_taken_back_says_that_no_correction_was() {
    // A correction stands in the side view's landmark file and in no other, so two files
    // cannot give it back. The set must say so and not pass for the whole set put in.
    let folder = draw_made_up_with("taken-back-without-side", MADE_UP_SET_BACK, &[]);
    let came_back = take_back(&folder, false);
    assert!(corrections_of(&came_back).is_empty());
    let said = text(&came_back);
    assert!(
        said.contains("No side view was given") && said.contains("not taken back"),
        "{said}"
    );
}

#[test]
fn a_file_given_as_the_side_view_that_is_another_views_is_refused() {
    let folder = draw_made_up_with("taken-back-wrong-side", MADE_UP_SET_BACK, &[]);
    let (front, three_quarter) = (
        folder.join("front.landmarks.toml"),
        folder.join("three-quarter.landmarks.toml"),
    );
    let output = support::maquette(&[
        "proportions-from-landmarks",
        "--front",
        path_text(&front),
        "--three-quarter",
        path_text(&three_quarter),
        "--side",
        path_text(&three_quarter),
    ]);
    assert!(!output.status.success());
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(
        message.contains("three-quarter.landmarks.toml")
            && message.contains("three-quarter view, not of a side view"),
        "{message}"
    );
    assert!(output.stdout.is_empty(), "a set was written all the same");
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

#[test]
fn a_correction_the_list_does_not_define_or_with_a_wrong_amount_is_refused_in_plain_words() {
    let folder = scratch("refused-corrections");
    let whole = text(&support::repository().join(MADE_UP));
    for (name, corrections, says) in [
        // A correction is named and defined in the list, like a proportion.
        (
            "unknown",
            "[corrections.longer_nose]\ntip = 0.1\n",
            vec!["longer_nose", "no such correction"],
        ),
        // It gives every one of its amounts,
        (
            "lacking",
            "[corrections.lower_face_set_back]\nchin = 0.0625\n",
            vec!["lower_face_set_back", "lacks", "mouth"],
        ),
        // and no other,
        (
            "stray",
            "[corrections.lower_face_set_back]\nchin = 0.0625\nmouth = 0.0375\nnose = 0.01\n",
            vec!["lower_face_set_back", "nose"],
        ),
        // and a set-back does not push forward.
        (
            "forward",
            "[corrections.lower_face_set_back]\nchin = -0.0625\nmouth = 0.0375\n",
            vec!["lower_face_set_back", "chin", "below zero"],
        ),
    ] {
        let set = folder.join(format!("{name}.toml"));
        std::fs::write(&set, format!("{whole}\n{corrections}")).expect("written");
        let out = folder.join(format!("{name}-out"));
        let output = maquette(&[
            "draw",
            "--proportions",
            path_text(&set),
            "--out",
            path_text(&out),
        ]);
        assert!(!output.status.success(), "{name} was drawn");
        let message = String::from_utf8_lossy(&output.stderr);
        for word in says {
            assert!(message.contains(word), "{name}: {message}");
        }
        assert!(
            !out.exists(),
            "{name}: files were written for a refused set"
        );
    }
}
