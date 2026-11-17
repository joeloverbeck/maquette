//! The first table as it is put to Jon at look 6, and the head drawn from it: what the
//! committed files say of themselves, and that they agree with each other.
//!
//! The table was built from readings in the reference folder, which no clean clone has, so
//! nothing here builds it again. These tests hold the table, the head's set of proportions
//! and Saffi's own set against each other, and run on the drawn head the checks every
//! armature passes. They pin no bytes: neither the table nor the head is accepted.

mod support;

use std::collections::BTreeMap;

use support::{
    LandmarkFile, assert_close, draw, maquette_ok, number, proportions, repository, text,
};

const TABLE: &str = "tables/adult-women/table.toml";
const HEAD: &str = "design/look-6-first-table/saffi-with-table-values.toml";
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
fn the_table_states_a_number_only_where_five_works_give_a_range() {
    let table = toml_of(TABLE);
    let stated = table["table"].as_table().expect("a [table]");
    assert_eq!(stated["works"].as_integer(), Some(6));
    let entries = entries(&table);
    assert_eq!(entries.len(), 74);
    let mut counted: BTreeMap<&str, i64> = BTreeMap::new();
    for (name, entry) in &entries {
        let standing = entry["standing"].as_str().expect("a standing");
        *counted.entry(standing).or_default() += 1;
        let works = entry["works"].as_integer().expect("a count of works");
        let numbered = ["lowest", "middle", "highest"].map(|key| entry.contains_key(key));
        if standing == "given" {
            assert!(works >= 5, "{name} is given with {works} works");
            assert_eq!(numbered, [true; 3], "{name} is given and lacks a value");
            let [lowest, middle, highest] =
                ["lowest", "middle", "highest"].map(|key| number(&entry[key]));
            assert!(lowest <= middle && middle <= highest, "{name}");
        } else {
            assert!(works < 5 || standing == "convention", "{name}");
            assert_eq!(
                numbered, [false; 3],
                "{name} is {standing} and states a number"
            );
        }
    }
    for (standing, key) in [
        ("given", "given"),
        ("too thin", "too_thin"),
        ("no work", "no_work"),
        ("convention", "conventions"),
    ] {
        assert_eq!(
            stated[key].as_integer(),
            counted.get(standing).copied(),
            "{standing}"
        );
    }

    // What the ticket asks the table to say of itself.
    let rules = table["rules"]
        .as_table()
        .expect("the table states its rules");
    assert!(
        rules["separate_ranges"]
            .as_str()
            .is_some_and(|rule| rule.contains("does not rule on which combinations"))
    );
    assert!(
        rules["single_references"]
            .as_str()
            .is_some_and(|rule| rule.contains("each one reference's number"))
    );
    // Depth is not a table, and no three-quarter view was measured.
    for view in table["view"].as_array().expect("the views") {
        let (what, works) = (
            view["view"].as_str().expect("a view"),
            view["works"].as_integer().expect("a count"),
        );
        let standing = view["standing"].as_str().expect("a standing");
        assert_eq!(standing == "a table", what == "front", "{what}");
        assert_eq!(works >= 5, what == "front", "{what}");
    }
}

#[test]
fn the_table_lists_its_works_by_work_and_designer_and_names_nothing_else() {
    let table = toml_of(TABLE);
    let works = table["work"].as_array().expect("the table lists its works");
    assert_eq!(works.len(), 6);
    let mut designers = Vec::new();
    for work in works {
        let work = work.as_table().expect("a work is a table");
        // The work with its year, and its character designer: no picture's address, no
        // reader's account, and nothing tied to a number.
        let keys: Vec<&str> = work.keys().map(String::as_str).collect();
        assert_eq!(
            keys,
            [
                "age_band",
                "depth_from",
                "designer",
                "sex",
                "species",
                "views",
                "work"
            ]
        );
        assert!(
            work["work"]
                .as_str()
                .is_some_and(|work| work.contains("(TV anime, 20"))
        );
        assert_eq!(work["age_band"].as_str(), Some("adult"));
        assert_eq!(work["sex"].as_str(), Some("female"));
        designers.push(work["designer"].as_str().expect("a designer"));
    }
    designers.sort_unstable();
    designers.dedup();
    assert_eq!(designers.len(), 6, "six works by six character designers");
    let file = text(&repository().join(TABLE));
    for private in [
        "http",
        ".png",
        ".jpg",
        ".reading.toml",
        "jons-check",
        "first-reader",
    ] {
        assert!(!file.contains(private), "the table holds `{private}`");
    }
}

#[test]
fn saffi_is_inside_nineteen_ranges_outside_six_and_has_none_for_the_rest() {
    let printed = maquette_ok(&[
        "set-against-table",
        "--proportions",
        SAFFIS,
        "--table",
        TABLE,
    ]);
    let against: toml::Table = printed.parse().expect("the comparison is TOML");
    let summary = against["summary"].as_table().expect("a summary");
    assert_eq!(summary["set_against"].as_integer(), Some(74));
    assert_eq!(summary["inside"].as_integer(), Some(19));
    assert_eq!(summary["outside"].as_integer(), Some(6));
    assert_eq!(summary["no_range"].as_integer(), Some(49));
    // Her numbers are in no range's ends: the table was not built on her.
    let table = entries(&toml_of(TABLE));
    let hers = proportions(&repository().join(SAFFIS));
    for (name, entry) in &table {
        for key in ["lowest", "middle", "highest"] {
            if let Some(value) = entry.get(key) {
                assert!(
                    (number(value) - hers[name]).abs() > 1e-6,
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
    assert_eq!(sources.len(), 74);
    let mut counted = [0_i64; 3];
    for (name, said) in sources {
        let said = said.as_str().expect("a source is text");
        let entry = &table[name];
        let works = entry["works"].as_integer().expect("a count of works");
        if let Some(of) = said.strip_prefix("the table: ") {
            counted[0] += 1;
            assert_eq!(entry["standing"].as_str(), Some("given"), "{name}");
            assert_eq!(of, format!("the middle of {works} works"), "{name}");
            assert_close(values[name], number(&entry["middle"]), 1e-9, name);
        } else if let Some(of) = said.strip_prefix("a thin average: ") {
            counted[1] += 1;
            assert_eq!(entry["standing"].as_str(), Some("too thin"), "{name}");
            assert!((2..=4).contains(&works), "{name}: {works} works");
            assert_eq!(of, format!("of {works} works"), "{name}");
        } else {
            counted[2] += 1;
            assert!(said.starts_with("borrowed: "), "{name}: {said}");
            assert_ne!(entry["standing"].as_str(), Some("given"), "{name}");
            assert_close(values[name], hers[name], 1e-9, name);
        }
    }
    assert_eq!(counted, [25, 16, 33]);
    let stated = head["source"].as_table().expect("a [source]");
    assert_eq!(stated["from_the_table"].as_integer(), Some(25));
    assert_eq!(stated["thin_averages"].as_integer(), Some(16));
    assert_eq!(stated["borrowed"].as_integer(), Some(33));
    assert_eq!(stated["borrowed_from"].as_str(), Some(SAFFIS));
    // The ear, the neck and the jaw point are hers whole, and the head carries no correction.
    for (name, said) in sources {
        if ["ear_", "neck_", "jaw_"]
            .iter()
            .any(|form| name.starts_with(form))
        {
            assert!(
                said.as_str()
                    .is_some_and(|said| said.starts_with("borrowed"))
            );
        }
    }
    assert!(!head.contains_key("corrections"));
}

/// The drawn head's three landmark files.
fn drawn() -> [LandmarkFile; 3] {
    let folder = draw("first-table-head", HEAD, &[]);
    ["front", "three-quarter", "side"]
        .map(|view| LandmarkFile::read(&folder.join(format!("{view}.landmarks.toml"))))
}

#[test]
fn the_drawn_head_has_every_landmark_at_one_height_in_all_three_views() {
    let [front, three_quarter, side] = drawn();
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
    let [front, three_quarter, side] = drawn();
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
