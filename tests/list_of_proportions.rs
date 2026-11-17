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

#[test]
fn the_list_defines_the_named_correction_its_view_and_each_amount_with_its_rule() {
    let printed = maquette_ok(&["list-proportions"]);
    let corrections: Vec<&str> = printed
        .lines()
        .skip_while(|line| !line.starts_with("Corrections, "))
        .skip(1)
        .take_while(|line| !line.is_empty())
        .collect();
    // One correction: it is named, and it says which view it changes and that no other is.
    let named: Vec<&&str> = corrections
        .iter()
        .filter(|line| line.starts_with("  ") && !line.starts_with("   "))
        .collect();
    assert_eq!(named.len(), 1, "{corrections:?}");
    let correction = named[0];
    assert!(
        correction.starts_with("  lower_face_set_back, in the side view: "),
        "{correction}"
    );
    assert!(
        correction.contains("front view") && correction.contains("three-quarter view"),
        "it does not say what it leaves alone: {correction}"
    );
    // It says why it exists and where its amounts come from, and each amount has a rule.
    for label in ["    Why: ", "    Where the amounts come from: "] {
        let line = corrections
            .iter()
            .find(|line| line.starts_with(label))
            .unwrap_or_else(|| panic!("the correction has no line `{label}`"));
        assert!(line.len() > label.len() + 40, "`{label}` says too little");
    }
    for (amount, about) in [("chin", "iris"), ("mouth", "nose mark")] {
        let label = format!("    Amount `{amount}`: ");
        let line = corrections
            .iter()
            .find(|line| line.starts_with(&label))
            .unwrap_or_else(|| panic!("the correction has no amount `{amount}`"));
        assert!(
            line.contains(about),
            "the rule of `{amount}` does not name the {about}: {line}"
        );
    }
    // A correction is not a proportion: the count of proportions does not include it.
    assert!(!printed.contains("\nlower_face_set_back  ["), "{printed}");
}

#[test]
fn the_list_states_how_each_guide_is_built_in_the_side_view() {
    let printed = maquette_ok(&["list-proportions"]);
    // Depth is what a side view shows sideways.
    let depth = printed
        .lines()
        .skip_while(|line| !line.starts_with("Depth: "))
        .take_while(|line| !line.is_empty())
        .find(|line| line.trim_start().starts_with("Side view: "))
        .expect("how depth shows in a side view");
    assert!(depth.contains("90"), "{depth}");

    // Every guide whose construction the side view changes says so: the ones a plain
    // projection would draw at no width, the ones named for a side, and the outlines.
    let constructions: Vec<&str> = printed
        .lines()
        .skip_while(|line| *line != "How the guides are built in a view:")
        .skip(1)
        .take_while(|line| !line.is_empty())
        .collect();
    for guide in [
        "skull",
        "hair-volume",
        "face-outline",
        "centre-line",
        "level lines",
        "eye opening, lid line and brow line",
        "iris",
        "nose-mark and mouth-line",
        "ears",
        "neck",
        "shoulder-line",
    ] {
        let rule = constructions
            .iter()
            .find(|line| line.trim_start().starts_with(&format!("{guide}: ")))
            .unwrap_or_else(|| panic!("the list states no construction for `{guide}`"));
        assert!(
            rule.contains("side view"),
            "the construction of `{guide}` does not say how the side view shows it: {rule}"
        );
    }
}
