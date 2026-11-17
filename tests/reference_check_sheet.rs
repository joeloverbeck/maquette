//! The reference check sheet, through the command line: a sheet description and readings in
//! a reference folder in, one SVG out.
//!
//! Everything here is made up: the pictures are plain grey and show no drawing, and the
//! readings are views of the head invented in `tests/support/made_up_head.rs`.

mod support;

use std::path::{Path, PathBuf};

use base64::Engine;
use support::made_up_head::{MadeUp, seen};
use support::{failure, made_up_image, maquette_in, measure_ok, references, text};

/// A made-up reading of a made-up picture in the folder.
fn reading(folder: &Path, mut made_up: MadeUp, picture: &str, name: &str) {
    made_up.picture = picture.to_string();
    made_up.write(folder, name);
}

/// A made-up reference folder: two characters, the first read twice in a front and a
/// three-quarter view, the second read once from the front and set aside when turned.
fn folder(name: &str) -> PathBuf {
    let folder = references(name);
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
    // The second reader puts the nose mark's bottom 3 across and 4 down: 5 pixels, 1.042% of the head
    // height of 480, and does not place one ear.
    let mut second = front.clone();
    let at = second.get_mut("nose-mark-bottom").expect("in the head");
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
                .filter(|part| part.is_text())
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
    // The front view is the midpoint of its two readings, so the bottom of its nose mark stands 2
    // pixels lower than the three-quarter view shows it: 2 / 480 = 0.417% of head height,
    // which is the largest misfit.
    assert!(
        said("The views fit one head to within 0.42% of head height"),
        "{all}"
    );
    // The two front readings are 1.042% apart at one point, which is numbered and listed.
    assert!(
        said("Two readings agree to 1.04% of head height at worst; 1 point is over 1% apart"),
        "{all}"
    );
    assert!(said("nose-mark-bottom") && said("1.04%"), "{all}");
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
    // The nose mark's bottom, which the readers disagree on: at 500, 490 and at 503, 494.
    assert!(dots.contains(&(500.0, 490.0, "apart")), "{dots:?}");
    assert!(dots.contains(&(503.0, 494.0, "apart")), "{dots:?}");
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
fn a_drawing_published_as_a_jpeg_is_inside_the_sheet_as_the_jpeg_it_is() {
    let folder = references("reference-check-sheet-jpeg");
    support::made_up_jpeg(&folder.join("pictures/front.jpg"), 1000, 1000);
    reading(
        &folder,
        MadeUp::front("first", seen(0.0, 500.0)),
        "pictures/front.jpg",
        "first/front.toml",
    );
    std::fs::write(
        folder.join("sheet.toml"),
        r#"# A made-up sheet for a test.
note = "Made up for a test: the grey picture shows no drawing."
look = "No look is asked: a made-up sheet"

[[character]]
label = "Character 1"
readings = ["first/front.toml"]
"#,
    )
    .expect("the sheet description is written");

    let svg = rendered(&folder);
    let document = roxmltree::Document::parse(&svg).expect("the sheet is well-formed SVG");
    let pictures: Vec<roxmltree::Node> = document
        .descendants()
        .filter(|node| node.has_tag_name("image"))
        .collect();
    assert_eq!(pictures.len(), 1);
    let embedded = pictures[0]
        .attribute("href")
        .expect("an href")
        .strip_prefix("data:image/jpeg;base64,")
        .expect("a JPEG is embedded as a JPEG");
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(embedded)
        .expect("base64");
    assert_eq!(
        bytes,
        std::fs::read(folder.join("pictures/front.jpg")).expect("the picture"),
        "the sheet holds another picture than the one that was read"
    );
}

/// A made-up reference folder with one picture that holds two heads of one character: a
/// front view on the left, and on the right a side view at half its size. Each is read by
/// two readers.
fn folder_with_two_heads_in_one_picture(name: &str) -> PathBuf {
    let folder = references(name);
    made_up_image(&folder.join("pictures/two-heads.png"), 1000, 1000);
    let side: std::collections::BTreeMap<String, [f64; 2]> = seen(90.0, 700.0)
        .into_iter()
        .filter(|(name, _)| !name.contains("-left-"))
        .map(|(name, at)| (name, [350.0 + 0.5 * at[0], 100.0 + 0.5 * at[1]]))
        .collect();
    for reader in ["first", "second"] {
        reading(
            &folder,
            MadeUp::front(reader, seen(0.0, 300.0)),
            "pictures/two-heads.png",
            &format!("{reader}/left-head.toml"),
        );
        let mut made_up = MadeUp::turned(reader, side.clone());
        made_up.view = "side";
        made_up.stated_turn = 90.0;
        reading(
            &folder,
            made_up,
            "pictures/two-heads.png",
            &format!("{reader}/right-head.toml"),
        );
    }
    folder
}

#[test]
fn a_sheet_shows_two_heads_of_one_picture_as_two_drawings_and_says_which_each_is() {
    let folder = folder_with_two_heads_in_one_picture("reference-check-sheet-two-heads");
    std::fs::write(
        folder.join("sheet.toml"),
        r#"# A made-up sheet for a test.
note = "Made up for a test: the grey picture shows no drawing."
look = "No look is asked: a made-up sheet"

[[character]]
label = "work-1"
about = "a made-up character of a made-up work"

  [[character.drawing]]
  picture = "the left head of the made-up picture"
  readings = ["first/left-head.toml", "second/left-head.toml"]

  [[character.drawing]]
  picture = "the right head of the made-up picture"
  readings = ["first/right-head.toml", "second/right-head.toml"]
"#,
    )
    .expect("the sheet description is written");
    let svg = rendered(&folder);
    let document = roxmltree::Document::parse(&svg).expect("the sheet is well-formed SVG");
    let texts = texts(&document);
    let said = |part: &str| texts.iter().any(|text| text.contains(part));
    let all = texts.join("\n");

    // Two drawings of one picture: two cards, each holding the picture.
    assert!(said("1 character, 2 drawings"), "{all}");
    let pictures = document
        .descendants()
        .filter(|node| node.has_tag_name("image"))
        .count();
    assert_eq!(pictures, 2);

    // Each card says which character, work and picture it is, in words, and how high the
    // head is in the picture: 2.4 units of 200 pixels, and of 100 in the side view.
    assert!(
        said("work-1 · a made-up character of a made-up work"),
        "{all}"
    );
    assert!(
        said("the left head of the made-up picture · head 480 px high"),
        "{all}"
    );
    assert!(
        said("the right head of the made-up picture · head 240 px high"),
        "{all}"
    );
    // What each reader judged stands with its reasons.
    assert!(said("first judged a front view, level"), "{all}");
    assert!(
        said("second judged a side view facing the viewer's right, turned 90°, level"),
        "{all}"
    );

    // The header: what the numbers are set against, that a side view's reading is untested,
    // and each head's height.
    assert!(
        said("nothing on this sheet is set against a position that is known"),
        "{all}"
    );
    assert!(
        said(
            "The reading of a side view has never been tested on a drawing whose positions are known"
        ),
        "{all}"
    );
    assert!(said("work-1: front 480, side 240"), "{all}");
    assert!(said("about 480 pixels high"), "{all}");

    // The front and the side view stand beside each other: no three-quarter view was read
    // for anyone, so the sheet says that once and keeps no empty card for it.
    assert!(
        said("No three-quarter view was read for any character on this sheet"),
        "{all}"
    );
    assert!(
        !said("No three-quarter view of this character was read"),
        "{all}"
    );
    let cards: Vec<&str> = document
        .descendants()
        .filter(|node| node.has_tag_name("g"))
        .filter_map(|node| node.attribute("transform"))
        .collect();
    assert_eq!(cards.len(), 2, "{cards:?}");
    assert!(cards[0].starts_with("translate(80 "), "{cards:?}");
    assert!(cards[1].starts_with("translate(1100 "), "{cards:?}");

    // They are fitted to one head with no three-quarter view between them.
    assert!(
        said("The views fit one head to within 0% of head height"),
        "{all}"
    );

    // Named without its drawings, the four readings of one picture are taken as four
    // readings of one drawing, and the sheet says how to name them apart.
    std::fs::write(
        folder.join("sheet.toml"),
        r#"look = "No look is asked: a made-up sheet"

[[character]]
label = "work-1"
readings = ["first/left-head.toml", "second/left-head.toml", "first/right-head.toml", "second/right-head.toml"]
"#,
    )
    .expect("the sheet description is written");
    let output = maquette_in(
        Some(&folder),
        &[
            "reference-check-sheet",
            "--sheet",
            "sheet.toml",
            "--out",
            "sheets/refused.svg",
        ],
    );
    let message = failure(&output);
    assert!(message.contains("[[character.drawing]]"), "{message}");
    assert!(!folder.join("sheets/refused.svg").exists());
}

/// The dots of one drawing of a sheet, by class: `picture` counts the drawings from the top.
fn dots_of(document: &roxmltree::Document, picture: usize, class: &str) -> usize {
    let pictures: Vec<roxmltree::Node> = document
        .descendants()
        .filter(|node| node.has_tag_name("image"))
        .collect();
    pictures[picture]
        .parent()
        .expect("the drawing's frame")
        .descendants()
        .filter(|node| node.has_tag_name("circle") && node.attribute("class") == Some(class))
        .count()
}

#[test]
fn a_sheet_is_rendered_when_two_readings_cannot_be_compared_and_the_card_says_what_was_not() {
    let folder = folder("reference-check-sheet-not-compared");
    // The second reader of the first character's front view states the hair's top as not
    // seen: the two readings have no head height to state a disagreement in.
    let mut second = seen(0.0, 500.0);
    second.remove("hair-top");
    reading(
        &folder,
        MadeUp::front("second", second),
        "pictures/one-front.png",
        "second/one-front.toml",
    );
    let svg = rendered(&folder);
    let document = roxmltree::Document::parse(&svg).expect("the sheet is well-formed SVG");
    let texts = texts(&document);
    let said = |part: &str| texts.iter().any(|text| text.contains(part));
    let all = texts.join("\n");

    // The card says that the two were not compared, and why.
    assert!(said("The two readings could not be compared"), "{all}");
    assert!(
        texts
            .join(" ")
            .contains("places no hair top or no chin, so there is no head height"),
        "{all}"
    );
    // Its points are shown all the same: 21 both place, midway and not called agreed, and the
    // hair's top, which one places.
    assert_eq!(dots_of(&document, 0, "not-compared"), 21);
    assert_eq!(dots_of(&document, 0, "by-one"), 1);
    assert_eq!(dots_of(&document, 0, "agreed"), 0);
    assert_eq!(dots_of(&document, 0, "apart"), 0);

    // The rest of the sheet stands as it would: the other drawing's agreement, the other
    // character, and the fit, whose head height is then the three-quarter view's.
    assert!(
        said("Two readings agree to 0% of head height at worst"),
        "{all}"
    );
    assert!(
        said("The views fit one head to within 0% of head height"),
        "{all}"
    );
    assert!(said("Character 2 <made up & escaped>"), "{all}");
    assert!(said("2 characters, 4 drawings"), "{all}");
}

#[test]
fn a_sheet_says_what_is_not_two_readings_of_a_drawing_and_what_followed_other_instructions() {
    let folder = folder("reference-check-sheet-readings");
    // Both readings of the first character's front view carry one reader's label, and both
    // readings of its three-quarter view follow an earlier revision of the instructions.
    reading(
        &folder,
        MadeUp::front("first", seen(0.0, 500.0)),
        "pictures/one-front.png",
        "second/one-front.toml",
    );
    for reader in ["first", "second"] {
        let mut earlier = MadeUp::turned(reader, seen(29.0, 470.0));
        earlier.instructions = Some("made-up earlier revision".to_string());
        reading(
            &folder,
            earlier,
            "pictures/one-turned.png",
            &format!("{reader}/one-turned.toml"),
        );
    }
    let svg = rendered(&folder);
    let document = roxmltree::Document::parse(&svg).expect("the sheet is well-formed SVG");
    let texts = texts(&document);
    let said = |part: &str| texts.iter().any(|text| text.contains(part));
    let all = texts.join("\n");

    // The front view is not measured, and the card says why in full. It is not a judgement
    // of a reader's, so nothing offers to overrule it.
    assert!(
        said("Not the two readings of one drawing: not measured"),
        "{all}"
    );
    // A reason runs over several lines under the card.
    let running = texts.join(" ");
    assert!(
        running.contains("both carry the reader's label \"first\"")
            && running.contains("two separate readers")
            && running.contains("The reason is under the card"),
        "{all}"
    );
    assert_eq!(dots_of(&document, 0, "agreed"), 0);
    assert!(said("One usable view, so nothing to fit"), "{all}");
    // The second character's drawing that its readers set aside still offers it.
    assert_eq!(
        texts
            .iter()
            .filter(|text| text.contains("You can overrule this"))
            .count(),
        1,
        "{all}"
    );

    // The three-quarter view is measured, and the card says which instructions it followed.
    assert!(
        said("Two readings agree to 0% of head height at worst"),
        "{all}"
    );
    assert!(
        running.contains("was read under revision \"made-up earlier revision\"")
            && running.contains(&support::instructions_revision()),
        "{all}"
    );
}

#[test]
fn a_sheet_is_not_rendered_from_a_reading_named_twice_or_one_that_does_not_pass_its_check() {
    let folder = folder("reference-check-sheet-unchecked");
    let description = text(&folder.join("sheet.toml"));
    let refused = |name: &str, description: &str| -> String {
        std::fs::write(folder.join(name), description).expect("written");
        let output = maquette_in(
            Some(&folder),
            &[
                "reference-check-sheet",
                "--sheet",
                name,
                "--out",
                "refused.svg",
            ],
        );
        let message = failure(&output);
        assert!(!folder.join("refused.svg").exists());
        message
    };

    // One reading named twice for a character.
    let message = refused(
        "twice.toml",
        &description.replace("\"second/one-front.toml\"", "\"./first/one-front.toml\""),
    );
    assert!(
        message.contains("Character 1")
            && message.contains("first/one-front.toml")
            && message.contains("twice"),
        "{message}"
    );

    // A reading that leaves a landmark neither placed nor stated as not seen. The character
    // and the reading are named.
    let whole = text(&folder.join("first/two-front.toml"));
    let unfinished = whole.replace(
        "ear-left-base-outer = \"made up: not in the made-up head\"\n",
        "",
    );
    assert_ne!(unfinished, whole);
    std::fs::write(folder.join("first/two-front.toml"), unfinished).expect("written");
    let message = refused("unfinished.toml", &description);
    assert!(
        message.contains("Character 2")
            && message.contains("first/two-front.toml")
            && message.contains("ear-left-base-outer")
            && message.contains("check-reading"),
        "{message}"
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
