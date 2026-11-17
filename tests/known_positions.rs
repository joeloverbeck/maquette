//! A reading scored against a drawing whose true positions are on record, and two sets of
//! proportions set against each other, through the command line.
//!
//! The record is `tests/fixtures/known-positions/made-up-known-positions.toml`, which says it
//! is made up, and the readings are written by the test: each lands a known distance from
//! the record's position, so every share below is worked by hand. No drawing was read.

mod support;

use std::path::Path;

use support::{
    MADE_UP, assert_close, maquette_in, maquette_ok, measure_ok, number, references, scratch, text,
};

const KNOWN: &str = "tests/fixtures/known-positions/made-up-known-positions.toml";

fn write_reading(
    folder: &Path,
    name: &str,
    view: &str,
    placed: &[(&str, f64, f64)],
    not_seen: &[&str],
) {
    let mut out = String::from(
        "# A made-up reading for a test. No drawing was read.\ninstructions = \"made up\"\n",
    );
    out.push_str("\n[reader]\nname = \"made-up reader\"\naids = \"made up\"\n");
    out.push_str("\n[image]\nfile = \"made-up.png\"\nsha256 = \"made-up-digest\"\nwidth = 1000\nheight = 1000\n");
    out.push_str(&format!("\n[judgement]\nview = \"{view}\"\n"));
    if view != "front" {
        out.push_str("faces = \"the viewer's right\"\nturn_degrees = 30.0\n");
    }
    out.push_str("level = true\nset_aside = false\n\n[landmarks]\n");
    for (landmark, x, y) in placed {
        out.push_str(&format!("{landmark} = {{ x = {x:?}, y = {y:?} }}\n"));
    }
    out.push_str("\n[not_seen]\n");
    for landmark in not_seen {
        out.push_str(&format!(
            "{landmark} = \"made up: not placed for a test\"\n"
        ));
    }
    std::fs::write(folder.join(name), out).expect("the made-up reading is written");
}

fn scored(folder: &Path, reading: &str, out: &str) -> toml::Table {
    measure_ok(
        folder,
        &[
            "score-reading",
            "--reading",
            reading,
            "--known-positions",
            KNOWN,
            "--out",
            out,
        ],
    );
    text(&folder.join(out)).parse().expect("the score is TOML")
}

fn entries<'a>(table: &'a toml::Table, list: &str) -> Vec<&'a toml::Table> {
    table
        .get(list)
        .and_then(toml::Value::as_array)
        .map(|entries| entries.iter().filter_map(toml::Value::as_table).collect())
        .unwrap_or_default()
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
fn a_reading_is_scored_against_known_positions_by_kind_of_landmark() {
    let folder = references("known-positions-front");
    write_reading(
        &folder,
        "front.toml",
        "front",
        &[
            // 3 across and 4 down from 500, 500: 5 units, 1.25% of the head height of 400.
            ("chin", 503.0, 504.0),
            // The hair's top is scored in height only: 2 units, 0.5%.
            ("hair-top", 300.0, 102.0),
            ("eye-left-outer-corner", 650.0, 300.0),
            // 2 units: 0.5%.
            ("eye-right-outer-corner", 352.0, 300.0),
            ("eye-left-inner-corner", 550.0, 302.0),
            ("eye-right-inner-corner", 450.0, 302.0),
            ("eye-left-opening-top", 600.0, 290.0),
            ("eye-left-opening-bottom", 600.0, 310.0),
            ("eye-right-opening-top", 400.0, 290.0),
            ("eye-right-opening-bottom", 400.0, 310.0),
            ("nose-mark-bottom", 500.0, 380.0),
            // 4 units is exactly 1%, which is not above 1%.
            ("mouth-line-centre", 500.0, 424.0),
            ("mouth-line-left-end", 530.0, 418.0),
            // The record's band runs from 660, 250 to 660, 258, so its middle is 660, 254:
            // 3 across and 4 down, 5 units, 1.25%.
            ("brow-line-left-outer-end", 663.0, 258.0),
            // Under the hair in the drawing, and placed all the same: 1 unit, 0.25%.
            ("face-outline-left-top", 641.0, 270.0),
            // A crossing is scored sideways only: 3 units, 0.75%.
            ("face-outline-left-at-eye-level", 645.0, 300.0),
            // A side extent is scored sideways only: 2 units, 0.5%.
            ("hair-left-extent", 702.0, 100.0),
            // The record has no ear in this view.
            ("ear-left-top", 600.0, 90.0),
        ],
        &["mouth-line-right-end", "hair-right-extent"],
    );
    let found = scored(&folder, "front.toml", "front-score.toml");

    assert_close(
        number(&found["yardstick"]["head_height"]),
        400.0,
        1e-9,
        "the yardstick",
    );
    let summary = &found["summary"];
    // Sixteen placed landmarks that can be seen and have a known position.
    assert_eq!(summary["scored"].as_integer(), Some(16));
    // 1.25 + 0.5 + 0.5 + 1.0 + 1.25 + 0.75 + 0.5 = 5.75, over 16.
    assert_close(
        number(&summary["mean_percent_of_head_height"]),
        5.75 / 16.0,
        0.0005,
        "the mean",
    );
    assert_close(
        number(&summary["worst_percent_of_head_height"]),
        1.25,
        1e-9,
        "the worst",
    );
    assert_eq!(summary["worst"].as_str(), Some("brow-line-left-outer-end"));
    assert_eq!(summary["above_1_percent"].as_integer(), Some(2));
    assert_eq!(summary["above_2_percent"].as_integer(), Some(0));
    // Nine of the sixteen are exact, so the median is 0.
    assert_close(
        number(&summary["median_percent_of_head_height"]),
        0.0,
        1e-9,
        "the median",
    );

    let kind = |name: &str| -> toml::Table {
        entries(&found, "kind")
            .into_iter()
            .find(|entry| entry["kind"].as_str() == Some(name))
            .unwrap_or_else(|| panic!("no kind `{name}`"))
            .clone()
    };
    for (name, count, mean, worst, worst_landmark) in [
        ("eye opening", 8, 0.0625, 0.5, "eye-right-outer-corner"),
        ("chin", 1, 1.25, 1.25, "chin"),
        ("hair", 2, 0.5, 0.5, "hair-left-extent"),
        ("nose mark", 1, 0.0, 0.0, "nose-mark-bottom"),
        ("mouth line", 2, 0.5, 1.0, "mouth-line-centre"),
        ("brow line", 1, 1.25, 1.25, "brow-line-left-outer-end"),
        (
            "face outline",
            1,
            0.75,
            0.75,
            "face-outline-left-at-eye-level",
        ),
    ] {
        let kind = kind(name);
        assert_eq!(kind["scored"].as_integer(), Some(count), "{name}");
        // The file gives a share to a thousandth of a percent.
        assert_close(
            number(&kind["mean_percent_of_head_height"]),
            mean,
            0.001,
            name,
        );
        assert_close(
            number(&kind["worst_percent_of_head_height"]),
            worst,
            1e-9,
            name,
        );
        assert_eq!(kind["worst"].as_str(), Some(worst_landmark), "{name}");
    }
    assert_eq!(entries(&found, "kind").len(), 7);

    // What cannot be seen in the picture is reported apart and scores nothing.
    let covered = entries(&found, "placed_though_covered");
    assert_eq!(covered.len(), 1);
    assert_eq!(
        covered[0]["landmark"].as_str(),
        Some("face-outline-left-top")
    );
    assert_close(
        number(&covered[0]["percent_of_head_height"]),
        0.25,
        1e-9,
        "the covered point",
    );
    assert_eq!(
        texts(&found["not_seen"]["though_nothing_covers_it"]),
        ["hair-right-extent", "mouth-line-right-end"]
    );
    assert_eq!(
        texts(&found["not_scored"]["no_known_position"]),
        ["ear-left-top"]
    );
}

#[test]
fn a_turned_reading_is_scored_in_its_own_view_and_a_covered_point_rightly_left_out_is_said_so() {
    let folder = references("known-positions-turned");
    write_reading(
        &folder,
        "turned.toml",
        "three-quarter",
        &[
            ("chin", 570.0, 500.0),
            ("hair-top", 490.0, 98.0),
            // The near side's outline, which the record holds as the jaw line: 3 units
            // sideways, over this view's head height of 402.
            ("face-outline-right-at-nose-level", 403.0, 385.0),
            ("ear-right-base-outer", 380.0, 170.0),
        ],
        &["ear-left-base-outer"],
    );
    let found = scored(&folder, "turned.toml", "turned-score.toml");
    assert_close(
        number(&found["yardstick"]["head_height"]),
        402.0,
        1e-9,
        "the yardstick",
    );
    assert_eq!(found["summary"]["scored"].as_integer(), Some(4));
    assert_close(
        number(&found["summary"]["worst_percent_of_head_height"]),
        3.0 / 402.0 * 100.0,
        0.0005,
        "the worst",
    );
    assert_eq!(
        found["summary"]["worst"].as_str(),
        Some("face-outline-right-at-nose-level")
    );
    assert_eq!(
        texts(&found["not_seen"]["and_covered_in_the_drawing"]),
        ["ear-left-base-outer"]
    );
    assert!(texts(&found["not_seen"]["though_nothing_covers_it"]).is_empty());
}

#[test]
fn two_sets_of_proportions_are_set_against_each_other() {
    // The made-up set against itself with three values changed. Head height is
    // 1.3127 + 1 = 2.3127 units.
    let folder = scratch("compare-proportions");
    let changed = text(&support::repository().join(MADE_UP))
        // 0.03 of a unit: 1.2972% of head height.
        .replace("chin_depth = 0.1875", "chin_depth = 0.2175")
        // 0.01 of a unit: 0.4324%.
        .replace("jaw_drop = 0.9127", "jaw_drop = 0.9027")
        .replace("skull_radius = 0.9379", "skull_radius = 1.0379");
    let path = folder.join("changed.toml");
    std::fs::write(&path, changed).expect("written");
    let printed = maquette_ok(&[
        "compare-proportions",
        "--first",
        path.to_str().expect("text"),
        "--second",
        MADE_UP,
    ]);
    let found: toml::Table = printed.parse().expect("the comparison is TOML");
    assert_close(number(&found["head_height"]), 2.3127, 1e-9, "head height");
    let entry = |name: &str| -> toml::Table {
        entries(&found, "proportion")
            .into_iter()
            .find(|entry| entry["name"].as_str() == Some(name))
            .unwrap_or_else(|| panic!("{name} is not compared"))
            .clone()
    };
    let chin = entry("chin_depth");
    assert_close(number(&chin["first"]), 0.2175, 1e-9, "first");
    assert_close(number(&chin["second"]), 0.1875, 1e-9, "second");
    assert_close(number(&chin["difference"]), 0.03, 1e-9, "difference");
    assert_close(
        number(&chin["percent_of_head_height"]),
        0.03 / 2.3127 * 100.0,
        0.0005,
        "share",
    );
    assert_eq!(chin["above_1_percent"].as_bool(), Some(true));
    assert_close(number(&entry("jaw_drop")["difference"]), -0.01, 1e-9, "jaw");
    assert_eq!(entry("jaw_drop")["above_1_percent"].as_bool(), Some(false));
    assert_eq!(entry("skull_radius")["kind"].as_str(), Some("convention"));

    // The measured proportions are summed up apart from the conventions, by what they are.
    let summary = |name: &str| {
        found["summary"][name]
            .as_table()
            .expect("a summary")
            .clone()
    };
    assert_eq!(summary("depths")["above_1_percent"].as_integer(), Some(1));
    assert_close(
        number(&summary("depths")["largest_percent_of_head_height"]),
        0.03 / 2.3127 * 100.0,
        0.0005,
        "the largest depth",
    );
    assert_eq!(summary("depths")["largest"].as_str(), Some("chin_depth"));
    assert_eq!(summary("heights")["above_1_percent"].as_integer(), Some(0));
    assert_eq!(summary("heights")["largest"].as_str(), Some("jaw_drop"));
    assert_close(
        number(&summary("widths")["largest_percent_of_head_height"]),
        0.0,
        1e-9,
        "the widths",
    );
    // The skull's radius is 0.1 of a unit larger: 4.3240%.
    assert_eq!(
        summary("conventions")["above_1_percent"].as_integer(),
        Some(1)
    );

    // A set that lacks some proportions is compared in the ones it has.
    let partial = text(&path).replace("chin_depth = 0.2175\n", "");
    std::fs::write(&path, partial).expect("written");
    let output = maquette_in(
        None,
        &[
            "compare-proportions",
            "--first",
            path.to_str().expect("text"),
            "--second",
            MADE_UP,
        ],
    );
    assert!(output.status.success());
    let found: toml::Table = String::from_utf8(output.stdout)
        .expect("text")
        .parse()
        .expect("TOML");
    assert_eq!(texts(&found["only_in_second"]), ["chin_depth"]);
    assert!(texts(&found["only_in_first"]).is_empty());
}

/// Writes a record's own positions as a reading of each view, over a made-up grey picture.
fn record_as_readings(folder: &Path, record: &str) {
    for (view, picture, width) in [
        ("front", "front.png", 1000),
        ("three-quarter", "turned.png", 1001),
    ] {
        support::made_up_image(&folder.join(picture), width, 1000);
        measure_ok(
            folder,
            &[
                "known-positions-as-reading",
                "--known-positions",
                record,
                "--view",
                view,
                "--image",
                picture,
                "--out",
                &format!("{view}.toml"),
            ],
        );
    }
}

#[test]
fn a_records_own_positions_written_as_a_reading_score_nothing_off() {
    let folder = references("known-positions-as-reading");
    record_as_readings(&folder, KNOWN);
    let written = text(&folder.join("front.toml"));
    assert!(written.contains("Not a reading"), "{written}");
    // What the hair covers in the made-up record is stated as not seen, with what covers it.
    assert!(
        written.contains("face-outline-left-top = \"covered in the drawing by head/hair_front\""),
        "{written}"
    );
    let found = scored(&folder, "front.toml", "front-score.toml");
    // The made-up record's front view gives 19 landmarks a reader is asked for, one of them
    // under the hair. The band of its one brow end is read as its middle.
    assert_eq!(found["summary"]["scored"].as_integer(), Some(18));
    assert_close(
        number(&found["summary"]["worst_percent_of_head_height"]),
        0.0,
        1e-9,
        "the worst",
    );
    assert!(entries(&found, "placed_though_covered").is_empty());
    assert_eq!(
        texts(&found["not_seen"]["and_covered_in_the_drawing"]),
        ["face-outline-left-top"]
    );
}

#[test]
fn saffis_record_under_a_readers_names_gives_back_her_turn_and_her_heights() {
    // Saffi's hand-made views are on record as numbers, so the measuring commands can be run
    // on them with no drawing and no reader: this is what they make of exact positions. Her
    // S2 underdrawings alone are used, because those are the drawings a reader is shown.
    let record = "data/saffi/hand-made-views.toml";
    let folder = references("known-positions-saffi");
    record_as_readings(&folder, record);
    let read = |command: &str, out: &str| -> toml::Table {
        measure_ok(
            &folder,
            &[
                command,
                "--front",
                "front.toml",
                "--three-quarter",
                "three-quarter.toml",
                "--out",
                out,
            ],
        );
        text(&folder.join(out)).parse().expect("TOML")
    };

    // The record gives the turn as 28.96 degrees over 18 pairs under its own names, 27.85
    // over the 15 with no covered point, and calls it known to about a degree. Under a
    // reader's names fewer pairs are counted, and the turn must still be that turn.
    let recorded: toml::Table = text(&support::repository().join("data/saffi/views-compared.toml"))
        .parse()
        .expect("TOML");
    let turn = read("turn-from-readings", "turn.toml");
    let degrees = number(&turn["turn"]["degrees"]);
    assert!(
        (degrees - number(&recorded["turn"]["degrees"])).abs() < 1.0
            && (degrees - number(&recorded["turn"]["uncovered_degrees"])).abs() < 1.0,
        "the turn is {degrees}"
    );

    // Her S2 proportions, as the head model takes them from the record at the pinned turn.
    let s2 = folder.join("s2-proportions.toml");
    maquette_ok(&[
        "proportions-from-hand-made-views",
        "--record",
        record,
        "--out",
        s2.to_str().expect("text"),
    ]);
    read("proportions-from-readings", "proportions.toml");
    let printed = maquette_ok(&[
        "compare-proportions",
        "--first",
        folder.join("proportions.toml").to_str().expect("text"),
        "--second",
        s2.to_str().expect("text"),
    ]);
    let compared: toml::Table = printed.parse().expect("TOML");
    // A height does not depend on the turn, so every height both give is the same.
    let heights = &compared["summary"]["heights"];
    assert!(heights["compared"].as_integer().expect("a count") >= 20);
    assert_close(
        number(&heights["largest_percent_of_head_height"]),
        0.0,
        1e-9,
        "the heights",
    );
    // A width is taken through the cosine of the turn and a depth through its sine, so the
    // degree between the two turns shows there, and a depth carries about twice as much.
    assert!(number(&compared["summary"]["widths"]["largest_percent_of_head_height"]) < 0.5);
    assert!(number(&compared["summary"]["depths"]["largest_percent_of_head_height"]) < 1.5);
    // What a reader cannot be asked for is not given: the skull, which is not read and
    // which the rule cannot build because the hair covers both temples of her front view.
    let only_recorded = texts(&compared["only_in_second"]);
    for name in [
        "skull_radius",
        "skull_centre_rise",
        "skull_centre_depth",
        "face_top_half_width",
    ] {
        assert!(
            only_recorded.contains(&name.to_string()),
            "{only_recorded:?}"
        );
    }
    assert!(texts(&compared["only_in_first"]).is_empty());

    // The fit is reported, and it is not zero: her two drawings are not one head exactly.
    let fit = read("fit-of-views", "fit.toml");
    let percent = number(&fit["fit"]["percent_of_head_height"]);
    assert!(percent > 0.0 && percent < 3.0, "the fit is {percent}");
}
