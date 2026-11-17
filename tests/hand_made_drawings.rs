//! Reading a hand-made drawing, through the command line: an SVG in, a list of its groups and
//! shapes out; and the pinned drawings in, the records of `data/saffi/` out.
//!
//! The drawing used for the listing is made up (`tests/fixtures/drawings/made-up-drawing.svg`)
//! and says so. Saffi's drawings are private and no clean clone has them, so the tests that
//! need them are marked `#[ignore]` and say where they look. The others check what a clean
//! clone can: that the command refuses what it must, and that the numbers the final record
//! derives follow from the positions it and her record give.

mod support;

use std::collections::BTreeMap;

use support::{
    MADE_UP, assert_close, draw, maquette, maquette_ok, number, path_text, repository, scratch,
    text,
};

const MADE_UP_DRAWING: &str = "tests/fixtures/drawings/made-up-drawing.svg";
const RECORD: &str = "data/saffi/hand-made-views.toml";
const FINAL_RECORD: &str = "data/saffi/final-drawings.toml";

/// The digest recorded for her S2 front underdrawing.
const FRONT_SHA256: &str = "34ee7b92a8422031a20f7d2f195d6952d16e1099de3002f5fffcb6697a57f76f";

#[test]
fn a_drawing_is_listed_as_its_groups_and_shapes_with_their_points() {
    let listed = maquette_ok(&["list-drawing", "--svg", MADE_UP_DRAWING]);
    let expected = "\
viewBox: 0 0 100 100
Each shape is listed under its address: the id of its group, a slash, and its name.
Nodes are numbered from 0 in path order; segment k runs from node k to node k + 1.

group head
  circle head/skull  (filled)
      centre 50, 40, radius 30
  group eye_R
    path eye_R/white  (filled)
        node 0: 20, 40
        node 1: 40, 40  (curved from the node before)
        node 2: 20, 40  (curved from the node before)
        closed
        reaches x 20 to 40, y 32.5 to 47.5
    path eye_R/lid  (not filled)
        node 0: 20, 40
        node 1: 30, 34  (straight from the node before)
        node 2: 40, 40  (straight from the node before)
        reaches x 20 to 40, y 34 to 40
  line head/mouth  (not filled)
      node 0: 40, 70
      node 1: 60, 70  (straight from the node before)
      reaches x 40 to 60, y 70 to 70
  polygon head/patch  (filled, opacity 0)
      node 0: 60, 20
      node 1: 70, 20  (straight from the node before)
      node 2: 65, 30  (straight from the node before)
      closed
      reaches x 60 to 70, y 20 to 30
";
    // Worked by hand from the made-up drawing. The opening's upper curve runs from 20, 40 to
    // 40, 40 with both handles at y = 30, so it is highest halfway, at (40 + 3 * 30 + 3 * 30 +
    // 40) / 8 = 32.5, and its lower curve mirrors it at 47.5. A shape is named by its
    // `data-name` where it has one: `lid`, not `lid-2`.
    assert_eq!(listed, expected);
}

#[test]
fn a_drawing_that_moves_its_coordinates_or_curves_another_way_is_refused_in_plain_words() {
    let folder = scratch("drawings-refused");
    let made_up = text(&repository().join(MADE_UP_DRAWING));

    let moved = folder.join("moved.svg");
    std::fs::write(
        &moved,
        made_up.replace(
            "<g id=\"eye_R\">",
            "<g id=\"eye_R\" transform=\"translate(5 0)\">",
        ),
    )
    .expect("written");
    let output = maquette(&["list-drawing", "--svg", path_text(&moved)]);
    assert!(!output.status.success());
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(
        message.contains("eye_R") && message.contains("transform"),
        "{message}"
    );
    assert!(output.stdout.is_empty(), "a refused drawing was listed");

    let arc = folder.join("arc.svg");
    std::fs::write(
        &arc,
        made_up.replace("M20 40 L30 34 L40 40", "M20 40 A10 10 0 0 1 40 40"),
    )
    .expect("written");
    let output = maquette(&["list-drawing", "--svg", path_text(&arc)]);
    assert!(!output.status.success());
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(
        message.contains("lid-2") && message.contains("`A`"),
        "{message}"
    );
}

#[test]
fn a_path_that_goes_on_after_closing_without_a_new_start_is_refused() {
    // SVG lets a path draw on after `Z` from the subpath's first point, as a new subpath that
    // no `M` starts. The reading does not follow that, so it refuses the path: listing it as
    // one closed outline would put the stretch after `Z` on the wrong side of the closing.
    let folder = scratch("drawings-on-after-closing");
    let made_up = text(&repository().join(MADE_UP_DRAWING));
    let closed = "M20 40 C20 30 40 30 40 40 C40 50 20 50 20 40 Z";
    assert!(made_up.contains(closed), "the made-up drawing has changed");

    for (name, more) in [("line", " L30 45"), ("curve", " C25 41 28 43 30 45")] {
        let drawing = folder.join(format!("{name}.svg"));
        std::fs::write(
            &drawing,
            made_up.replace(closed, &format!("{closed}{more}")),
        )
        .expect("written");
        let output = maquette(&["list-drawing", "--svg", path_text(&drawing)]);
        assert!(!output.status.success(), "{name}: the path was read");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(
            message.contains("white") && message.contains("`Z`") && message.contains("`M`"),
            "{name}: {message}"
        );
        assert!(output.stdout.is_empty(), "a refused drawing was listed");
    }

    // A new `M` after `Z` starts a second subpath, which is read.
    let drawing = folder.join("two-subpaths.svg");
    std::fs::write(
        &drawing,
        made_up.replace(closed, &format!("{closed} M25 40 L35 40 L30 45 Z")),
    )
    .expect("written");
    let listed = maquette_ok(&["list-drawing", "--svg", path_text(&drawing)]);
    assert!(
        listed.contains("subpath 1\n        node 0: 25, 40\n"),
        "{listed}"
    );
}

/// Lists a drawing written for one test, and returns the line that names each shape.
fn shapes_listed(name: &str, svg: &str) -> Vec<String> {
    let folder = scratch(name);
    let drawing = folder.join("drawing.svg");
    std::fs::write(&drawing, svg).expect("written");
    maquette_ok(&["list-drawing", "--svg", path_text(&drawing)])
        .lines()
        .map(str::trim)
        .filter(|line| line.ends_with(')') && line.contains("  ("))
        .map(str::to_string)
        .collect()
}

#[test]
fn an_armature_maquette_draws_is_listed_with_no_guide_filled() {
    // The SVG contract has every guide stroked and unfilled, and the generator says so once,
    // on the layer's group. A listing that called the guides filled would send a reading to
    // look for cover that is not there.
    let folder = draw("drawings-own-armature", MADE_UP, &[]);
    for view in ["front", "three-quarter", "side"] {
        let svg = folder.join(format!("{view}.svg"));
        let listed = maquette_ok(&["list-drawing", "--svg", path_text(&svg)]);
        let filled: Vec<&str> = listed
            .lines()
            .filter(|line| line.contains("(filled"))
            .collect();
        assert!(
            listed.contains("(not filled)"),
            "the {view} view lists no shape"
        );
        assert!(
            filled.is_empty(),
            "the {view} view has guides listed as filled: {filled:?}"
        );
    }
}

#[test]
fn a_shape_takes_its_fill_from_the_group_it_is_in_unless_it_gives_its_own() {
    // A made-up drawing: nobody is drawn in it. SVG hands a `fill` down from a group to the
    // shapes inside it, and a shape's own `fill` stands over the group's.
    let listed = shapes_listed(
        "drawings-fill-handed-down",
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
  <g id="lines" fill="none" stroke="#000000">
    <circle id="ring" cx="50" cy="50" r="30"/>
    <g id="inner">
      <rect id="frame" x="10" y="10" width="20" height="20"/>
      <rect id="plate" x="40" y="40" width="20" height="20" fill="#cccccc"/>
    </g>
  </g>
  <g id="paint">
    <circle id="disc" cx="50" cy="50" r="10"/>
  </g>
</svg>
"##,
    );
    assert_eq!(
        listed,
        [
            "circle lines/ring  (not filled)",
            "rect inner/frame  (not filled)",
            "rect inner/plate  (filled)",
            "circle paint/disc  (filled)",
        ]
    );
}

#[test]
fn a_fill_given_in_a_style_is_read_and_stands_over_one_given_as_an_attribute() {
    // A made-up drawing: nobody is drawn in it. An SVG may give a paint in `style`, and where
    // an element gives it both ways the one in `style` is what is drawn.
    let listed = shapes_listed(
        "drawings-fill-in-style",
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
  <g id="styled">
    <circle id="ring" cx="50" cy="50" r="30" style="stroke:#000000; fill : none ;stroke-width:2"/>
    <circle id="disc" cx="50" cy="50" r="10" style="stroke:none;fill:#cccccc"/>
    <rect id="both" x="10" y="10" width="20" height="20" fill="#cccccc" style="fill:none"/>
    <rect id="neither" x="40" y="40" width="20" height="20" style="stroke:#000000"/>
  </g>
  <g id="lines" style="fill:none">
    <circle id="handed" cx="50" cy="50" r="5"/>
    <circle id="own" cx="50" cy="50" r="5" style="fill:#333333"/>
  </g>
</svg>
"##,
    );
    assert_eq!(
        listed,
        [
            "circle styled/ring  (not filled)",
            "circle styled/disc  (filled)",
            "rect styled/both  (not filled)",
            "rect styled/neither  (filled)",
            "circle lines/handed  (not filled)",
            "circle lines/own  (filled)",
        ]
    );
}

#[test]
fn a_shape_whose_opacity_is_nothing_is_listed_so_however_the_nought_is_written() {
    // A made-up drawing: nobody is drawn in it.
    let listed = shapes_listed(
        "drawings-opacity-nought",
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
  <g id="head">
    <circle id="a" cx="50" cy="50" r="30" opacity="0"/>
    <circle id="b" cx="50" cy="50" r="30" opacity="0.0"/>
    <circle id="c" cx="50" cy="50" r="30" opacity=" 0.00 "/>
    <circle id="d" cx="50" cy="50" r="30" opacity="0.5"/>
    <circle id="e" cx="50" cy="50" r="30"/>
  </g>
</svg>
"##,
    );
    assert_eq!(
        listed,
        [
            "circle head/a  (filled, opacity 0)",
            "circle head/b  (filled, opacity 0)",
            "circle head/c  (filled, opacity 0)",
            "circle head/d  (filled)",
            "circle head/e  (filled)",
        ]
    );
}

#[test]
fn without_the_drawings_the_record_command_stops_with_a_plain_message_and_writes_nothing() {
    let folder = scratch("drawings-absent");
    let out = folder.join("out");
    let absent = folder.join("no-such-folder");
    let output = maquette(&[
        "record-hand-made-drawings",
        "--underdrawings",
        path_text(&absent),
        "--final-drawings",
        path_text(&absent),
        "--out",
        path_text(&out),
    ]);
    assert!(!output.status.success());
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(
        message.contains("is not at")
            && message.contains("no-such-folder")
            && message.contains("the committed numbers stand as recorded"),
        "{message}"
    );
    assert!(!out.exists(), "files were written without the drawings");
}

#[test]
fn a_drawing_whose_digest_is_not_the_recorded_one_is_refused() {
    // A made-up drawing under the name of her S2 front underdrawing: it is not that drawing,
    // and its digest says so.
    let folder = scratch("drawings-other-digest");
    std::fs::copy(
        repository().join(MADE_UP_DRAWING),
        folder.join("s2-base-underdrawing-A-front.svg"),
    )
    .expect("copied");
    let out = folder.join("out");
    let output = maquette(&[
        "record-hand-made-drawings",
        "--underdrawings",
        path_text(&folder),
        "--final-drawings",
        path_text(&folder),
        "--out",
        path_text(&out),
    ]);
    assert!(!output.status.success());
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(
        message.contains("s2-base-underdrawing-A-front.svg")
            && message.contains(FRONT_SHA256)
            && message.contains("nothing was read"),
        "{message}"
    );
    assert!(!out.exists(), "files were written from another drawing");
}

/// The landmarks a record gives a view: each name with its x and y.
fn landmarks(record: &toml::Table, view: &str) -> BTreeMap<String, [f64; 2]> {
    record["views"][view]["landmarks"]
        .as_table()
        .expect("the landmarks")
        .iter()
        .map(|(name, entry)| (name.clone(), [number(&entry["x"]), number(&entry["y"])]))
        .collect()
}

#[test]
fn what_her_final_record_derives_follows_from_the_positions_the_two_records_give() {
    let record: toml::Table = text(&repository().join(RECORD)).parse().expect("TOML");
    let final_record: toml::Table = text(&repository().join(FINAL_RECORD))
        .parse()
        .expect("TOML");
    // Head height is her S2 front view's: hair top at y = 86.682, chin at 567.
    let head_height = 567.0 - 86.682;
    assert_close(
        number(&final_record["levels"]["head_height"]),
        head_height,
        0.0005,
        "head height",
    );

    let mut final_landmarks = 0;
    for view in ["front", "three_quarter"] {
        let (s2, theirs) = (landmarks(&record, view), landmarks(&final_record, view));
        final_landmarks += theirs.len();
        // Each final landmark replaces one her record has, and `moved_from_s2` is the
        // final position minus that one.
        let moved = final_record["views"][view]["moved_from_s2"]
            .as_table()
            .expect("moved_from_s2");
        assert_eq!(
            moved.keys().collect::<Vec<_>>(),
            theirs.keys().collect::<Vec<_>>(),
            "{view}"
        );
        for (name, position) in &theirs {
            let was = s2
                .get(name)
                .unwrap_or_else(|| panic!("{view}: her record has no {name} to replace"));
            let what = format!("{view}: {name}");
            assert_close(
                number(&moved[name]["dx"]),
                position[0] - was[0],
                0.00051,
                &what,
            );
            assert_close(
                number(&moved[name]["dy"]),
                position[1] - was[1],
                0.00051,
                &what,
            );
            // Only her ears and her lids are replaced.
            assert!(
                name.starts_with("ear_")
                    || name.starts_with("eye_left_")
                    || name.starts_with("eye_right_")
                    || name.starts_with("lid_line_"),
                "{what} is neither an ear nor a lid"
            );
        }
        // The frame is her S2 underdrawing's: the same skull circle and the same chin.
        let section = &final_record["views"][view];
        assert_eq!(
            [
                number(&section["skull_centre"]["x"]),
                number(&section["skull_centre"]["y"])
            ],
            s2["skull_centre"],
            "{view}"
        );
        assert_eq!(
            [number(&section["chin"]["x"]), number(&section["chin"]["y"])],
            s2["chin"],
            "{view}"
        );
    }
    assert_eq!(final_landmarks, 72, "two ears and two eyes in each view");

    // Levels: each listed difference is the three-quarter height minus the front height.
    let (front, turned) = (
        landmarks(&final_record, "front"),
        landmarks(&final_record, "three_quarter"),
    );
    let differences = final_record["levels"]["difference"]
        .as_array()
        .expect("[[levels.difference]]");
    assert_eq!(
        final_record["levels"]["different"].as_integer(),
        Some(differences.len() as i64)
    );
    let mut above = 0;
    for entry in differences {
        let name = entry["landmark"].as_str().expect("a name");
        let difference = turned[name][1] - front[name][1];
        assert_close(number(&entry["difference"]), difference, 0.00051, name);
        assert_close(
            number(&entry["percent_of_head_height"]),
            difference.abs() / head_height * 100.0,
            0.0051,
            name,
        );
        let is_above = difference.abs() / head_height > 0.01;
        assert_eq!(
            entry["above_one_percent"].as_bool(),
            Some(is_above),
            "{name}"
        );
        above += usize::from(is_above);
    }
    assert_eq!(
        final_record["levels"]["above_one_percent"].as_integer(),
        Some(above as i64)
    );
    // The disagreement the two final drawings carry, which is reported and not corrected: the
    // near ear's outer base is at y = 186.989 in the front view and 191.834 in the
    // three-quarter view, 4.845 units apart, which is 1.01% of head height.
    let near_ear = differences
        .iter()
        .find(|entry| entry["landmark"].as_str() == Some("ear_right_base_outer"))
        .expect("the near ear's outer base");
    assert_eq!(number(&near_ear["difference"]), 4.845);
    assert_eq!(number(&near_ear["percent_of_head_height"]), 1.01);

    // Spacings: each pair's ratio is its three-quarter spacing over its front spacing.
    for entry in final_record["spacing"]["pair"]
        .as_array()
        .expect("[[spacing.pair]]")
    {
        let name = entry["name"].as_str().expect("a name");
        let spacing = |landmarks: &BTreeMap<String, [f64; 2]>| {
            let left = name.replacen('_', "_left_", 1);
            let right = name.replacen('_', "_right_", 1);
            // `lid_line_inner_lower` is `lid_line_left_inner_lower`: the side follows the stem.
            let (left, right) = if name.starts_with("lid_line_") {
                (
                    name.replacen("lid_line_", "lid_line_left_", 1),
                    name.replacen("lid_line_", "lid_line_right_", 1),
                )
            } else {
                (left, right)
            };
            landmarks[&left][0] - landmarks[&right][0]
        };
        let (front_spacing, turned_spacing) = (spacing(&front), spacing(&turned));
        assert_close(
            number(&entry["front_spacing"]),
            front_spacing,
            0.00051,
            name,
        );
        assert_close(
            number(&entry["ratio"]),
            turned_spacing / front_spacing,
            0.000051,
            name,
        );
        assert_close(
            number(&entry["turn_degrees"]),
            (turned_spacing / front_spacing).acos().to_degrees(),
            0.0051,
            name,
        );
    }
}

/// Where the private drawings are, for the test below: the folder of the S2 underdrawings and
/// the folder of the final drawings. Both are named by the environment, never by this file.
const UNDERDRAWINGS: &str = "MAQUETTE_UNDERDRAWINGS";
const FINAL_DRAWINGS: &str = "MAQUETTE_FINAL_DRAWINGS";

/// Run where the drawings are, with both folders named:
/// `MAQUETTE_UNDERDRAWINGS=… MAQUETTE_FINAL_DRAWINGS=… cargo test --test hand_made_drawings -- --ignored`
#[test]
#[ignore = "needs Saffi's private drawings, which no clean clone has"]
fn the_committed_records_are_what_the_command_writes_from_the_drawings() {
    let folder_of = |variable: &str| {
        std::env::var(variable).unwrap_or_else(|_| {
            panic!("{variable} does not name the folder of the drawings, so nothing was checked")
        })
    };
    let out = scratch("drawings-recorded");
    maquette_ok(&[
        "record-hand-made-drawings",
        "--underdrawings",
        &folder_of(UNDERDRAWINGS),
        "--final-drawings",
        &folder_of(FINAL_DRAWINGS),
        "--out",
        path_text(&out),
    ]);
    for name in [
        "hand-made-views.toml",
        "views-compared.toml",
        "final-drawings.toml",
    ] {
        let written = text(&out.join(name));
        let committed = text(&repository().join("data/saffi").join(name));
        assert!(
            written == committed,
            "data/saffi/{name} is not what the command writes from the drawings"
        );
    }
}
