//! The table built from both batches of references, and the heads drawn for Jon's looks:
//! what the committed files say of themselves, and that they agree with each other.
//!
//! The table was built from readings in the reference folder, which no clean clone has, so
//! nothing here builds it again. These tests hold the table and Saffi's set against each
//! other, and run on a drawn head the checks every armature passes. They pin no bytes:
//! neither the table nor a head drawn from it is accepted.

mod support;

use std::collections::BTreeMap;

use support::{
    LandmarkFile, assert_close, draw, maquette_ok, number, proportions, repository, text,
};

const TABLE: &str = "tables/women-and-teenage-girls/table.toml";
const HEAD: &str = "design/look-8-both-batches/head-from-the-table.toml";
const SAFFIS: &str = "data/saffi/proportions.toml";

fn toml_of(path: &str) -> toml::Table {
    text(&repository().join(path))
        .parse()
        .unwrap_or_else(|error| panic!("{path}: {error}"))
}

/// The table's entries, by the proportion's name.
fn entries(table: &toml::Table) -> BTreeMap<String, toml::Table> {
    table["proportion"]
        .as_array()
        .expect("the table lists its proportions")
        .iter()
        .map(|entry| {
            let entry = entry.as_table().expect("an entry is a table").clone();
            (entry["name"].as_str().expect("a name").to_string(), entry)
        })
        .collect()
}

#[test]
fn the_table_states_a_number_only_where_five_characters_give_a_range() {
    let table = toml_of(TABLE);
    let stated = table["table"].as_table().expect("a [table]");
    let characters = table["character"]
        .as_array()
        .expect("the table lists its characters");
    assert_eq!(
        stated["characters"].as_integer(),
        Some(characters.len() as i64)
    );
    let entries = entries(&table);
    let listed = maquette_ok(&["list-proportions"])
        .lines()
        .filter(|line| line.ends_with("  [measured]") || line.ends_with("  [convention]"))
        .count();
    assert_eq!(entries.len(), listed);
    let mut counted: BTreeMap<&str, i64> = BTreeMap::new();
    for (name, entry) in &entries {
        let standing = entry["standing"].as_str().expect("a standing");
        *counted.entry(standing).or_default() += 1;
        let behind = entry["characters"]
            .as_integer()
            .expect("a count of characters");
        let works = entry["works"].as_integer().expect("a count of works");
        let most = entry["most_from_one_work"].as_integer().expect("a count");
        assert!(works <= behind && most <= behind, "{name}");
        assert!(behind == 0 || (works >= 1 && most >= 1), "{name}");
        let numbered = ["lowest", "middle", "highest"].map(|key| entry.contains_key(key));
        if standing == "given" {
            assert!(behind >= 5, "{name} is given with {behind} characters");
            assert_eq!(numbered, [true; 3], "{name} is given and lacks a value");
            let [lowest, middle, highest] =
                ["lowest", "middle", "highest"].map(|key| number(&entry[key]));
            assert!(lowest <= middle && middle <= highest, "{name}");
        } else {
            assert!(behind < 5 || standing == "convention", "{name}");
            assert_eq!(
                numbered, [false; 3],
                "{name} is {standing} and states a number"
            );
        }
        // A band's own range is stated only where five of its characters give it.
        for band in entry
            .get("bands")
            .and_then(toml::Value::as_array)
            .into_iter()
            .flatten()
        {
            let of = band["characters"].as_integer().expect("a count");
            assert_eq!(
                band.as_table().expect("a band").contains_key("lowest"),
                of >= 5 && standing != "convention",
                "{name}: {band:?}"
            );
        }
    }
    for (standing, key) in [
        ("given", "given"),
        ("too thin", "too_thin"),
        ("no character", "no_character"),
        ("convention", "conventions"),
    ] {
        assert_eq!(
            stated[key].as_integer(),
            Some(counted.get(standing).copied().unwrap_or(0)),
            "{standing}"
        );
    }

    // What the ticket asks the table to say of itself.
    let rules = table["rules"]
        .as_table()
        .expect("the table states its rules");
    for (rule, says) in [
        ("separate_ranges", "does not rule on which combinations"),
        ("single_references", "each one reference's number"),
        ("a_character", "A table counts characters"),
        ("works", "a range resting on one hand can be seen"),
        ("age_bands", "each band's own count of characters"),
    ] {
        assert!(
            rules[rule].as_str().is_some_and(|text| text.contains(says)),
            "{rule}"
        );
    }
}

#[test]
fn the_table_lists_its_characters_by_work_and_designer_and_names_nothing_else() {
    let table = toml_of(TABLE);
    let characters = table["character"]
        .as_array()
        .expect("the table lists its characters");
    for character in characters {
        let character = character.as_table().expect("a character is a table");
        // Its label, its work and designer as its readings name them, its band, sex and
        // species, and the views it gives: no picture's address, no reader's account, and
        // nothing tied to a number.
        let keys: Vec<&str> = character.keys().map(String::as_str).collect();
        assert_eq!(
            keys,
            [
                "age_band",
                "depth_from",
                "designer",
                "label",
                "sex",
                "species",
                "views",
                "work"
            ]
        );
        assert!(
            ["adult", "teenage"].contains(&character["age_band"].as_str().expect("a band")),
            "{character:?}"
        );
    }
    let file = text(&repository().join(TABLE));
    for private in [
        "http",
        ".png",
        ".jpg",
        ".webp",
        ".reading.toml",
        "jons-check.toml",
        "first-reader",
    ] {
        assert!(!file.contains(private), "the table holds `{private}`");
    }
}

#[test]
fn saffis_numbers_are_at_no_end_of_a_range_of_the_table() {
    let printed = maquette_ok(&[
        "set-against-table",
        "--proportions",
        SAFFIS,
        "--table",
        TABLE,
    ]);
    let against: toml::Table = printed.parse().expect("the comparison is TOML");
    let summary = against["summary"].as_table().expect("a summary");
    let count = |key: &str| summary[key].as_integer().expect("a count");
    assert_eq!(
        count("inside") + count("outside") + count("no_range"),
        count("set_against")
    );
    // Her numbers are in no range's ends: the table was not built on her.
    let table = entries(&toml_of(TABLE));
    let hers = proportions(&repository().join(SAFFIS));
    for (name, entry) in &table {
        for key in ["lowest", "middle", "highest"] {
            if let (Some(value), Some(her)) = (entry.get(key), hers.get(name)) {
                assert!(
                    (number(value) - her).abs() > 1e-6,
                    "the {key} value of {name} is Saffi's own number"
                );
            }
        }
    }
}

#[test]
fn the_head_takes_the_tables_middles_and_borrows_saffis_numbers_as_it_says() {
    let head = toml_of(HEAD);
    let table = entries(&toml_of(TABLE));
    let hers = proportions(&repository().join(SAFFIS));
    let values = proportions(&repository().join(HEAD));
    let sources = head["sources"]
        .as_table()
        .expect("the head says its sources");
    assert_eq!(sources.len(), table.len());
    let mut counted: BTreeMap<&str, i64> = BTreeMap::new();
    for (name, said) in sources {
        let said = said.as_str().expect("a source is text");
        let entry = &table[name];
        let behind = entry["characters"].as_integer().expect("a count");
        let kind = said.split(':').next().expect("a kind");
        *counted.entry(kind).or_default() += 1;
        match kind {
            "the table" => {
                assert_eq!(entry["standing"].as_str(), Some("given"), "{name}");
                assert_eq!(
                    said,
                    format!("the table: the middle of {behind} characters"),
                    "{name}"
                );
                assert_close(values[name], number(&entry["middle"]), 1e-9, name);
            }
            "a thin average" => {
                assert_eq!(entry["standing"].as_str(), Some("too thin"), "{name}");
                assert!((2..=4).contains(&behind), "{name}: {behind} characters");
                assert_eq!(
                    said,
                    format!("a thin average: of {behind} characters"),
                    "{name}"
                );
            }
            "borrowed" => {
                // An ear is taken whole, so even a proportion of it that has a range is
                // borrowed with the rest of it.
                if !name.starts_with("ear_") {
                    assert_ne!(entry["standing"].as_str(), Some("given"), "{name}");
                }
                if name.starts_with("jaw_") && !name.starts_with("jaw_line_") {
                    // The jaw point hangs on the chin: its depth is her offset from her own
                    // chin, placed from this head's chin.
                    let expected = match name.as_str() {
                        "jaw_depth" => {
                            values["chin_depth"] + hers["jaw_depth"] - hers["chin_depth"]
                        }
                        _ => hers[name],
                    };
                    assert_close(values[name], expected, 1e-6, name);
                } else {
                    assert_close(values[name], hers[name], 1e-9, name);
                }
            }
            "left out" => {
                // A proportion a set may leave out, which too few characters give and her set
                // does not have: the head is drawn without it.
                assert!(!values.contains_key(name), "{name}");
                assert!(!hers.contains_key(name), "{name}");
                assert!(behind < 2, "{name}: {behind} characters");
            }
            other => panic!("{name}: an unknown source `{other}`"),
        }
    }
    let stated = head["source"].as_table().expect("a [source]");
    for (key, kind) in [
        ("from_the_table", "the table"),
        ("thin_averages", "a thin average"),
        ("borrowed", "borrowed"),
        ("left_out", "left out"),
    ] {
        assert_eq!(
            stated[key].as_integer(),
            Some(counted.get(kind).copied().unwrap_or(0)),
            "{key}"
        );
    }
    assert_eq!(stated["borrowed_from"].as_str(), Some(SAFFIS));
    // An ear is taken whole, and the head carries no correction.
    for (name, said) in sources {
        if name.starts_with("ear_") {
            assert!(
                said.as_str()
                    .is_some_and(|said| said.starts_with("borrowed")),
                "{name}"
            );
        }
    }
    assert!(!head.contains_key("corrections"));
}

/// The drawn head's three landmark files.
fn drawn(name: &str) -> [LandmarkFile; 3] {
    let folder = draw(name, HEAD, &[]);
    ["front", "three-quarter", "side"]
        .map(|view| LandmarkFile::read(&folder.join(format!("{view}.landmarks.toml"))))
}

#[test]
fn the_drawn_head_has_every_landmark_at_one_height_in_all_three_views() {
    let [front, three_quarter, side] = drawn("first-table-head-heights");
    assert!(front.landmarks.len() > 60);
    for (name, at) in &front.landmarks {
        for (view, file) in [("three-quarter", &three_quarter), ("side", &side)] {
            let other = file
                .landmarks
                .get(name)
                .unwrap_or_else(|| panic!("the {view} view has no {name}"));
            assert_close(other.y, at.y, 0.0011, &format!("{name} in the {view} view"));
        }
    }
}

#[test]
fn the_drawn_heads_sideways_positions_are_the_ones_its_front_view_and_depths_give() {
    let [front, three_quarter, side] = drawn("first-table-head-sideways");
    // No correction: the side view is a flat projection, and says so.
    assert!(!side.table.contains_key("corrections"));
    for (view, file, turn_degrees) in [
        ("three-quarter", &three_quarter, 29.0_f64),
        ("side", &side, 90.0),
    ] {
        let (sin, cos) = turn_degrees.to_radians().sin_cos();
        let axis_depth = file.frame("turn_axis_depth");
        for (name, at) in &front.landmarks {
            let offset = at.x - front.frame("turn_axis_x");
            let depth = at.depth.expect("a landmark has a depth");
            let expected = file.frame("turn_axis_x") + offset * cos + (depth - axis_depth) * sin;
            let shown = &file.landmarks[name];
            assert!(
                shown.drawn_at_depth.is_none(),
                "{name} is moved in the {view} view"
            );
            assert_close(
                shown.x,
                expected,
                0.002,
                &format!("{name} in the {view} view"),
            );
        }
    }
}
