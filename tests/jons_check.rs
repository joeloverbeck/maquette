//! The record of Jon's check of a reference check sheet, through the command line: starting
//! it, saying what it does, the count of accepted works for each view, and the measuring
//! commands taking only what he accepted.
//!
//! Everything here is made up: the pictures are plain grey, the readings are of a made-up
//! head, and the verdicts are the test's, not Jon's.

mod support;

use std::path::{Path, PathBuf};

use support::made_up_head::{MadeUp, seen};
use support::{failure, maquette_in, measure_ok, references, text};

/// The made-up head's side view at half the front view's size: the near side only.
fn side_points() -> std::collections::BTreeMap<String, [f64; 2]> {
    seen(90.0, 700.0)
        .into_iter()
        .filter(|(name, _)| !name.contains("-left-"))
        .map(|(name, at)| (name, [350.0 + 0.5 * at[0], 100.0 + 0.5 * at[1]]))
        .collect()
}

fn write(folder: &Path, mut made_up: MadeUp, work: usize, picture: &str, name: &str) {
    made_up.work = format!("made-up work {work}");
    made_up.picture = format!("pictures/{picture}");
    made_up.write(folder, name);
}

/// A made-up reference folder with six characters of six made-up works, and the sheet
/// description that names them.
///
/// - work-1: a front and a side view, each read by two readers.
/// - work-2: a front view read by two readers, and a side view both set aside.
/// - work-3: a front view both readers placed and one of them set aside.
/// - work-4 to work-6: a front view read by two readers.
fn folder(name: &str) -> PathBuf {
    let folder = references(name);
    let mut description = String::from(
        "note = \"Made up for a test: the grey pictures show no drawing.\"\nlook = \"No look is asked: a made-up sheet\"\n",
    );
    for work in 1..=6 {
        description.push_str(&format!("\n[[character]]\nlabel = \"work-{work}\"\n"));
        let mut drawing = |view: &str, readings: [String; 2]| {
            description.push_str(&format!(
                "\n  [[character.drawing]]\n  picture = \"the {view} head of made-up work {work}\"\n  readings = [\"{}\", \"{}\"]\n",
                readings[0], readings[1]
            ));
        };
        let named = |view: &str| -> [String; 2] {
            ["first", "second"].map(|reader| format!("{reader}/work-{work}-{view}.toml"))
        };
        let front = named("front");
        for (reader, path) in ["first", "second"].into_iter().zip(&front) {
            let mut made_up = MadeUp::front(reader, seen(0.0, 500.0));
            if work == 3 && reader == "second" {
                // The second reader placed the landmarks and still set the drawing aside.
                made_up.set_aside = true;
            }
            write(
                &folder,
                made_up,
                work,
                &format!("work-{work}-front.png"),
                path,
            );
        }
        if work == 3 {
            // A drawing that is set aside has nothing placed in a made-up reading: put the
            // first reader's positions into the second reader's file, as a reader who went
            // on to place them would have.
            let first = text(&folder.join(&front[0]));
            let placed = first
                .split("[landmarks]")
                .nth(1)
                .expect("the first reading places landmarks");
            let second = folder.join(&front[1]);
            let aside = text(&second);
            std::fs::write(&second, format!("{aside}{placed}")).expect("written");
        }
        drawing("front", front);
        if work <= 2 {
            let side = named("side");
            for (reader, path) in ["first", "second"].into_iter().zip(&side) {
                let mut made_up = MadeUp::turned(reader, side_points());
                made_up.view = "side";
                made_up.stated_turn = 90.0;
                if work == 2 {
                    made_up.level = false;
                    made_up.set_aside = true;
                }
                write(
                    &folder,
                    made_up,
                    work,
                    &format!("work-{work}-side.png"),
                    path,
                );
            }
            drawing("side", side);
        }
    }
    std::fs::write(folder.join("sheet.toml"), description).expect("written");
    for path in ["first/work-3-front.toml", "second/work-3-front.toml"] {
        measure_ok(&folder, &["check-reading", "--reading", path]);
    }
    folder
}

fn started(folder: &Path) -> String {
    measure_ok(
        folder,
        &[
            "start-jons-check",
            "--sheet",
            "sheet.toml",
            "--out",
            "checks/check.toml",
        ],
    );
    text(&folder.join("checks/check.toml"))
}

/// Writes into a started record what the test has Jon say of one character: its verdict,
/// and a change to the lines of one of its drawings.
fn say(record: &str, label: &str, verdict: &str, drawing: &[(&str, &str, &str)]) -> String {
    let (before, rest) = record
        .split_once(&format!("label = \"{label}\"\n"))
        .unwrap_or_else(|| panic!("the record has no {label}"));
    let (mut of_character, after) = match rest.split_once("\n[[character]]") {
        Some((of_character, after)) => {
            (of_character.to_string(), format!("\n[[character]]{after}"))
        }
        None => (rest.to_string(), String::new()),
    };
    of_character = of_character.replacen("verdict = \"\"", &format!("verdict = \"{verdict}\""), 1);
    for (card, from, to) in drawing {
        let (head, of_drawing) = of_character
            .split_once(&format!("card = \"{card}\"\n"))
            .unwrap_or_else(|| panic!("{label} has no {card} card"));
        assert!(of_drawing.contains(from), "{label}, {card}: no `{from}`");
        of_character = format!(
            "{head}card = \"{card}\"\n{}",
            of_drawing.replacen(from, to, 1)
        );
    }
    format!("{before}label = \"{label}\"\n{of_character}{after}")
}

fn filled(folder: &Path, record: &str) {
    let record = record
        .replace("look = \"\"", "look = \"Look 4 of 8, made up\"")
        .replace("checked_on = \"\"", "checked_on = \"a made-up day\"")
        .replace("recorded_by = \"\"", "recorded_by = \"a test\"");
    std::fs::write(folder.join("checks/check.toml"), record).expect("written");
}

fn what_is_said(folder: &Path) -> (String, bool) {
    let output = maquette_in(
        Some(folder),
        &["jons-check", "--check", "checks/check.toml"],
    );
    (
        String::from_utf8_lossy(&output.stdout).into_owned()
            + &String::from_utf8_lossy(&output.stderr),
        output.status.success(),
    )
}

#[test]
fn a_record_is_started_from_a_sheet_and_says_what_it_lacks_until_jon_has_spoken() {
    let folder = folder("jons-check-started");
    let record = started(&folder);
    // Every character and every drawing of the sheet, with each reading and its digest.
    for work in 1..=6 {
        assert!(
            record.contains(&format!("label = \"work-{work}\"")),
            "{record}"
        );
    }
    assert_eq!(record.matches("[[character.drawing]]").count(), 8);
    let digest = support::sha256_of(&folder.join("first/work-1-front.toml"));
    assert!(
        record.contains(&format!(
            "{{ path = \"first/work-1-front.toml\", sha256 = \"{digest}\" }}"
        )),
        "{record}"
    );
    // What each reader judged stands beside its reading, for whoever writes the record.
    assert!(
        record.contains(
            "# second judged a side view facing the viewer's right, not level, and set it aside."
        ),
        "{record}"
    );
    // A record that is there is not written over.
    let again = maquette_in(
        Some(&folder),
        &[
            "start-jons-check",
            "--sheet",
            "sheet.toml",
            "--out",
            "checks/check.toml",
        ],
    );
    assert!(failure(&again).contains("not written over"));

    // As started it gives no verdict, so nothing is accepted and the record is not complete.
    let (said, complete) = what_is_said(&folder);
    assert!(!complete, "{said}");
    assert!(said.contains("work-1: not yet checked"), "{said}");
    assert!(said.contains("`work-6` has no verdict"), "{said}");
    assert!(said.contains("`look` is empty"), "{said}");
    assert!(said.contains("front: 0 works: too thin"), "{said}");
}

#[test]
fn a_record_says_what_it_does_and_counts_the_accepted_works_for_each_view() {
    let folder = folder("jons-check-said");
    let mut record = started(&folder);
    // Made up: work-1 accepted, with one point of its front view marked as off and the
    // eye opening's height marked as off for the work.
    record = say(
        &record,
        "work-1",
        "accepted",
        &[(
            "front",
            "off = []",
            "off = [\"nose-mark-bottom\", \"brow-line-*\"]",
        )],
    );
    record = record.replacen(
        "proportions_off = []",
        "proportions_off = [\"eye_opening_height\"]",
        1,
    );
    // work-2 accepted: its side view stays set aside, as both readers judged.
    record = say(&record, "work-2", "accepted", &[]);
    // work-3 accepted, and the front view one reader set aside is measured after all.
    record = say(
        &record,
        "work-3",
        "accepted",
        &[("front", "ruling = \"\"", "ruling = \"measure\"")],
    );
    record = say(&record, "work-4", "accepted", &[]);
    record = say(&record, "work-5", "accepted", &[]);
    record = say(&record, "work-6", "rejected", &[]);
    record = record.replace(
        "label = \"work-6\"\nverdict = \"rejected\"\nwhy = \"\"",
        "label = \"work-6\"\nverdict = \"rejected\"\nwhy = \"made up: no reason\"",
    );
    filled(&folder, &record);

    let (said, complete) = what_is_said(&folder);
    assert!(complete, "{said}");
    assert!(said.contains("Look 4 of 8, made up"), "{said}");
    assert!(
        said.contains("front, the front head of made-up work 1: measured from its 2 readings, as its readers judged it. Left out by Jon's marks: nose-mark-bottom, brow-line-*."),
        "{said}"
    );
    assert!(
        said.contains("Marked as off by Jon and not given for this work: eye_opening_height."),
        "{said}"
    );
    assert!(
        said.contains("side, the side head of made-up work 2: not measured: set aside by first"),
        "{said}"
    );
    assert!(
        said.contains("front, the front head of made-up work 3: measured as a front view by Jon's ruling, from the reading by first and by second."),
        "{said}"
    );
    assert!(
        said.contains("work-6: rejected: \"made up: no reason\". Nothing of it is measured."),
        "{said}"
    );
    // Five accepted works have a measured front view, which is what a table needs. One has
    // a measured side view, and so one gives depth: both too thin.
    assert!(said.contains("Accepted works for each view, of 6 characters on the sheet:"));
    assert!(said.contains("  front: 5 works\n"), "{said}");
    assert!(
        said.contains("  three-quarter: 0 works: too thin, a view needs 5\n"),
        "{said}"
    );
    assert!(
        said.contains("  side: 1 work: too thin, a view needs 5\n"),
        "{said}"
    );
    assert!(
        said.contains("  depth, from a front and a side view of one character: 1 work: too thin"),
        "{said}"
    );

    // A mark that names no landmark of the view, a proportion the list does not have and a
    // verdict that is neither word are said, and the record is not complete.
    let spoiled = record
        .replace("nose-mark-bottom", "nose-bottom")
        .replace("eye_opening_height", "eye_height")
        .replacen("verdict = \"rejected\"", "verdict = \"maybe\"", 1);
    filled(&folder, &spoiled);
    let (said, complete) = what_is_said(&folder);
    assert!(!complete, "{said}");
    assert!(
        said.contains("`nose-bottom` is marked as off on the front view of `work-1`"),
        "{said}"
    );
    assert!(said.contains("`eye_height` is marked as off"), "{said}");
    assert!(said.contains("\"maybe\""), "{said}");
}

fn taken(folder: &Path, views: &[&str], out: &str) -> std::process::Output {
    let mut arguments = vec!["proportions-from-readings"];
    arguments.extend_from_slice(views);
    arguments.extend(["--out", out]);
    maquette_in(Some(folder), &arguments)
}

#[test]
fn measuring_takes_only_what_jon_accepted_and_leaves_out_what_he_marked() {
    let folder = folder("jons-check-measuring");
    let mut record = started(&folder);
    record = say(
        &record,
        "work-1",
        "accepted",
        &[("front", "off = []", "off = [\"nose-mark-*\"]")],
    );
    record = record.replacen(
        "proportions_off = []",
        "proportions_off = [\"eye_opening_height\"]",
        1,
    );
    record = say(
        &record,
        "work-2",
        "accepted",
        &[("front", "ruling = \"\"", "ruling = \"set aside\"")],
    );
    record = say(
        &record,
        "work-3",
        "accepted",
        &[("front", "ruling = \"\"", "ruling = \"measure\"")],
    );
    record = say(&record, "work-6", "rejected", &[]);
    filled(&folder, &record);
    let front = |work: usize| format!("first/work-{work}-front.toml,second/work-{work}-front.toml");
    let check = ["--check", "checks/check.toml"];
    let with_check = |views: &[&str], out: &str| {
        let mut arguments = views.to_vec();
        arguments.extend(check);
        taken(&folder, &arguments, out)
    };

    // Accepted, with marks: the nose mark is left out of the front view, so its width is
    // not given and its height comes from the side view; the opening's height is not
    // given, and the eye line it gives still is.
    let output = with_check(
        &[
            "--front",
            &front(1),
            "--side",
            "first/work-1-side.toml,second/work-1-side.toml",
        ],
        "work-1.toml",
    );
    assert!(output.status.success(), "{}", failure(&output));
    let found: toml::Table = text(&folder.join("work-1.toml")).parse().expect("TOML");
    let given = found["proportions"].as_table().expect("[proportions]");
    let not_given = found["not_given"].as_table().expect("[not_given]");
    assert!(not_given.contains_key("nose_mark_corner_offset"));
    assert!(given.contains_key("nose_mark_bottom_drop"));
    assert!(given.contains_key("nose_mark_bottom_depth"));
    assert!(
        not_given["eye_opening_height"]
            .as_str()
            .expect("a reason")
            .contains("marked as off by Jon"),
        "{not_given:?}"
    );
    assert!(given.contains_key("hair_top_rise"));
    let checked = found["source"]["jons_check"]
        .as_str()
        .expect("what Jon's check says");
    assert!(
        checked.contains("the character `work-1` is accepted")
            && checked.contains("Left out by Jon's marks: nose-mark-*"),
        "{checked}"
    );

    // With no record, the same readings are measured and the file says they are not checked.
    let output = taken(&folder, &["--front", &front(1)], "unchecked.toml");
    assert!(output.status.success(), "{}", failure(&output));
    let found: toml::Table = text(&folder.join("unchecked.toml")).parse().expect("TOML");
    assert!(
        found["source"]["jons_check"]
            .as_str()
            .expect("what is said")
            .contains("No record of Jon's check was given"),
        "{found:?}"
    );
    assert!(
        found["proportions"]
            .as_table()
            .expect("given")
            .contains_key("eye_opening_height")
    );

    // A drawing a reader set aside is measured where Jon rules so, and not otherwise.
    let output = taken(&folder, &["--front", &front(3)], "work-3-unchecked.toml");
    assert!(failure(&output).contains("sets the drawing aside"));
    let output = with_check(&["--front", &front(3)], "work-3.toml");
    assert!(output.status.success(), "{}", failure(&output));

    // A drawing Jon sets aside, a character he rejected, one he has not checked, and a
    // reading that is not on the sheet he checked are not measured.
    for (work, why) in [
        (2, "set aside by Jon"),
        (6, "Jon rejected the character `work-6`"),
        (4, "has no verdict"),
    ] {
        let output = with_check(&["--front", &front(work)], "refused.toml");
        let message = failure(&output);
        assert!(message.contains(why), "work-{work}: {message}");
    }
    MadeUp::front("third", seen(0.0, 500.0)).write(&folder, "third/elsewhere.toml");
    let output = with_check(&["--front", "third/elsewhere.toml"], "refused.toml");
    assert!(failure(&output).contains("is not a reading of the record"));
    // Two characters' views are not measured together.
    let output = with_check(
        &[
            "--front",
            &front(3),
            "--side",
            "first/work-1-side.toml,second/work-1-side.toml",
        ],
        "refused.toml",
    );
    assert!(failure(&output).contains("two characters"));
    assert!(!folder.join("refused.toml").exists());

    // A reading that has changed since the record was started is not the one he checked.
    let reading = folder.join("first/work-1-front.toml");
    std::fs::write(&reading, text(&reading) + "\n# changed after the check\n").expect("written");
    let output = with_check(&["--front", &front(1)], "changed.toml");
    assert!(failure(&output).contains("has changed since"));
    let (said, complete) = what_is_said(&folder);
    assert!(!complete && said.contains("has changed since"), "{said}");
}
