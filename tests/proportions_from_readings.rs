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
    maquette_in, maquette_ok, measure_ok, number, path_text, proportions, scratch, text,
};

const SKULL: [&str; 3] = ["skull_radius", "skull_centre_rise", "skull_centre_depth"];

/// A made-up reference folder with readings of the made-up head's front view and
/// three-quarter view. `seen` says which landmarks the readers place; the rest are stated as
/// not seen.
fn folder_with_readings(name: &str, seen: &dyn Fn(&str) -> bool) -> PathBuf {
    let generated = draw(&format!("{name}-generated"), MADE_UP, &[]);
    let folder = scratch(name);
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
