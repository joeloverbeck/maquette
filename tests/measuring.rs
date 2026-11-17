//! What readings say when they are set against each other, through the command line: the
//! agreement between two readings of a drawing, the turn a three-quarter drawing shows, and
//! the fit of a character's views to one head.
//!
//! Every reading here is made up: a view of the head invented in
//! `tests/support/made_up_head.rs`, whose numbers are chosen so that the sums can be done by
//! hand. Its unit is 200 pixels and its head height 2.4 units, so 480 pixels: 1% of head
//! height is 4.8 pixels. No reading was read from a drawing: the pictures behind the
//! readings are plain grey.

mod support;

use std::path::{Path, PathBuf};

use support::made_up_head::{MadeUp, flipped, seen};
use support::{assert_close, failure, maquette_in, measure_ok, number, references, text};

/// Runs a measuring command that writes a file, and reads the file.
fn measured(folder: &Path, arguments: &[&str], out: &str) -> toml::Table {
    let mut arguments = arguments.to_vec();
    arguments.extend(["--out", out]);
    measure_ok(folder, &arguments);
    text(&folder.join(out)).parse().expect("the result is TOML")
}

fn entries<'a>(table: &'a toml::Table, list: &str) -> Vec<&'a toml::Table> {
    table
        .get(list)
        .and_then(toml::Value::as_array)
        .map(|entries| entries.iter().filter_map(toml::Value::as_table).collect())
        .unwrap_or_default()
}

fn names(table: &toml::Table, list: &str) -> Vec<String> {
    entries(table, list)
        .iter()
        .map(|entry| entry["landmark"].as_str().expect("a name").to_string())
        .collect()
}

fn texts(value: &toml::Value) -> Vec<String> {
    value
        .as_array()
        .expect("a list")
        .iter()
        .map(|entry| entry.as_str().expect("text").to_string())
        .collect()
}

fn folder(name: &str) -> PathBuf {
    references(name)
}

// ---------------------------------------------------------------- agreement

#[test]
fn a_disagreement_between_two_readings_above_1_percent_of_head_height_is_flagged() {
    let folder = folder("measuring-agreement");
    let first = seen(0.0, 500.0);
    let mut second = first.clone();
    // 3 across and 4 down is 5 pixels: 5 / 480 = 1.0417% of head height. Flagged.
    let at = second.get_mut("mouth-line-left-end").expect("in the head");
    *at = [at[0] + 3.0, at[1] + 4.0];
    // 4.5 pixels is 0.9375%. Not flagged.
    second.get_mut("nose-mark-bottom").expect("in the head")[1] += 4.5;
    // The hair's top says how high the hair reaches: two readers may put it anywhere along a
    // level top, so it is compared in height only. 30 pixels across, 1.2 down: 0.25%.
    let at = second.get_mut("hair-top").expect("in the head");
    *at = [at[0] + 30.0, at[1] + 1.2];
    // The second reader does not place one ear.
    second.remove("ear-left-top");
    MadeUp::front("first", first).write(&folder, "first.toml");
    MadeUp::front("second", second).write(&folder, "second.toml");

    let found = measured(
        &folder,
        &[
            "agreement-of-readings",
            "--first",
            "first.toml",
            "--second",
            "second.toml",
        ],
        "agreement.toml",
    );
    // Head height is the mean of the two readings': 480 and 478.8, so 479.4.
    assert_close(
        number(&found["yardstick"]["head_height"]),
        479.4,
        1e-9,
        "the yardstick",
    );
    assert_eq!(names(&found, "above"), ["mouth-line-left-end"]);
    let above = entries(&found, "above")[0];
    assert_eq!(above["number"].as_integer(), Some(1));
    assert_close(number(&above["distance"]), 5.0, 1e-9, "the distance");
    assert_close(
        number(&above["percent_of_head_height"]),
        5.0 / 479.4 * 100.0,
        0.0005,
        "the share",
    );

    let under = entries(&found, "at_or_under");
    let share = |name: &str| -> f64 {
        let entry = under
            .iter()
            .find(|entry| entry["landmark"].as_str() == Some(name))
            .unwrap_or_else(|| panic!("{name} is not listed"));
        number(&entry["percent_of_head_height"])
    };
    assert_close(
        share("nose-mark-bottom"),
        4.5 / 479.4 * 100.0,
        0.0005,
        "the nose mark",
    );
    assert_close(
        share("hair-top"),
        1.2 / 479.4 * 100.0,
        0.0005,
        "the hair's top",
    );
    assert_close(share("chin"), 0.0, 1e-9, "the chin");

    let summary = &found["summary"];
    // 22 landmarks in the made-up head, one of them placed by the first reading only.
    assert_eq!(summary["compared"].as_integer(), Some(21));
    assert_eq!(summary["above_1_percent"].as_integer(), Some(1));
    assert_close(
        number(&summary["largest_percent_of_head_height"]),
        5.0 / 479.4 * 100.0,
        0.0005,
        "the largest",
    );
    assert_eq!(texts(&summary["placed_by_first_only"]), ["ear-left-top"]);
    assert!(texts(&summary["placed_by_second_only"]).is_empty());

    // The printed summary says the same, for a person.
    let printed = measure_ok(
        &folder,
        &[
            "agreement-of-readings",
            "--first",
            "first.toml",
            "--second",
            "second.toml",
            "--out",
            "again.toml",
        ],
    );
    assert!(printed.contains("1 of 21"), "{printed}");
    assert!(printed.contains("mouth-line-left-end"), "{printed}");
    assert!(printed.contains("1.043%"), "{printed}");
}

#[test]
fn two_readings_that_are_not_of_one_picture_are_not_compared() {
    let folder = folder("measuring-agreement-other-picture");
    MadeUp::front("first", seen(0.0, 500.0)).write(&folder, "first.toml");
    MadeUp::turned("second", seen(30.0, 500.0)).write(&folder, "second.toml");
    let output = maquette_in(
        Some(&folder),
        &[
            "agreement-of-readings",
            "--first",
            "first.toml",
            "--second",
            "second.toml",
        ],
    );
    let message = failure(&output);
    assert!(message.contains("not of one picture"), "{message}");
}

// ---------------------------------------------------------------- turn

fn turn_of(folder: &Path, front: &str, turned: &str, out: &str) -> toml::Table {
    measured(
        folder,
        &[
            "turn-from-readings",
            "--front",
            front,
            "--three-quarter",
            turned,
        ],
        out,
    )
}

#[test]
fn the_turn_is_recovered_from_made_up_heads_projected_at_known_turns() {
    let folder = folder("measuring-turn");
    MadeUp::front("first", seen(0.0, 500.0)).write(&folder, "front.toml");
    for turn in [12.0, 20.0, 29.0, 36.87, 45.0, 60.0] {
        // The turned drawing is another picture: its head stands elsewhere on the page.
        MadeUp::turned("first", seen(turn, 470.0)).write(&folder, "turned.toml");
        let found = turn_of(&folder, "front.toml", "turned.toml", "turn.toml");
        assert_close(
            number(&found["turn"]["degrees"]),
            turn,
            0.0005,
            "the turn recovered",
        );
        assert_eq!(
            found["turn"]["faces"].as_str(),
            Some("the viewer's right"),
            "{turn}"
        );
        // Six of the made-up head's nine pairs mark a place sideways: the eye's corners, the
        // brow's ends, the nose mark's corners and the mouth's ends. The top and the bottom
        // of the eye opening and the ear's top are where an outline runs level.
        assert_eq!(found["turn"]["pairs"].as_integer(), Some(6), "{turn}");
        // The reader's own estimate stands beside it and is not used.
        assert_close(
            number(
                &found["turn"]["stated_by_the_readers"]
                    .as_array()
                    .expect("a list")[0],
            ),
            30.0,
            1e-9,
            "the stated turn",
        );
        for pair in entries(&found, "pair") {
            assert_close(
                number(&pair["turn_degrees"]),
                turn,
                0.0005,
                pair["pair"].as_str().expect("a name"),
            );
            assert_close(
                number(&pair["misfit_percent_of_head_height"]),
                0.0,
                1e-9,
                "a rigid head's misfit",
            );
        }
    }
}

#[test]
fn a_head_that_faces_the_viewers_left_gives_the_same_turn() {
    let folder = folder("measuring-turn-left");
    MadeUp::front("first", seen(0.0, 500.0)).write(&folder, "front.toml");
    let mut turned = MadeUp::turned("first", flipped(&seen(29.0, 470.0)));
    turned.faces = Some("the viewer's left");
    turned.write(&folder, "turned.toml");
    let found = turn_of(&folder, "front.toml", "turned.toml", "turn.toml");
    assert_close(number(&found["turn"]["degrees"]), 29.0, 0.0005, "the turn");
    assert_eq!(found["turn"]["faces"].as_str(), Some("the viewer's left"));
}

#[test]
fn the_turn_is_taken_from_the_midpoints_of_two_readings_of_each_drawing() {
    let folder = folder("measuring-turn-two-readings");
    MadeUp::front("first", seen(0.0, 500.0)).write(&folder, "front.toml");
    // Two readings of the turned drawing that err in opposite ways: each moves every point of
    // the character's left 2 pixels, one outward and one inward. Their midpoints are the head's.
    let truth = seen(29.0, 470.0);
    for (name, shift) in [("turned-first.toml", 2.0), ("turned-second.toml", -2.0)] {
        let points = truth
            .iter()
            .map(|(landmark, at)| {
                let moved = if landmark.contains("-left-") {
                    [at[0] + shift, at[1]]
                } else {
                    *at
                };
                (landmark.clone(), moved)
            })
            .collect();
        MadeUp::turned(if shift > 0.0 { "first" } else { "second" }, points).write(&folder, name);
    }
    let one = turn_of(&folder, "front.toml", "turned-first.toml", "one.toml");
    assert!(
        (number(&one["turn"]["degrees"]) - 29.0).abs() > 0.2,
        "one reading alone should be off"
    );
    let both = turn_of(
        &folder,
        "front.toml",
        "turned-first.toml,turned-second.toml",
        "both.toml",
    );
    assert_close(number(&both["turn"]["degrees"]), 29.0, 0.0005, "the turn");
}

#[test]
fn a_drawing_set_aside_or_not_level_is_not_measured() {
    let folder = folder("measuring-set-aside");
    MadeUp::front("first", seen(0.0, 500.0)).write(&folder, "front.toml");
    let mut tilted = MadeUp::turned("first", seen(29.0, 470.0));
    tilted.level = false;
    tilted.write(&folder, "tilted.toml");
    let mut aside = MadeUp::turned("first", seen(29.0, 470.0));
    aside.set_aside = true;
    aside.write(&folder, "aside.toml");
    for (reading, what) in [
        ("tilted.toml", "level"),
        ("aside.toml", "sets the drawing aside"),
    ] {
        for command in [
            "turn-from-readings",
            "fit-of-views",
            "proportions-from-readings",
        ] {
            let output = maquette_in(
                Some(&folder),
                &[command, "--front", "front.toml", "--three-quarter", reading],
            );
            let message = failure(&output);
            assert!(message.contains(what), "{command} {reading}: {message}");
        }
    }
    // A front reading named where a three-quarter one belongs is refused too.
    let output = maquette_in(
        Some(&folder),
        &[
            "turn-from-readings",
            "--front",
            "front.toml",
            "--three-quarter",
            "front.toml",
        ],
    );
    assert!(failure(&output).contains("front view"));
}

#[test]
fn a_turn_that_cannot_be_worked_out_is_a_failure_and_nothing_is_written() {
    let folder = folder("measuring-no-turn");
    let refused = |front: &str, turned: &str| -> Vec<String> {
        [
            "turn-from-readings",
            "fit-of-views",
            "proportions-from-readings",
        ]
        .iter()
        .map(|command| {
            let output = maquette_in(
                Some(&folder),
                &[
                    command,
                    "--front",
                    front,
                    "--three-quarter",
                    turned,
                    "--out",
                    "refused.toml",
                ],
            );
            let message = failure(&output);
            assert!(
                !folder.join("refused.toml").exists(),
                "{command} wrote a file"
            );
            message
        })
        .collect()
    };
    MadeUp::front("first", seen(0.0, 500.0)).write(&folder, "front.toml");
    MadeUp::turned("first", seen(29.0, 470.0)).write(&folder, "turned.toml");

    // A front reading that puts both points of every pair at one place sideways: there is
    // no spacing for the three-quarter view's to have narrowed from.
    let mut no_spacing = seen(0.0, 500.0);
    for (name, at) in no_spacing.iter_mut() {
        if name.contains("-left-") || name.contains("-right-") {
            at[0] = 500.0;
        }
    }
    MadeUp::front("first", no_spacing).write(&folder, "no-spacing.toml");
    for message in refused("no-spacing.toml", "turned.toml") {
        assert!(
            message.contains("no spacing")
                && message.contains("eye-outer-corner")
                && message.contains("mouth-line-end"),
            "{message}"
        );
        assert!(!message.contains("NaN"), "{message}");
    }

    // One such pair is enough: its own turn would be no number.
    let mut one_pair = seen(0.0, 500.0);
    for side in ["left", "right"] {
        one_pair
            .get_mut(&format!("nose-mark-{side}-corner"))
            .expect("in the head")[0] = 500.0;
    }
    MadeUp::front("first", one_pair).write(&folder, "one-pair.toml");
    for message in refused("one-pair.toml", "turned.toml") {
        assert!(
            message.contains("no spacing")
                && message.contains("nose-mark-corner")
                && !message.contains("eye-outer-corner"),
            "{message}"
        );
    }

    // A three-quarter drawing whose pairs stand wider apart than the front view's, here by
    // a quarter: a turn narrows a spacing, so no turn gives this, and 0 degrees is not it.
    let wider = seen(0.0, 470.0)
        .into_iter()
        .map(|(name, at)| (name, [470.0 + (at[0] - 470.0) * 1.25, at[1]]))
        .collect();
    MadeUp::turned("first", wider).write(&folder, "wider.toml");
    for message in refused("front.toml", "wider.toml") {
        assert!(
            message.contains("wider apart") && message.contains("1.25"),
            "{message}"
        );
    }
}

// ---------------------------------------------------------------- fit

fn fit_of(folder: &Path, front: &str, turned: &str, out: &str) -> toml::Table {
    measured(
        folder,
        &["fit-of-views", "--front", front, "--three-quarter", turned],
        out,
    )
}

#[test]
fn the_fit_is_zero_for_a_made_up_rigid_head() {
    let folder = folder("measuring-fit-rigid");
    MadeUp::front("first", seen(0.0, 500.0)).write(&folder, "front.toml");
    for turn in [20.0, 29.0, 45.0] {
        MadeUp::turned("first", seen(turn, 470.0)).write(&folder, "turned.toml");
        let found = fit_of(&folder, "front.toml", "turned.toml", "fit.toml");
        assert_eq!(
            number(&found["fit"]["percent_of_head_height"]),
            0.0,
            "a rigid head turned {turn} degrees"
        );
        for misfit in entries(&found, "misfit") {
            assert_eq!(number(&misfit["percent_of_head_height"]), 0.0);
        }
    }
}

#[test]
fn the_fit_is_above_zero_for_a_perturbed_head_and_names_the_worst_misfit() {
    let folder = folder("measuring-fit-perturbed");
    MadeUp::front("first", seen(0.0, 500.0)).write(&folder, "front.toml");

    // One landmark 6 pixels lower in the turned view than one head shows it: 6 / 480 = 1.25%
    // of head height. A height says nothing of the turn, so every other misfit stays zero.
    let mut lower = seen(29.0, 470.0);
    lower
        .get_mut("brow-line-left-outer-end")
        .expect("in the head")[1] += 6.0;
    MadeUp::turned("first", lower).write(&folder, "lower.toml");
    let found = fit_of(&folder, "front.toml", "lower.toml", "fit-level.toml");
    assert_close(
        number(&found["fit"]["percent_of_head_height"]),
        1.25,
        1e-9,
        "the fit",
    );
    let worst = entries(&found, "misfit")[0];
    assert_eq!(worst["kind"].as_str(), Some("level"));
    assert_eq!(worst["landmark"].as_str(), Some("brow-line-left-outer-end"));
    assert_close(
        number(&worst["percent_of_head_height"]),
        1.25,
        1e-9,
        "the worst",
    );
    assert_eq!(
        number(&entries(&found, "misfit")[1]["percent_of_head_height"]),
        0.0
    );

    // The front view's nose mark 7.2 pixels off the centre line. The centre line is the mean
    // of nine sideways positions: the chin, the nose mark's bottom, the mouth line's centre
    // and the midpoints of the six counted pairs. Eight stand at x = 500, so the mean is
    // 500 + 7.2 / 9 and the nose mark stands 7.2 * 8 / 9 = 6.4 pixels from it: 1.333%.
    let mut off_centre = seen(0.0, 500.0);
    off_centre.get_mut("nose-mark-bottom").expect("in the head")[0] += 7.2;
    MadeUp::front("first", off_centre).write(&folder, "off-centre.toml");
    MadeUp::turned("first", seen(29.0, 470.0)).write(&folder, "turned.toml");
    let found = fit_of(&folder, "off-centre.toml", "turned.toml", "fit-centre.toml");
    let worst = entries(&found, "misfit")[0];
    assert_eq!(worst["kind"].as_str(), Some("centre"));
    assert_eq!(worst["landmark"].as_str(), Some("nose-mark-bottom"));
    assert_close(
        number(&found["fit"]["percent_of_head_height"]),
        6.4 / 480.0 * 100.0,
        0.0005,
        "the fit",
    );

    // One pair too wide in the turned view. The fitted turn moves a little with it, so the
    // figure is not worked by hand here: the fit is above zero and the pair is the worst.
    let mut wide = seen(29.0, 470.0);
    wide.get_mut("mouth-line-left-end").expect("in the head")[0] += 9.0;
    MadeUp::turned("first", wide).write(&folder, "wide.toml");
    let found = fit_of(&folder, "front.toml", "wide.toml", "fit-spacing.toml");
    assert!(number(&found["fit"]["percent_of_head_height"]) > 1.0);
    let worst = entries(&found, "misfit")[0];
    assert_eq!(worst["kind"].as_str(), Some("spacing"));
    assert_eq!(worst["landmark"].as_str(), Some("mouth-line-end"));
}

#[test]
fn a_side_view_counts_in_the_fit_by_its_heights() {
    let folder = folder("measuring-fit-side");
    MadeUp::front("first", seen(0.0, 500.0)).write(&folder, "front.toml");
    MadeUp::turned("first", seen(29.0, 470.0)).write(&folder, "turned.toml");
    // A side view is the same head turned 90 degrees, and shows the near side only: the
    // character's right, when it faces the viewer's right.
    let side_view = |points: std::collections::BTreeMap<String, [f64; 2]>| {
        let mut side = MadeUp::turned("first", points);
        side.picture = "made-up-side.png".to_string();
        side.view = "side";
        side.stated_turn = 90.0;
        side.points.retain(|name, _| !name.contains("-left-"));
        side
    };
    let fit = |side: &str, out: &str| {
        measured(
            &folder,
            &[
                "fit-of-views",
                "--front",
                "front.toml",
                "--three-quarter",
                "turned.toml",
                "--side",
                side,
            ],
            out,
        )
    };
    side_view(seen(90.0, 430.0)).write(&folder, "side.toml");
    let found = fit("side.toml", "fit-rigid.toml");
    assert_eq!(number(&found["fit"]["percent_of_head_height"]), 0.0);

    // The near brow's outer end 6 pixels lower in the side view than in the two others:
    // 6 / 480 = 1.25% of head height.
    let mut lower = seen(90.0, 430.0);
    lower
        .get_mut("brow-line-right-outer-end")
        .expect("in the head")[1] += 6.0;
    side_view(lower).write(&folder, "side-lower.toml");
    let found = fit("side-lower.toml", "fit-lower.toml");
    assert_close(
        number(&found["fit"]["percent_of_head_height"]),
        1.25,
        1e-9,
        "the fit",
    );
    let worst = entries(&found, "misfit")[0];
    assert_eq!(worst["kind"].as_str(), Some("level"));
    assert_eq!(
        worst["landmark"].as_str(),
        Some("brow-line-right-outer-end")
    );
    assert!(
        worst["what"].as_str().expect("words").contains("side view"),
        "{worst:?}"
    );
}

#[test]
fn a_front_and_a_side_view_are_fitted_with_no_three_quarter_view_at_any_two_sizes() {
    let folder = folder("measuring-fit-front-and-side");
    MadeUp::front("first", seen(0.0, 500.0)).write(&folder, "front.toml");
    // The side view drawn at 0.6 of the front view's size, and moved: each view is measured
    // in its own unit, so the two still show one head.
    let side_view = |points: std::collections::BTreeMap<String, [f64; 2]>| {
        let mut side = MadeUp::turned("first", points);
        side.picture = "made-up-side.png".to_string();
        side.view = "side";
        side.stated_turn = 90.0;
        side.points.retain(|name, _| !name.contains("-left-"));
        for at in side.points.values_mut() {
            *at = [100.0 + 0.6 * at[0], 150.0 + 0.6 * at[1]];
        }
        side
    };
    let fit = |side: &str, out: &str| {
        measured(
            &folder,
            &["fit-of-views", "--front", "front.toml", "--side", side],
            out,
        )
    };
    side_view(seen(90.0, 430.0)).write(&folder, "side.toml");
    let found = fit("side.toml", "fit-rigid.toml");
    assert_close(
        number(&found["fit"]["percent_of_head_height"]),
        0.0,
        1e-9,
        "a rigid head",
    );
    // No three-quarter view: no turn, and no spacing to set against one.
    assert!(!found.contains_key("turn"), "{found:?}");
    assert_eq!(found["fit"]["spacing_compared"].as_integer(), Some(0));
    assert!(found["fit"]["level_compared"].as_integer() > Some(5));
    assert!(found["fit"]["centre_compared"].as_integer() > Some(5));

    // The near brow's outer end 6 pixels of the front view's size lower in the side view:
    // 6 / 480 = 1.25% of head height, which is the front view's.
    let mut lower = seen(90.0, 430.0);
    lower
        .get_mut("brow-line-right-outer-end")
        .expect("in the head")[1] += 6.0;
    side_view(lower).write(&folder, "side-lower.toml");
    let found = fit("side-lower.toml", "fit-lower.toml");
    assert_close(
        number(&found["fit"]["percent_of_head_height"]),
        1.25,
        1e-9,
        "the fit",
    );
    let worst = entries(&found, "misfit")[0];
    assert_eq!(worst["kind"].as_str(), Some("level"));
    assert_eq!(
        worst["landmark"].as_str(),
        Some("brow-line-right-outer-end")
    );

    // A front view alone has nothing to be fitted with.
    let alone = maquette_in(Some(&folder), &["fit-of-views", "--front", "front.toml"]);
    let message = failure(&alone);
    assert!(
        message.contains("a three-quarter view or a side view"),
        "{message}"
    );
}

// ---------------------------------------------------------------- what is measured

/// The four commands that work something out from readings, on the readings named.
fn measuring_commands<'a>(front: &'a str, turned: &'a str, second: &'a str) -> [Vec<&'a str>; 4] {
    [
        vec![
            "agreement-of-readings",
            "--first",
            front,
            "--second",
            second,
        ],
        vec![
            "turn-from-readings",
            "--front",
            front,
            "--three-quarter",
            turned,
        ],
        vec!["fit-of-views", "--front", front, "--three-quarter", turned],
        vec![
            "proportions-from-readings",
            "--front",
            front,
            "--three-quarter",
            turned,
        ],
    ]
}

/// The three of them that take a view as one reading or two, with the agreement of the two
/// readings named beside them.
fn commands_on_two_readings<'a>(
    first: &'a str,
    second: &'a str,
    both: &'a str,
    turned: &'a str,
) -> Vec<Vec<&'a str>> {
    let mut commands = measuring_commands(both, turned, second).to_vec();
    commands[0] = vec![
        "agreement-of-readings",
        "--first",
        first,
        "--second",
        second,
    ];
    commands
}

#[test]
fn a_reading_that_does_not_pass_its_check_is_not_measured() {
    let folder = folder("measuring-unchecked");
    MadeUp::front("first", seen(0.0, 500.0)).write(&folder, "front.toml");
    MadeUp::front("second", seen(0.0, 500.0)).write(&folder, "front-second.toml");
    MadeUp::turned("first", seen(29.0, 470.0)).write(&folder, "turned.toml");

    // One landmark the view asks for is neither placed nor stated as not seen.
    let whole = text(&folder.join("front.toml"));
    let unfinished = whole.replace(
        "ear-left-base-outer = \"made up: not in the made-up head\"\n",
        "",
    );
    assert_ne!(unfinished, whole);
    std::fs::write(folder.join("unfinished.toml"), unfinished).expect("written");
    for arguments in measuring_commands("unfinished.toml", "turned.toml", "front-second.toml") {
        let message = failure(&maquette_in(Some(&folder), &arguments));
        let command = arguments[0];
        assert!(
            message.contains("unfinished.toml")
                && message.contains("ear-left-base-outer")
                && message.contains("check-reading"),
            "{command}: {message}"
        );
    }
    // The second of two readings is held to the same.
    let message = failure(&maquette_in(
        Some(&folder),
        &[
            "turn-from-readings",
            "--front",
            "front-second.toml,unfinished.toml",
            "--three-quarter",
            "turned.toml",
        ],
    ));
    assert!(message.contains("unfinished.toml"), "{message}");

    // The picture is no longer the one that was read.
    support::made_up_image(&folder.join("made-up-front.png"), 990, 1000);
    for arguments in measuring_commands("front.toml", "turned.toml", "front-second.toml") {
        let message = failure(&maquette_in(Some(&folder), &arguments));
        let command = arguments[0];
        assert!(
            message.contains("front.toml")
                && message.contains("made-up-front.png")
                && message.contains("not the picture that was read"),
            "{command}: {message}"
        );
    }

    // The picture is gone: nothing says the reading is of it.
    std::fs::remove_file(folder.join("made-up-three-quarter.png")).expect("removed");
    std::fs::remove_file(folder.join("made-up-front.png")).expect("removed");
    MadeUp::front("first", seen(0.0, 500.0)).write(&folder, "front-again.toml");
    for command in [
        "turn-from-readings",
        "fit-of-views",
        "proportions-from-readings",
    ] {
        let message = failure(&maquette_in(
            Some(&folder),
            &[
                command,
                "--front",
                "front-again.toml",
                "--three-quarter",
                "turned.toml",
            ],
        ));
        assert!(
            message.contains("turned.toml")
                && message.contains("made-up-three-quarter.png")
                && message.contains("not in the reference folder"),
            "{command}: {message}"
        );
    }
}

#[test]
fn readings_under_another_revision_of_the_instructions_are_measured_and_the_output_says_so() {
    let folder = folder("measuring-other-revision");
    let own = support::instructions_revision();
    let earlier = |mut reading: MadeUp| {
        reading.instructions = Some("made-up earlier revision".to_string());
        reading
    };
    earlier(MadeUp::front("first", seen(0.0, 500.0))).write(&folder, "front-first.toml");
    earlier(MadeUp::front("second", seen(0.0, 500.0))).write(&folder, "front-second.toml");
    MadeUp::front("third", seen(0.0, 500.0)).write(&folder, "front-third.toml");
    MadeUp::front("fourth", seen(0.0, 500.0)).write(&folder, "front-fourth.toml");
    MadeUp::turned("first", seen(29.0, 470.0)).write(&folder, "turned.toml");
    // Where a file says which readings it is worked out from.
    let said = |found: &toml::Table| -> Vec<String> {
        found
            .get("readings")
            .or(found.get("source"))
            .and_then(|table| table.get("other_instructions"))
            .map(texts)
            .unwrap_or_default()
    };
    let names_both = |line: &str| line.contains("made-up earlier revision") && line.contains(&own);

    // Two readings of one drawing under one earlier revision are measured, and it is said:
    // in the file, and in what is printed for a person.
    for arguments in commands_on_two_readings(
        "front-first.toml",
        "front-second.toml",
        "front-first.toml,front-second.toml",
        "turned.toml",
    ) {
        let command = arguments[0];
        let notes = said(&measured(&folder, &arguments, "found.toml"));
        assert_eq!(notes.len(), 1, "{command}: {notes:?}");
        assert!(names_both(&notes[0]), "{command}: {notes:?}");
        if command != "agreement-of-readings" {
            // The view that was read under it is named; the other is not.
            assert!(
                notes[0].contains("front view") && !notes[0].contains("three-quarter"),
                "{command}: {notes:?}"
            );
        }
        let mut with_out = arguments.clone();
        with_out.extend(["--out", "found-again.toml"]);
        let printed = measure_ok(&folder, &with_out);
        assert!(names_both(&printed), "{command}: {printed}");
    }

    // Readings under the program's own revision carry no such note.
    for arguments in commands_on_two_readings(
        "front-third.toml",
        "front-fourth.toml",
        "front-third.toml,front-fourth.toml",
        "turned.toml",
    ) {
        let found = measured(&folder, &arguments, "found-own.toml");
        assert!(said(&found).is_empty(), "{}: {found:?}", arguments[0]);
    }

    // Two readings of one drawing under two revisions are not two readings of it alike.
    for arguments in commands_on_two_readings(
        "front-first.toml",
        "front-third.toml",
        "front-first.toml,front-third.toml",
        "turned.toml",
    ) {
        let message = failure(&maquette_in(Some(&folder), &arguments));
        let command = arguments[0];
        assert!(
            message.contains("made-up earlier revision")
                && message.contains(&own)
                && message.contains("one revision"),
            "{command}: {message}"
        );
    }
}

#[test]
fn one_reading_named_twice_or_two_by_one_reader_are_not_the_two_readings_of_a_drawing() {
    let folder = folder("measuring-not-two-readings");
    MadeUp::front("first", seen(0.0, 500.0)).write(&folder, "front.toml");
    MadeUp::turned("first", seen(29.0, 470.0)).write(&folder, "turned.toml");
    std::fs::create_dir(folder.join("copies")).expect("made");
    std::fs::copy(folder.join("front.toml"), folder.join("copies/front.toml")).expect("copied");

    // The same file, under one spelling of its path or two.
    for (second, both) in [
        ("front.toml", "front.toml,front.toml"),
        ("./front.toml", "front.toml,./front.toml"),
    ] {
        for arguments in commands_on_two_readings("front.toml", second, both, "turned.toml") {
            let message = failure(&maquette_in(Some(&folder), &arguments));
            assert!(
                message.contains("front.toml") && message.contains("twice"),
                "{} {both}: {message}",
                arguments[0]
            );
        }
    }

    // Another file that carries the same reader's label.
    for arguments in commands_on_two_readings(
        "front.toml",
        "copies/front.toml",
        "front.toml,copies/front.toml",
        "turned.toml",
    ) {
        let message = failure(&maquette_in(Some(&folder), &arguments));
        assert!(
            message.contains("made-up-front.png")
                && message.contains("\"first\"")
                && message.contains("separate readers"),
            "{}: {message}",
            arguments[0]
        );
    }
}
