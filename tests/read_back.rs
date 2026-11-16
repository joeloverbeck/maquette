//! The read-back, through the command line, where VectorCraft is not installed.
//!
//! `cargo test` on a clean clone and in GitHub Actions has no VectorCraft. So these tests
//! never start it, and run the program with no PATH to be sure of that. They give the
//! command recordings of what VectorCraft replied and check what the command makes of them.
//!
//! The recordings are in `tests/fixtures/read-back/`. Each says it is a recording and which
//! VectorCraft version it is of. `maquette read-back-hard-case --record` wrote them from the
//! three-quarter view of the made-up set of proportions: the view as generated, and the two
//! spoiled copies of the SVG contract's hard case. The landmark file beside them is the one
//! written with that view. Where a test needs a reply VectorCraft did not give, it changes
//! one thing in the recording of the view as generated and says which.
//!
//! A recording shows what one version did once. The live read-back, against whichever
//! version is installed, is `tests/read_back_in_vectorcraft.rs`, which is run by hand.

mod support;

use serde_json::{Value, json};
use support::{draw, path_text, repository, scratch, text};

const LANDMARKS: &str = "tests/fixtures/read-back/made-up-three-quarter.landmarks.toml";
const AS_GENERATED: &str = "tests/fixtures/read-back/as-generated.json";
const SPOILED_HIDDEN: &str = "tests/fixtures/read-back/spoiled-hidden.json";
const SPOILED_EXCHANGED: &str = "tests/fixtures/read-back/spoiled-exchanged.json";

struct Ran {
    status: Option<i32>,
    printed: String,
    message: String,
}

/// Runs the program with no PATH, so that no `vectorcraft-cli` can be found.
fn maquette_without_vectorcraft(arguments: &[&str]) -> Ran {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_maquette"))
        .args(arguments)
        .current_dir(repository())
        .env("PATH", "")
        .output()
        .expect("the program starts");
    Ran {
        status: output.status.code(),
        printed: String::from_utf8_lossy(&output.stdout).into_owned(),
        message: String::from_utf8_lossy(&output.stderr).into_owned(),
    }
}

fn read_back(landmarks: &str, replies: &str) -> Ran {
    maquette_without_vectorcraft(&["read-back", "--landmarks", landmarks, "--replies", replies])
}

/// The recording of the view as generated, with one thing changed, read back.
fn read_back_changed(name: &str, change: impl FnOnce(&mut Value)) -> Ran {
    let mut recording: Value =
        serde_json::from_str(&text(&repository().join(AS_GENERATED))).expect("JSON");
    change(&mut recording);
    let path = scratch(name).join("changed.json");
    std::fs::write(&path, recording.to_string()).expect("the changed recording is written");
    read_back(LANDMARKS, path_text(&path))
}

/// Applies a change to every object of that name, anywhere in a reply.
fn for_each_named(value: &mut Value, name: &str, change: &mut dyn FnMut(&mut Value)) {
    match value {
        Value::Object(entries) => {
            if entries.get("name").and_then(Value::as_str) == Some(name) {
                change(value);
                return;
            }
            for (_, entry) in entries.iter_mut() {
                for_each_named(entry, name, change);
            }
        }
        Value::Array(items) => {
            for item in items {
                for_each_named(item, name, change);
            }
        }
        _ => {}
    }
}

/// Changes how `inspect_document` shows the object of that name.
fn inspected(recording: &mut Value, name: &str, mut change: impl FnMut(&mut Value)) {
    let mut found = 0;
    for_each_named(&mut recording["inspect_document"], name, &mut |object| {
        found += 1;
        change(object);
    });
    assert_eq!(found, 1, "{name} is shown once by inspect_document");
}

/// Changes what `document.node` gives for the object of that name.
fn node(recording: &mut Value, name: &str, mut change: impl FnMut(&mut Value)) {
    let mut found = 0;
    for_each_named(&mut recording["document_node"], name, &mut |object| {
        found += 1;
        change(object);
    });
    assert_eq!(found, 1, "{name} is given once by document.node");
}

/// Moves one anchor of a path sideways.
fn move_anchor(recording: &mut Value, guide: &str, anchor: usize, by: f64) {
    node(recording, guide, |object| {
        let x = &mut object["kind"]["path"]["subpaths"][0]["anchors"][anchor]["p"][0];
        *x = json!(x.as_f64().expect("a number") + by);
    });
}

#[test]
fn where_vectorcraft_is_not_installed_it_says_so_and_that_is_not_a_failure() {
    let folder = draw("read-back-not-installed", support::MADE_UP, &[]);
    let landmarks = folder.join("three-quarter.landmarks.toml");
    for command in ["read-back", "read-back-hard-case"] {
        let ran = maquette_without_vectorcraft(&[command, "--landmarks", path_text(&landmarks)]);
        // Neither the status of a pass nor the status of a failed check.
        assert_eq!(ran.status, Some(2), "{command}");
        assert!(
            ran.message.contains("vectorcraft-cli is not installed"),
            "{}",
            ran.message
        );
        assert!(
            ran.message.contains("neither a pass nor a failure"),
            "{}",
            ran.message
        );
        assert!(
            ran.printed.is_empty(),
            "nothing is printed as if something had been read back"
        );
    }
    // A landmark file that is not there is an ordinary error, with another status.
    let missing =
        maquette_without_vectorcraft(&["read-back", "--landmarks", "no-such.landmarks.toml"]);
    assert_eq!(missing.status, Some(1));
}

#[test]
fn a_recording_of_the_view_as_generated_passes_and_is_reported_as_a_recording() {
    let ran = read_back(LANDMARKS, AS_GENERATED);
    assert_eq!(ran.status, Some(0), "{}{}", ran.printed, ran.message);
    // The version is the one the recording carries, not one written into the program.
    let recording: Value =
        serde_json::from_str(&text(&repository().join(AS_GENERATED))).expect("JSON");
    let version = recording["vectorcraft_version"]
        .as_str()
        .expect("a version");
    assert!(
        ran.printed.contains(&format!(
            "in VectorCraft {version} (headless MCP server), from a recording, not a live run"
        )),
        "{}",
        ran.printed
    );
    for line in [
        "1. Import warning: none. pass",
        "2. Names: 26 objects read back. The landmark file gives 26 names: 1 layer, 3 groups and 22 guides, each to be read back once. pass",
        "the 5 closed ones included. pass",
        "Expected values come from it, never from the SVG.",
        // What the SVG does not carry cannot be read back, and the read-back says which.
        "Not compared: 10 positions of the landmark file are neither an anchor nor a centre of any guide",
        "`eye-left-opening-centre`",
        "The handles between anchors are not in the landmark file and are not compared.",
        "PASSED",
    ] {
        assert!(
            ran.printed.contains(line),
            "`{line}` is not in:\n{}",
            ran.printed
        );
    }
    assert!(
        ran.printed
            .contains("3. Positions: 80 anchors of 22 guides compared")
    );
    assert!(!ran.printed.contains("FAIL"));
    // The read-back shows the tree a person can check: a group, and a guide inside it.
    assert!(
        ran.printed
            .contains("  neck [Group] visible\n    neck-right [Path, open] visible")
    );
}

#[test]
fn a_recording_carries_no_path_of_the_machine_it_was_made_on() {
    for recording in [AS_GENERATED, SPOILED_HIDDEN, SPOILED_EXCHANGED] {
        let recorded = text(&repository().join(recording));
        assert!(
            recorded.contains("\"recording\": \"A recording of"),
            "{recording}"
        );
        for sign in ["/home/", "/Users/", "/tmp/", "\\\\"] {
            assert!(!recorded.contains(sign), "{recording} holds `{sign}`");
        }
    }
}

#[test]
fn the_spoiled_copy_with_a_hidden_guide_fails_because_its_name_is_not_read_back() {
    let ran = read_back(LANDMARKS, SPOILED_HIDDEN);
    assert_eq!(ran.status, Some(1), "{}", ran.printed);
    // VectorCraft dropped the guide and warned of nothing.
    assert!(ran.printed.contains("1. Import warning: none. pass"));
    assert!(ran.printed.contains("2. Names: 25 objects read back."));
    assert!(ran.printed.contains("each to be read back once. FAIL"));
    assert!(
        ran.printed
            .contains("  - names: `brow-line-left` is in the landmark file and is not read back"),
        "{}",
        ran.printed
    );
    assert!(ran.printed.contains("FAILED: 1 failure"), "{}", ran.printed);
}

#[test]
fn the_spoiled_copy_with_a_pairs_names_exchanged_fails_on_positions_alone() {
    let ran = read_back(LANDMARKS, SPOILED_EXCHANGED);
    assert_eq!(ran.status, Some(1), "{}", ran.printed);
    // The same set of names, each once, and no warning: only the positions show it.
    assert!(ran.printed.contains("1. Import warning: none. pass"));
    assert!(ran.printed.contains("2. Names: 26 objects read back."));
    assert!(ran.printed.contains("each to be read back once. pass"));
    assert!(ran.printed.contains("4. Paint: every guide to be visible, stroked and unfilled, the 5 closed ones included. pass"));
    let failures: Vec<&str> = ran
        .printed
        .lines()
        .filter(|line| line.starts_with("  - "))
        .collect();
    // Each opening has four anchors, and every one of them is where the other's should be.
    assert_eq!(failures.len(), 8, "{}", ran.printed);
    for failure in &failures {
        assert!(
            failure.starts_with("  - positions: `eye-opening-left`")
                || failure.starts_with("  - positions: `eye-opening-right`"),
            "{failure}"
        );
    }
    assert!(
        failures
            .iter()
            .any(|failure| failure.contains("`eye-opening-left`"))
    );
    assert!(
        failures
            .iter()
            .any(|failure| failure.contains("`eye-opening-right`"))
    );
}

#[test]
fn an_import_warning_fails() {
    let ran = read_back_changed("read-back-warning", |recording| {
        recording["open_file"]["warnings"] = json!(["the file uses something that was dropped"]);
    });
    assert_eq!(ran.status, Some(1));
    assert!(
        ran.printed.contains("1. Import warning: 1. FAIL"),
        "{}",
        ran.printed
    );
    assert!(
        ran.printed.contains(
            "  - import warning: the import warns: \"the file uses something that was dropped\""
        ),
        "{}",
        ran.printed
    );
}

#[test]
fn an_anchor_further_than_a_thousandth_of_a_unit_from_the_landmark_file_fails() {
    // The nose mark's second anchor is its bottom.
    let moved = read_back_changed("read-back-anchor-moved", |recording| {
        move_anchor(recording, "nose-mark", 1, 0.002);
    });
    assert_eq!(moved.status, Some(1));
    assert!(
        moved.printed.contains("within 0.001 unit"),
        "{}",
        moved.printed
    );
    assert!(
        moved
            .printed
            .contains("  - positions: `nose-mark`: its anchor `nose-mark-bottom` is read back at"),
        "{}",
        moved.printed
    );
    assert!(
        moved.printed.contains("FAILED: 1 failure"),
        "{}",
        moved.printed
    );

    // Half a thousandth is inside the tolerance.
    let within = read_back_changed("read-back-anchor-within", |recording| {
        move_anchor(recording, "nose-mark", 1, 0.0005);
    });
    assert_eq!(within.status, Some(0), "{}", within.printed);
}

#[test]
fn a_circle_is_checked_through_its_four_outer_points() {
    // A circle comes back as four anchors, right, bottom, left and top, and none at its
    // centre. Its right anchor 0.004 further out: that anchor is off, and so is the radius
    // the two side anchors give, by half as much.
    let ran = read_back_changed("read-back-circle", |recording| {
        move_anchor(recording, "skull", 0, 0.004);
    });
    assert_eq!(ran.status, Some(1));
    assert!(
        ran.printed
            .contains("  - positions: `skull`: its anchor `skull-left-extent` is read back at"),
        "{}",
        ran.printed
    );
    assert!(
        ran.printed.contains(
            "  - positions: `skull`: its sideways radius, half of right to left, is read back as"
        ),
        "{}",
        ran.printed
    );

    // The top anchor 0.004 to the side: the centre it carries is off.
    let ran = read_back_changed("read-back-circle-centre", |recording| {
        move_anchor(recording, "skull", 3, 0.004);
    });
    assert_eq!(ran.status, Some(1));
    assert!(
        ran.printed
            .contains("  - positions: `skull`: its centre x, from the top anchor, is read back as"),
        "{}",
        ran.printed
    );
}

#[test]
fn a_guide_with_another_number_of_anchors_or_in_pieces_fails() {
    let fewer = read_back_changed("read-back-anchors-fewer", |recording| {
        node(recording, "face-outline", |object| {
            let anchors = object["kind"]["path"]["subpaths"][0]["anchors"]
                .as_array_mut()
                .expect("anchors");
            anchors.pop();
        });
    });
    assert_eq!(fewer.status, Some(1));
    assert!(
        fewer.printed.contains(
            "  - positions: `face-outline` is read back with 10 anchors, and the landmark file gives it 11"
        ),
        "{}",
        fewer.printed
    );

    let pieces = read_back_changed("read-back-pieces", |recording| {
        node(recording, "face-outline", |object| {
            let subpaths = object["kind"]["path"]["subpaths"]
                .as_array_mut()
                .expect("subpaths");
            let copy = subpaths[0].clone();
            subpaths.push(copy);
        });
    });
    assert_eq!(pieces.status, Some(1));
    assert!(
        pieces
            .printed
            .contains("  - positions: `face-outline` is read back as 2 pieces"),
        "{}",
        pieces.printed
    );
}

#[test]
fn a_guide_that_is_filled_unstroked_or_hidden_fails() {
    let filled = read_back_changed("read-back-filled", |recording| {
        inspected(recording, "eye-opening-left", |object| {
            object["fill"] = json!("#000000");
        });
    });
    assert_eq!(filled.status, Some(1));
    assert!(
        filled
            .printed
            .contains("  - paint: `eye-opening-left` is filled: fill #000000"),
        "{}",
        filled.printed
    );

    let unstroked = read_back_changed("read-back-unstroked", |recording| {
        inspected(recording, "centre-line", |object| {
            object["stroke"] = json!("None");
        });
    });
    assert_eq!(unstroked.status, Some(1));
    assert!(
        unstroked
            .printed
            .contains("  - paint: `centre-line` has no stroke"),
        "{}",
        unstroked.printed
    );

    let no_width = read_back_changed("read-back-no-width", |recording| {
        inspected(recording, "centre-line", |object| {
            object["strokeWidth"] = json!(0.0);
        });
    });
    assert_eq!(no_width.status, Some(1));
    assert!(
        no_width
            .printed
            .contains("  - paint: `centre-line` has no stroke"),
        "{}",
        no_width.printed
    );

    let hidden = read_back_changed("read-back-hidden", |recording| {
        inspected(recording, "ear-left", |object| {
            object["visible"] = json!(false);
        });
    });
    assert_eq!(hidden.status, Some(1));
    assert!(
        hidden.printed.contains("  - paint: `ear-left` is hidden"),
        "{}",
        hidden.printed
    );
    assert!(hidden.printed.contains("ear-left [Path, open] HIDDEN"));
    assert!(
        hidden.printed.contains("FAILED: 1 failure"),
        "{}",
        hidden.printed
    );
}

#[test]
fn a_guide_inside_a_hidden_group_is_not_visible() {
    let ran = read_back_changed("read-back-hidden-group", |recording| {
        inspected(recording, "neck", |object| {
            object["visible"] = json!(false);
        });
    });
    assert_eq!(ran.status, Some(1));
    for guide in ["neck-left", "neck-right"] {
        assert!(
            ran.printed.contains(&format!(
                "  - paint: `{guide}` is not shown, because `neck` above it is hidden"
            )),
            "{}",
            ran.printed
        );
    }
    assert!(
        ran.printed.contains("FAILED: 2 failures"),
        "{}",
        ran.printed
    );
}

#[test]
fn a_name_read_back_twice_an_object_without_a_name_and_a_stray_name_fail() {
    let twice = read_back_changed("read-back-name-twice", |recording| {
        node(recording, "brow-line-left", |object| {
            object["name"] = json!("brow-line-right");
        });
    });
    assert_eq!(twice.status, Some(1));
    for line in [
        "  - names: `brow-line-right` is read back 2 times, and a name stands once",
        "  - names: `brow-line-left` is in the landmark file and is not read back",
    ] {
        assert!(
            twice.printed.contains(line),
            "`{line}` is not in:\n{}",
            twice.printed
        );
    }

    let unnamed = read_back_changed("read-back-unnamed", |recording| {
        node(recording, "mouth-line", |object| {
            object.as_object_mut().expect("an object").remove("name");
        });
    });
    assert_eq!(unnamed.status, Some(1));
    assert!(
        unnamed
            .printed
            .contains("  - names: an object without a name is read back: a Path"),
        "{}",
        unnamed.printed
    );

    let stray = read_back_changed("read-back-stray", |recording| {
        node(recording, "mouth-line", |object| {
            object["name"] = json!("mouth");
        });
    });
    assert_eq!(stray.status, Some(1));
    assert!(
        stray
            .printed
            .contains("  - names: `mouth` is read back, a Path, and is not in the landmark file"),
        "{}",
        stray.printed
    );
}

#[test]
fn a_group_that_arrives_as_something_else_fails() {
    let ran = read_back_changed("read-back-kind", |recording| {
        inspected(recording, "neck", |object| {
            object["kind"] = json!("Path");
        });
    });
    assert_eq!(ran.status, Some(1));
    assert!(
        ran.printed.contains(
            "  - names: `neck` is read back as a Path, and the landmark file gives a Group"
        ),
        "{}",
        ran.printed
    );
}

#[test]
fn the_expected_positions_are_the_landmark_files() {
    // The same recording, and a landmark file that puts the chin 0.002 unit to the side:
    // the two guides that pass through the chin no longer agree with what was read back.
    let landmarks = text(&repository().join(LANDMARKS));
    let table: toml::Table = landmarks.parse().expect("TOML");
    let chin = support::number(&table["landmarks"]["chin"]["x"]);
    let line = landmarks
        .lines()
        .find(|line| line.starts_with("chin = "))
        .expect("the chin's line");
    let moved_line = line.replacen(
        &format!("x = {chin}"),
        &format!("x = {:.3}", chin + 0.002),
        1,
    );
    assert_ne!(line, moved_line, "the chin's line was changed");
    let path = scratch("read-back-landmark-moved").join("moved.landmarks.toml");
    std::fs::write(&path, landmarks.replacen(line, &moved_line, 1)).expect("written");

    let ran = read_back(path_text(&path), AS_GENERATED);
    assert_eq!(ran.status, Some(1), "{}", ran.printed);
    let failures: Vec<&str> = ran
        .printed
        .lines()
        .filter(|line| line.starts_with("  - "))
        .collect();
    assert_eq!(failures.len(), 2, "{}", ran.printed);
    for guide in ["face-outline", "centre-line"] {
        assert!(
            failures.iter().any(|failure| failure.starts_with(&format!(
                "  - positions: `{guide}`: its anchor `chin` is read back at"
            ))),
            "{}",
            ran.printed
        );
    }
}

#[test]
fn a_file_that_does_not_say_it_is_a_recording_is_refused() {
    let folder = scratch("read-back-not-a-recording");
    let unmarked = folder.join("unmarked.json");
    std::fs::write(&unmarked, "{\"open_file\": {\"warnings\": []}}").expect("written");
    let ran = read_back(LANDMARKS, path_text(&unmarked));
    assert_eq!(ran.status, Some(1));
    assert!(
        ran.message.contains("does not say it is a recording"),
        "{}",
        ran.message
    );
    assert!(ran.printed.is_empty());

    let ran = read_back(LANDMARKS, "Cargo.toml");
    assert_eq!(ran.status, Some(1));
    assert!(ran.message.contains("is not valid JSON"), "{}", ran.message);
}

#[test]
fn replies_in_another_shape_stop_the_read_back_in_plain_words() {
    // VectorCraft changes almost daily. If a version answers in another shape, nothing is
    // passed or failed on a guess.
    let ran = read_back_changed("read-back-shape", |recording| {
        recording["open_file"]
            .as_object_mut()
            .expect("an object")
            .remove("warnings");
    });
    assert_eq!(ran.status, Some(1));
    assert!(ran.printed.is_empty());
    assert!(
        ran.message
            .contains("VectorCraft's reply to `open_file` has no list `warnings`"),
        "{}",
        ran.message
    );
    assert!(
        ran.message.contains("probes have to be rerun"),
        "{}",
        ran.message
    );
}
