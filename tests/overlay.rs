//! The hand-made views laid over a generated armature, through the command line: two landmark
//! files and a record in, one overlay SVG per view out, drawn in the generated view's own
//! units so that it can be placed exactly over the generated SVG on a review sheet.
//!
//! The first tests run on the made-up files in `tests/fixtures/differences/`. Every expected
//! position is worked by hand from the lay-over their comments state: the front file is
//! drawn at half the record's size, so a record position x, y stands at
//! 250 + (x - 500) / 2, 150 + (y - 300) / 2 in it; the three-quarter file is at the record's
//! size, shifted, so a record position stands at x - 15, y + 10. The last test runs on Saffi
//! and checks that her overlay carries the numbers her differences file gives.

mod support;

use std::path::{Path, PathBuf};

use support::{assert_close, maquette, maquette_ok, path_text, scratch, text};

const MADE_UP_RECORD: &str = "tests/fixtures/differences/made-up-hand-made-views.toml";
const MADE_UP_FRONT: &str = "tests/fixtures/differences/made-up-front.landmarks.toml";
const MADE_UP_THREE_QUARTER: &str =
    "tests/fixtures/differences/made-up-three-quarter.landmarks.toml";

/// Writes the overlays of two landmark files into a fresh folder and returns it.
fn overlays(name: &str, front: &str, three_quarter: &str, record: &str) -> PathBuf {
    let folder = scratch(name);
    maquette_ok(&[
        "overlay-hand-made-views",
        "--front",
        front,
        "--three-quarter",
        three_quarter,
        "--record",
        record,
        "--out",
        path_text(&folder),
    ]);
    folder
}

fn made_up(name: &str) -> PathBuf {
    overlays(name, MADE_UP_FRONT, MADE_UP_THREE_QUARTER, MADE_UP_RECORD)
}

fn number_attribute(node: roxmltree::Node, name: &str) -> f64 {
    node.attribute(name)
        .unwrap_or_else(|| panic!("<{}> has no {name}", node.tag_name().name()))
        .parse()
        .unwrap_or_else(|_| panic!("{name} is a number"))
}

/// One view's overlay, read.
struct Overlay {
    width: f64,
    height: f64,
    view_box: String,
    /// The record's name of each hand-made landmark marked, where the mark stands, and
    /// whether it is marked as covered by the head.
    marks: Vec<(String, [f64; 2], bool)>,
    /// Each numbered difference: its number as written on the drawing, the two ends of the
    /// line between the generated and the hand-made position, and its badge's centre and
    /// radius.
    numbered: Vec<Numbered>,
}

struct Numbered {
    number: String,
    generated: [f64; 2],
    hand_made: [f64; 2],
    badge: [f64; 2],
    radius: f64,
}

fn read(path: &Path) -> Overlay {
    let source = text(path);
    let document = roxmltree::Document::parse(&source).expect("the overlay is XML");
    let root = document.root_element();
    let group = |id: &str| {
        root.descendants()
            .find(|node| node.attribute("id") == Some(id))
            .unwrap_or_else(|| panic!("no group `{id}` in {}", path.display()))
    };
    let marks = group("hand-made-landmarks")
        .children()
        .filter(|node| node.is_element())
        .map(|node| {
            let id = node.attribute("id").expect("a mark has an id");
            let name = id
                .strip_prefix("hand-made-")
                .unwrap_or_else(|| panic!("`{id}` is named hand-made-<the record's name>"));
            let at = [number_attribute(node, "cx"), number_attribute(node, "cy")];
            (
                name.to_string(),
                at,
                node.attribute("class") == Some("covered"),
            )
        })
        .collect();
    let numbered = group("differences")
        .children()
        .filter(|node| node.is_element())
        .map(|difference| {
            let element = |class: &str| {
                difference
                    .children()
                    .find(|node| node.attribute("class") == Some(class))
                    .unwrap_or_else(|| panic!("a numbered difference has a `{class}`"))
            };
            let apart = element("apart");
            let badge = element("badge");
            let label = element("number");
            Numbered {
                number: label.text().expect("the number is written").to_string(),
                generated: [number_attribute(apart, "x1"), number_attribute(apart, "y1")],
                hand_made: [number_attribute(apart, "x2"), number_attribute(apart, "y2")],
                badge: [number_attribute(badge, "cx"), number_attribute(badge, "cy")],
                radius: number_attribute(badge, "r"),
            }
        })
        .collect();
    Overlay {
        width: number_attribute(root, "width"),
        height: number_attribute(root, "height"),
        view_box: root.attribute("viewBox").expect("a viewBox").to_string(),
        marks,
        numbered,
    }
}

fn mark<'a>(overlay: &'a Overlay, name: &str) -> &'a (String, [f64; 2], bool) {
    let found: Vec<_> = overlay.marks.iter().filter(|mark| mark.0 == name).collect();
    assert_eq!(found.len(), 1, "`{name}` is marked once");
    found[0]
}

fn assert_at(actual: [f64; 2], expected: [f64; 2], what: &str) {
    assert_close(actual[0], expected[0], 0.0005, &format!("{what}, x"));
    assert_close(actual[1], expected[1], 0.0005, &format!("{what}, y"));
}

#[test]
fn each_overlay_has_its_generated_views_size() {
    let folder = made_up("overlay-size");
    let front = read(&folder.join("front.hand-made-overlay.svg"));
    assert_eq!((front.width, front.height), (500.0, 500.0));
    assert_eq!(front.view_box, "0 0 500 500");
    let three_quarter = read(&folder.join("three-quarter.hand-made-overlay.svg"));
    assert_eq!(
        (three_quarter.width, three_quarter.height),
        (1000.0, 1000.0)
    );
    assert_eq!(three_quarter.view_box, "0 0 1000 1000");
}

#[test]
fn every_recorded_landmark_is_marked_where_the_lay_over_puts_it_in_the_views_units() {
    let folder = made_up("overlay-marks");
    let front = read(&folder.join("front.hand-made-overlay.svg"));
    // The record gives the front view 17 positions, read or not: a pupil is marked too.
    assert_eq!(front.marks.len(), 17);
    assert_at(
        mark(&front, "eye_right_outer_corner").1,
        [175.0, 150.0],
        "350, 300",
    );
    assert_at(
        mark(&front, "shoulder_left_point").1,
        [400.0, 350.0],
        "800, 700",
    );
    assert_at(
        mark(&front, "hair_right_extent").1,
        [150.0, 111.0],
        "300, 222",
    );
    assert_at(
        mark(&front, "pupil_left_centre").1,
        [300.0, 150.0],
        "600, 300",
    );

    let three_quarter = read(&folder.join("three-quarter.hand-made-overlay.svg"));
    assert_eq!(three_quarter.marks.len(), 10);
    assert_at(mark(&three_quarter, "chin").1, [555.0, 510.0], "570, 500");
    assert_at(
        mark(&three_quarter, "lid_line_left_inner_upper").1,
        [585.0, 310.0],
        "600, 300",
    );
}

#[test]
fn a_position_the_head_covers_is_marked_as_covered_and_no_other() {
    let folder = made_up("overlay-covered");
    let three_quarter = read(&folder.join("three-quarter.hand-made-overlay.svg"));
    let covered: Vec<&str> = three_quarter
        .marks
        .iter()
        .filter(|mark| mark.2)
        .map(|mark| mark.0.as_str())
        .collect();
    assert_eq!(covered, ["ear_left_base_outer"]);
    assert_at(
        mark(&three_quarter, "ear_left_base_outer").1,
        [625.0, 190.0],
        "640, 180",
    );
    let front = read(&folder.join("front.hand-made-overlay.svg"));
    assert!(front.marks.iter().all(|mark| !mark.2));
}

#[test]
fn every_difference_above_one_percent_is_numbered_between_its_two_positions() {
    let folder = made_up("overlay-numbered");
    let front = read(&folder.join("front.hand-made-overlay.svg"));
    let numbers: Vec<&str> = front.numbered.iter().map(|n| n.number.as_str()).collect();
    assert_eq!(numbers, ["1", "2", "3", "4"]);
    // 1: the left shoulder point, generated 840, 700 laid over, hand-made 800, 700.
    assert_at(front.numbered[0].generated, [420.0, 350.0], "1, generated");
    assert_at(front.numbered[0].hand_made, [400.0, 350.0], "1, hand-made");
    // 3: the right eye's outer corner, generated 353, 304 laid over, hand-made 350, 300.
    assert_at(front.numbered[2].generated, [176.5, 152.0], "3, generated");
    assert_at(front.numbered[2].hand_made, [175.0, 150.0], "3, hand-made");

    let three_quarter = read(&folder.join("three-quarter.hand-made-overlay.svg"));
    let numbers: Vec<&str> = three_quarter
        .numbered
        .iter()
        .map(|n| n.number.as_str())
        .collect();
    assert_eq!(numbers, ["5", "6", "7"]);
    // 5: the far ear's outer base, at 625, 198 in the landmark file; the record's 640, 180.
    assert_at(
        three_quarter.numbered[0].generated,
        [625.0, 198.0],
        "5, generated",
    );
    assert_at(
        three_quarter.numbered[0].hand_made,
        [625.0, 190.0],
        "5, hand-made",
    );
    // 6: a lid line's inner end is midway between the band's two corners: 600, 305.
    assert_at(
        three_quarter.numbered[1].hand_made,
        [585.0, 315.0],
        "6, hand-made",
    );
}

#[test]
fn the_numbers_stand_inside_the_view_and_clear_of_one_another() {
    let folder = made_up("overlay-badges");
    for view in ["front", "three-quarter"] {
        let overlay = read(&folder.join(format!("{view}.hand-made-overlay.svg")));
        badges_clear(&overlay, view);
    }
}

fn badges_clear(overlay: &Overlay, view: &str) {
    for (index, first) in overlay.numbered.iter().enumerate() {
        let [x, y] = first.badge;
        let r = first.radius;
        assert!(
            x - r >= 0.0 && y - r >= 0.0 && x + r <= overlay.width && y + r <= overlay.height,
            "{view}: number {} stands outside the view",
            first.number
        );
        for second in &overlay.numbered[index + 1..] {
            let apart = (x - second.badge[0]).hypot(y - second.badge[1]);
            assert!(
                apart >= r + second.radius,
                "{view}: numbers {} and {} overlap",
                first.number,
                second.number
            );
        }
    }
}

#[test]
fn the_same_files_give_the_same_bytes() {
    let first = made_up("overlay-bytes-1");
    let second = made_up("overlay-bytes-2");
    for name in [
        "front.hand-made-overlay.svg",
        "three-quarter.hand-made-overlay.svg",
    ] {
        assert_eq!(text(&first.join(name)), text(&second.join(name)), "{name}");
    }
}

#[test]
fn landmark_files_given_for_the_wrong_view_are_refused() {
    let folder = scratch("overlay-wrong-view");
    let output = maquette(&[
        "overlay-hand-made-views",
        "--front",
        MADE_UP_THREE_QUARTER,
        "--three-quarter",
        MADE_UP_FRONT,
        "--record",
        MADE_UP_RECORD,
        "--out",
        path_text(&folder),
    ]);
    assert!(!output.status.success());
    assert!(!folder.join("front.hand-made-overlay.svg").exists());
}

#[test]
fn saffis_overlays_carry_the_numbers_of_her_differences_file() {
    let drawn = support::draw("overlay-saffi-drawn", "data/saffi/proportions.toml", &[]);
    let front = drawn.join("front.landmarks.toml");
    let three_quarter = drawn.join("three-quarter.landmarks.toml");
    let record = "data/saffi/hand-made-views.toml";
    let differences_file = drawn.join("differences.toml");
    maquette_ok(&[
        "differences-from-hand-made-views",
        "--front",
        path_text(&front),
        "--three-quarter",
        path_text(&three_quarter),
        "--record",
        record,
        "--out",
        path_text(&differences_file),
    ]);
    let differences: toml::Table = text(&differences_file).parse().expect("TOML");
    let folder = overlays(
        "overlay-saffi",
        path_text(&front),
        path_text(&three_quarter),
        record,
    );
    let hand_made: toml::Table = text(&support::repository().join(record))
        .parse()
        .expect("TOML");

    for (view, table, record_view) in [
        ("front", "front", "front"),
        ("three-quarter", "three-quarter", "three_quarter"),
    ] {
        let overlay = read(&folder.join(format!("{view}.hand-made-overlay.svg")));
        let listed = differences[table]["above_one_percent"]
            .as_array()
            .expect("a list");
        let numbers: Vec<String> = listed
            .iter()
            .map(|entry| entry["number"].as_integer().expect("a number").to_string())
            .collect();
        let drawn: Vec<String> = overlay.numbered.iter().map(|n| n.number.clone()).collect();
        assert_eq!(drawn, numbers, "{view}: the numbers on the drawing");
        badges_clear(&overlay, view);

        // Laid over by the differences file's own lay-over, each end is the position the
        // file lists.
        let lay_over = &differences[table]["lay_over"];
        let scale = support::number(&lay_over["scale"]);
        let shift = [
            support::number(&lay_over["shift_x"]),
            support::number(&lay_over["shift_y"]),
        ];
        let laid = |at: [f64; 2]| [shift[0] + scale * at[0], shift[1] + scale * at[1]];
        for (entry, numbered) in listed.iter().zip(&overlay.numbered) {
            let listed_at = |key: &str| {
                [
                    support::number(&entry[key]["x"]),
                    support::number(&entry[key]["y"]),
                ]
            };
            let what = format!("{view} number {}", numbered.number);
            let close = |a: [f64; 2], b: [f64; 2], end: &str| {
                assert_close(a[0], b[0], 0.002, &format!("{what}, {end}, x"));
                assert_close(a[1], b[1], 0.002, &format!("{what}, {end}, y"));
            };
            close(
                laid(numbered.generated),
                listed_at("generated"),
                "generated",
            );
            close(
                laid(numbered.hand_made),
                listed_at("hand_made"),
                "hand-made",
            );
        }

        // Every position the record gives the view is marked, and laid over it stands where
        // the record puts it.
        let recorded = hand_made["views"][record_view]["landmarks"]
            .as_table()
            .expect("the record's landmarks");
        assert_eq!(
            overlay.marks.len(),
            recorded.len(),
            "{view}: every position"
        );
        for (name, at, _) in &overlay.marks {
            let entry = &recorded[name.as_str()];
            let expected = [support::number(&entry["x"]), support::number(&entry["y"])];
            let actual = laid(*at);
            assert_close(actual[0], expected[0], 0.002, &format!("{view} {name}, x"));
            assert_close(actual[1], expected[1], 0.002, &format!("{view} {name}, y"));
        }
    }

    let three_quarter = read(&folder.join("three-quarter.hand-made-overlay.svg"));
    let mut covered: Vec<&str> = three_quarter
        .marks
        .iter()
        .filter(|mark| mark.2)
        .map(|mark| mark.0.as_str())
        .collect();
    covered.sort();
    assert_eq!(
        covered,
        [
            "ear_left_base_inner",
            "ear_left_base_outer",
            "ear_left_inner_extent"
        ]
    );
}
