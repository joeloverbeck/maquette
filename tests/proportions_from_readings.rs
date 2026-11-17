//! The proportions readings give, and the rule that builds the skull where nothing is drawn,
//! through the command line.
//!
//! The readings here are made up: the generator draws the made-up set of proportions in
//! `tests/fixtures/made-up-proportions.toml`, and the test copies the positions of its
//! landmark files into two readings under the names a reader uses, leaving the skull out as
//! a reader does. No drawing was read, and the picture behind them is plain grey.

mod support;

use std::path::{Path, PathBuf};

use support::{
    LandmarkFile, MADE_UP, assert_close, draw, failure, filled_in, made_up_image, maquette,
    maquette_in, maquette_ok, measure_ok, number, path_text, proportions, references, scratch,
    text,
};

const SKULL: [&str; 3] = ["skull_radius", "skull_centre_rise", "skull_centre_depth"];

/// A made-up reference folder with readings of the made-up head's front view and
/// three-quarter view. `seen` says which landmarks the readers place; the rest are stated as
/// not seen.
fn folder_with_readings(name: &str, seen: &dyn Fn(&str) -> bool) -> PathBuf {
    let generated = draw(&format!("{name}-generated"), MADE_UP, &[]);
    let folder = references(name);
    for (view, faces) in [("front", None), ("three-quarter", Some("right"))] {
        let picture = format!("{view}.png");
        made_up_image(&folder.join(&picture), 1000, 1000);
        let mut arguments = vec![
            "start-reading",
            "--image",
            &picture,
            "--view",
            view,
            "--reader",
            "made-up reader",
            "--out",
        ];
        let out = format!("{view}.toml");
        arguments.push(&out);
        if let Some(faces) = faces {
            arguments.extend(["--faces", faces]);
        }
        measure_ok(&folder, &arguments);

        let file = LandmarkFile::read(&generated.join(format!("{view}.landmarks.toml")));
        let place = |landmark: &str| -> Option<[f64; 2]> {
            if !seen(landmark) {
                return None;
            }
            // A reader places the near side's outline under the face outline's name; the list
            // of proportions calls it the jaw line in a turned view.
            let generated =
                if view == "three-quarter" && landmark.starts_with("face-outline-right-at") {
                    landmark.replace("face-outline", "jaw-line")
                } else {
                    landmark.to_string()
                };
            let point = file.point(&generated);
            Some([point.x, point.y])
        };
        let filled = filled_in(&text(&folder.join(&out)), 30.0, &place);
        std::fs::write(folder.join(&out), filled).expect("the reading is written");
        measure_ok(&folder, &["check-reading", "--reading", &out]);
    }
    folder
}

fn taken(folder: &Path) -> toml::Table {
    measure_ok(
        folder,
        &[
            "proportions-from-readings",
            "--front",
            "front.toml",
            "--three-quarter",
            "three-quarter.toml",
            "--out",
            "proportions.toml",
        ],
    );
    text(&folder.join("proportions.toml"))
        .parse()
        .expect("the proportions are TOML")
}

#[test]
fn readings_of_a_made_up_head_give_back_its_proportions_with_the_skull_built_by_rule() {
    let folder = folder_with_readings("proportions-from-readings", &|_| true);
    let found = taken(&folder);
    let put_in = proportions(&support::repository().join(MADE_UP));
    let given = found["proportions"].as_table().expect("[proportions]");

    assert_eq!(given.len(), put_in.len(), "every proportion is given");
    for (name, value) in &put_in {
        if SKULL.contains(&name.as_str()) {
            continue;
        }
        // The landmark files keep positions to a thousandth of 200 to a unit, and the turn is
        // worked out from them, not given.
        assert_close(number(&given[name]), *value, 2e-4, name);
    }

    // The readings place no skull, so it is built by rule from the made-up set's ears and
    // temples, by hand:
    //   centre rise  = (0.9373 + 0.9871) / 2 = 0.9622      the middle of the ear's base
    //   centre depth = (-0.7621 - 0.5877) / 2 = -0.6749
    //   radius = sqrt(0.6877^2 + (0.2131 - 0.9622)^2 + (-0.0619 + 0.6749)^2) = 1.187372
    //            from that centre to the temple: its half-width, its rise and its depth
    assert_close(number(&given["skull_centre_rise"]), 0.9622, 2e-4, "rise");
    assert_close(number(&given["skull_centre_depth"]), -0.6749, 2e-4, "depth");
    assert_close(number(&given["skull_radius"]), 1.187372, 2e-4, "radius");
    assert!(
        found["source"]["skull"]
            .as_str()
            .expect("how the skull was built")
            .starts_with("built by rule"),
        "{:?}",
        found["source"]["skull"]
    );

    // The turn is the one the readings show, not the 30 degrees the readers stated.
    assert_close(
        number(&found["source"]["turn_degrees"]),
        29.0,
        0.01,
        "the turn",
    );
    assert_eq!(found["source"]["not_given"].as_integer(), Some(0));

    // Every proportion is given, so the file is a set of proportions the generator draws.
    let drawn = scratch("proportions-from-readings-drawn");
    maquette_ok(&[
        "draw",
        "--proportions",
        path_text(&folder.join("proportions.toml")),
        "--out",
        path_text(&drawn),
    ]);
}

#[test]
fn readings_that_hide_the_ears_give_the_other_proportions_and_say_what_is_not_given() {
    let folder = folder_with_readings("proportions-from-readings-hidden", &|landmark| {
        !landmark.starts_with("ear-")
    });
    let found = taken(&folder);
    let given = found["proportions"].as_table().expect("[proportions]");
    let not_given = found["not_given"].as_table().expect("[not_given]");
    let put_in = proportions(&support::repository().join(MADE_UP));

    // The 12 proportions of the ears (an offset, a rise and a depth for each base point, an
    // offset and a rise for the top and for each side extent), and the 3 of the skull, which
    // the rule builds from the ears.
    assert_eq!(not_given.len(), 15, "{not_given:?}");
    assert_eq!(given.len(), put_in.len() - 15);
    for name in SKULL {
        let why = not_given[name].as_str().expect("a reason");
        assert!(why.contains("ear_base_outer_rise"), "{why}");
        assert!(!given.contains_key(name));
    }
    assert!(not_given.contains_key("ear_top_rise"));
    assert_close(
        number(&given["chin_depth"]),
        put_in["chin_depth"],
        2e-4,
        "the chin's depth",
    );

    // What is left is not a set of proportions, and the generator says so.
    let output = maquette(&[
        "draw",
        "--proportions",
        path_text(&folder.join("proportions.toml")),
        "--out",
        path_text(&scratch("proportions-from-readings-hidden-drawn")),
    ]);
    assert!(failure(&output).contains("lacks 15 proportions"));
}

/// The depths a side reading gives, by the landmarks a side view asks for.
const DEPTHS_FROM_A_SIDE_VIEW: [&str; 15] = [
    "hair_half_depth",
    "hair_centre_depth",
    "chin_depth",
    "jaw_line_depth_at_nose_level",
    "jaw_line_depth_at_mouth_level",
    "eye_wrap_depth",
    "iris_depth",
    "nose_mark_corner_depth",
    "nose_mark_bottom_depth",
    "mouth_end_depth",
    "mouth_centre_depth",
    "ear_base_outer_depth",
    "ear_base_inner_depth",
    "neck_depth",
    "shoulder_depth",
];

/// The depths it does not give: a side view is asked for no temple, no cheek point and no
/// jaw point.
const DEPTHS_NOT_FROM_A_SIDE_VIEW: [&str; 5] = [
    "face_top_depth",
    "face_depth_at_eye_level",
    "face_depth_at_nose_level",
    "face_depth_at_mouth_level",
    "jaw_depth",
];

/// How far above the front view's nose mark a made-up side reading places the tip of the
/// nose, in units: the generator draws the made-up head at 200 pixels to the unit.
const TIP_ABOVE_THE_NOSE_MARK: f64 = 0.1;

/// The points of the profile a side view is asked for since revision 2026-11-17.4, and the
/// proportions each gives: a height where the list takes one, and a depth.
const PROFILE: [(&str, Option<&str>, &str); 7] = [
    ("centre-line-at-brow-level", None, "forehead_depth"),
    ("centre-line-at-eye-level", None, "bridge_depth"),
    ("jaw-right-angle", Some("jaw_angle_drop"), "jaw_angle_depth"),
    (
        "neck-front-top",
        Some("neck_front_top_drop"),
        "neck_front_top_depth",
    ),
    (
        "neck-front-base",
        Some("neck_front_base_drop"),
        "neck_front_base_depth",
    ),
    (
        "neck-back-top",
        Some("neck_back_top_drop"),
        "neck_back_top_depth",
    ),
    (
        "neck-back-base",
        Some("neck_back_base_drop"),
        "neck_back_base_depth",
    ),
];

/// Made up: the generator draws no point of the forehead and no angle of the jaw, so a made-up
/// side reading places them where these say, in units: the forehead's depth, at the height of
/// the brow's front end, and the jaw angle's drop and depth.
const MADE_UP_FOREHEAD_DEPTH: f64 = 0.24;
const MADE_UP_JAW_ANGLE: [f64; 2] = [0.71, -0.48];

/// Where a made-up side reading places a point of the profile in the generated side view, as
/// that view draws it, facing the viewer's right at its own size; and the drop and the depth
/// that place gives, in units, worked out here from the landmark file's frame by the list's
/// definitions: a drop is how far below the eye line, and depth zero shows at the turn
/// axis's x less the axis's own depth.
fn profile_point(file: &LandmarkFile, landmark: &str) -> ([f64; 2], f64, f64) {
    let unit = file.frame("unit");
    let eye_line = file.frame("eye_line_y");
    let zero_x = file.frame("turn_axis_x") - file.frame("turn_axis_depth");
    let at = |drop: f64, depth: f64| ([zero_x + depth * unit, eye_line + drop * unit], drop, depth);
    match landmark {
        "centre-line-at-brow-level" => {
            let brow = file.point("brow-line-right-inner-end");
            at((brow.y - eye_line) / unit, MADE_UP_FOREHEAD_DEPTH)
        }
        "jaw-right-angle" => at(MADE_UP_JAW_ANGLE[0], MADE_UP_JAW_ANGLE[1]),
        _ => {
            let point = file.point(landmark);
            at((point.y - eye_line) / unit, (point.x - zero_x) / unit)
        }
    }
}

/// What a side view gives for each proportion of the profile, by [`profile_point`].
fn profile_given() -> std::collections::BTreeMap<&'static str, f64> {
    let generated = draw("proportions-profile-generated", MADE_UP, &[]);
    let file = LandmarkFile::read(&generated.join("side.landmarks.toml"));
    let mut given = std::collections::BTreeMap::new();
    for (landmark, drop, depth) in PROFILE {
        let (_, at_drop, at_depth) = profile_point(&file, landmark);
        if let Some(drop) = drop {
            given.insert(drop, at_drop);
        }
        given.insert(depth, at_depth);
    }
    given
}

/// A made-up reading of the made-up head's side view, as the generator draws it, on a
/// picture of its own: drawn `scale` times the size of the front view and moved by `shift`,
/// and facing the viewer's left when `faces_left`, which flips the picture left to right.
fn side_reading(folder: &Path, name: &str, scale: f64, shift: [f64; 2], faces_left: bool) {
    // A scratch folder of this reading's own: two tests may each make a reading named `side`.
    let of_folder = folder
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("folder");
    let generated = draw(&format!("{of_folder}-{name}-generated"), MADE_UP, &[]);
    let file = LandmarkFile::read(&generated.join("side.landmarks.toml"));
    let picture = format!("{name}.png");
    // Another size than the other pictures, so that it is another picture.
    made_up_image(&folder.join(&picture), 1000, 990);
    let out = format!("{name}.toml");
    measure_ok(
        folder,
        &[
            "start-reading",
            "--image",
            &picture,
            "--view",
            "side",
            "--faces",
            if faces_left { "left" } else { "right" },
            "--reader",
            "made-up reader",
            "--out",
            &out,
        ],
    );
    let place = |landmark: &str| -> Option<[f64; 2]> {
        // The generator draws the head facing the viewer's right, where the near side is the
        // character's right. Facing the viewer's left, the near side is her left.
        let facing_right = landmark.replace("-left-", "-right-");
        let generated = if facing_right.starts_with("face-outline-right-at") {
            // The back edge of the jaw, which the list of proportions calls the jaw line.
            facing_right.replace("face-outline", "jaw-line")
        } else if facing_right.starts_with("iris-right-") && facing_right.ends_with("-extent") {
            // The generator draws the iris from its edge, as a line with no width.
            "iris-right-centre".to_string()
        } else {
            facing_right
        };
        let point = match PROFILE.iter().any(|(profile, _, _)| *profile == generated) {
            true => {
                let ([x, y], _, _) = profile_point(&file, &generated);
                support::Point {
                    x,
                    y,
                    depth: None,
                    drawn_at_depth: None,
                }
            }
            false => file.point(&generated),
        };
        let x = shift[0] + scale * point.x;
        let x = if faces_left { 1000.0 - x } else { x };
        // A reader of a profile places `nose-mark-bottom` under the tip of the nose, which
        // stands above the front view's nose mark: made up here as a tenth of a unit higher.
        let y = if landmark == "nose-mark-bottom" {
            point.y - TIP_ABOVE_THE_NOSE_MARK * 200.0
        } else {
            point.y
        };
        Some([x, shift[1] + scale * y])
    };
    let filled = filled_in(&text(&folder.join(&out)), 90.0, &place);
    std::fs::write(folder.join(&out), filled).expect("the reading is written");
    measure_ok(folder, &["check-reading", "--reading", &out]);
}

fn taken_from(folder: &Path, views: &[&str], out: &str) -> toml::Table {
    let mut arguments = vec!["proportions-from-readings"];
    arguments.extend_from_slice(views);
    arguments.extend(["--out", out]);
    measure_ok(folder, &arguments);
    text(&folder.join(out))
        .parse()
        .expect("the proportions are TOML")
}

#[test]
fn a_front_view_alone_gives_the_heights_and_the_widths_and_says_why_no_depth() {
    let folder = folder_with_readings("proportions-from-a-front-view", &|_| true);
    let found = taken_from(&folder, &["--front", "front.toml"], "front-alone.toml");
    let put_in = proportions(&support::repository().join(MADE_UP));
    let given = found["proportions"].as_table().expect("[proportions]");
    let not_given = found["not_given"].as_table().expect("[not_given]");

    // Not given: the 20 depths, which a side view could or could not give, and the skull's
    // three, which the rule builds from depths.
    let depths: Vec<&str> = DEPTHS_FROM_A_SIDE_VIEW
        .iter()
        .chain(&DEPTHS_NOT_FROM_A_SIDE_VIEW)
        .copied()
        .collect();
    // Nor the tip of the nose: a front view draws the nose mark, at the base of the nose.
    assert_eq!(
        not_given.len(),
        depths.len() + SKULL.len() + 1,
        "{not_given:?}"
    );
    let why = not_given["nose_tip_drop"].as_str().expect("a reason");
    assert!(why.contains("not the tip of the nose"), "{why}");
    // The proportions of the profile, which only a side view is asked for and a set may leave
    // out, are neither given nor said to be missing: no view that could give them was read.
    for name in profile_given().keys() {
        assert!(!not_given.contains_key(*name), "{name}");
        assert!(!given.contains_key(*name), "{name}");
    }
    for name in &depths {
        let why = not_given[*name].as_str().expect("a reason");
        assert!(why.contains("a front view shows no depth"), "{name}: {why}");
    }
    for name in SKULL {
        assert!(not_given.contains_key(name), "{name}");
    }
    // Given: every height and every width, as they were put in.
    assert_eq!(given.len(), put_in.len() - not_given.len());
    for (name, value) in given {
        assert_close(number(value), put_in[name], 2e-4, name);
    }
    assert!(given.contains_key("neck_base_drop") && given.contains_key("iris_width"));
    let source = found["source"].as_table().expect("[source]");
    assert!(
        source["depth_from"]
            .as_str()
            .expect("where depth is from")
            .contains("no view"),
        "{source:?}"
    );
    assert!(!source.contains_key("turn_degrees"), "{source:?}");
}

#[test]
fn a_front_and_a_side_view_give_depth_whatever_size_and_way_the_side_view_is_drawn() {
    let folder = folder_with_readings("proportions-from-front-and-side", &|_| true);
    // The side view at the front view's size, facing the viewer's right; at 0.55 of it and
    // moved, facing the viewer's left; and at 1.3 of it, off to the left and up.
    side_reading(&folder, "side", 1.0, [0.0, 0.0], false);
    side_reading(&folder, "side-small-left", 0.55, [130.0, 210.0], true);
    side_reading(&folder, "side-large", 1.3, [-220.0, -100.0], false);
    let put_in = proportions(&support::repository().join(MADE_UP));
    let profile = profile_given();

    for side in ["side", "side-small-left", "side-large"] {
        let found = taken_from(
            &folder,
            &["--front", "front.toml", "--side", &format!("{side}.toml")],
            &format!("with-{side}.toml"),
        );
        let given = found["proportions"].as_table().expect("[proportions]");
        let not_given = found["not_given"].as_table().expect("[not_given]");

        // Every depth a side view shows is the one put in, at any size: each view is
        // measured in its own unit, eye line to chin. Positions are kept to a thousandth of
        // a pixel, which a smaller drawing turns into more of a unit.
        for name in DEPTHS_FROM_A_SIDE_VIEW {
            assert_close(
                number(&given[name]),
                put_in[name],
                4e-4,
                &format!("{side}: {name}"),
            );
        }
        // What it does not show is not given, and the skull the rule builds from the
        // temple's depth is not built.
        for name in DEPTHS_NOT_FROM_A_SIDE_VIEW {
            let why = not_given[name].as_str().expect("a reason");
            assert!(why.contains("side view"), "{side}: {name}: {why}");
        }
        for name in SKULL {
            let why = not_given[name].as_str().expect("a reason");
            assert!(why.contains("face_top_depth"), "{side}: {name}: {why}");
        }
        assert_eq!(
            not_given.len(),
            DEPTHS_NOT_FROM_A_SIDE_VIEW.len() + SKULL.len(),
            "{side}: {not_given:?}"
        );
        // Heights and widths are the front view's. The tip of the nose is the one height of
        // the list's first part that a side view gives of its own: where it places
        // `nose-mark-bottom`, in its own unit, while the nose mark's bottom keeps the front
        // view's height. The points of the profile give their heights and depths, in the side
        // view's own unit, which the made-up set leaves out.
        assert_eq!(
            given.len(),
            put_in.len() - not_given.len() + profile.len(),
            "{side}"
        );
        for (name, value) in given {
            let expected = if name == "nose_tip_drop" {
                put_in["nose_mark_bottom_drop"] - TIP_ABOVE_THE_NOSE_MARK
            } else if let Some(profile) = profile.get(name.as_str()) {
                *profile
            } else {
                put_in[name]
            };
            assert_close(number(value), expected, 4e-4, &format!("{side}: {name}"));
        }
        assert!(given.contains_key("nose_tip_drop"), "{side}");
        let source = found["source"].as_table().expect("[source]");
        assert!(
            source["depth_from"]
                .as_str()
                .expect("where depth is from")
                .contains("side view"),
            "{source:?}"
        );
        assert_eq!(
            source["faces"].as_str(),
            Some(if side == "side-small-left" {
                "the viewer's left"
            } else {
                "the viewer's right"
            })
        );
        let rules = format!("{:?}", source["side_view_rules"]);
        assert!(rules.contains("its own unit"), "{rules}");
    }

    // A three-quarter view and a side view are not taken together.
    let both = maquette_in(
        Some(&folder),
        &[
            "proportions-from-readings",
            "--front",
            "front.toml",
            "--three-quarter",
            "three-quarter.toml",
            "--side",
            "side.toml",
        ],
    );
    assert!(failure(&both).contains("not both"), "{}", failure(&both));
    // And a side view is not named where a three-quarter view belongs.
    let misnamed = maquette_in(
        Some(&folder),
        &[
            "proportions-from-readings",
            "--front",
            "front.toml",
            "--three-quarter",
            "side.toml",
        ],
    );
    assert!(
        failure(&misnamed).contains("side view"),
        "{}",
        failure(&misnamed)
    );
}

#[test]
fn a_height_the_front_view_hides_is_taken_from_the_side_view_but_not_an_ears_side_extent() {
    // The front view's readers see no nose mark, no ear and no brow: hair is over them.
    let folder = folder_with_readings("proportions-front-hides", &|landmark| {
        !landmark.starts_with("nose-mark")
            && !landmark.starts_with("ear-")
            && !landmark.starts_with("brow-")
    });
    side_reading(&folder, "side", 0.8, [40.0, 60.0], false);
    let found = taken_from(
        &folder,
        &["--front", "front.toml", "--side", "side.toml"],
        "taken.toml",
    );
    let given = found["proportions"].as_table().expect("[proportions]");
    let not_given = found["not_given"].as_table().expect("[not_given]");
    let put_in = proportions(&support::repository().join(MADE_UP));
    // Heights the side view shows stand in, and its depths are given as ever.
    for name in [
        "brow_inner_end_rise",
        "ear_base_outer_rise",
        "ear_top_rise",
        "nose_mark_bottom_depth",
        "ear_base_inner_depth",
    ] {
        assert_close(number(&given[name]), put_in[name], 4e-4, name);
    }
    // A width needs the front view.
    for name in [
        "nose_mark_corner_offset",
        "brow_outer_end_offset",
        "ear_top_offset",
    ] {
        assert!(not_given.contains_key(name), "{name} is given");
    }
    // In a profile the ear's extents are toward the back and the front of the head: other
    // points of the ear than its extents to the sides, so their heights do not stand in. Nor
    // do the nose mark's bottom and corner, which a side view puts under the tip of the nose
    // and at the middle of the nostril.
    for name in [
        "ear_outer_extent_rise",
        "ear_inner_extent_rise",
        "nose_mark_bottom_drop",
        "nose_mark_corner_drop",
    ] {
        let why = not_given[name].as_str().expect("a reason");
        assert!(why.contains("side view"), "{name}: {why}");
    }
}

#[test]
fn a_skull_that_is_drawn_is_kept_and_the_rule_is_set_against_it() {
    // The generator draws the made-up skull as a construction circle, and the landmark files
    // carry it: taken back, it is that circle and not the rule's.
    let generated = draw("skull-drawn-is-kept", MADE_UP, &[]);
    let out = generated.join("taken-back.toml");
    maquette_ok(&[
        "proportions-from-landmarks",
        "--front",
        path_text(&generated.join("front.landmarks.toml")),
        "--three-quarter",
        path_text(&generated.join("three-quarter.landmarks.toml")),
        "--out",
        path_text(&out),
    ]);
    let back = proportions(&out);
    assert_close(back["skull_radius"], 0.9379, 2e-5, "the drawn radius");
    assert_close(back["skull_centre_rise"], 0.3623, 2e-5, "the drawn rise");
    assert_close(back["skull_centre_depth"], -0.6375, 2e-5, "the drawn depth");

    // `skull-by-rule` runs the rule on a set of proportions and sets it against the set's own
    // skull, part by part. It needs no reference folder. By hand, for the made-up set, with
    // head height 1.3127 + 1 = 2.3127:
    //   centre rise:  rule 0.9622, the set's 0.3623: 0.5999 apart, 25.9394% of head height
    //   centre depth: rule -0.6749, the set's -0.6375: 0.0374 apart, 1.6172%
    //   radius, from the rule's centre: 1.187372, the set's 0.9379: 0.249472, 10.7870%
    //   radius, from the set's own centre to the temple:
    //     sqrt(0.6877^2 + (0.2131 - 0.3623)^2 + (-0.0619 + 0.6375)^2) = 0.909124: 1.2442% short
    let output = maquette_in(None, &["skull-by-rule", "--proportions", MADE_UP]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let found: toml::Table = String::from_utf8(output.stdout)
        .expect("text")
        .parse()
        .expect("the result is TOML");
    let part = |name: &str| found["part"][name].as_table().expect("a part").clone();
    for (name, rule, set, percent) in [
        ("centre_rise", 0.9622, 0.3623, 25.9394),
        ("centre_depth", -0.6749, -0.6375, 1.6172),
        ("radius", 1.187372, 0.9379, 10.7870),
        ("radius_from_the_sets_centre", 0.909124, 0.9379, 1.2442),
    ] {
        let part = part(name);
        assert_close(number(&part["by_rule"]), rule, 1e-6, name);
        assert_close(number(&part["the_sets"]), set, 1e-9, name);
        assert_close(
            number(&part["difference"]),
            rule - set,
            1e-6,
            &format!("{name}: the difference"),
        );
        assert_close(
            number(&part["percent_of_head_height"]),
            percent,
            0.0005,
            &format!("{name}: the share"),
        );
    }
    assert_close(number(&found["head_height"]), 2.3127, 1e-9, "head height");
}

#[test]
fn the_list_states_the_skull_rule_and_keeps_the_skull_a_convention() {
    let printed = maquette_ok(&["list-proportions"]);
    let entry = |name: &str| -> String {
        printed
            .split("\n\n")
            .find(|entry| entry.starts_with(&format!("{name}  [")))
            .unwrap_or_else(|| panic!("{name} is not in the list"))
            .to_string()
    };
    for name in SKULL {
        let entry = entry(name);
        assert!(
            entry.starts_with(&format!("{name}  [convention]")),
            "{entry}"
        );
        assert!(
            entry.contains("By rule: "),
            "{name} states no rule: {entry}"
        );
    }
    assert!(entry("skull_centre_rise").contains("ear"));
    assert!(entry("skull_centre_depth").contains("ear"));
    assert!(entry("skull_radius").contains("temple"));
    // The hair's top is not used: its thickness varies from work to work.
    let skull = printed
        .lines()
        .find(|line| line.trim_start().starts_with("skull: "))
        .expect("the skull's construction");
    assert!(skull.contains("ear") && skull.contains("temple"), "{skull}");
    assert!(skull.contains("hair"), "{skull}");
}
