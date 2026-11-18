//! The proportion table, through the command line: building one from the readings Jon
//! accepted, and setting a set of proportions against one.
//!
//! Everything here is made up: the pictures are plain grey, the readings are of a made-up
//! head changed by hand for each made-up work, the verdicts are the test's and not Jon's,
//! and no table built here is a proportion table of anything.

mod support;

use std::path::{Path, PathBuf};

use support::made_up_head::{EYE_LINE_Y, MadeUp, UNIT, seen};
use support::{
    assert_close, failure, maquette_in, maquette_ok, measure_ok, number, path_text, references,
    scratch, text,
};

/// One made-up work: what its made-up character's drawings show.
#[derive(Clone)]
struct Work {
    /// How high the hair's top stands above the eye line in its front view, in units.
    hair_top_rise: f64,
    /// How high its ears' tops stand, or none where the made-up hair covers the ears.
    ear_top_rise: Option<f64>,
    /// Whether its front view shows the brows.
    brows: bool,
    /// The chin's depth as its side view shows it, where it has a side view.
    chin_depth: Option<f64>,
    /// Whether the first reader calls its front view not level and sets it aside.
    front_set_aside: bool,
    /// Whether its readers place the neck: in its front view both points where the neck ends,
    /// 0.45 to each side and 1.6 below the eye line; in its side view, where it has one, the
    /// neck's front base at depth 0.05 and 1.75 down and its back base at depth -0.85 and 1.45
    /// down. Neither view places the neck's top ends.
    neck: bool,
    /// Whether both readers set its side view aside.
    side_set_aside: bool,
    /// The work its readings name, where it is not a made-up work of its own: two made-up
    /// characters can name one work.
    work: Option<String>,
    /// The age band its readings give: "adult" or "teenage".
    age_band: &'static str,
    /// Whether this batch has a front view of it: a character read again in a later batch
    /// may have only the view read again.
    front: bool,
    /// More points its side view's readers place, in the side view's pixels, where it has
    /// one: the eye line shows at y = 300, depth zero at x = 700, and a unit is 100 pixels.
    side_points: Vec<(&'static str, [f64; 2])>,
    /// The revision of the reading instructions its side view was read under, where it is an
    /// earlier one, which asked for none of the points of the profile asked for since.
    side_read_under: Option<&'static str>,
    /// The landmarks its side view's readers state as not seen.
    side_unseen: Vec<&'static str>,
}

impl Work {
    fn with_hair_top(rise: f64) -> Work {
        Work {
            hair_top_rise: rise,
            ear_top_rise: None,
            brows: true,
            chin_depth: None,
            neck: false,
            front_set_aside: false,
            side_set_aside: false,
            work: None,
            age_band: "adult",
            front: true,
            side_points: Vec::new(),
            side_read_under: None,
            side_unseen: Vec::new(),
        }
    }
}

/// The points of the profile a side view is asked for since revision 2026-11-17.4 of the
/// reading instructions, and was not before.
const PROFILE: [&str; 5] = [
    "centre-line-at-eye-level",
    "centre-line-at-brow-level",
    "jaw-right-angle",
    "neck-front-top",
    "neck-back-top",
];

/// Made-up characters labelled `work-<n>`, numbered from `first`.
fn labelled(works: &[Work], first: usize) -> Vec<(String, Work)> {
    works
        .iter()
        .enumerate()
        .map(|(at, work)| (format!("work-{}", first + at), work.clone()))
        .collect()
}

/// The six made-up works most tests build a table from. Their numbers are chosen so that
/// every figure of the table can be worked out by hand.
///
/// - Hair top: 1.2, 1.5, 1.3, 1.9, 1.4, 1.6.
/// - Ears' tops: 1.3, 1.1, 1.25, 1.35, 1.2, and covered in work 6.
/// - Brows: shown in works 1 to 4, covered in works 5 and 6.
/// - A side view in works 1 and 2, with the chin at depth 0.2 and 0.3.
fn six_works() -> Vec<Work> {
    let rises = [1.2, 1.5, 1.3, 1.9, 1.4, 1.6];
    let ears = [
        Some(1.3),
        Some(1.1),
        Some(1.25),
        Some(1.35),
        Some(1.2),
        None,
    ];
    let depths = [Some(0.2), Some(0.3), None, None, None, None];
    (0..6)
        .map(|at| Work {
            ear_top_rise: ears[at],
            brows: at < 4,
            chin_depth: depths[at],
            ..Work::with_hair_top(rises[at])
        })
        .collect()
}

/// A made-up reference folder with one character for each made-up work, each drawing read by
/// two readers who place the same points, the sheet description that names them, and the
/// record of the check started from it, with nothing said in it yet.
fn folder(name: &str, works: &[Work]) -> PathBuf {
    folder_with(name, works, &|_| String::new())
}

/// The made-up reference folder of [`folder`], with what `more` adds to the sheet: it writes
/// its own files into the folder and returns the lines of its characters.
fn folder_with(name: &str, works: &[Work], more: &dyn Fn(&Path) -> String) -> PathBuf {
    let folder = references(name);
    write_batch(&folder, &labelled(works, 1), more);
    folder
}

/// Writes a made-up batch into `folder`, which may be the reference folder or a folder in it:
/// a character for each labelled made-up work, each drawing read by two readers who place the
/// same points, the sheet description that names them, and the record of the check started
/// from it at the top of the batch's folder, with nothing said in it yet. The batch's own
/// folder is the reference folder each of its commands is run in.
fn write_batch(folder: &Path, characters: &[(String, Work)], more: &dyn Fn(&Path) -> String) {
    std::fs::create_dir_all(folder).expect("the batch's folder can be made");
    let mut description = String::from(
        "note = \"Made up for a test: the grey pictures show no drawing.\"\nlook = \"No look is asked: a made-up sheet\"\n",
    );
    for (label, work) in characters {
        let number = label.trim_start_matches("work-");
        let work_named = work
            .work
            .clone()
            .unwrap_or_else(|| format!("made-up work {number}"));
        description.push_str(&format!("\n[[character]]\nlabel = \"{label}\"\n"));
        let named = |view: &str| -> [String; 2] {
            ["first", "second"].map(|reader| format!("{reader}/{label}-{view}.toml"))
        };
        let mut drawing = |view: &str, readings: &[String; 2]| {
            description.push_str(&format!(
                "\n  [[character.drawing]]\n  picture = \"the {view} head of made-up work {number}\"\n  readings = [\"{}\", \"{}\"]\n",
                readings[0], readings[1]
            ));
        };

        let mut points = seen(0.0, 500.0);
        points.insert(
            "hair-top".to_string(),
            [500.0, EYE_LINE_Y - work.hair_top_rise * UNIT],
        );
        for side in ["left", "right"] {
            match work.ear_top_rise {
                Some(rise) => {
                    points
                        .get_mut(&format!("ear-{side}-top"))
                        .expect("the made-up head has ears")[1] = EYE_LINE_Y - rise * UNIT;
                }
                None => {
                    points.remove(&format!("ear-{side}-top"));
                }
            }
            if !work.brows {
                points.remove(&format!("brow-line-{side}-inner-end"));
                points.remove(&format!("brow-line-{side}-outer-end"));
            }
        }
        if work.neck {
            // The made-up head's front view stands at x = 500, 200 pixels to the unit.
            points.insert("neck-left-base".to_string(), [590.0, 720.0]);
            points.insert("neck-right-base".to_string(), [410.0, 720.0]);
        }
        let front = named("front");
        for (reader, path) in ["first", "second"].into_iter().zip(&front) {
            if !work.front {
                break;
            }
            let mut made_up = MadeUp::front(reader, points.clone());
            if work.front_set_aside && reader == "first" {
                made_up.level = false;
                made_up.set_aside = true;
            }
            made_up.work = work_named.clone();
            made_up.age_band = work.age_band.to_string();
            made_up.picture = format!("pictures/{label}-front.png");
            made_up.write(folder, path);
        }
        if work.front {
            drawing("front", &front);
        }

        if let Some(chin_depth) = work.chin_depth {
            // The made-up head's side view at half the front view's size, the near side only.
            // Depth zero shows at x = 700, and a unit is 100 pixels there.
            let mut points: std::collections::BTreeMap<String, [f64; 2]> = seen(90.0, 700.0)
                .into_iter()
                .filter(|(name, _)| !name.contains("-left-"))
                .map(|(name, at)| (name, [350.0 + 0.5 * at[0], 100.0 + 0.5 * at[1]]))
                .collect();
            points.get_mut("chin").expect("the made-up head has a chin")[0] =
                700.0 + chin_depth * 100.0;
            if work.neck {
                // The eye line shows at y = 300 here.
                points.insert("neck-front-base".to_string(), [705.0, 475.0]);
                points.insert("neck-back-base".to_string(), [615.0, 445.0]);
            }
            for (name, at) in &work.side_points {
                points.insert(name.to_string(), *at);
            }
            for name in &work.side_unseen {
                points.remove(*name);
            }
            let side = named("side");
            for (reader, path) in ["first", "second"].into_iter().zip(&side) {
                let mut made_up = MadeUp::turned(reader, points.clone());
                made_up.view = "side";
                made_up.stated_turn = 90.0;
                if work.side_set_aside {
                    made_up.level = false;
                    made_up.set_aside = true;
                }
                made_up.work = work_named.clone();
                made_up.age_band = work.age_band.to_string();
                made_up.picture = format!("pictures/{label}-side.png");
                if let Some(revision) = work.side_read_under {
                    made_up.instructions = Some(revision.to_string());
                    made_up.not_asked = PROFILE.iter().map(|name| name.to_string()).collect();
                }
                made_up.write(folder, path);
            }
            drawing("side", &side);
        }
    }
    description.push_str(&more(folder));
    std::fs::write(folder.join("sheet.toml"), description).expect("written");
    measure_ok(
        folder,
        &[
            "start-jons-check",
            "--sheet",
            "sheet.toml",
            "--out",
            "check.toml",
        ],
    );
}

/// Writes into the started record what the test has Jon say: every character accepted but
/// the ones `rejected` names, and the record's own three lines filled in.
fn checked(folder: &Path, rejected: &[&str]) {
    let path = folder.join("check.toml");
    let mut record = text(&path)
        .replace("verdict = \"\"", "verdict = \"accepted\"")
        .replace("look = \"\"", "look = \"Look 4 of 8, made up\"")
        .replace("checked_on = \"\"", "checked_on = \"a made-up day\"")
        .replace("recorded_by = \"\"", "recorded_by = \"a test\"");
    for label in rejected {
        let of = format!("label = \"{label}\"\nverdict = \"accepted\"");
        assert!(record.contains(&of), "the record has no {label}");
        record = record.replace(&of, &format!("label = \"{label}\"\nverdict = \"rejected\""));
    }
    std::fs::write(path, record).expect("written");
}

fn build(folder: &Path) -> std::process::Output {
    build_from(folder, &["check.toml"])
}

/// Runs `table-from-readings` in `folder` on the records named, each named from it, with a
/// comma between them.
fn build_from(folder: &Path, records: &[&str]) -> std::process::Output {
    let records = records.join(",");
    maquette_in(
        Some(folder),
        &[
            "table-from-readings",
            "--check",
            &records,
            "--out",
            "results/table.toml",
        ],
    )
}

/// The table the command writes into the made-up reference folder, and what it printed.
fn built(folder: &Path) -> (toml::Table, String) {
    built_from(folder, &["check.toml"])
}

/// The table built from the records named, and what it printed.
fn built_from(folder: &Path, records: &[&str]) -> (toml::Table, String) {
    let output = build_from(folder, records);
    assert!(
        output.status.success(),
        "the table was not built:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let table = text(&folder.join("results/table.toml"))
        .parse()
        .expect("the table is TOML");
    (
        table,
        String::from_utf8(output.stdout).expect("the output is text"),
    )
}

/// One proportion's entry in a table.
fn entry<'a>(table: &'a toml::Table, name: &str) -> &'a toml::Table {
    table["proportion"]
        .as_array()
        .expect("the table lists its proportions")
        .iter()
        .filter_map(toml::Value::as_table)
        .find(|entry| entry["name"].as_str() == Some(name))
        .unwrap_or_else(|| panic!("the table has no entry for {name}"))
}

fn assert_values(table: &toml::Table, name: &str, expected: [f64; 3], works: i64) {
    let entry = entry(table, name);
    assert_eq!(
        entry["characters"].as_integer(),
        Some(works),
        "{name}: characters"
    );
    for (key, expected) in ["lowest", "middle", "highest"].into_iter().zip(expected) {
        assert_close(
            number(&entry[key]),
            expected,
            1e-6,
            &format!("{name}: {key}"),
        );
    }
}

/// A proportion the table lists as too thin: with its count of works, and with no number.
fn assert_thin(table: &toml::Table, name: &str, works: i64) {
    let entry = entry(table, name);
    assert_eq!(
        entry["characters"].as_integer(),
        Some(works),
        "{name}: characters"
    );
    assert_eq!(entry["standing"].as_str(), Some("too thin"), "{name}");
    for key in ["lowest", "middle", "highest"] {
        assert!(!entry.contains_key(key), "{name} states a {key} value");
    }
}

#[test]
fn a_table_gives_each_proportion_its_lowest_middle_and_highest_value_and_its_count_of_works() {
    let folder = folder("table-values", &six_works());
    checked(&folder, &[]);
    let (table, _) = built(&folder);

    // Six works: the middle is midway between the third and the fourth, 1.4 and 1.5.
    assert_values(&table, "hair_top_rise", [1.2, 1.45, 1.9], 6);
    assert_eq!(
        entry(&table, "hair_top_rise")["standing"].as_str(),
        Some("given")
    );
    // Five works, since the made-up hair covers the ears of the sixth: the middle is the
    // third of 1.1, 1.2, 1.25, 1.3, 1.35.
    assert_values(&table, "ear_top_rise", [1.1, 1.25, 1.35], 5);
    assert_eq!(
        entry(&table, "ear_top_rise")["standing"].as_str(),
        Some("given")
    );
    // A proportion every made-up work draws alike: the mouth, 0.7 below the eye line.
    assert_values(&table, "mouth_centre_drop", [0.7, 0.7, 0.7], 6);

    // The table has an entry for every proportion of the list, in the list's order.
    let listed: Vec<String> = maquette_ok(&["list-proportions"])
        .lines()
        .filter_map(|line| {
            line.strip_suffix("  [measured]")
                .or_else(|| line.strip_suffix("  [convention]"))
                .map(str::to_string)
        })
        .collect();
    assert_eq!(listed.len(), 86);
    let entries: Vec<&str> = table["proportion"]
        .as_array()
        .expect("the table lists its proportions")
        .iter()
        .map(|entry| entry["name"].as_str().expect("a name"))
        .collect();
    assert_eq!(entries, listed);
}

#[test]
fn a_table_counts_characters_and_says_how_many_works_stand_behind_each_proportion() {
    // Made up: works 5 and 6 are two characters of one work, as several characters of one
    // work's sheets can be.
    let mut works = six_works();
    for at in [4, 5] {
        works[at].work = Some("one made-up work of two characters".to_string());
    }
    let folder = folder("table-characters", &works);
    checked(&folder, &[]);
    let (table, printed) = built(&folder);

    // Every accepted character counts once: the six hair tops are all there.
    assert_eq!(table["table"]["characters"].as_integer(), Some(6));
    assert_eq!(table["table"]["works"].as_integer(), Some(5));
    assert_values(&table, "hair_top_rise", [1.2, 1.45, 1.9], 6);
    let hair = entry(&table, "hair_top_rise");
    assert_eq!(hair["works"].as_integer(), Some(5));
    assert_eq!(hair["most_from_one_work"].as_integer(), Some(2));
    // Work 6's hair covers its ears: five characters of five works give the ears' tops.
    let ears = entry(&table, "ear_top_rise");
    assert_eq!(ears["characters"].as_integer(), Some(5));
    assert_eq!(ears["works"].as_integer(), Some(5));
    assert_eq!(ears["most_from_one_work"].as_integer(), Some(1));
    // The view is counted in characters, with the works beside them.
    assert_eq!(view(&table, "front")["characters"].as_integer(), Some(6));
    assert_eq!(view(&table, "front")["works"].as_integer(), Some(5));

    // The table lists each character with the work and the designer its readings name.
    let listed = table["character"]
        .as_array()
        .expect("the table lists its characters");
    assert_eq!(listed.len(), 6);
    let of_shared: Vec<&str> = listed
        .iter()
        .filter(|character| {
            character["work"].as_str() == Some("one made-up work of two characters")
        })
        .map(|character| character["label"].as_str().expect("a label"))
        .collect();
    assert_eq!(of_shared, ["work-5", "work-6"]);
    assert!(
        printed.contains("A table of 86 proportions from 6 characters of 5 works"),
        "{printed}"
    );
}

/// Has Jon say what [`checked`] has him say, at another look.
fn checked_at(folder: &Path, look: &str, rejected: &[&str]) {
    checked(folder, rejected);
    let path = folder.join("check.toml");
    let record = text(&path).replace(
        "look = \"Look 4 of 8, made up\"",
        &format!("look = \"{look}\""),
    );
    std::fs::write(path, record).expect("written");
}

#[test]
fn a_table_is_built_from_the_records_of_several_checks_each_read_in_its_own_folder() {
    // Made up: two batches, each in a folder of its own with its pictures, its readings, its
    // sheet and the record of its check, all named from that folder, as a batch is laid out.
    let folder = references("table-several-records");
    let works = six_works();
    write_batch(&folder.join("batch-a"), &labelled(&works[..3], 1), &|_| {
        String::new()
    });
    write_batch(&folder.join("batch-b"), &labelled(&works[3..], 4), &|_| {
        String::new()
    });
    checked_at(&folder.join("batch-a"), "Look 4 of 8, made up", &[]);
    checked_at(&folder.join("batch-b"), "Look 7 of 8, made up", &[]);

    // Each record says what it does when it is named from the folder that holds both.
    let said = measure_ok(&folder, &["jons-check", "--check", "batch-b/check.toml"]);
    assert!(said.contains("The record is complete."), "{said}");

    // One table from both: the six characters, three of each batch.
    let (table, printed) = built_from(&folder, &["batch-a/check.toml", "batch-b/check.toml"]);
    assert_eq!(table["table"]["characters"].as_integer(), Some(6));
    assert_values(&table, "hair_top_rise", [1.2, 1.45, 1.9], 6);
    assert_thin(&table, "chin_depth", 2);
    assert_eq!(
        table["table"]["built_from"].as_str(),
        Some(
            "the records of Jon's checks at Look 4 of 8, made up, checked on a made-up day, and at Look 7 of 8, made up, checked on a made-up day"
        )
    );
    assert!(
        printed.contains("from 6 characters of 6 works"),
        "{printed}"
    );

    // A table needs a record: none named is refused.
    let message = failure(&maquette_in(
        Some(&folder),
        &["table-from-readings", "--out", "results/none.toml"],
    ));
    assert!(message.contains("--check"), "{message}");
}

/// Two made-up batches for the tests of a character named in two records: the six works in
/// `batch-a`, their side views read under revision 2026-11-17.3, and in `batch-b` the side
/// views of works 1 and 2 read again under the program's own revision, with no front view,
/// the bridge of the nose at depth 0.35, and their chins placed further forward, at 0.5 and
/// 0.6, than the first readings put them.
fn read_again(name: &str) -> PathBuf {
    read_again_with(name, Some("2026-11-17.3"), None)
}

/// The batches of [`read_again`], with the first side views read under `earlier`, or under
/// the program's own revision where it is none, and the readings made again naming `work`
/// as their work, where it is given.
fn read_again_with(name: &str, earlier: Option<&'static str>, work: Option<&str>) -> PathBuf {
    let folder = references(name);
    let mut works = six_works();
    for first in &mut works[..2] {
        first.side_read_under = earlier;
    }
    write_batch(&folder.join("batch-a"), &labelled(&works, 1), &|_| {
        String::new()
    });
    let again: Vec<Work> = [0.5, 0.6]
        .into_iter()
        .zip(&works)
        .map(|(chin, first)| Work {
            front: false,
            chin_depth: Some(chin),
            side_points: vec![("centre-line-at-eye-level", [735.0, 300.0])],
            side_read_under: None,
            work: work.map(str::to_string).or(first.work.clone()),
            ..first.clone()
        })
        .collect();
    write_batch(&folder.join("batch-b"), &labelled(&again, 1), &|_| {
        String::new()
    });
    checked_at(&folder.join("batch-a"), "Look 4 of 8, made up", &[]);
    checked_at(&folder.join("batch-b"), "Look 7 of 8, made up", &[]);
    folder
}

#[test]
fn a_drawing_read_again_under_a_later_revision_adds_what_its_first_readings_were_not_asked_for() {
    let folder = read_again("table-read-again");
    let (table, _) = built_from(&folder, &["batch-a/check.toml", "batch-b/check.toml"]);

    // A label in both records is one character: six, as in the first batch alone.
    assert_eq!(table["table"]["characters"].as_integer(), Some(6));
    assert_eq!(view(&table, "side")["characters"].as_integer(), Some(2));
    // The chin is given by the first readings, which were asked for it, and the bridge by
    // the readings made again, which were the first asked for it.
    assert_thin(&table, "chin_depth", 2);
    assert_thin(&table, "bridge_depth", 2);
    let made = maquette_in(
        Some(&folder),
        &[
            "set-from-table",
            "--check",
            "batch-a/check.toml,batch-b/check.toml",
            "--borrow",
            path_text(&support::repository().join("data/saffi/proportions.toml")),
            "--out",
            "results/set.toml",
        ],
    );
    assert!(made.status.success(), "{}", failure(&made));
    let set: toml::Table = text(&folder.join("results/set.toml"))
        .parse()
        .expect("the set is TOML");
    let value = |name: &str| number(&set["proportions"][name]);
    assert_close(value("chin_depth"), 0.25, 1e-6, "the first readings' chins");
    assert_close(value("bridge_depth"), 0.35, 1e-6, "the bridge read again");
}

#[test]
fn a_drawing_read_again_gives_nothing_its_first_readings_were_asked_for_and_did_not_see() {
    // Made up: the first readings of works 1 and 2's side views were asked for the point
    // under the tip of the nose and did not see it; the readings made again place it. What the
    // first readings were asked for is theirs to give or not, and Jon checked them so: the
    // tip of the nose is not taken from the later ones.
    let folder = references("table-read-again-unseen");
    let mut works = six_works();
    for first in &mut works[..2] {
        first.side_read_under = Some("2026-11-17.3");
        first.side_unseen = vec!["nose-mark-bottom"];
    }
    write_batch(&folder.join("batch-a"), &labelled(&works, 1), &|_| {
        String::new()
    });
    let again: Vec<Work> = works[..2]
        .iter()
        .map(|first| Work {
            front: false,
            side_points: vec![("centre-line-at-eye-level", [735.0, 300.0])],
            side_read_under: None,
            side_unseen: Vec::new(),
            ..first.clone()
        })
        .collect();
    write_batch(&folder.join("batch-b"), &labelled(&again, 1), &|_| {
        String::new()
    });
    checked_at(&folder.join("batch-a"), "Look 4 of 8, made up", &[]);
    checked_at(&folder.join("batch-b"), "Look 7 of 8, made up", &[]);
    let (table, _) = built_from(&folder, &["batch-a/check.toml", "batch-b/check.toml"]);
    for name in ["nose_mark_bottom_depth", "nose_tip_drop"] {
        assert_eq!(
            entry(&table, name)["characters"].as_integer(),
            Some(0),
            "{name}"
        );
    }
    // The bridge, which the first readings were not asked for, still comes from the later.
    assert_thin(&table, "bridge_depth", 2);
}

#[test]
fn what_two_records_say_of_one_character_must_agree() {
    let both = ["batch-a/check.toml", "batch-b/check.toml"];

    // A character rejected in one record and accepted in the other: which verdict stands
    // is not decided, and no table is built.
    let folder = read_again("table-two-verdicts");
    checked_again(&folder.join("batch-b"), "work-1");
    let message = failure(&build_from(&folder, &both));
    assert!(
        message.contains("`work-1`") && message.contains("rejected"),
        "{message}"
    );

    // One label whose readings name two works is not one character.
    let folder = read_again_with(
        "table-two-works",
        Some("2026-11-17.3"),
        Some("another work"),
    );
    let message = failure(&build_from(&folder, &both));
    assert!(
        message.contains("`work-1`") && message.contains("another work"),
        "{message}"
    );

    // One drawing read twice under one revision: which readings give it is not decided.
    let folder = read_again_with("table-one-revision", None, None);
    let message = failure(&build_from(&folder, &both));
    assert!(
        message.contains("`work-1`") && message.contains("one revision"),
        "{message}"
    );
}

#[test]
fn the_ranges_of_each_age_band_stand_side_by_side_and_say_whether_they_overlap() {
    // Made up: five adult characters whose hair tops stand 1.0 to 1.4 above the eye line,
    // and five teenage ones at 1.5 to 1.9; the hair of the last teenage one covers its ears.
    let works: Vec<Work> = [1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9]
        .into_iter()
        .enumerate()
        .map(|(at, rise)| Work {
            age_band: if at < 5 { "adult" } else { "teenage" },
            ear_top_rise: (at < 9).then_some(1.25),
            ..Work::with_hair_top(rise)
        })
        .collect();
    let folder = folder("table-age-bands", &works);
    checked(&folder, &[]);
    let (table, printed) = built(&folder);

    // The table's range is of every character, whatever its band.
    assert_values(&table, "hair_top_rise", [1.0, 1.45, 1.9], 10);
    // Beside it, each band's own range: here they do not overlap.
    let band = |name: &str, band: &str| -> toml::Table {
        entry(&table, name)["bands"]
            .as_array()
            .expect("the bands")
            .iter()
            .filter_map(toml::Value::as_table)
            .find(|entry| entry["band"].as_str() == Some(band))
            .unwrap_or_else(|| panic!("{name} has no {band} band"))
            .clone()
    };
    for (which, expected) in [("adult", [1.0, 1.2, 1.4]), ("teenage", [1.5, 1.7, 1.9])] {
        let of = band("hair_top_rise", which);
        assert_eq!(of["characters"].as_integer(), Some(5), "{which}");
        for (key, expected) in ["lowest", "middle", "highest"].into_iter().zip(expected) {
            assert_close(number(&of[key]), expected, 1e-6, &format!("{which}: {key}"));
        }
    }
    assert_eq!(
        entry(&table, "hair_top_rise")["bands_overlap"].as_bool(),
        Some(false)
    );
    // Every character draws the mouth alike, so there the two ranges overlap.
    assert_eq!(
        entry(&table, "mouth_centre_drop")["bands_overlap"].as_bool(),
        Some(true)
    );
    // A band with fewer than five characters behind a proportion is too thin for a range of
    // its own, and states no number.
    let teenage_ears = band("ear_top_rise", "teenage");
    assert_eq!(teenage_ears["characters"].as_integer(), Some(4));
    assert!(!teenage_ears.contains_key("lowest"));
    assert!(!entry(&table, "ear_top_rise").contains_key("bands_overlap"));
    assert_eq!(
        table["table"]["bands"]
            .as_array()
            .expect("the bands")
            .iter()
            .map(|band| band["band"].as_str().expect("a band"))
            .collect::<Vec<_>>(),
        ["adult", "teenage"]
    );
    assert!(
        printed.contains("and do not overlap in 1: hair_top_rise."),
        "{printed}"
    );
}

/// One view's entry in a table, by what it is a count of.
fn view<'a>(table: &'a toml::Table, what: &str) -> &'a toml::Table {
    table["view"]
        .as_array()
        .expect("the table lists its views")
        .iter()
        .filter_map(toml::Value::as_table)
        .find(|entry| entry["view"].as_str() == Some(what))
        .unwrap_or_else(|| panic!("the table has no view `{what}`"))
}

#[test]
fn a_view_or_a_proportion_with_fewer_than_five_works_is_too_thin_and_has_no_range() {
    let folder = folder("table-too-thin", &six_works());
    checked(&folder, &[]);
    let (table, printed) = built(&folder);

    // The front view has six works, and is a table.
    let front = view(&table, "front");
    assert_eq!(front["characters"].as_integer(), Some(6));
    assert_eq!(front["standing"].as_str(), Some("a table"));
    // Two made-up works have a side view, so depth has two works: too thin, and not a table.
    for what in [
        "side",
        "depth, from a front and a side view of one character",
    ] {
        let thin = view(&table, what);
        assert_eq!(thin["characters"].as_integer(), Some(2), "{what}");
        assert_eq!(
            thin["standing"].as_str(),
            Some("too thin: not a table"),
            "{what}"
        );
    }
    assert!(
        printed.contains(
            "  depth, from a front and a side view of one character: 2 characters, of 2 works: too thin, a view needs 5\n"
        ),
        "{printed}"
    );

    // A depth is listed with its two works and no number: what two works give, the chin at
    // 0.2 and at 0.3, is those two works' own numbers.
    assert_thin(&table, "chin_depth", 2);
    let chin = entry(&table, "chin_depth");
    assert_eq!(chin["dimension"].as_str(), Some("depth"));
    let why = chin["not_given_by"]
        .as_array()
        .expect("why the rest do not");
    assert_eq!(why.len(), 1, "{why:?}");
    assert!(
        why[0]
            .as_str()
            .expect("text")
            .starts_with("4 characters: a front view shows no depth"),
        "{why:?}"
    );

    // A proportion with fewer than five works is too thin inside a view that has six: the
    // made-up brows show in four works.
    assert_thin(&table, "brow_inner_end_rise", 4);
    // Only a proportion with a range is stated in numbers, anywhere in the file.
    let numbered = table["proportion"]
        .as_array()
        .expect("entries")
        .iter()
        .filter(|entry| entry.get("middle").is_some())
        .count() as i64;
    assert_eq!(table["table"]["given"].as_integer(), Some(numbered));
    // The made-up head has no jaw point: no work gives it, and the entry has no value.
    let jaw = entry(&table, "jaw_drop");
    assert_eq!(jaw["characters"].as_integer(), Some(0));
    assert_eq!(jaw["standing"].as_str(), Some("no character"));
    assert!(!jaw.contains_key("lowest") && !jaw.contains_key("middle"));
    // A convention is never a measurement, and has no range whatever the works give.
    assert_eq!(
        entry(&table, "skull_radius")["standing"].as_str(),
        Some("convention")
    );

    // The counts the table states are the counts of its entries.
    let counted = |standing: &str| {
        table["proportion"]
            .as_array()
            .expect("entries")
            .iter()
            .filter(|entry| entry["standing"].as_str() == Some(standing))
            .count() as i64
    };
    let stated = table["table"].as_table().expect("a [table]");
    assert_eq!(stated["proportions"].as_integer(), Some(86));
    assert_eq!(stated["given"].as_integer(), Some(counted("given")));
    assert_eq!(stated["too_thin"].as_integer(), Some(counted("too thin")));
    assert_eq!(
        stated["no_character"].as_integer(),
        Some(counted("no character"))
    );
    assert_eq!(stated["conventions"].as_integer(), Some(3));
    assert_eq!(counted("convention"), 3);
    assert_eq!(stated["characters_for_a_table"].as_integer(), Some(5));
}

#[test]
fn a_drawing_that_is_set_aside_or_not_level_and_a_rejected_character_are_left_out() {
    let mut works = six_works();
    // Made up: the first reader calls work 4's front view not level and sets it aside, and
    // both readers set work 2's side view aside. Work 4 has the highest hair top, 1.9.
    works[3].front_set_aside = true;
    works[1].side_set_aside = true;
    let folder = folder("table-left-out", &works);
    checked(&folder, &[]);
    let (table, printed) = built(&folder);

    // Five works are left with a measured front view: 1.2, 1.3, 1.4, 1.5 and 1.6.
    assert_eq!(view(&table, "front")["characters"].as_integer(), Some(5));
    assert_values(&table, "hair_top_rise", [1.2, 1.4, 1.6], 5);
    assert_eq!(
        entry(&table, "hair_top_rise")["standing"].as_str(),
        Some("given")
    );
    // The ears of work 4 went with its front view: four works are left, 1.1 to 1.3.
    assert_thin(&table, "ear_top_rise", 4);
    // Only work 1's side view is measured, so depth is one work's.
    assert_thin(&table, "chin_depth", 1);
    // The table lists the five characters it was measured on, and says what it left out.
    let listed: Vec<&str> = table["character"]
        .as_array()
        .expect("the table lists its characters")
        .iter()
        .map(|character| character["work"].as_str().expect("a work"))
        .collect();
    assert_eq!(
        listed,
        [
            "made-up work 1",
            "made-up work 2",
            "made-up work 3",
            "made-up work 5",
            "made-up work 6"
        ]
    );
    let left_out: Vec<&str> = table["table"]["left_out"]
        .as_array()
        .expect("what was left out")
        .iter()
        .map(|line| line.as_str().expect("text"))
        .collect();
    assert!(
        left_out.contains(&"the side view of `work-2` is not measured")
            && left_out.contains(&"the front view of `work-4` is not measured"),
        "{left_out:?}"
    );
    assert!(
        printed.contains("Left out: the front view of `work-4` is not measured."),
        "{printed}"
    );

    // Made up: Jon rejects work 6 as well. Four works are left, so the front view is too
    // thin, it is not called a table, and no proportion has a range.
    checked_again(&folder, "work-6");
    let (table, printed) = built(&folder);
    let front = view(&table, "front");
    assert_eq!(front["characters"].as_integer(), Some(4));
    assert_eq!(front["standing"].as_str(), Some("too thin: not a table"));
    assert_thin(&table, "hair_top_rise", 4);
    assert_eq!(table["table"]["given"].as_integer(), Some(0));
    assert!(
        printed.contains("  front: 4 characters, of 4 works: too thin, a view needs 5\n"),
        "{printed}"
    );
}

/// Has Jon reject one more character in a record that is already filled in.
fn checked_again(folder: &Path, label: &str) {
    let path = folder.join("check.toml");
    let of = format!("label = \"{label}\"\nverdict = \"accepted\"");
    let record = text(&path);
    assert!(record.contains(&of), "the record has no accepted {label}");
    std::fs::write(
        path,
        record.replace(&of, &format!("label = \"{label}\"\nverdict = \"rejected\"")),
    )
    .expect("written");
}

#[test]
fn a_table_states_what_it_does_not_say_and_lists_the_characters_it_was_measured_on() {
    let folder = folder("table-statements", &six_works());
    checked(&folder, &[]);
    let (table, printed) = built(&folder);
    let file = text(&folder.join("results/table.toml"));

    // Its ranges are separate and do not rule on which combinations go together, and its
    // lowest and highest values are single references' numbers.
    let rules = table["rules"]
        .as_table()
        .expect("the table states its rules");
    let separate = rules["separate_ranges"].as_str().expect("text");
    assert!(
        separate.starts_with("Each range is separate. The table does not rule on which combinations of proportions go together"),
        "{separate}"
    );
    let single = rules["single_references"].as_str().expect("text");
    assert!(
        single.starts_with("A lowest value and a highest value are each one reference's number"),
        "{single}"
    );
    // Both are said to whoever builds it, too.
    assert!(
        printed.contains(separate) && printed.contains(single),
        "{printed}"
    );
    assert!(
        rules["comparison_case"]
            .as_str()
            .expect("text")
            .starts_with("No comparison case's numbers are in a table"),
    );
    assert!(
        file.contains("It is a proposal until Jon has accepted it"),
        "{file}"
    );

    // The list of characters: each with its work as its readings name it, and the views
    // measured of it.
    let works = table["character"]
        .as_array()
        .expect("the table lists its characters");
    assert_eq!(works.len(), 6);
    assert_eq!(works[0]["label"].as_str(), Some("work-1"));
    assert_eq!(works[0]["work"].as_str(), Some("made-up work 1"));
    assert_eq!(works[0]["designer"].as_str(), Some("a made-up designer"));
    assert_eq!(works[0]["age_band"].as_str(), Some("adult"));
    // Where a picture was taken from stays in its reading: an address can name a character,
    // and the table is written to be published. The reader's account of the manner stays
    // there too.
    assert!(
        !works[0].as_table().expect("a table").contains_key("source")
            && !works[0].as_table().expect("a table").contains_key("manner")
            && !file.contains("a made-up source"),
        "{file}"
    );
    let views = |work: &toml::Value| -> Vec<String> {
        work["views"]
            .as_array()
            .expect("views")
            .iter()
            .map(|view| view.as_str().expect("a view").to_string())
            .collect()
    };
    assert_eq!(views(&works[0]), ["front", "side"]);
    assert_eq!(views(&works[5]), ["front"]);
    // No character is tied to a number: an entry of the list holds texts and nothing else.
    for work in works {
        for (key, value) in work.as_table().expect("a character is a table") {
            assert!(
                value.is_str()
                    || value
                        .as_array()
                        .is_some_and(|views| views.iter().all(toml::Value::is_str)),
                "{key} of a character is not text"
            );
        }
    }
    // It says which look it was built from, and not where the record is kept.
    assert_eq!(
        table["table"]["built_from"].as_str(),
        Some("the record of Jon's check at Look 4 of 8, made up, checked on a made-up day")
    );
    assert!(!file.contains("check.toml"), "{file}");
    // No reading is named in it, and no picture.
    assert!(
        !file.contains("work-1-front") && !file.contains(".png"),
        "{file}"
    );
}

#[test]
fn a_record_that_is_not_complete_builds_no_table() {
    let folder = folder("table-record-not-complete", &six_works());
    // Nothing is said in the record yet: no character has a verdict.
    let message = failure(&build(&folder));
    assert!(
        message.contains("is not complete, so no table is built from it")
            && message.contains("`work-1` has no verdict"),
        "{message}"
    );
    assert!(!folder.join("results/table.toml").exists());
}

/// Saffi's own numbers, as the repository holds them: her set of proportions, and the
/// record of her hand-made views' positions.
const SAFFIS_SET: &str = "data/saffi/proportions.toml";
const SAFFIS_POSITIONS: &str = "data/saffi/hand-made-views.toml";

#[test]
fn a_comparison_cases_numbers_cannot_be_handed_to_a_table() {
    let folder = folder("table-comparison-case-set", &six_works());
    checked(&folder, &[]);
    let set = support::repository().join(SAFFIS_SET);

    // The command takes the record of Jon's check and nothing else: a set of proportions
    // cannot be named beside it.
    let output = maquette_in(
        Some(&folder),
        &[
            "table-from-readings",
            "--check",
            "check.toml",
            "--proportions",
            path_text(&set),
            "--out",
            "results/table.toml",
        ],
    );
    assert!(
        failure(&output).contains("`--proportions` is not an option of this command"),
        "{}",
        failure(&output)
    );
    // Nor in the record's place: her numbers are in the repository, and the command reads
    // only from the reference folder.
    let output = maquette_in(
        Some(&folder),
        &[
            "table-from-readings",
            "--check",
            path_text(&set),
            "--out",
            "results/table.toml",
        ],
    );
    assert!(
        failure(&output).contains("is outside the reference folder"),
        "{}",
        failure(&output)
    );
    // Copied into the reference folder and named in the record as a reading of work 6, her
    // set is still no reading: the record is then not complete, and nothing is built.
    std::fs::create_dir_all(folder.join("copied")).expect("made");
    std::fs::copy(&set, folder.join("copied/set.toml")).expect("copied");
    let path = folder.join("check.toml");
    let record = text(&path);
    let named = format!(
        "{{ path = \"first/work-6-front.toml\", sha256 = \"{}\" }}",
        support::sha256_of(&folder.join("first/work-6-front.toml"))
    );
    assert!(record.contains(&named), "{record}");
    std::fs::write(
        &path,
        record.replace(
            &named,
            &format!(
                "{{ path = \"copied/set.toml\", sha256 = \"{}\" }}",
                support::sha256_of(&folder.join("copied/set.toml"))
            ),
        ),
    )
    .expect("written");
    let message = failure(&build(&folder));
    assert!(
        message.contains("is not complete, so no table is built from it")
            && message.contains("copied/set.toml: the reading has no [reader] table"),
        "{message}"
    );
    assert!(!folder.join("results/table.toml").exists());
}

#[test]
fn a_comparison_cases_recorded_positions_written_as_a_reading_are_refused() {
    // The one path the program has from recorded positions to something a record of Jon's
    // check can name: `known-positions-as-reading`, here on Saffi's own record. Made up: the
    // sheet shows it as a seventh character, and the test has Jon accept it.
    let folder = folder_with("table-comparison-case-positions", &six_works(), &|folder| {
        support::made_up_image(&folder.join("pictures/hand-made-front.png"), 1000, 1000);
        measure_ok(
            folder,
            &[
                "known-positions-as-reading",
                "--known-positions",
                SAFFIS_POSITIONS,
                "--view",
                "front",
                "--image",
                "pictures/hand-made-front.png",
                "--out",
                "copied/front.toml",
            ],
        );
        "\n[[character]]\nlabel = \"the-comparison-case\"\n\n  [[character.drawing]]\n  picture = \"a grey picture, made up: nobody read it\"\n  readings = [\"copied/front.toml\"]\n".to_string()
    });
    checked(&folder, &[]);
    // The record itself is complete: the measuring commands may run on such a reading.
    let said = measure_ok(&folder, &["jons-check", "--check", "check.toml"]);
    assert!(
        said.contains("the-comparison-case: accepted")
            && said.contains("front: 7 characters, of 7 works"),
        "{said}"
    );

    let message = failure(&build(&folder));
    assert!(
        message.contains("the front view of `the-comparison-case` is not a reading")
            && message.contains("never enters one"),
        "{message}"
    );
    assert!(!folder.join("results/table.toml").exists());

    // With that character rejected, the table is built from the six made-up works alone.
    checked_again(&folder, "the-comparison-case");
    let (table, _) = built(&folder);
    assert_values(&table, "hair_top_rise", [1.2, 1.45, 1.9], 6);
}

/// A made-up set of proportions for setting against the made-up table: a few proportions,
/// each chosen for the verdict it must get. Head height is 1.45 + 1 = 2.45 units.
const MADE_UP_SET: &str = "\
# A made-up set of proportions for a test. No number here was measured, and it is no
# character's.

[proportions]
hair_top_rise = 1.45
ear_top_rise = 1.1
mouth_centre_drop = 0.9
eye_inner_corner_drop = 0.0
chin_depth = 0.25
jaw_drop = 0.9
skull_radius = 1.0

[corrections.lower_face_set_back]
chin = 0.1
mouth = 0.05
";

/// Sets a set of proportions against the table of a made-up reference folder, with no
/// reference folder configured, and returns what was written and what was printed.
fn set_against(name: &str, set: &str, table: &Path) -> (toml::Table, String) {
    let scratch = scratch(name);
    let set_path = scratch.join("set.toml");
    std::fs::write(&set_path, set).expect("written");
    let out = scratch.join("against.toml");
    let output = maquette_in(
        None,
        &[
            "set-against-table",
            "--proportions",
            path_text(&set_path),
            "--table",
            path_text(table),
            "--out",
            path_text(&out),
        ],
    );
    assert!(
        output.status.success(),
        "the set was not set against the table:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    (
        text(&out).parse().expect("the comparison is TOML"),
        String::from_utf8(output.stdout).expect("the output is text"),
    )
}

#[test]
fn a_set_of_proportions_is_told_inside_outside_or_no_range_for_each_proportion() {
    let folder = folder("table-set-against", &six_works());
    checked(&folder, &[]);
    built(&folder);
    let (against, printed) = set_against(
        "set-against-table",
        MADE_UP_SET,
        &folder.join("results/table.toml"),
    );
    let verdict = |name: &str| {
        entry(&against, name)["verdict"]
            .as_str()
            .expect("a verdict")
    };

    // Inside: within the range, and on its lowest value.
    assert_eq!(verdict("hair_top_rise"), "inside");
    assert_eq!(verdict("ear_top_rise"), "inside");
    let inside = entry(&against, "hair_top_rise");
    assert_close(number(&inside["value"]), 1.45, 1e-9, "the set's value");
    assert_close(number(&inside["lowest"]), 1.2, 1e-9, "the range's lowest");
    assert_close(number(&inside["highest"]), 1.9, 1e-9, "the range's highest");
    assert_eq!(inside["characters"].as_integer(), Some(6));

    // Outside, above: every made-up work draws the mouth 0.7 below the eye line, and the
    // set has it at 0.9. That is 0.2 units beyond, and 0.2 / 2.45 of head height.
    let above = entry(&against, "mouth_centre_drop");
    assert_eq!(above["verdict"].as_str(), Some("outside"));
    assert_eq!(above["side"].as_str(), Some("above the highest value"));
    assert_close(number(&above["beyond"]), 0.2, 1e-9, "how far beyond");
    assert_close(
        number(&above["beyond_percent_of_head_height"]),
        100.0 * 0.2 / 2.45,
        1e-3,
        "how far beyond, of head height",
    );
    // Outside, below: the inner corners are 0.02 below the eye line in every made-up work.
    let below = entry(&against, "eye_inner_corner_drop");
    assert_eq!(below["verdict"].as_str(), Some("outside"));
    assert_eq!(below["side"].as_str(), Some("below the lowest value"));
    assert_close(number(&below["beyond"]), 0.02, 1e-9, "how far beyond");

    // No range to be set against: too thin, no work, and a convention.
    for (name, why) in [
        (
            "chin_depth",
            "too thin: 2 characters stand behind it, and a range needs 5",
        ),
        ("jaw_drop", "no character gives it"),
        (
            "skull_radius",
            "a convention: it is built by rule, never measured, and has no valid range",
        ),
    ] {
        let entry = entry(&against, name);
        assert_eq!(entry["verdict"].as_str(), Some("no range"), "{name}");
        assert_eq!(entry["why"].as_str(), Some(why), "{name}");
        assert!(!entry.contains_key("side"), "{name}");
    }
    // Nothing stands beside a thin one but its count of works: the table states no number
    // for it.
    let thin = entry(&against, "chin_depth");
    assert_eq!(thin["characters"].as_integer(), Some(2));
    assert!(!thin.contains_key("lowest") && !thin.contains_key("highest"));

    // The counts, and what the set does not give.
    let summary = against["summary"].as_table().expect("a summary");
    assert_eq!(summary["set_against"].as_integer(), Some(7));
    assert_eq!(summary["inside"].as_integer(), Some(2));
    assert_eq!(summary["outside"].as_integer(), Some(2));
    assert_eq!(summary["no_range"].as_integer(), Some(3));
    assert_eq!(summary["not_in_the_set"].as_integer(), Some(79));
    assert_eq!(against["proportion"].as_array().expect("entries").len(), 7);
    // A correction is no proportion: the table has no range for it, and the file says so.
    assert_eq!(
        against["source"]["corrections_not_set_against"]
            .as_array()
            .expect("the corrections")[0]
            .as_str(),
        Some("lower_face_set_back")
    );

    // The proportions outside are named to whoever runs it, with how far, and as a finding.
    assert!(
        printed
            .contains("2 inside their valid range, 2 outside, 3 with no range to be set against"),
        "{printed}"
    );
    assert!(
        printed.contains(
            "  mouth_centre_drop: 0.9 is above the highest value 0.7 by 0.2 units, 8.16% of head height\n"
        ),
        "{printed}"
    );
    assert!(
        printed.contains(
            "  eye_inner_corner_drop: 0.0 is below the lowest value 0.02 by 0.02 units, 0.82% of head height\n"
        ),
        "{printed}"
    );
    assert!(printed.contains("a finding, not a fault"), "{printed}");
}

#[test]
fn what_is_no_set_or_no_table_is_not_set_against_anything() {
    let folder = folder("table-set-against-refusals", &six_works());
    checked(&folder, &[]);
    built(&folder);
    let table = folder.join("results/table.toml");
    let scratch = scratch("set-against-table-refusals");
    let against = |set: &Path, table: &Path| {
        maquette_in(
            None,
            &[
                "set-against-table",
                "--proportions",
                path_text(set),
                "--table",
                path_text(table),
            ],
        )
    };

    // A name the list of proportions does not have.
    let unknown = scratch.join("unknown.toml");
    std::fs::write(
        &unknown,
        "[proportions]\nhair_top_rise = 1.4\nnose_length = 0.3\n",
    )
    .expect("written");
    let message = failure(&against(&unknown, &table));
    assert!(
        message.contains("`nose_length`") && message.contains("the list of proportions has no"),
        "{message}"
    );
    // A set of proportions is not a table.
    let set = scratch.join("set.toml");
    std::fs::write(&set, MADE_UP_SET).expect("written");
    let message = failure(&against(&set, &set));
    assert!(message.contains("is not a proportion table"), "{message}");
    // A table that calls a proportion given with fewer than five works behind it is refused:
    // a range is never taken from a thin proportion, whatever a file says.
    let spoiled = scratch.join("spoiled-table.toml");
    let thin = "name = \"chin_depth\"\nkind = \"measured\"\ndimension = \"depth\"\ncharacters = 2\nworks = 2\nmost_from_one_work = 1\nstanding = \"too thin\"";
    let file = text(&table);
    assert!(file.contains(thin), "{file}");
    std::fs::write(
        &spoiled,
        file.replace(thin, &thin.replace("too thin", "given")),
    )
    .expect("written");
    let message = failure(&against(&set, &spoiled));
    assert!(
        message.contains("`chin_depth`") && message.contains("2 characters"),
        "{message}"
    );

    // Printed whole when no file is named, and with no reference folder configured.
    let output = against(&set, &table);
    assert!(output.status.success(), "{}", failure(&output));
    let printed = String::from_utf8(output.stdout).expect("text");
    assert!(
        printed.contains("[[proportion]]\nname = \"hair_top_rise\""),
        "{printed}"
    );
    // Building a table is measuring, and stops with no reference folder.
    let output = maquette_in(None, &["table-from-readings", "--check", "check.toml"]);
    assert!(
        failure(&output).contains("MAQUETTE_REFERENCES"),
        "{}",
        failure(&output)
    );
}

#[test]
fn the_help_names_both_commands() {
    let help = maquette_ok(&["help"]);
    assert!(
        help.contains("maquette table-from-readings --check <check>"),
        "{help}"
    );
    assert!(
        help.contains("maquette set-against-table --proportions <file> --table <file>"),
        "{help}"
    );
    assert!(
        help.contains("maquette set-from-table --check <check>[,<check>...] --borrow <set>"),
        "{help}"
    );
    assert!(
        help.contains("maquette overlay-sources --proportions <file> --armature <folder>"),
        "{help}"
    );
}

/// Makes a whole set from the table of a made-up reference folder, borrowing from the
/// made-up set of proportions, and returns the set's file as text and as TOML.
fn set_from_table(folder: &Path) -> (String, toml::Table) {
    measure_ok(
        folder,
        &[
            "set-from-table",
            "--check",
            "check.toml",
            "--borrow",
            support::MADE_UP,
            "--out",
            "results/set.toml",
        ],
    );
    let file = text(&folder.join("results/set.toml"));
    let table = file.parse().expect("the set is TOML");
    (file, table)
}

#[test]
fn a_whole_set_is_made_from_a_table_and_says_where_each_number_comes_from() {
    // Made up: a third work has a side view, with the chin at depth 0.7. The three chins
    // are 0.2, 0.3 and 0.7: their middle is 0.3 and their average 0.4.
    let mut works = six_works();
    works[2].chin_depth = Some(0.7);
    for work in &mut works {
        work.neck = true;
    }
    let folder = folder("table-set-from-table", &works);
    checked(&folder, &[]);
    let (file, set) = set_from_table(&folder);
    let borrowed = support::proportions(&support::repository().join(support::MADE_UP));
    let value = |name: &str| number(&set["proportions"][name]);
    let source = |name: &str| set["sources"][name].as_str().expect("a source").to_string();

    // A proportion with a range takes the table's middle value.
    assert_eq!(
        source("hair_top_rise"),
        "the table: the middle of 6 characters"
    );
    assert_close(value("hair_top_rise"), 1.45, 1e-6, "the table's middle");
    // One that two to four works give takes their average, not their middle, and is thin.
    assert_eq!(source("chin_depth"), "a thin average: of 3 characters");
    assert_close(value("chin_depth"), 0.4, 1e-6, "the average of three works");
    assert_eq!(
        source("brow_inner_end_rise"),
        "a thin average: of 4 characters"
    );
    assert_close(value("brow_inner_end_rise"), 0.3, 1e-6, "the brows");
    // What no work gives, and a convention, are borrowed as they stand.
    assert_eq!(source("face_top_rise"), "borrowed: no character gives it");
    assert_close(
        value("face_top_rise"),
        borrowed["face_top_rise"],
        1e-9,
        "borrowed",
    );
    // A borrowed point that hangs on a measured one keeps its offset from it. The jaw point
    // hangs on the chin: it is borrowed as how far the borrowed set's jaw point stands from
    // that set's own chin, and placed from this head's chin. The chin is one unit below the
    // eye line and on the centre line in every head, so the jaw point's height and its
    // half-width come over as they stand. Its depth does not: this head's chin stands at
    // 0.4, the borrowed set's at 0.1875 with its jaw point at 0.1621, 0.0254 behind it.
    let hangs = "borrowed: as its offset from the chin, on which the jaw point hangs; no character gives it";
    for name in ["jaw_drop", "jaw_half_width", "jaw_depth"] {
        assert_eq!(source(name), hangs, "{name}");
    }
    assert_close(value("jaw_drop"), borrowed["jaw_drop"], 1e-9, "borrowed");
    assert_close(
        value("jaw_half_width"),
        borrowed["jaw_half_width"],
        1e-9,
        "borrowed",
    );
    assert_close(
        value("jaw_depth"),
        0.4 + (0.1621 - 0.1875),
        1e-6,
        "from the chin",
    );
    assert_eq!(
        source("skull_radius"),
        "borrowed: a convention, which is never measured"
    );
    assert_close(
        value("skull_radius"),
        borrowed["skull_radius"],
        1e-9,
        "borrowed",
    );
    // The neck is taken whole too. Every made-up front places both of its bases, 0.45 to each
    // side and 1.6 down, and the three side views its front base at depth 0.05 and its back
    // base at -0.85, so the works give its width, where it ends and its depth, -0.4. No view
    // places its top ends, and the borrowed set leaves them out: they are left out, and that
    // does not take the rest of the neck from the borrowed set, since a neck without its top
    // ends is drawn as the front view's column, from the other three.
    assert_eq!(
        source("neck_half_width"),
        "the table: the middle of 6 characters"
    );
    assert_close(value("neck_half_width"), 0.45, 1e-6, "the neck's width");
    assert_eq!(
        source("neck_base_drop"),
        "the table: the middle of 6 characters"
    );
    assert_close(value("neck_base_drop"), 1.6, 1e-6, "where the neck ends");
    assert_eq!(source("neck_depth"), "a thin average: of 3 characters");
    assert_close(value("neck_depth"), -0.4, 1e-6, "the neck's depth");
    assert_eq!(
        source("neck_front_base_depth"),
        "a thin average: of 3 characters"
    );
    for name in ["neck_back_top_drop", "neck_back_top_depth"] {
        assert!(
            source(name).starts_with("left out: no character gives it"),
            "{name}: {}",
            source(name)
        );
    }
    // The ear is taken whole: five works give the height of its top, 1.1 to 1.35, and none
    // gives its base, so the top is borrowed with the rest of it.
    assert_eq!(
        source("ear_top_rise"),
        "borrowed: the ear is taken whole, and the characters do not give all of it"
    );
    assert_close(
        value("ear_top_rise"),
        borrowed["ear_top_rise"],
        1e-9,
        "borrowed",
    );

    // The counts are those of the sources, and every proportion of the list has one.
    let sources = set["sources"].as_table().expect("the sources");
    assert_eq!(sources.len(), 86);
    let counted = |kind: &str| {
        sources
            .values()
            .filter(|said| said.as_str().is_some_and(|said| said.starts_with(kind)))
            .count() as i64
    };
    let stated = set["source"].as_table().expect("a [source]");
    assert_eq!(
        stated["from_the_table"].as_integer(),
        Some(counted("the table:"))
    );
    assert_eq!(
        stated["thin_averages"].as_integer(),
        Some(counted("a thin average:"))
    );
    assert_eq!(stated["borrowed"].as_integer(), Some(counted("borrowed:")));
    assert_eq!(stated["left_out"].as_integer(), Some(counted("left out:")));
    assert_eq!(
        counted("the table:")
            + counted("a thin average:")
            + counted("borrowed:")
            + counted("left out:"),
        86
    );
    // The points of the profile that only a side view gives: no made-up work gives them, and
    // the made-up set they would be borrowed from leaves them out, so the made set does too.
    assert!(
        source("jaw_angle_depth").starts_with("left out: no character gives it"),
        "{}",
        source("jaw_angle_depth")
    );
    assert!(
        !set["proportions"]
            .as_table()
            .expect("[proportions]")
            .contains_key("jaw_angle_depth")
    );
    assert_eq!(stated["borrowed_from"].as_str(), Some(support::MADE_UP));

    // It carries no correction, and says what the list's rule would set back. At a share of
    // 1 that rule draws the chin straight below the iris's centre: the chin stands 0.4 in
    // front of the eyes, and the made-up head has no iris, so the iris's depth is borrowed.
    assert!(!file.contains("[corrections"), "{file}");
    let correction = stated["correction"].as_str().expect("text");
    let would = format!("{:.6}", 0.4 - borrowed["iris_depth"]);
    let would = would.trim_end_matches('0');
    assert!(
        correction.starts_with("The set carries no correction.")
            && correction.contains(&format!(
                "would set the chin back by {would} of a unit at a share of 1"
            )),
        "{correction}"
    );
    // It says it is no table, and `maquette draw` reads it as it reads any set.
    assert!(
        file.contains("It is no proportion table and no part of one"),
        "{file}"
    );
    let drawn = scratch("set-from-table-drawn");
    maquette_ok(&[
        "draw",
        "--proportions",
        path_text(&folder.join("results/set.toml")),
        "--out",
        path_text(&drawn),
    ]);
    assert!(drawn.join("side.svg").exists());

    // Made up: only work 1 has a side view. One work's depth is one reference's number: it
    // is not drawn, and the borrowed set's stands in.
    let mut works = six_works();
    works[1].chin_depth = None;
    let one_work = folder_with("table-set-from-one-work", &works, &|_| String::new());
    checked(&one_work, &[]);
    let (_, set) = set_from_table(&one_work);
    assert_eq!(
        set["sources"]["chin_depth"].as_str(),
        Some("borrowed: one character gives it, and one reference's number is not drawn")
    );
    assert_close(
        number(&set["proportions"]["chin_depth"]),
        borrowed["chin_depth"],
        1e-9,
        "borrowed",
    );
}

/// The dots of one kind in a view's overlay of sources: where each stands.
fn dots(overlay: &str, group: &str) -> Vec<[f64; 2]> {
    let document = roxmltree::Document::parse(overlay).expect("the overlay is an SVG");
    let of_kind = document
        .descendants()
        .find(|node| node.attribute("id") == Some(group))
        .unwrap_or_else(|| panic!("the overlay has no group `{group}`"));
    of_kind
        .children()
        .filter(|node| node.has_tag_name("circle"))
        .map(|circle| {
            ["cx", "cy"].map(|key| {
                circle
                    .attribute(key)
                    .and_then(|value| value.parse().ok())
                    .expect("a dot has a place")
            })
        })
        .collect()
}

fn has_dot(dots: &[[f64; 2]], at: [f64; 2]) -> bool {
    dots.iter()
        .any(|dot| (dot[0] - at[0]).abs() < 0.002 && (dot[1] - at[1]).abs() < 0.002)
}

#[test]
fn the_points_of_a_drawn_head_are_marked_by_where_their_numbers_come_from() {
    let mut works = six_works();
    works[2].chin_depth = Some(0.7);
    let folder = folder("table-overlay-sources", &works);
    checked(&folder, &[]);
    set_from_table(&folder);
    let set = folder.join("results/set.toml");
    let drawn = scratch("overlay-sources");
    maquette_ok(&[
        "draw",
        "--proportions",
        path_text(&set),
        "--out",
        path_text(&drawn),
    ]);
    let printed = maquette_ok(&[
        "overlay-sources",
        "--proportions",
        path_text(&set),
        "--armature",
        path_text(&drawn),
        "--out",
        path_text(&drawn),
    ]);
    // A set made before the list had the points of the profile, which a set may leave out,
    // says nothing of where they come from, and is overlaid all the same.
    let profile = [
        "forehead_depth",
        "bridge_depth",
        "jaw_angle_",
        "neck_front_",
        "neck_back_",
    ];
    let earlier: String = text(&set)
        .lines()
        .filter(|line| !profile.iter().any(|name| line.starts_with(name)))
        .map(|line| format!("{line}\n"))
        .collect();
    assert!(!earlier.contains("forehead_depth"), "{earlier}");
    let earlier_set = drawn.join("earlier-set.toml");
    std::fs::write(&earlier_set, earlier).expect("written");
    let earlier_out = scratch("overlay-sources-earlier-set");
    maquette_ok(&[
        "overlay-sources",
        "--proportions",
        path_text(&earlier_set),
        "--armature",
        path_text(&drawn),
        "--out",
        path_text(&earlier_out),
    ]);
    let place = |view: &str, name: &str| -> [f64; 2] {
        let point =
            support::LandmarkFile::read(&drawn.join(format!("{view}.landmarks.toml"))).point(name);
        [point.x, point.y]
    };
    let overlay = |view: &str| text(&drawn.join(format!("{view}.sources-overlay.svg")));
    let front = overlay("front");
    let side = overlay("side");

    // The front view shows no depth. The mouth's centre stands at a height the table gives.
    let at = place("front", "mouth-line-centre");
    assert!(has_dot(&dots(&front, "from-the-table"), at), "{front}");
    // The side view shows its depth as well, which three works give: a thin average. A
    // point is no better measured than its weakest number.
    let at = place("side", "mouth-line-centre");
    assert!(has_dot(&dots(&side, "thin-averages"), at), "{side}");
    assert!(!has_dot(&dots(&side, "from-the-table"), at), "{side}");
    // The jaw point is in no made-up work, so it is borrowed in every view.
    let at = place("front", "face-outline-left-jaw");
    assert!(has_dot(&dots(&front, "borrowed"), at), "{front}");
    // The chin in the front view stands on the centre line, one unit below the eye line:
    // it rests on no proportion, and has no dot. In the side view it stands at its depth.
    let at = place("front", "chin");
    for group in ["from-the-table", "thin-averages", "borrowed"] {
        assert!(!has_dot(&dots(&front, group), at), "{group}: {front}");
    }
    assert!(has_dot(
        &dots(&side, "thin-averages"),
        place("side", "chin")
    ));
    // A brow's end takes its depth from the line through the eye's corners. The table gives
    // the corners' offsets and three works how far the eye wraps: thin in the side view.
    // In the front view only its offset and its height count, which four works give.
    let at = place("side", "brow-line-right-inner-end");
    assert!(has_dot(&dots(&side, "thin-averages"), at), "{side}");
    // The skull is a convention, and borrowed: a point of the form that is no landmark.
    let at = place("front", "skull-top");
    assert!(has_dot(&dots(&front, "borrowed"), at), "{front}");
    // The overlay is as large as the view, so that it lies over it exactly.
    assert!(
        front.contains("width=\"1000\" height=\"1000\" viewBox=\"0 0 1000 1000\""),
        "{front}"
    );

    // The counts printed are the dots drawn.
    let line = format!(
        "front: {} points rest on the table's middle values, {} on thin averages, {} on borrowed numbers.\n",
        dots(&front, "from-the-table").len(),
        dots(&front, "thin-averages").len(),
        dots(&front, "borrowed").len()
    );
    assert!(printed.contains(&line), "{printed}");

    // A set that does not say where its numbers come from is not marked.
    let output = support::maquette(&[
        "overlay-sources",
        "--proportions",
        support::MADE_UP,
        "--armature",
        path_text(&drawn),
        "--out",
        path_text(&drawn),
    ]);
    assert!(
        failure(&output).contains("has no [sources] table"),
        "{}",
        failure(&output)
    );
}

#[test]
fn the_bridge_of_a_drawn_head_is_marked_by_its_own_depth_where_the_set_gives_one() {
    // The batches of `read_again`: two characters' side views place the bridge of the nose,
    // so the set gives it as a thin average, and the side view draws the bridge from it and
    // not from the skull.
    let folder = read_again("table-overlay-bridge");
    let made = maquette_in(
        Some(&folder),
        &[
            "set-from-table",
            "--check",
            "batch-a/check.toml,batch-b/check.toml",
            "--borrow",
            support::MADE_UP,
            "--out",
            "results/set.toml",
        ],
    );
    assert!(made.status.success(), "{}", failure(&made));
    let set = folder.join("results/set.toml");
    let drawn = scratch("overlay-bridge");
    maquette_ok(&[
        "draw",
        "--proportions",
        path_text(&set),
        "--out",
        path_text(&drawn),
    ]);
    maquette_ok(&[
        "overlay-sources",
        "--proportions",
        path_text(&set),
        "--armature",
        path_text(&drawn),
        "--out",
        path_text(&drawn),
    ]);
    let side = text(&drawn.join("side.sources-overlay.svg"));
    let point = support::LandmarkFile::read(&drawn.join("side.landmarks.toml"))
        .point("centre-line-at-eye-level");
    let at = [point.x, point.y];
    assert!(has_dot(&dots(&side, "thin-averages"), at), "{side}");
    assert!(!has_dot(&dots(&side, "borrowed"), at), "{side}");
}
