//! The table built from both batches of references, and the heads drawn for Jon's looks:
//! what the committed files say of themselves, and that they agree with each other.
//!
//! The table was built from readings in the reference folder, which no clean clone has, so
//! nothing here builds it again. These tests hold the table and Saffi's set against each
//! other, and run on a drawn head the checks every armature passes. They pin no bytes: Jon
//! accepted the table at look 8, but it cannot be built again here, so its digest stands in
//! its README; and a head drawn from it is no character's armature.

mod support;

use std::collections::BTreeMap;

use support::{
    LandmarkFile, assert_close, draw, maquette_ok, number, proportions, repository, sha256_of, text,
};

const TABLE: &str = "tables/women-and-teenage-girls/table.toml";
const TABLES_README: &str = "tables/women-and-teenage-girls/README.md";
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
    // Jon accepted a front view of 20 characters across his two checks, as the record of look
    // 7 on #18 counts them: 6 of the first batch and 14 of the second. Each gives the table.
    assert_eq!(stated["characters"].as_integer(), Some(20));
    assert_eq!(characters.len(), 20);
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

/// A count as the README writes it in words, as in "six teenage characters".
fn in_words(count: i64) -> String {
    let words = [
        "none", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve",
    ];
    usize::try_from(count)
        .ok()
        .and_then(|index| words.get(index))
        .map_or_else(|| count.to_string(), |word| (*word).to_string())
}

/// What a `[[view]]` of the table states, by the view's name: characters, works and most from
/// one work.
fn view_counts(table: &toml::Table, view: &str) -> [i64; 3] {
    let found = table["view"]
        .as_array()
        .expect("the table lists its views")
        .iter()
        .find(|entry| entry["view"].as_str() == Some(view))
        .unwrap_or_else(|| panic!("the table has no view `{view}`"));
    ["characters", "works", "most_from_one_work"]
        .map(|key| found[key].as_integer().expect("a count"))
}

#[test]
fn the_tables_readme_states_the_tables_own_numbers() {
    // Every number the README states about the table, set against the table and against what
    // `set-against-table` prints. A count written by hand drifts from the file it counts: the
    // README once gave the teenage characters a work too many. A number added to the README
    // gets its line here.
    let readme = text(&repository().join(TABLES_README));
    let table = toml_of(TABLE);
    let entries = entries(&table);
    let stated = table["table"].as_table().expect("a [table]");
    let count = |key: &str| stated[key].as_integer().expect("a count");
    let says = |sentence: String| {
        assert!(
            readme.contains(&sentence),
            "the README does not say \"{sentence}\""
        );
    };

    // The digest it gives is the accepted table's.
    says(format!(
        "its SHA-256 digest is `{}`",
        sha256_of(&repository().join(TABLE))
    ));

    says(format!(
        "{} characters of {} works, one drawing of each in each view",
        count("characters"),
        count("works")
    ));
    let [front, front_works, front_most] = view_counts(&table, "front");
    says(format!(
        "a front view of all {front} characters, of {front_works} works, {front_most} of them from one work"
    ));
    let [side, side_works, side_most] = view_counts(&table, "side");
    says(format!(
        "A side view of {side}, of {side_works} works, {side_most} of them from one"
    ));
    let [turned, _, _] = view_counts(
        &table,
        "depth, from a front and a three-quarter view of one character",
    );
    says(format!("Three-quarter views of {turned} give depth"));
    says(format!(
        "{} of the {} proportions with a valid range. {} are too thin",
        count("given"),
        count("proportions"),
        count("too_thin")
    ));
    says(format!(
        "{} are given by none; the skull's {} are conventions",
        count("no_character"),
        count("conventions")
    ));

    // The ranges that rest mostly on one work, each named with its count.
    let mostly_one = entries
        .values()
        .filter(|entry| entry["standing"].as_str() == Some("given"))
        .filter(|entry| {
            2 * entry["most_from_one_work"].as_integer().expect("a count")
                > entry["characters"].as_integer().expect("a count")
        })
        .count();
    says(format!(
        "{} of the {} have more than half their characters from one work",
        in_words(i64::try_from(mostly_one).expect("a count")),
        count("given")
    ));
    let mut named = 0;
    for (at, _) in readme.match_indices("` (") {
        let name = readme[..at].rsplit('`').next().expect("a name");
        let rest = &readme[at + 3..];
        let Some((most, behind)) = rest.split(')').next().and_then(|n| n.split_once(" of ")) else {
            continue;
        };
        let entry = &entries[name];
        assert_eq!(
            [most, behind].map(|n| n.parse::<i64>().expect("a count")),
            ["most_from_one_work", "characters"]
                .map(|key| entry[key].as_integer().expect("a count")),
            "{name}"
        );
        named += 1;
    }
    assert_eq!(named, mostly_one, "the README names each of them");

    // The age bands.
    let bands = stated["bands"].as_array().expect("the bands");
    let band = |name: &str| {
        bands
            .iter()
            .find(|band| band["band"].as_str() == Some(name))
            .and_then(|band| band["characters"].as_integer())
            .expect("a band's count")
    };
    says(format!(
        "{} adult characters and {} teenage ones",
        band("adult"),
        band("teenage")
    ));
    let both_ranged = entries
        .values()
        .filter(|entry| entry.contains_key("bands_overlap"))
        .count();
    let overlapping = entries
        .values()
        .filter(|entry| entry.get("bands_overlap").and_then(toml::Value::as_bool) == Some(true))
        .count();
    says(format!(
        "a range of its own in {both_ranged} proportions, and in all {overlapping} the two ranges overlap"
    ));
    let teenage_works: std::collections::BTreeSet<&str> = table["character"]
        .as_array()
        .expect("the characters")
        .iter()
        .filter(|character| character["age_band"].as_str() == Some("teenage"))
        .map(|character| character["work"].as_str().expect("a work"))
        .collect();
    says(format!(
        "{} teenage characters of {} works",
        in_words(band("teenage")),
        in_words(i64::try_from(teenage_works.len()).expect("a count"))
    ));

    // The profile's table: each row's proportions, a `*` standing for the rest of a name.
    let mut rows = 0;
    for row in readme.lines().filter(|line| line.starts_with("| `")) {
        let cells: Vec<&str> = row.split('|').map(str::trim).collect();
        let (names, behind, standing) = (cells[1], cells[2], cells[3]);
        let (characters, works) = match behind.split_once(", of ") {
            Some((characters, works)) => (characters, works.strip_suffix(" works")),
            None => (behind, None),
        };
        for pattern in names.split(", ").map(|name| name.trim_matches('`')) {
            let matching: Vec<&toml::Table> = entries
                .iter()
                .filter(|(name, _)| match pattern.strip_suffix('*') {
                    Some(start) => name.starts_with(start),
                    None => name.as_str() == pattern,
                })
                .map(|(_, entry)| entry)
                .collect();
            assert!(!matching.is_empty(), "no proportion is `{pattern}`");
            for entry in matching {
                assert_eq!(
                    entry["characters"].as_integer(),
                    characters.parse().ok(),
                    "{pattern}"
                );
                assert_eq!(entry["standing"].as_str(), Some(standing), "{pattern}");
                if let Some(works) = works {
                    assert_eq!(entry["works"].as_integer(), works.parse().ok(), "{pattern}");
                }
            }
        }
        rows += 1;
    }
    assert!(rows > 0, "the README has the profile's table");

    // Saffi against it.
    let printed = maquette_ok(&[
        "set-against-table",
        "--proportions",
        SAFFIS,
        "--table",
        TABLE,
    ]);
    let against: toml::Table = printed.parse().expect("the comparison is TOML");
    let summary = against["summary"].as_table().expect("a summary");
    let of = |key: &str| summary[key].as_integer().expect("a count");
    says(format!(
        "inside {} ranges and outside {}; {} have no range to be set against, and she gives none of the {} proportions of the profile",
        of("inside"),
        of("outside"),
        of("no_range"),
        of("not_in_the_set")
    ));
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
