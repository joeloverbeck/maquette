//! The reference check sheet, through the command line: a sheet description and readings in
//! a reference folder in, one SVG out.
//!
//! Everything here is made up: the pictures are plain grey and show no drawing, and the
//! readings are views of the head invented in `tests/support/made_up_head.rs`.

mod support;

use std::path::{Path, PathBuf};

use base64::Engine;
use support::made_up_head::{MadeUp, seen};
use support::{failure, made_up_image, maquette_in, measure_ok, scratch, text};

fn digest(path: &Path) -> String {
    let output = std::process::Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("sha256sum runs");
    String::from_utf8(output.stdout)
        .expect("text")
        .split_whitespace()
        .next()
        .expect("a digest")
        .to_string()
}

/// A made-up reading of a made-up picture in the folder.
fn reading(folder: &Path, mut made_up: MadeUp, picture: &str, name: &str) {
    made_up.picture = picture.to_string();
    made_up.sha256 = digest(&folder.join(picture));
    made_up.write(folder, name);
}

/// A made-up reference folder: two characters, the first read twice in a front and a
/// three-quarter view, the second read once from the front and set aside when turned.
fn folder(name: &str) -> PathBuf {
    let folder = scratch(name);
    std::fs::create_dir_all(folder.join("pictures")).expect("made");
    for (picture, width) in [
        ("one-front.png", 1000),
        ("one-turned.png", 1001),
        ("two-front.png", 1002),
        ("two-turned.png", 1003),
    ] {
        made_up_image(&folder.join("pictures").join(picture), width, 1000);
    }
    let front = seen(0.0, 500.0);
    // The second reader puts one mouth end 3 across and 4 down: 5 pixels, 1.042% of the head
    // height of 480, and does not place one ear.
    let mut second = front.clone();
    let at = second.get_mut("mouth-line-left-end").expect("in the head");
    *at = [at[0] + 3.0, at[1] + 4.0];
    second.remove("ear-right-top");
    reading(
        &folder,
        MadeUp::front("first", front.clone()),
        "pictures/one-front.png",
        "first/one-front.toml",
    );
    reading(
        &folder,
        MadeUp::front("second", second),
        "pictures/one-front.png",
        "second/one-front.toml",
    );
    for reader in ["first", "second"] {
        reading(
            &folder,
            MadeUp::turned(reader, seen(29.0, 470.0)),
            "pictures/one-turned.png",
            &format!("{reader}/one-turned.toml"),
        );
        let mut aside = MadeUp::turned(reader, seen(29.0, 470.0));
        aside.level = false;
        aside.set_aside = true;
        reading(
            &folder,
            aside,
            "pictures/two-turned.png",
            &format!("{reader}/two-turned.toml"),
        );
    }
    reading(
        &folder,
        MadeUp::front("first", front),
        "pictures/two-front.png",
        "first/two-front.toml",
    );
    std::fs::write(
        folder.join("sheet.toml"),
        r#"# A made-up sheet for a test.
note = "Made up for a test: the grey pictures show no drawing."
look = "No look is asked: a made-up sheet"

[[character]]
label = "Character 1"
readings = ["first/one-front.toml", "second/one-front.toml", "first/one-turned.toml", "second/one-turned.toml"]

[[character]]
label = "Character 2 <made up & escaped>"
readings = ["first/two-front.toml", "first/two-turned.toml", "second/two-turned.toml"]
"#,
    )
    .expect("the sheet description is written");
    folder
}

fn rendered(folder: &Path) -> String {
    measure_ok(
        folder,
        &[
            "reference-check-sheet",
            "--sheet",
            "sheet.toml",
            "--out",
            "sheets/sheet.svg",
        ],
    );
    text(&folder.join("sheets/sheet.svg"))
}

fn texts(document: &roxmltree::Document) -> Vec<String> {
    document
        .descendants()
        .filter(|node| node.has_tag_name("text"))
        .map(|node| {
            node.descendants()
                .filter_map(|part| part.text())
                .collect::<String>()
        })
        .collect()
}

#[test]
fn a_sheet_shows_each_characters_views_with_the_points_read_and_what_was_set_aside() {
    let folder = folder("reference-check-sheet");
    let svg = rendered(&folder);
    let document = roxmltree::Document::parse(&svg).expect("the sheet is well-formed SVG");
    let root = document.root_element();
    assert_eq!(root.attribute("width"), Some("3200"));
    let texts = texts(&document);
    let said = |part: &str| texts.iter().any(|text| text.contains(part));
    let all = texts.join("\n");

    assert!(said("Reference check sheet"), "{all}");
    assert!(said("No look is asked: a made-up sheet"), "{all}");
    assert!(said("2 characters, 4 drawings"), "{all}");
    assert!(said("Made up for a test"), "{all}");
    // The sheet says what its numbers are measured against.
    assert!(
        said("share of head height, from the hair's top to the chin"),
        "{all}"
    );

    // Each character's heading says what it is, from its readings.
    assert!(
        said("Character 1 · adult woman, human · a made-up work, a made-up designer"),
        "{all}"
    );
    assert!(said("Character 2 <made up & escaped>"), "{all}");

    // Character 1: two views read, and no side view.
    assert!(said("Front · level · straight on"), "{all}");
    assert!(
        said("Three-quarter · turned 29° toward the viewer's right · level"),
        "{all}"
    );
    assert!(said("No side view of this character was read"), "{all}");
    // The front view is the midpoint of its two readings, so its left mouth end stands 2
    // pixels lower than the three-quarter view shows it: 2 / 480 = 0.417% of head height,
    // which is the largest misfit.
    assert!(
        said("The views fit one head to within 0.42% of head height"),
        "{all}"
    );
    // The two front readings are 1.042% apart at one point, which is numbered and listed.
    assert!(
        said("Two readings agree to 1.04% of head height at worst; 1 point is more than 1% apart"),
        "{all}"
    );
    assert!(said("mouth-line-left-end") && said("1.04%"), "{all}");
    assert!(said("Placed by one reader only: ear-right-top"), "{all}");
    assert!(
        said("Two readings agree to 0% of head height at worst"),
        "{all}"
    );
    assert!(said("Source: a made-up source"), "{all}");
    // The reasons the readers gave stand under the drawing.
    assert!(said("made up: the view"), "{all}");
    assert!(said("made up: the level"), "{all}");
    assert!(said("made up: the turn"), "{all}");

    // Character 2: one view read once, the other set aside.
    assert!(said("One reading only"), "{all}");
    assert!(said("Set aside by the readers: not measured"), "{all}");
    assert!(said("made up: the head looks down"), "{all}");
    assert!(said("You can overrule this"), "{all}");
    assert!(said("One usable view, so nothing to fit"), "{all}");

    // The decision asked, and no look counted.
    assert!(said("is every head level and in the view stated"), "{all}");

    // Every drawing is inside the sheet, whole: the sheet names no file of the folder.
    let pictures: Vec<roxmltree::Node> = document
        .descendants()
        .filter(|node| node.has_tag_name("image"))
        .collect();
    assert_eq!(pictures.len(), 4);
    let embedded = pictures[0]
        .attribute("href")
        .expect("an href")
        .strip_prefix("data:image/png;base64,")
        .expect("an embedded PNG");
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(embedded)
        .expect("base64");
    assert_eq!(
        bytes,
        std::fs::read(folder.join("pictures/one-front.png")).expect("the picture")
    );
    assert!(
        !svg.contains(folder.to_str().expect("text")),
        "the sheet says where the reference folder is"
    );

    // The points stand in the picture's own pixels, inside the drawing's own frame.
    let frame = pictures[0].parent().expect("the drawing's frame");
    let dots: Vec<(f64, f64, &str)> = frame
        .descendants()
        .filter(|node| node.has_tag_name("circle"))
        .map(|node| {
            let number = |name: &str| -> f64 {
                node.attribute(name)
                    .expect("a number")
                    .parse()
                    .expect("a number")
            };
            (
                number("cx"),
                number("cy"),
                node.attribute("class").unwrap_or(""),
            )
        })
        .collect();
    // 22 landmarks in the made-up head: 20 the readings agree on, one they put more than 1%
    // apart, which is drawn at both places, and one only the first places.
    let count = |class: &str| dots.iter().filter(|dot| dot.2 == class).count();
    assert_eq!(count("agreed"), 20, "{dots:?}");
    assert_eq!(count("apart"), 2, "{dots:?}");
    assert_eq!(count("by-one"), 1, "{dots:?}");
    // The chin of the made-up head is at 500, 600.
    assert!(
        dots.iter()
            .any(|dot| dot.0 == 500.0 && dot.1 == 600.0 && dot.2 == "agreed"),
        "{dots:?}"
    );
    // The mouth end the readers disagree on: at 528, 540 and at 531, 544.
    assert!(dots.contains(&(528.0, 540.0, "apart")), "{dots:?}");
    assert!(dots.contains(&(531.0, 544.0, "apart")), "{dots:?}");
    // No points are drawn on a drawing that is set aside.
    let aside = pictures[3].parent().expect("the drawing's frame");
    assert_eq!(
        aside
            .descendants()
            .filter(|node| node.has_tag_name("circle"))
            .count(),
        0
    );
}

#[test]
fn a_sheet_is_not_rendered_from_a_picture_that_has_changed_or_a_view_read_twice_over() {
    let folder = folder("reference-check-sheet-refused");
    // The same view of one character from two pictures.
    let description = text(&folder.join("sheet.toml"));
    std::fs::write(
        folder.join("twice.toml"),
        description.replace("\"first/two-turned.toml\"", "\"first/one-front.toml\""),
    )
    .expect("written");
    let output = maquette_in(
        Some(&folder),
        &[
            "reference-check-sheet",
            "--sheet",
            "twice.toml",
            "--out",
            "twice.svg",
        ],
    );
    let message = failure(&output);
    assert!(
        message.contains("two drawings") && message.contains("front"),
        "{message}"
    );

    // A picture that is no longer the one that was read.
    made_up_image(&folder.join("pictures/one-front.png"), 990, 1000);
    let output = maquette_in(
        Some(&folder),
        &[
            "reference-check-sheet",
            "--sheet",
            "sheet.toml",
            "--out",
            "changed.svg",
        ],
    );
    let message = failure(&output);
    assert!(
        message.contains("one-front.png") && message.contains("digest"),
        "{message}"
    );
    assert!(!folder.join("changed.svg").exists());
}
