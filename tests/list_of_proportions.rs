//! The list of proportions, as the program prints it.

mod support;

use support::maquette_ok;

struct Entry {
    name: String,
    kind: String,
    lines: Vec<String>,
}

/// The printed list, cut into one entry for each proportion.
fn printed_entries(printed: &str) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    for line in printed.lines() {
        if let Some((name, kind)) = line.split_once("  [")
            && !name.starts_with(' ')
            && kind.ends_with(']')
        {
            entries.push(Entry {
                name: name.to_string(),
                kind: kind.trim_end_matches(']').to_string(),
                lines: Vec::new(),
            });
            continue;
        }
        if let Some(entry) = entries.last_mut() {
            entry.lines.push(line.trim().to_string());
        }
    }
    entries
}

#[test]
fn every_proportion_has_a_name_a_mark_and_a_definition_for_each_view() {
    let printed = maquette_ok(&["list-proportions"]);
    let entries = printed_entries(&printed);
    assert!(
        entries.len() > 40,
        "expected the whole list, found {} proportions",
        entries.len()
    );

    let mut names = std::collections::BTreeSet::new();
    for entry in &entries {
        assert!(
            names.insert(entry.name.clone()),
            "{} is listed twice",
            entry.name
        );
        assert!(
            entry.kind == "measured" || entry.kind == "convention",
            "{} is marked `{}`, neither measured nor convention",
            entry.name,
            entry.kind
        );
        for label in ["What: ", "Front view: ", "Three-quarter view: "] {
            let line = entry
                .lines
                .iter()
                .find(|line| line.starts_with(label))
                .unwrap_or_else(|| panic!("{} has no line `{label}`", entry.name));
            assert!(
                line.len() > label.len() + 8,
                "{}: `{label}` says too little",
                entry.name
            );
        }
    }
}

#[test]
fn the_skull_is_a_convention_and_drawn_features_are_measured() {
    let printed = maquette_ok(&["list-proportions"]);
    let entries = printed_entries(&printed);
    let kind = |name: &str| -> &str {
        &entries
            .iter()
            .find(|entry| entry.name == name)
            .unwrap_or_else(|| panic!("{name} is not in the list"))
            .kind
    };
    // The skull lies under the hair: nothing is drawn behind it.
    assert_eq!(kind("skull_radius"), "convention");
    assert_eq!(kind("skull_centre_rise"), "convention");
    assert_eq!(kind("skull_centre_depth"), "convention");
    // An eye corner and the chin are drawn, and depth is part of the list.
    assert_eq!(kind("eye_outer_corner_offset"), "measured");
    assert_eq!(kind("chin_depth"), "measured");
    assert_eq!(kind("nose_mark_bottom_depth"), "measured");
}

#[test]
fn the_unit_the_eye_line_and_the_yardstick_are_stated_with_their_reasons() {
    let printed = maquette_ok(&["list-proportions"]);
    assert!(printed.contains("Unit: eye line to chin"), "{printed}");
    assert!(printed.contains("Eye line: "), "{printed}");
    assert!(printed.contains("Head height: "), "{printed}");
    assert!(printed.contains("Depth: "), "{printed}");
    let unit_section = printed
        .split("Eye line: ")
        .next()
        .expect("text before the eye line");
    assert!(
        unit_section.contains("Why: "),
        "the unit's reason is missing"
    );
    assert!(
        unit_section.contains("hair"),
        "the unit's reason does not mention the hair"
    );
}
