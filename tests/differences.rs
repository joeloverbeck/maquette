//! The differences between a generated armature and a hand-made one, through the command
//! line: two landmark files and a record in, a list of differences out.
//!
//! The first tests run on made-up files in `tests/fixtures/differences/`, which say they are
//! made up. Every expected number below is worked by hand from the numbers typed there, and
//! the comments in those files give the arithmetic. The last tests run on Saffi: they check
//! that her list is produced and agrees with her record, never that it is empty. Whether a
//! difference matters is Jon's look.

mod support;

use support::{assert_close, maquette, maquette_ok, number, path_text, scratch, text};

const MADE_UP_RECORD: &str = "tests/fixtures/differences/made-up-hand-made-views.toml";
const MADE_UP_FRONT: &str = "tests/fixtures/differences/made-up-front.landmarks.toml";
const MADE_UP_THREE_QUARTER: &str =
    "tests/fixtures/differences/made-up-three-quarter.landmarks.toml";

const MADE_UP_FINAL_RECORD: &str = "tests/fixtures/differences/made-up-final-drawings.toml";

/// Runs the comparison and reads the file it writes.
fn differences(name: &str, front: &str, three_quarter: &str, record: &str) -> toml::Table {
    differences_with(name, front, three_quarter, record, None)
}

/// Runs the comparison, with the record of the final drawings when one is given, and reads
/// the file it writes.
fn differences_with(
    name: &str,
    front: &str,
    three_quarter: &str,
    record: &str,
    final_record: Option<&str>,
) -> toml::Table {
    let folder = scratch(name);
    let out = folder.join("differences.toml");
    let mut arguments = vec![
        "differences-from-hand-made-views",
        "--front",
        front,
        "--three-quarter",
        three_quarter,
        "--record",
        record,
        "--out",
        path_text(&out),
    ];
    if let Some(final_record) = final_record {
        arguments.extend(["--final-record", final_record]);
    }
    maquette_ok(&arguments);
    text(&out).parse().expect("the differences file is TOML")
}

fn made_up(name: &str) -> toml::Table {
    differences(name, MADE_UP_FRONT, MADE_UP_THREE_QUARTER, MADE_UP_RECORD)
}

fn entries<'a>(table: &'a toml::Table, view: &str, list: &str) -> Vec<&'a toml::Table> {
    table[view][list]
        .as_array()
        .unwrap_or_else(|| panic!("[[{view}.{list}]]"))
        .iter()
        .map(|entry| entry.as_table().expect("a difference is a table"))
        .collect()
}

fn names(table: &toml::Table, view: &str, list: &str) -> Vec<String> {
    entries(table, view, list)
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

#[test]
fn every_difference_above_one_percent_is_listed_and_numbered_largest_first() {
    let table = made_up("differences-listed");

    // The front view, laid over at twice its size. Head height is 400, so 1% is 4 units.
    let front = entries(&table, "front", "above_one_percent");
    let listed: Vec<(i64, &str, f64, f64)> = front
        .iter()
        .map(|entry| {
            (
                entry["number"].as_integer().expect("a number"),
                entry["landmark"].as_str().expect("a name"),
                number(&entry["distance"]),
                number(&entry["percent_of_head_height"]),
            )
        })
        .collect();
    assert_eq!(
        listed,
        vec![
            (1, "shoulder-left-point", 40.0, 10.0),
            (2, "hair-right-extent", 8.0, 2.0),
            (3, "eye-right-outer-corner", 5.0, 1.25),
            (4, "mouth-line-centre", 4.004, 1.001),
        ]
    );

    // The three-quarter view carries the numbering on.
    let turned = entries(&table, "three-quarter", "above_one_percent");
    let listed: Vec<(i64, &str, f64, f64)> = turned
        .iter()
        .map(|entry| {
            (
                entry["number"].as_integer().expect("a number"),
                entry["landmark"].as_str().expect("a name"),
                number(&entry["distance"]),
                number(&entry["percent_of_head_height"]),
            )
        })
        .collect();
    assert_eq!(
        listed,
        vec![
            (5, "ear-left-base-outer", 8.0, 2.0),
            (6, "lid-line-left-inner-end", 6.0, 1.5),
            (7, "nose-mark-bottom", 4.012, 1.003),
        ]
    );
}

#[test]
fn a_position_the_head_covers_is_compared_and_marked_as_covered() {
    let table = made_up("differences-covered");
    let turned = entries(&table, "three-quarter", "above_one_percent");
    let ear = turned
        .iter()
        .find(|entry| entry["landmark"].as_str() == Some("ear-left-base-outer"))
        .expect("the far ear's base is compared");
    assert_eq!(ear["covered_by_the_head"].as_bool(), Some(true));
    assert_eq!(number(&ear["hand_made"]["x"]), 640.0);
    assert_eq!(number(&ear["hand_made"]["y"]), 180.0);
    // Nothing else is marked.
    for view in ["front", "three-quarter"] {
        for list in ["above_one_percent", "at_or_under_one_percent"] {
            for entry in entries(&table, view, list) {
                if entry["landmark"].as_str() != Some("ear-left-base-outer") {
                    assert!(entry.get("covered_by_the_head").is_none());
                }
            }
        }
    }
}

#[test]
fn two_landmarks_at_one_position_of_a_view_are_one_difference() {
    let table = made_up("differences-one-position");
    let under = entries(&table, "front", "at_or_under_one_percent");
    let outline = under
        .iter()
        .find(|entry| entry["landmark"].as_str() == Some("face-outline-left-at-eye-level"))
        .expect("the outline at the eye level is compared");
    assert_eq!(texts(&outline["also"]), ["jaw-line-left-at-eye-level"]);
    assert_eq!(number(&outline["distance"]), 0.0);
    let compared: Vec<String> = ["above_one_percent", "at_or_under_one_percent"]
        .iter()
        .flat_map(|list| names(&table, "front", list))
        .collect();
    assert!(!compared.contains(&"jaw-line-left-at-eye-level".to_string()));
    // And it is not said to lack a counterpart.
    assert!(
        !texts(&table["front"]["without_counterpart"]["generated"])
            .contains(&"jaw-line-left-at-eye-level".to_string())
    );
}

#[test]
fn a_difference_of_exactly_one_percent_is_not_above_it() {
    let table = made_up("differences-boundary");
    let under = entries(&table, "front", "at_or_under_one_percent");
    let nose = under
        .iter()
        .find(|entry| entry["landmark"].as_str() == Some("nose-mark-bottom"))
        .expect("the nose mark's bottom is listed at or under 1%");
    assert_eq!(number(&nose["distance"]), 4.0);
    assert_eq!(number(&nose["percent_of_head_height"]), 1.0);
    assert!(
        nose.get("number").is_none(),
        "only a difference above 1% is numbered"
    );
    // The one 0.004 unit further is above it.
    assert!(names(&table, "front", "above_one_percent").contains(&"mouth-line-centre".to_string()));
    assert!(!names(&table, "front", "above_one_percent").contains(&"nose-mark-bottom".to_string()));
}

#[test]
fn each_difference_gives_both_positions_in_the_records_units() {
    let table = made_up("differences-positions");
    let front = entries(&table, "front", "above_one_percent");
    let corner = front
        .iter()
        .find(|entry| entry["landmark"].as_str() == Some("eye-right-outer-corner"))
        .expect("the corner");
    // 176.5, 152 in the file, drawn at half size: 500 + 2 (176.5 - 250), 300 + 2 (152 - 150).
    assert_eq!(number(&corner["generated"]["x"]), 353.0);
    assert_eq!(number(&corner["generated"]["y"]), 304.0);
    assert_eq!(number(&corner["hand_made"]["x"]), 350.0);
    assert_eq!(number(&corner["hand_made"]["y"]), 300.0);
    assert_eq!(texts(&corner["hand_made_from"]), ["eye_right_outer_corner"]);
    assert_eq!(corner["compared"].as_str(), Some("position"));

    // A lid line's end is the middle of the record's band, so two of its positions stand behind it.
    let turned = entries(&table, "three-quarter", "above_one_percent");
    let lid = turned
        .iter()
        .find(|entry| entry["landmark"].as_str() == Some("lid-line-left-inner-end"))
        .expect("the lid line's end");
    assert_eq!(number(&lid["generated"]["x"]), 600.0);
    assert_eq!(number(&lid["generated"]["y"]), 311.0);
    assert_eq!(number(&lid["hand_made"]["x"]), 600.0);
    assert_eq!(number(&lid["hand_made"]["y"]), 305.0);
    assert_eq!(
        texts(&lid["hand_made_from"]),
        ["lid_line_left_inner_lower", "lid_line_left_inner_upper"]
    );
}

#[test]
fn a_record_of_final_drawings_replaces_the_records_landmarks_of_the_same_name() {
    let find = |table: &toml::Table, view: &str, name: &str| -> toml::Table {
        ["above_one_percent", "at_or_under_one_percent"]
            .iter()
            .flat_map(|list| entries(table, view, list))
            .find(|entry| entry["landmark"].as_str() == Some(name))
            .unwrap_or_else(|| panic!("{name} is compared in the {view} view"))
            .clone()
    };

    // Without it the generated corner stands on the record's, at 650, 300.
    let without = made_up("differences-without-final");
    let corner = find(&without, "front", "eye-left-outer-corner");
    assert_eq!(number(&corner["distance"]), 0.0);
    assert!(corner.get("from_final_drawings").is_none());
    assert!(without.get("final_drawings").is_none());

    let table = differences_with(
        "differences-with-final",
        MADE_UP_FRONT,
        MADE_UP_THREE_QUARTER,
        MADE_UP_RECORD,
        Some(MADE_UP_FINAL_RECORD),
    );
    assert_eq!(table["final_drawings"].as_str(), Some(MADE_UP_FINAL_RECORD));
    assert_eq!(
        table["final_drawings_what"].as_str(),
        Some("Made-up final positions for the tests. No drawing stands behind them.")
    );

    // With it the corner is measured against 646, 303: 4 across and 3 down is 5 units, 1.25%
    // of the head height of 400, so it is now above 1% and has a number.
    let corner = find(&table, "front", "eye-left-outer-corner");
    assert_eq!(number(&corner["hand_made"]["x"]), 646.0);
    assert_eq!(number(&corner["hand_made"]["y"]), 303.0);
    assert_eq!(number(&corner["distance"]), 5.0);
    assert_eq!(number(&corner["percent_of_head_height"]), 1.25);
    assert_eq!(corner["from_final_drawings"].as_bool(), Some(true));
    assert!(corner.get("number").is_some());

    // The lid line's inner end is midway between the final lower corner at 604, 312 and the
    // record's own upper corner at 600, 300: 602, 306. The generated end is at 600, 311.
    let lid = find(&table, "three-quarter", "lid-line-left-inner-end");
    assert_eq!(number(&lid["hand_made"]["x"]), 602.0);
    assert_eq!(number(&lid["hand_made"]["y"]), 306.0);
    assert_close(
        number(&lid["distance"]),
        (2.0_f64).hypot(5.0),
        0.0005,
        "lid",
    );
    assert_eq!(lid["from_final_drawings"].as_bool(), Some(true));

    // A landmark the final record does not give is the record's own, and is not marked.
    let other = find(&table, "front", "eye-right-outer-corner");
    assert_eq!(number(&other["hand_made"]["x"]), 350.0);
    assert!(other.get("from_final_drawings").is_none());
}

#[test]
fn the_lay_over_is_stated_and_is_one_rule_for_every_landmark_of_a_view() {
    let table = made_up("differences-lay-over");
    assert!(
        table["rules"]["lay_over"]
            .as_str()
            .expect("the rule")
            .contains("eye line"),
        "the rule says what is made to coincide"
    );
    // Front: unit 100 on 200, eye line 150 on 300, axis 250 on 500.
    let front = table["front"]["lay_over"].as_table().expect("the lay-over");
    assert_eq!(number(&front["scale"]), 2.0);
    assert_eq!(number(&front["shift_x"]), 0.0);
    assert_eq!(number(&front["shift_y"]), 0.0);
    // Three-quarter: unit 200 on 200, eye line 310 on 300, axis 480 on 495.
    let turned = table["three-quarter"]["lay_over"]
        .as_table()
        .expect("the lay-over");
    assert_eq!(number(&turned["scale"]), 1.0);
    assert_eq!(number(&turned["shift_x"]), 15.0);
    assert_eq!(number(&turned["shift_y"]), -10.0);

    // Every listed position of a view follows from its file's by that one scale and shift.
    let file: toml::Table = text(&support::repository().join(MADE_UP_FRONT))
        .parse()
        .expect("TOML");
    for list in ["above_one_percent", "at_or_under_one_percent"] {
        for entry in entries(&table, "front", list) {
            let name = entry["landmark"].as_str().expect("a name");
            let point = file["landmarks"]
                .get(name)
                .or_else(|| file["view_points"].get(name))
                .unwrap_or_else(|| panic!("{name} is in the landmark file"));
            assert_close(
                number(&entry["generated"]["x"]),
                2.0 * number(&point["x"]),
                1e-9,
                name,
            );
            assert_close(
                number(&entry["generated"]["y"]),
                2.0 * number(&point["y"]),
                1e-9,
                name,
            );
        }
    }
}

#[test]
fn shares_are_of_the_hand_made_front_views_head_height() {
    let table = made_up("differences-yardstick");
    // Hair top at y = 100 and chin at y = 500 in the record's front view.
    assert_eq!(number(&table["yardstick"]["head_height"]), 400.0);
    assert_eq!(number(&table["yardstick"]["one_percent"]), 4.0);
    // 4.012 units in the three-quarter view is above 1% of 400. Of that view's own head
    // height, 402, it would be under.
    assert!(
        names(&table, "three-quarter", "above_one_percent")
            .contains(&"nose-mark-bottom".to_string())
    );
}

#[test]
fn where_the_face_outline_crosses_a_level_is_compared_sideways_only() {
    // The record takes such a crossing at the height of the drawn level line, which need not
    // be the eye line, the nose level or the mouth level. Saffi's drawn eye level line is at
    // y = 366 in her front view, and with her final drawings' lids her eye line is at
    // (353.785 + 390) / 2 = 371.893: 5.9 units lower, which is 1.2% of her head height and
    // says nothing about her face outline.
    let (_, table) = hers("saffi-differences-crossings", &[]);
    for (view, name) in [
        ("front", "face-outline-left-at-eye-level"),
        ("front", "face-outline-right-at-eye-level"),
        ("three-quarter", "face-outline-left-at-eye-level"),
    ] {
        let crossing = ["above_one_percent", "at_or_under_one_percent"]
            .iter()
            .flat_map(|list| entries(&table, view, list))
            .find(|entry| entry["landmark"].as_str() == Some(name))
            .unwrap_or_else(|| panic!("{name} is compared in the {view} view"));
        assert_eq!(crossing["compared"].as_str(), Some("sideways"), "{name}");
        let (generated, hand_made) = (&crossing["generated"], &crossing["hand_made"]);
        assert_eq!(number(&hand_made["y"]), 366.0, "{view}: {name}");
        assert!(
            number(&generated["y"]) - number(&hand_made["y"]) > 5.0,
            "{view}: {name} is no test of the rule"
        );
        assert_close(
            number(&crossing["distance"]),
            (number(&generated["x"]) - number(&hand_made["x"])).abs(),
            0.00051,
            name,
        );
        assert!(crossing.get("number").is_none(), "{view}: {name}");
    }
    assert!(
        table["rules"]["one_direction"]
            .as_str()
            .expect("the rule")
            .contains("crosses"),
        "the rule does not speak of the crossings"
    );
}

#[test]
fn the_top_of_the_hair_is_compared_in_height_and_its_sides_sideways() {
    let table = made_up("differences-one-direction");
    let under = entries(&table, "front", "at_or_under_one_percent");
    let find = |name: &str| {
        *under
            .iter()
            .find(|entry| entry["landmark"].as_str() == Some(name))
            .unwrap_or_else(|| panic!("{name} is at or under 1%"))
    };
    // 20 units to the side and 1 unit lower: only the height counts.
    let top = find("hair-top");
    assert_eq!(top["compared"].as_str(), Some("height"));
    assert_eq!(number(&top["distance"]), 1.0);
    assert_eq!(number(&top["percent_of_head_height"]), 0.25);
    // 4 units to the side and 83 lower: only the sideways position counts.
    let side = find("hair-left-extent");
    assert_eq!(side["compared"].as_str(), Some("sideways"));
    assert_eq!(number(&side["distance"]), 4.0);
    assert!(
        table["rules"]["one_direction"]
            .as_str()
            .expect("the rule")
            .contains("hair"),
        "the rule is stated"
    );
}

#[test]
fn a_hand_made_position_is_compared_under_the_name_the_reading_rules_give_it() {
    let table = made_up("differences-names");
    // The near side's outline in a turned view is the jaw line: no difference there.
    let under = entries(&table, "three-quarter", "at_or_under_one_percent");
    let jaw_line = under
        .iter()
        .find(|entry| entry["landmark"].as_str() == Some("jaw-line-right-at-nose-level"))
        .expect("the jaw line at the nose level is compared");
    assert_eq!(number(&jaw_line["distance"]), 0.0);
    assert_eq!(
        texts(&jaw_line["hand_made_from"]),
        ["face_outline_right_at_nose_level"]
    );
    let compared: Vec<String> = ["above_one_percent", "at_or_under_one_percent"]
        .iter()
        .flat_map(|list| names(&table, "three-quarter", list))
        .collect();
    assert!(!compared.contains(&"face-outline-right-at-nose-level".to_string()));
}

#[test]
fn what_has_no_counterpart_is_named_on_both_sides() {
    let table = made_up("differences-counterparts");
    let front = table["front"]["without_counterpart"]
        .as_table()
        .expect("the lists");
    assert_eq!(texts(&front["generated"]), ["centre-line-at-eye-level"]);
    assert_eq!(texts(&front["hand_made"]), ["pupil_left_centre"]);

    let turned = table["three-quarter"]["without_counterpart"]
        .as_table()
        .expect("the lists");
    assert_eq!(
        texts(&turned["generated"]),
        ["face-outline-right-at-nose-level"]
    );
    assert!(texts(&turned["hand_made"]).is_empty());
}

#[test]
fn the_counts_add_up() {
    let table = made_up("differences-counts");
    let front = table["front"]["counts"].as_table().expect("the counts");
    // Sixteen positions of the front file have a hand-made position; two of the file's
    // landmarks share one of them.
    assert_eq!(front["compared"].as_integer(), Some(16));
    assert_eq!(front["above_one_percent"].as_integer(), Some(4));
    assert_eq!(front["at_or_under_one_percent"].as_integer(), Some(12));
    let turned = table["three-quarter"]["counts"]
        .as_table()
        .expect("the counts");
    assert_eq!(turned["compared"].as_integer(), Some(9));
    assert_eq!(turned["above_one_percent"].as_integer(), Some(3));
    assert_eq!(turned["at_or_under_one_percent"].as_integer(), Some(6));
    assert_eq!(table["compared"].as_integer(), Some(25));
    assert_eq!(table["above_one_percent"].as_integer(), Some(7));
}

#[test]
fn the_same_input_gives_the_same_file_and_a_list_a_person_can_read() {
    let folder = scratch("differences-twice");
    let run = |name: &str| {
        let out = folder.join(name);
        let printed = maquette_ok(&[
            "differences-from-hand-made-views",
            "--front",
            MADE_UP_FRONT,
            "--three-quarter",
            MADE_UP_THREE_QUARTER,
            "--record",
            MADE_UP_RECORD,
            "--out",
            path_text(&out),
        ]);
        (text(&out), printed)
    };
    let (first, printed) = run("first.toml");
    let (second, _) = run("second.toml");
    assert!(first == second, "the two files differ");
    // With a file written, the numbered list is printed for a person.
    for line in [
        "1  shoulder-left-point",
        "4  mouth-line-centre",
        "5  ear-left-base-outer",
        "6  lid-line-left-inner-end",
    ] {
        assert!(printed.contains(line), "`{line}` is not in:\n{printed}");
    }
    assert!(printed.contains("10.000%"), "{printed}");
}

#[test]
fn a_path_with_a_quote_a_backslash_or_a_line_break_is_written_as_valid_toml() {
    // The differences file carries the paths it was given as TOML text, so a path that holds
    // a character TOML must escape still gives a file that reads back to that same path.
    let folder = scratch("differences-odd-paths");
    let odd = |name: &str, from: &str| {
        let path = folder.join(name);
        std::fs::copy(support::repository().join(from), &path).expect("copied");
        path
    };
    let record = odd(
        "record \"made up\"\\\nwith a line break.toml",
        MADE_UP_RECORD,
    );
    let front = odd("front\tview.landmarks.toml", MADE_UP_FRONT);
    let table = differences(
        "differences-odd-paths-out",
        path_text(&front),
        MADE_UP_THREE_QUARTER,
        path_text(&record),
    );
    assert_eq!(table["record"].as_str(), Some(path_text(&record)));
    assert_eq!(
        table["front"]["landmark_file"].as_str(),
        Some(path_text(&front))
    );
}

#[test]
fn files_given_for_the_wrong_view_are_refused() {
    let output = maquette(&[
        "differences-from-hand-made-views",
        "--front",
        MADE_UP_THREE_QUARTER,
        "--three-quarter",
        MADE_UP_FRONT,
        "--record",
        MADE_UP_RECORD,
    ]);
    assert!(!output.status.success());
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(
        message.contains("is the landmark file of a three-quarter view, not of a front view"),
        "{message}"
    );
}

#[test]
fn a_fault_in_a_record_is_reported_under_the_path_of_the_record_it_is_in() {
    let folder = scratch("differences-fault-in-a-record");
    let spoiled = |name: &str, from: &str, good: &str, bad: &str| {
        let original = text(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(from)
                .as_path(),
        );
        assert!(original.contains(good), "{from} has changed");
        let path = folder.join(name);
        std::fs::write(&path, original.replace(good, bad)).expect("written");
        path
    };
    // The final record gives a landmark a word where its x belongs.
    let final_with_a_word = spoiled(
        "final-with-a-word.toml",
        MADE_UP_FINAL_RECORD,
        "eye_left_outer_corner = { x = 646.0,",
        "eye_left_outer_corner = { x = \"four\",",
    );
    // The final record gives a view no digest.
    let final_without_a_digest = spoiled(
        "final-without-a-digest.toml",
        MADE_UP_FINAL_RECORD,
        "[views.front]\nsha256 = \"made-up: there is no final drawing\"\n",
        "[views.front]\n",
    );
    // The record itself gives a landmark a word where its x belongs.
    let record_with_a_word = spoiled(
        "record-with-a-word.toml",
        MADE_UP_RECORD,
        "chin = { x = 500.0,",
        "chin = { x = \"five hundred\",",
    );

    let cases = [
        (
            MADE_UP_RECORD,
            path_text(&final_with_a_word),
            "final-with-a-word.toml",
            "made-up-hand-made-views.toml",
            "eye_left_outer_corner",
        ),
        (
            MADE_UP_RECORD,
            path_text(&final_without_a_digest),
            "final-without-a-digest.toml",
            "made-up-hand-made-views.toml",
            "sha256",
        ),
        (
            path_text(&record_with_a_word),
            MADE_UP_FINAL_RECORD,
            "record-with-a-word.toml",
            "made-up-final-drawings.toml",
            "chin",
        ),
    ];
    for (record, final_record, at_fault, sound, what) in cases {
        let commands: [Vec<&str>; 3] = [
            vec![
                "differences-from-hand-made-views",
                "--front",
                MADE_UP_FRONT,
                "--three-quarter",
                MADE_UP_THREE_QUARTER,
            ],
            vec![
                "overlay-hand-made-views",
                "--front",
                MADE_UP_FRONT,
                "--three-quarter",
                MADE_UP_THREE_QUARTER,
                "--out",
                path_text(&folder),
            ],
            vec!["proportions-from-hand-made-views"],
        ];
        for mut arguments in commands {
            arguments.extend(["--record", record, "--final-record", final_record]);
            let output = maquette(&arguments);
            assert!(!output.status.success(), "{arguments:?} read {at_fault}");
            let message = String::from_utf8_lossy(&output.stderr);
            assert!(
                message.contains(at_fault) && message.contains(what),
                "{}: the message does not name {at_fault} and `{what}`: {message}",
                arguments[0]
            );
            assert!(
                !message.contains(sound),
                "{}: the message names {sound}, which has no fault: {message}",
                arguments[0]
            );
        }
    }
}

// Saffi. Nothing below says her list is short, long or empty, and nothing pins its length.
// Jon accepted her armature at look 3 and `tests/saffi.rs` pins its bytes; the list is not
// part of what he accepted, and it moves with anything a later look moves.

const HER_RECORD: &str = "data/saffi/hand-made-views.toml";
/// Her final drawings' ears and lids, which replace her record's (Jon, 2026-11-16).
const HER_FINAL_RECORD: &str = "data/saffi/final-drawings.toml";
const HER_PROPORTIONS: &str = "data/saffi/proportions.toml";
/// Hair top at y = 86.682 and chin at y = 567 in her hand-made front view.
const HER_HEAD_HEIGHT: f64 = 567.0 - 86.682;

/// Draws her and compares the drawing with her record.
fn hers(name: &str, placement: &[&str]) -> (std::path::PathBuf, toml::Table) {
    let folder = support::draw(&format!("{name}-drawn"), HER_PROPORTIONS, placement);
    let table = differences_with(
        name,
        path_text(&folder.join("front.landmarks.toml")),
        path_text(&folder.join("three-quarter.landmarks.toml")),
        HER_RECORD,
        Some(HER_FINAL_RECORD),
    );
    (folder, table)
}

#[test]
fn her_list_is_produced_and_every_line_of_it_follows_from_its_own_positions() {
    let (_, table) = hers("saffi-differences", &[]);
    assert_close(
        number(&table["yardstick"]["head_height"]),
        HER_HEAD_HEIGHT,
        0.0005,
        "head height",
    );
    let mut next_number = 1;
    for view in ["front", "three-quarter"] {
        let above = entries(&table, view, "above_one_percent");
        let under = entries(&table, view, "at_or_under_one_percent");
        let counts = table[view]["counts"].as_table().expect("the counts");
        assert_eq!(
            counts["compared"].as_integer(),
            Some((above.len() + under.len()) as i64)
        );
        assert!(
            above.len() + under.len() > 40,
            "{view}: most landmarks are compared"
        );

        let mut last = f64::INFINITY;
        for (list, is_above) in [(&above, true), (&under, false)] {
            if !is_above {
                last = f64::INFINITY;
            }
            for entry in list {
                let name = entry["landmark"].as_str().expect("a name");
                let sideways = number(&entry["generated"]["x"]) - number(&entry["hand_made"]["x"]);
                let height = number(&entry["generated"]["y"]) - number(&entry["hand_made"]["y"]);
                let expected = match entry["compared"].as_str().expect("what is compared") {
                    "position" => sideways.hypot(height),
                    "height" => height.abs(),
                    "sideways" => sideways.abs(),
                    other => panic!("{name} is compared in `{other}`"),
                };
                let distance = number(&entry["distance"]);
                assert_close(distance, expected, 0.00051, name);
                assert_close(
                    number(&entry["percent_of_head_height"]),
                    distance / HER_HEAD_HEIGHT * 100.0,
                    0.00051,
                    name,
                );
                assert_eq!(distance > HER_HEAD_HEIGHT / 100.0, is_above, "{name}");
                assert!(distance <= last, "{view}: {name} is out of order");
                last = distance;
                if is_above {
                    assert_eq!(entry["number"].as_integer(), Some(next_number), "{name}");
                    next_number += 1;
                } else {
                    assert!(entry.get("number").is_none(), "{name}");
                }
            }
        }
    }
    assert_eq!(
        table["above_one_percent"].as_integer(),
        Some(next_number - 1)
    );
}

#[test]
fn her_hand_made_positions_in_the_list_are_her_records() {
    let (_, table) = hers("saffi-differences-record", &[]);
    let find = |view: &str, name: &str| -> toml::Table {
        ["above_one_percent", "at_or_under_one_percent"]
            .iter()
            .flat_map(|list| entries(&table, view, list))
            .find(|entry| entry["landmark"].as_str() == Some(name))
            .unwrap_or_else(|| panic!("{name} is compared in the {view} view"))
            .clone()
    };
    // As data/saffi/hand-made-views.toml gives them.
    let chin = find("front", "chin");
    assert_eq!(number(&chin["hand_made"]["x"]), 500.0);
    assert_eq!(number(&chin["hand_made"]["y"]), 567.0);
    let shoulder = find("three-quarter", "shoulder-left-point");
    assert_eq!(number(&shoulder["hand_made"]["x"]), 838.0);
    assert_eq!(number(&shoulder["hand_made"]["y"]), 744.0);
    assert_eq!(texts(&shoulder["hand_made_from"]), ["shoulder_left_point"]);
    // Her near-side outline at the nose level is at 394, 450, and it is the jaw line.
    let jaw_line = find("three-quarter", "jaw-line-right-at-nose-level");
    assert_eq!(number(&jaw_line["hand_made"]["x"]), 394.0);
    assert_eq!(
        texts(&jaw_line["hand_made_from"]),
        ["face_outline_right_at_nose_level"]
    );
    // Her far brow's inner end is midway between the band's corners at 600, 330 and 604, 341.
    let brow = find("three-quarter", "brow-line-left-inner-end");
    assert_eq!(number(&brow["hand_made"]["x"]), 602.0);
    assert_eq!(number(&brow["hand_made"]["y"]), 335.5);
    assert!(brow.get("from_final_drawings").is_none());
    // As data/saffi/final-drawings.toml gives them: her ears and her lids are her final
    // drawings'. The head covers her far ear's outer base, at 618.903, 167.565 in the final
    // rig's coordinates; her S2 underdrawing had it at 622, 160.
    let ear = find("three-quarter", "ear-left-base-outer");
    assert_eq!(ear["covered_by_the_head"].as_bool(), Some(true));
    assert_eq!(ear["from_final_drawings"].as_bool(), Some(true));
    assert_eq!(number(&ear["hand_made"]["x"]), 618.903);
    assert_eq!(number(&ear["hand_made"]["y"]), 167.565);
    // Her near eye's outer corner is at 384, 368.5 in the front of her S6 model sheet.
    let corner = find("front", "eye-right-outer-corner");
    assert_eq!(corner["from_final_drawings"].as_bool(), Some(true));
    assert_eq!(number(&corner["hand_made"]["x"]), 384.0);
    assert_eq!(number(&corner["hand_made"]["y"]), 368.5);
    // Her lid line's crest in the front view is midway between the highest points of the
    // band's two edges, at 402.346, 358.632 and 405.711, 347.692.
    let crest = find("front", "lid-line-right-crest");
    assert_eq!(number(&crest["hand_made"]["x"]), 404.029);
    assert_eq!(number(&crest["hand_made"]["y"]), 353.162);
    assert_eq!(
        table["final_drawings"].as_str(),
        Some("data/saffi/final-drawings.toml")
    );
}

#[test]
fn drawn_at_her_hand_made_size_and_place_nothing_has_to_be_moved_to_lay_her_over() {
    // The unit, the eye line and the axis of each hand-made view, as her set of proportions
    // records them under [source].
    let source: toml::Table = text(&support::repository().join(HER_PROPORTIONS))
        .parse()
        .expect("TOML");
    for (view, key, axis_option) in [
        ("front", "front", "--front-axis-x"),
        ("three-quarter", "three_quarter", "--three-quarter-axis-x"),
    ] {
        let frame = source["source"][key].as_table().expect("the frame");
        let (unit, eye_line, axis) = (
            number(&frame["unit"]).to_string(),
            number(&frame["eye_line_y"]).to_string(),
            number(&frame["turn_axis_x"]).to_string(),
        );
        let (folder, table) = hers(
            &format!("saffi-differences-at-{view}"),
            &[
                "--unit",
                &unit,
                "--eye-line-y",
                &eye_line,
                axis_option,
                &axis,
            ],
        );
        let lay_over = table[view]["lay_over"].as_table().expect("the lay-over");
        // Her file gives each unit to a thousandth, so it can be half a thousandth off a unit
        // of about 195: 2.6 parts in a million, and the scale is written to one part more.
        assert_close(number(&lay_over["scale"]), 1.0, 4e-6, "scale");
        assert_close(number(&lay_over["shift_x"]), 0.0, 0.001, "shift_x");
        assert_close(number(&lay_over["shift_y"]), 0.0, 0.001, "shift_y");

        // So every generated position in the list is the landmark file's own.
        let file = support::LandmarkFile::read(&folder.join(format!("{view}.landmarks.toml")));
        for list in ["above_one_percent", "at_or_under_one_percent"] {
            for entry in entries(&table, view, list) {
                let name = entry["landmark"].as_str().expect("a name");
                let point = file.point(name);
                assert_close(number(&entry["generated"]["x"]), point.x, 0.0011, name);
                assert_close(number(&entry["generated"]["y"]), point.y, 0.0011, name);
            }
        }
    }
}

#[test]
fn where_she_is_drawn_on_the_page_does_not_change_her_differences() {
    let (_, default) = hers("saffi-differences-default", &[]);
    let (_, moved) = hers(
        "saffi-differences-moved",
        &[
            "--unit",
            "150",
            "--eye-line-y",
            "333",
            "--front-axis-x",
            "420",
            "--three-quarter-axis-x",
            "610",
        ],
    );
    for view in ["front", "three-quarter"] {
        for list in ["above_one_percent", "at_or_under_one_percent"] {
            let (first, second) = (entries(&default, view, list), entries(&moved, view, list));
            let distances = |entries: &[&toml::Table]| -> std::collections::BTreeMap<String, f64> {
                entries
                    .iter()
                    .map(|entry| {
                        (
                            entry["landmark"].as_str().expect("a name").to_string(),
                            number(&entry["distance"]),
                        )
                    })
                    .collect()
            };
            let (first, second) = (distances(&first), distances(&second));
            assert_eq!(
                first.keys().collect::<Vec<_>>(),
                second.keys().collect::<Vec<_>>(),
                "{view} {list}"
            );
            // A landmark file carries three decimals, so a view drawn smaller is coarser.
            for (name, distance) in &first {
                assert_close(second[name], *distance, 0.005, name);
            }
        }
    }
}

#[test]
fn her_shoulder_points_differ_as_her_record_says_they_must() {
    // Known before any comparison: in her hand-made three-quarter view the shoulder points
    // are 742 apart (96 and 838), and a head drawn from her proportions turns them with the
    // head. Her front view has them 744 apart at its unit; at the three-quarter view's unit
    // and a turn of 29 degrees that shows as 744 * that unit / the front unit * cos 29. Each
    // side then differs by half of what is left.
    //
    // The units are eye line to chin, with the eye line midway between the top and the bottom
    // of her final drawings' eye openings and her chin at y = 567. Front view: both openings
    // run from 353.785 to 390. Three-quarter view: from 353.785 to 390 on the near eye and
    // from 355.176 to 390.163 on the far one.
    let front_unit = 567.0 - (353.785 + 390.0) / 2.0;
    let turned_unit = 567.0 - (353.785 + 390.0 + 355.176 + 390.163) / 4.0;
    let shown = 744.0 * turned_unit / front_unit * 29.0_f64.to_radians().cos();
    let expected = (742.0 - shown) / 2.0;
    let (_, table) = hers("saffi-differences-shoulders", &[]);
    let above = entries(&table, "three-quarter", "above_one_percent");
    for name in ["shoulder-left-point", "shoulder-right-point"] {
        let shoulder = above
            .iter()
            .find(|entry| entry["landmark"].as_str() == Some(name))
            .unwrap_or_else(|| panic!("{name} is above 1%"));
        assert_close(number(&shoulder["distance"]), expected, 0.01, name);
    }
}
