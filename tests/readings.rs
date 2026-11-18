//! The record of a reading and the reference folder, through the command line.
//!
//! Every picture and every reading here is made up: the pictures are plain grey and show no
//! drawing, and the positions are invented by the test. A reference folder is a scratch
//! folder of the test's own, named to the program the way a real one is, by the environment.

mod support;

use std::path::{Path, PathBuf};

use support::{
    failure, filled_in, made_up_image, maquette_in, maquette_ok, measure_ok, path_text, references,
    scratch, text,
};

/// Every command that measures: each reads or writes something that belongs in the reference
/// folder. The arguments are well formed and name files that need not exist.
const MEASURING: &[&[&str]] = &[
    &[
        "start-reading",
        "--image",
        "front.png",
        "--view",
        "front",
        "--reader",
        "first",
        "--out",
        "front.toml",
    ],
    &["check-reading", "--reading", "front.toml"],
    &[
        "enlarge-image",
        "--image",
        "front.png",
        "--left",
        "0",
        "--top",
        "0",
        "--width",
        "10",
        "--height",
        "10",
        "--factor",
        "2",
        "--out",
        "front-enlarged.png",
    ],
    &[
        "agreement-of-readings",
        "--first",
        "a.toml",
        "--second",
        "b.toml",
    ],
    &[
        "turn-from-readings",
        "--front",
        "a.toml",
        "--three-quarter",
        "b.toml",
    ],
    &[
        "fit-of-views",
        "--front",
        "a.toml",
        "--three-quarter",
        "b.toml",
    ],
    &[
        "proportions-from-readings",
        "--front",
        "a.toml",
        "--three-quarter",
        "b.toml",
    ],
    &[
        "score-reading",
        "--reading",
        "a.toml",
        "--known-positions",
        "tests/fixtures/differences/made-up-hand-made-views.toml",
    ],
    &[
        "known-positions-as-reading",
        "--known-positions",
        "tests/fixtures/differences/made-up-hand-made-views.toml",
        "--view",
        "front",
        "--image",
        "front.png",
        "--out",
        "front.toml",
    ],
    &[
        "reference-check-sheet",
        "--sheet",
        "sheet.toml",
        "--out",
        "sheet.svg",
    ],
];

#[test]
fn with_no_reference_folder_each_measuring_command_says_what_is_missing_and_where_it_is_configured()
{
    let absent = references("readings-no-folder").join("not-there");
    for arguments in MEASURING {
        for (configured, what) in [
            (None, "is not configured"),
            (Some(absent.as_path()), "does not exist"),
        ] {
            let message = failure(&maquette_in(configured, arguments));
            let command = arguments[0];
            assert!(
                message.contains("the reference folder") && message.contains(what),
                "{command}: {message}"
            );
            assert!(
                message.contains("MAQUETTE_REFERENCES"),
                "{command} does not say where the folder is configured: {message}"
            );
            assert!(
                message.contains("nothing can be measured"),
                "{command}: {message}"
            );
        }
    }
}

#[test]
fn with_no_reference_folder_drawing_still_works() {
    let folder = scratch("readings-draw-without-folder");
    let output = maquette_in(
        None,
        &[
            "draw",
            "--proportions",
            support::MADE_UP,
            "--out",
            path_text(&folder),
        ],
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    for file in ["front.svg", "three-quarter.svg", "side.svg"] {
        assert!(folder.join(file).is_file(), "{file} was not written");
    }
    let list = maquette_in(None, &["list-proportions"]);
    assert!(list.status.success());
    let instructions = maquette_in(None, &["reading-instructions"]);
    assert!(instructions.status.success());
}

/// `--references` names the reference folder on the command line, so that a command can begin
/// with the program's own name. It wins over the environment variable.
#[test]
fn the_reference_folder_can_be_named_on_the_command_line_and_that_wins_over_the_environment() {
    let named = folder_with_picture("readings-named-on-the-command-line");
    let absent = references("readings-named-absent").join("not-there");
    for environment in [None, Some(absent.as_path())] {
        let out = format!("front-{}.toml", environment.is_some());
        let output = maquette_in(
            environment,
            &[
                "start-reading",
                "--references",
                path_text(&named),
                "--image",
                "picture.png",
                "--view",
                "front",
                "--reader",
                "first made-up reader",
                "--out",
                &out,
            ],
        );
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(named.join(&out).is_file(), "{out} was not written");
    }
    // It may stand anywhere among the options, as every option may. The reading was only
    // started, so its check finds it incomplete: it was found in the folder named.
    let checked = maquette_in(
        Some(&absent),
        &[
            "check-reading",
            "--reading",
            "front-false.toml",
            "--references",
            path_text(&named),
        ],
    );
    let said = String::from_utf8_lossy(&checked.stdout);
    assert!(
        said.contains("The reading is not complete"),
        "{said}{}",
        String::from_utf8_lossy(&checked.stderr)
    );
}

#[test]
fn a_reference_folder_named_on_the_command_line_is_held_to_what_the_environment_is_held_to() {
    let absent = references("readings-named-folder-absent").join("not-there");
    for arguments in MEASURING {
        let mut with_option = arguments.to_vec();
        with_option.extend_from_slice(&["--references", path_text(&absent)]);
        let message = failure(&maquette_in(None, &with_option));
        let command = arguments[0];
        assert!(
            message.contains("does not exist")
                && message.contains("--references")
                && message.contains("nothing can be measured"),
            "{command}: {message}"
        );
    }
    let missing_value = failure(&maquette_in(None, &["check-reading", "--references"]));
    assert!(
        missing_value.contains("`--references` needs a value"),
        "{missing_value}"
    );
    let inside = scratch("readings-named-folder-in-a-repository");
    let message = failure(&maquette_in(
        None,
        &[
            "check-reading",
            "--references",
            path_text(&inside),
            "--reading",
            "front.toml",
        ],
    ));
    assert!(
        message.contains("is inside the Git repository")
            && message.contains("--references")
            && message.contains("outside every repository"),
        "{message}"
    );
}

/// A made-up reference folder with one made-up picture in it.
fn folder_with_picture(name: &str) -> PathBuf {
    let folder = references(name);
    made_up_image(&folder.join("picture.png"), 800, 900);
    folder
}

fn start(folder: &Path, view: &str, more: &[&str], out: &str) -> String {
    let mut arguments = vec![
        "start-reading",
        "--image",
        "picture.png",
        "--view",
        view,
        "--reader",
        "first made-up reader",
        "--out",
        out,
    ];
    arguments.extend_from_slice(more);
    measure_ok(folder, &arguments);
    text(&folder.join(out))
}

/// The landmarks a template asks for, in its order.
fn asked(template: &str) -> Vec<String> {
    support::asked_by(template)
}

#[test]
fn a_reading_is_started_with_the_pictures_digest_and_every_landmark_of_its_view() {
    let folder = folder_with_picture("readings-start");
    let template = start(&folder, "front", &[], "readings/front.toml");

    // The digest is the picture's own, worked out here by the test. The test's own sum is
    // first held to the answer the standard publishes for "abc" (FIPS 180-2, appendix B.1),
    // so that what the reading is held to is SHA-256 in lower-case hex and not merely
    // whatever the program writes.
    assert_eq!(
        support::sha256_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    let digest = support::sha256_of(&folder.join("picture.png"));
    assert!(
        template.contains(&format!("sha256 = \"{digest}\"")),
        "{template}"
    );
    assert!(template.contains("file = \"picture.png\""), "{template}");
    assert!(template.contains("width = 800"), "{template}");
    assert!(template.contains("height = 900"), "{template}");
    assert!(template.contains("view = \"front\""), "{template}");
    assert!(
        template.contains("name = \"first made-up reader\""),
        "{template}"
    );

    let front = asked(&template);
    // The centre line's four, and both sides of every pair.
    for name in [
        "hair-top",
        "chin",
        "nose-mark-bottom",
        "mouth-line-centre",
        "eye-left-outer-corner",
        "eye-right-inner-corner",
        "eye-left-opening-top",
        "eye-right-opening-bottom",
        "iris-left-outer-extent",
        "iris-right-top",
        "lid-line-left-crest",
        "brow-line-right-outer-end",
        "nose-mark-left-corner",
        "mouth-line-right-end",
        "ear-left-base-outer",
        "ear-right-top",
        "face-outline-left-top",
        "face-outline-right-at-eye-level",
        "face-outline-left-jaw",
        "hair-right-extent",
        "neck-left-base",
        "shoulder-right-point",
    ] {
        assert!(front.contains(&name.to_string()), "front: no `{name}`");
    }
    assert_eq!(front.len(), 60, "{front:?}");
    // The skull lies under the hair: it is built by rule and never read.
    assert!(
        !front.iter().any(|name| name.starts_with("skull")),
        "{front:?}"
    );

    // A turned view asks which way the head faces, and leaves out what the near side of the
    // head does not show as the same point: its outline above the nose level.
    let turned = start(
        &folder,
        "three-quarter",
        &["--faces", "right"],
        "readings/three-quarter.toml",
    );
    assert!(
        turned.contains("faces = \"the viewer's right\""),
        "{turned}"
    );
    let turned = asked(&turned);
    assert!(turned.contains(&"face-outline-left-top".to_string()));
    assert!(turned.contains(&"face-outline-right-at-nose-level".to_string()));
    assert!(!turned.contains(&"face-outline-right-top".to_string()));
    assert!(!turned.contains(&"face-outline-right-at-eye-level".to_string()));
    assert_eq!(turned.len(), 58, "{turned:?}");

    // A side view shows the near side only.
    let side = start(&folder, "side", &["--faces", "left"], "readings/side.toml");
    assert!(side.contains("faces = \"the viewer's left\""), "{side}");
    let side = asked(&side);
    assert!(side.contains(&"eye-left-outer-corner".to_string()));
    assert!(
        !side.iter().any(|name| name.contains("-right-")),
        "{side:?}"
    );
    assert!(side.contains(&"hair-back-extent".to_string()));
    assert!(side.contains(&"neck-front-base".to_string()));
}

#[test]
fn a_reading_is_started_of_a_jpeg_as_it_is_and_checked_against_it() {
    let folder = references("readings-jpeg");
    support::made_up_jpeg(&folder.join("picture.jpg"), 248, 240);
    let as_made = std::fs::read(folder.join("picture.jpg")).expect("read");

    measure_ok(
        &folder,
        &[
            "start-reading",
            "--image",
            "picture.jpg",
            "--view",
            "front",
            "--reader",
            "first made-up reader",
            "--out",
            "front.toml",
        ],
    );
    let template = text(&folder.join("front.toml"));

    // The digest is of the file as it is: the picture is not converted to be read.
    assert_eq!(
        std::fs::read(folder.join("picture.jpg")).expect("read"),
        as_made,
        "the picture was changed"
    );
    let digest = support::sha256_hex(&as_made);
    assert!(
        template.contains(&format!("sha256 = \"{digest}\"")),
        "{template}"
    );
    assert!(template.contains("file = \"picture.jpg\""), "{template}");
    assert!(template.contains("width = 248"), "{template}");
    assert!(template.contains("height = 240"), "{template}");

    // Filled in, the reading passes its check against the JPEG.
    let filled = filled_in(&template, 0.0, &|name| {
        Some([124.0, 20.0 + name.len() as f64 * 5.0])
    });
    std::fs::write(folder.join("front.toml"), &filled).expect("written");
    let printed = measure_ok(&folder, &["check-reading", "--reading", "front.toml"]);
    assert!(printed.contains("60 landmarks placed"), "{printed}");

    // And not once the JPEG has changed.
    support::made_up_jpeg(&folder.join("picture.jpg"), 249, 240);
    let output = maquette_in(Some(&folder), &["check-reading", "--reading", "front.toml"]);
    let said = String::from_utf8_lossy(&output.stdout).into_owned() + &failure(&output);
    assert!(said.contains("digest"), "{said}");
}

#[test]
fn a_reading_is_started_of_a_webp_as_it_is_and_checked_against_it() {
    let folder = references("readings-webp");
    support::made_up_webp(&folder.join("picture.webp"), 248, 240);
    let as_made = std::fs::read(folder.join("picture.webp")).expect("read");

    measure_ok(
        &folder,
        &[
            "start-reading",
            "--image",
            "picture.webp",
            "--view",
            "front",
            "--reader",
            "first made-up reader",
            "--out",
            "front.toml",
        ],
    );
    let template = text(&folder.join("front.toml"));

    // The digest is of the file as it is: the picture is not converted to be read.
    assert_eq!(
        std::fs::read(folder.join("picture.webp")).expect("read"),
        as_made,
        "the picture was changed"
    );
    let digest = support::sha256_hex(&as_made);
    assert!(
        template.contains(&format!("sha256 = \"{digest}\"")),
        "{template}"
    );
    assert!(template.contains("file = \"picture.webp\""), "{template}");
    assert!(template.contains("width = 248"), "{template}");
    assert!(template.contains("height = 240"), "{template}");

    // Filled in, the reading passes its check against the WebP.
    let filled = filled_in(&template, 0.0, &|name| {
        Some([124.0, 20.0 + name.len() as f64 * 5.0])
    });
    std::fs::write(folder.join("front.toml"), &filled).expect("written");
    let printed = measure_ok(&folder, &["check-reading", "--reading", "front.toml"]);
    assert!(printed.contains("60 landmarks placed"), "{printed}");

    // And not once the WebP has changed.
    support::made_up_webp(&folder.join("picture.webp"), 249, 240);
    let output = maquette_in(Some(&folder), &["check-reading", "--reading", "front.toml"]);
    let said = String::from_utf8_lossy(&output.stdout).into_owned() + &failure(&output);
    assert!(said.contains("digest"), "{said}");
}

#[test]
fn a_jpeg_whose_file_says_it_is_shown_turned_is_refused() {
    // A browser shows such a picture turned, and maquette reads its pixels as they are
    // stored: a reader's positions, an enlargement and a sheet would not agree.
    let folder = references("readings-jpeg-turned");
    support::made_up_jpeg_with_orientation(&folder.join("turned.jpg"), 248, 240, 6);
    let starting = |image: &str| {
        maquette_in(
            Some(&folder),
            &[
                "start-reading",
                "--image",
                image,
                "--view",
                "front",
                "--reader",
                "first made-up reader",
                "--out",
                "front.toml",
            ],
        )
    };
    let message = failure(&starting("turned.jpg"));
    assert!(
        message.contains("turned.jpg") && message.contains("shown turned or mirrored"),
        "{message}"
    );
    assert!(!folder.join("front.toml").exists());
    let output = maquette_in(
        Some(&folder),
        &[
            "enlarge-image",
            "--image",
            "turned.jpg",
            "--left",
            "0",
            "--top",
            "0",
            "--width",
            "10",
            "--height",
            "10",
            "--factor",
            "2",
            "--out",
            "part.png",
        ],
    );
    assert!(failure(&output).contains("shown turned or mirrored"));

    // One whose file says it is upright is read as ever.
    support::made_up_jpeg_with_orientation(&folder.join("upright.jpg"), 248, 240, 1);
    let output = starting("upright.jpg");
    assert!(output.status.success(), "{}", failure(&output));
}

#[test]
fn a_webp_whose_exif_says_it_is_shown_turned_is_refused_with_or_without_the_exif_prefix() {
    // A WebP keeps its EXIF in a chunk of its own. WebP's specification has the chunk begin
    // with the TIFF header; every WebP of the second batch begins it with "Exif" and two zero
    // bytes first, as a JPEG's segment does. A turned picture is refused in either form.
    use support::{EXIF_PREFIX, exif_orientation, made_up_webp_with_exif};
    let folder = references("readings-webp-turned");
    let starting = |image: &str| {
        maquette_in(
            Some(&folder),
            &[
                "start-reading",
                "--image",
                image,
                "--view",
                "front",
                "--reader",
                "first made-up reader",
                "--out",
                "front.toml",
            ],
        )
    };
    let turned = exif_orientation(6);
    for (name, exif) in [
        ("turned-as-specified.webp", turned.clone()),
        ("turned-with-prefix.webp", [EXIF_PREFIX, &turned].concat()),
    ] {
        made_up_webp_with_exif(&folder.join(name), 248, 240, &exif);
        let message = failure(&starting(name));
        assert!(
            message.contains(name) && message.contains("shown turned or mirrored"),
            "{message}"
        );
        assert!(!folder.join("front.toml").exists());
    }

    // EXIF that is not a TIFF directory says nothing that can be read about how the picture
    // is shown, so whether it is turned cannot be told: it is refused, and the message says so.
    let unreadable = [EXIF_PREFIX, b"made up: not a TIFF header"].concat();
    made_up_webp_with_exif(&folder.join("unreadable.webp"), 248, 240, &unreadable);
    let message = failure(&starting("unreadable.webp"));
    assert!(
        message.contains("unreadable.webp") && message.contains("EXIF cannot be read"),
        "{message}"
    );
    assert!(!folder.join("front.toml").exists());

    // One whose EXIF says it is upright is read as ever, in either form.
    for (name, exif) in [
        ("upright-as-specified.webp", exif_orientation(1)),
        (
            "upright-with-prefix.webp",
            [EXIF_PREFIX, &exif_orientation(1)].concat(),
        ),
    ] {
        made_up_webp_with_exif(&folder.join(name), 248, 240, &exif);
        let output = starting(name);
        assert!(output.status.success(), "{name}: {}", failure(&output));
        std::fs::remove_file(folder.join("front.toml")).expect("the started reading");
    }
}

#[test]
fn a_drawing_whose_set_aside_was_overruled_is_read_and_every_landmark_is_asked_for() {
    let folder = folder_with_picture("readings-overruled");
    let template = start(&folder, "front", &[], "front.toml");
    // The reader would have set it aside, and says so: the judgement stands beside the ruling.
    let judged = |reading: &str| {
        reading
            .replace("set_aside = false", "set_aside = true")
            .replace(
                "set_aside_why = \"\"",
                "set_aside_why = \"made up: the expression\"",
            )
    };
    let overruled = |reading: &str, by: &str| {
        reading
            .replace("# set_aside_overruled = true", "set_aside_overruled = true")
            .replace(
                "# set_aside_overruled_by = \"made up for a test\"",
                &format!("set_aside_overruled_by = \"{by}\""),
            )
    };
    let check = |reading: &str| {
        std::fs::write(folder.join("front.toml"), reading).expect("written");
        let output = maquette_in(Some(&folder), &["check-reading", "--reading", "front.toml"]);
        (
            String::from_utf8_lossy(&output.stdout).into_owned()
                + &String::from_utf8_lossy(&output.stderr),
            output.status.success(),
        )
    };
    let measure = || {
        maquette_in(
            Some(&folder),
            &["proportions-from-readings", "--front", "front.toml"],
        )
    };
    let placed_all = filled_in(&template, 0.0, &placed);
    let placed_none = filled_in(&template, 0.0, &|_| None)
        .lines()
        .filter(|line| !line.contains("made up: left out for a test"))
        .collect::<Vec<_>>()
        .join("\n");

    // Set aside and not overruled: nothing is asked for, and nothing is measured.
    let (said, complete) = check(&judged(&placed_none));
    assert!(complete && said.contains("set aside"), "{said}");
    assert!(failure(&measure()).contains("sets the drawing aside"));

    // Overruled: the drawing is read, so every landmark is asked for.
    let (said, complete) = check(&overruled(&judged(&placed_none), "made up: the checker"));
    assert!(
        !complete && said.contains("60 landmarks are neither placed"),
        "{said}"
    );
    let (said, complete) = check(&overruled(&judged(&placed_all), ""));
    assert!(
        !complete && said.contains("does not say who overruled"),
        "{said}"
    );
    let (said, complete) = check(&overruled(&judged(&placed_all), "made up: the checker"));
    assert!(complete && said.contains("60 landmarks placed"), "{said}");
    // And it is no longer refused as set aside. These made-up positions are no head, so
    // measuring stops further on, at the eye line.
    let message = failure(&measure());
    assert!(!message.contains("sets the drawing aside"), "{message}");
    assert!(message.contains("eye line"), "{message}");
}

/// The points of a profile that a side view was first asked for after look 6, where the side
/// view drawn without them failed: the bridge of the nose and the forehead on the front of the
/// face, the jaw's angle, and the top of the neck's two lines.
const PROFILE_SINCE_LOOK_6: [&str; 5] = [
    "centre-line-at-eye-level",
    "centre-line-at-brow-level",
    "jaw-right-angle",
    "neck-front-top",
    "neck-back-top",
];

#[test]
fn a_side_view_asks_for_the_bridge_the_forehead_the_jaws_angle_and_the_necks_top_ends() {
    let folder = folder_with_picture("readings-side-profile");
    let side = start(&folder, "side", &["--faces", "right"], "side.toml");
    let said_of = |landmark: &str| -> String {
        let lines: Vec<&str> = side.lines().collect();
        let at = lines
            .iter()
            .position(|line| *line == format!("# {landmark} = {{ x = , y = }}"))
            .unwrap_or_else(|| panic!("a side view does not ask for {landmark}"));
        lines[at - 1].to_string()
    };
    assert!(said_of("centre-line-at-eye-level").contains("eye line"));
    assert!(said_of("centre-line-at-brow-level").contains("brow"));
    assert!(said_of("jaw-right-angle").contains("toward the ear"));
    assert!(said_of("neck-front-top").contains("underside of the jaw"));
    assert!(said_of("neck-back-top").contains("back of the neck"));
    // The near side only: the head faces the viewer's right, so the near side is the right.
    assert!(!asked(&side).contains(&"jaw-left-angle".to_string()));

    // A front view and a three-quarter view ask for none of them.
    let front = asked(&start(&folder, "front", &[], "front.toml"));
    let turned = asked(&start(
        &folder,
        "three-quarter",
        &["--faces", "right"],
        "three-quarter.toml",
    ));
    for landmark in PROFILE_SINCE_LOOK_6 {
        assert!(!front.contains(&landmark.to_string()), "{landmark}");
        assert!(!turned.contains(&landmark.to_string()), "{landmark}");
    }

    // A side reading made under revision 2026-11-17.3, the revision the first batch's side
    // views were read at, was not asked for them, and is not held to them now.
    let earlier: String = filled_in(&side, 90.0, &|name| {
        Some([300.0 + name.len() as f64, 100.0 + name.len() as f64 * 4.0])
    })
    .lines()
    .filter(|line| {
        !PROFILE_SINCE_LOOK_6
            .iter()
            .any(|landmark| line.starts_with(&format!("{landmark} = ")))
    })
    .map(|line| match line.starts_with("instructions = ") {
        true => "instructions = \"2026-11-17.3\"\n".to_string(),
        false => format!("{line}\n"),
    })
    .collect();
    std::fs::write(folder.join("earlier.toml"), &earlier).expect("written");
    let output = maquette_in(
        Some(&folder),
        &["check-reading", "--reading", "earlier.toml"],
    );
    let said = String::from_utf8_lossy(&output.stdout).into_owned()
        + &String::from_utf8_lossy(&output.stderr);
    assert!(said.contains("2026-11-17.3"), "{said}");
    assert!(
        !said.contains("neither placed nor stated as not seen"),
        "{said}"
    );
    assert!(!said.contains("not a landmark"), "{said}");
}

#[test]
fn a_side_reading_is_started_with_what_each_landmark_is_in_a_profile() {
    let folder = folder_with_picture("readings-side-words");
    let side = start(&folder, "side", &["--faces", "right"], "side.toml");
    let front = start(&folder, "front", &[], "front.toml");
    let said_of = |template: &str, landmark: &str| -> String {
        let lines: Vec<&str> = template.lines().collect();
        let at = lines
            .iter()
            .position(|line| *line == format!("# {landmark} = {{ x = , y = }}"))
            .unwrap_or_else(|| panic!("no {landmark}"));
        lines[at - 1].to_string()
    };
    // In a profile the chin is where the front of the face turns under, the nose mark's
    // bottom is under the tip of the nose, and inner is toward the front of the face.
    assert!(said_of(&side, "chin").contains("turns to run back under the jaw"));
    assert!(said_of(&side, "nose-mark-bottom").contains("Under the tip of the nose"));
    assert!(said_of(&side, "eye-right-inner-corner").contains("front of the face"));
    assert!(said_of(&side, "ear-right-base-outer").contains("upper end"));
    // A front view is started with the words it always was.
    assert_eq!(
        said_of(&front, "chin"),
        "# The lowest point of the face outline."
    );
    assert_eq!(
        said_of(&front, "nose-mark-bottom"),
        "# The lowest point of the drawn nose mark."
    );
    assert_eq!(
        said_of(&front, "eye-right-inner-corner"),
        "# The point of the eye opening nearest the centre line."
    );
}

#[test]
fn a_file_in_none_of_the_formats_is_refused_and_the_message_names_the_three_that_are_read() {
    let folder = references("readings-none-of-the-formats");
    // The header of a GIF and nothing after it, and a file that is no picture at all.
    std::fs::write(
        folder.join("picture.gif"),
        b"GIF89a\x10\x00\x10\x00\x00\x00\x00;",
    )
    .expect("written");
    std::fs::write(folder.join("notes.jpg"), "made up: not a picture\n").expect("written");
    for image in ["picture.gif", "notes.jpg"] {
        let started = maquette_in(
            Some(&folder),
            &[
                "start-reading",
                "--image",
                image,
                "--view",
                "front",
                "--reader",
                "first made-up reader",
                "--out",
                "front.toml",
            ],
        );
        let enlarged = maquette_in(
            Some(&folder),
            &[
                "enlarge-image",
                "--image",
                image,
                "--left",
                "0",
                "--top",
                "0",
                "--width",
                "8",
                "--height",
                "8",
                "--factor",
                "2",
                "--out",
                "part.png",
            ],
        );
        for (command, output) in [("start-reading", started), ("enlarge-image", enlarged)] {
            let message = failure(&output);
            assert!(
                message.contains("a PNG, a JPEG or a WebP, the three formats maquette reads"),
                "{command} of {image}: {message}"
            );
        }
    }
    assert!(!folder.join("front.toml").exists());
    assert!(!folder.join("part.png").exists());
}

fn placed(name: &str) -> Option<[f64; 2]> {
    // Made-up places inside the picture, with the character's left toward the viewer's right.
    let across = if name.contains("-left-") {
        560.0
    } else if name.contains("-right-") {
        240.0
    } else {
        400.0
    };
    Some([across, 100.0 + name.len() as f64 * 7.0])
}

#[test]
fn a_filled_in_reading_passes_its_check_and_an_unfinished_one_is_told_what_it_lacks() {
    let folder = folder_with_picture("readings-check");
    let template = start(&folder, "front", &[], "front.toml");

    // As started, nothing is filled in.
    let unfinished = maquette_in(Some(&folder), &["check-reading", "--reading", "front.toml"]);
    let message = failure(&unfinished);
    let said = String::from_utf8_lossy(&unfinished.stdout).into_owned() + &message;
    for lack in [
        "work",
        "designer",
        "source",
        "age_band",
        "sex",
        "species",
        "manner",
        "view_why",
        "level",
        "chin",
        "60 landmarks",
    ] {
        assert!(said.contains(lack), "`{lack}` is not named: {said}");
    }

    // Filled in, with two landmarks stated as not seen.
    let filled = filled_in(&template, 0.0, &|name| {
        if name.starts_with("ear-left-") && name.contains("base") {
            None
        } else {
            placed(name)
        }
    });
    std::fs::write(folder.join("front.toml"), &filled).expect("written");
    let printed = measure_ok(&folder, &["check-reading", "--reading", "front.toml"]);
    assert!(printed.contains("58 landmarks placed"), "{printed}");
    assert!(printed.contains("2 stated as not seen"), "{printed}");

    // A landmark outside the picture, a pair whose names are exchanged, a name the view does
    // not ask for, and a landmark both placed and not seen.
    let spoiled = filled
        .replace("chin = { x = 400.0,", "chin = { x = 4000.0,")
        .replace(
            "mouth-line-left-end = { x = 560.0,",
            "mouth-line-left-end = { x = 200.0,",
        )
        .replace(
            "\n[not_seen]\n",
            "\n[not_seen]\nhair-top = \"made up\"\nskull-top = \"made up\"\n",
        );
    assert_ne!(spoiled, filled);
    std::fs::write(folder.join("spoiled.toml"), &spoiled).expect("written");
    let output = maquette_in(
        Some(&folder),
        &["check-reading", "--reading", "spoiled.toml"],
    );
    let said = String::from_utf8_lossy(&output.stdout).into_owned() + &failure(&output);
    assert!(
        said.contains("chin") && said.contains("outside the picture"),
        "{said}"
    );
    assert!(
        said.contains("mouth-line-left-end") && said.contains("exchanged"),
        "{said}"
    );
    assert!(said.contains("skull-top"), "{said}");
    assert!(said.contains("hair-top") && said.contains("both"), "{said}");

    // The picture has changed since it was read.
    made_up_image(&folder.join("picture.png"), 801, 900);
    let output = maquette_in(Some(&folder), &["check-reading", "--reading", "front.toml"]);
    let said = String::from_utf8_lossy(&output.stdout).into_owned() + &failure(&output);
    assert!(said.contains("digest"), "{said}");
}

#[test]
fn a_path_that_leaves_the_reference_folder_is_refused() {
    let folder = folder_with_picture("readings-outside");
    let elsewhere = references("readings-outside-elsewhere");
    for out in [
        "../escaped.toml".to_string(),
        elsewhere.join("escaped.toml").display().to_string(),
    ] {
        let output = maquette_in(
            Some(&folder),
            &[
                "start-reading",
                "--image",
                "picture.png",
                "--view",
                "front",
                "--reader",
                "made-up reader",
                "--out",
                &out,
            ],
        );
        let message = failure(&output);
        assert!(
            message.contains("outside the reference folder"),
            "{message}"
        );
    }
    assert!(!elsewhere.join("escaped.toml").exists());
    // The repository itself is outside the reference folder: nothing is read into a reading
    // from it either.
    let output = maquette_in(
        Some(&folder),
        &[
            "check-reading",
            "--reading",
            path_text(&support::repository().join("Cargo.toml")),
        ],
    );
    assert!(failure(&output).contains("outside the reference folder"));
}

/// References are kept outside every repository but a private store of references, so that none
/// can be committed where it may be published. A scratch
/// folder under `target/` is inside this one.
#[test]
fn a_reference_folder_inside_a_git_repository_is_refused() {
    let inside = scratch("readings-folder-in-a-repository");
    made_up_image(&inside.join("picture.png"), 800, 900);
    for (named, where_it_is) in [
        (support::repository(), "is a Git repository"),
        (inside.clone(), "is inside the Git repository"),
    ] {
        let output = maquette_in(
            Some(&named),
            &[
                "start-reading",
                "--image",
                "picture.png",
                "--view",
                "front",
                "--reader",
                "made-up reader",
                "--out",
                "front.toml",
            ],
        );
        let message = failure(&output);
        assert!(
            message.contains(where_it_is)
                && message.contains("outside every repository")
                && message.contains("MAQUETTE_REFERENCES"),
            "{}: {message}",
            named.display()
        );
        assert!(
            message.contains("nothing can be measured"),
            "{}: {message}",
            named.display()
        );
        assert!(!named.join("front.toml").exists());
    }
    // A worktree and a submodule have a file named `.git` that says where the repository is.
    let worktree = references("readings-folder-in-a-worktree");
    std::fs::write(worktree.join(".git"), "gitdir: /made/up/for/a/test\n").expect("written");
    std::fs::create_dir(worktree.join("references")).expect("made");
    let output = maquette_in(
        Some(&worktree.join("references")),
        &["check-reading", "--reading", "front.toml"],
    );
    assert!(failure(&output).contains("inside the Git repository"));

    // A folder that holds a repository deeper down is in none, and neither is one beside an
    // empty folder named `.git`: Git looks past that too.
    let outside = folder_with_picture("readings-folder-over-a-repository");
    std::fs::create_dir_all(outside.join("deeper/.git")).expect("made");
    std::fs::write(
        outside.join("deeper/.git/HEAD"),
        "ref: refs/heads/made-up\n",
    )
    .expect("written");
    std::fs::create_dir(outside.join(".git")).expect("made");
    start(&outside, "front", &[], "front.toml");
}

/// A private repository of references declares itself with a file at its root, and a folder in
/// it is a reference folder: at its root, or deeper, as a reader's own folder is. A checkout of
/// maquette is never one, whatever file it holds: maquette is public.
#[test]
fn a_private_repository_that_declares_itself_a_reference_store_holds_reference_folders() {
    let store = references("readings-folder-in-a-store");
    std::fs::create_dir_all(store.join(".git")).expect("made");
    std::fs::write(store.join(".git/HEAD"), "ref: refs/heads/made-up\n").expect("written");
    std::fs::write(
        store.join(".maquette-reference-store"),
        "made up for a test: a private repository of references\n",
    )
    .expect("written");
    made_up_image(&store.join("picture.png"), 800, 900);
    start(&store, "front", &[], "front.toml");
    let deeper = store.join("readers/first-reader");
    made_up_image(&deeper.join("picture.png"), 800, 900);
    start(&deeper, "front", &[], "front.toml");

    // The same file in a checkout of maquette, which has the list of proportions, is refused.
    let checkout = references("readings-folder-in-a-marked-checkout");
    std::fs::create_dir_all(checkout.join(".git")).expect("made");
    std::fs::write(checkout.join(".git/HEAD"), "ref: refs/heads/made-up\n").expect("written");
    std::fs::write(
        checkout.join(".maquette-reference-store"),
        "made up for a test\n",
    )
    .expect("written");
    std::fs::create_dir_all(checkout.join("proportions")).expect("made");
    std::fs::write(
        checkout.join("proportions/list.toml"),
        "# made up for a test\n",
    )
    .expect("written");
    made_up_image(&checkout.join("picture.png"), 800, 900);
    let output = maquette_in(
        Some(&checkout),
        &["check-reading", "--reading", "front.toml"],
    );
    let message = failure(&output);
    assert!(
        message.contains("is a Git repository") && message.contains("a checkout of maquette"),
        "{message}"
    );

    // Nor is a store that sits inside a checkout of maquette, which Git could then take in.
    let nested = checkout.join("inside");
    std::fs::create_dir_all(nested.join(".git")).expect("made");
    std::fs::write(nested.join(".git/HEAD"), "ref: refs/heads/made-up\n").expect("written");
    std::fs::write(
        nested.join(".maquette-reference-store"),
        "made up for a test\n",
    )
    .expect("written");
    let output = maquette_in(Some(&nested), &["check-reading", "--reading", "front.toml"]);
    let message = failure(&output);
    assert!(
        message.contains("inside a checkout of maquette"),
        "{message}"
    );
}

/// A link inside the reference folder that leads out of it is a way out of it like any other.
#[cfg(unix)]
#[test]
fn a_path_that_leaves_the_reference_folder_through_a_link_is_refused() {
    let folder = folder_with_picture("readings-outside-by-link");
    let elsewhere = references("readings-outside-by-link-elsewhere");
    std::fs::write(elsewhere.join("outside.toml"), "made up for a test\n").expect("written");
    std::os::unix::fs::symlink(&elsewhere, folder.join("link")).expect("a link to a folder");
    std::os::unix::fs::symlink(elsewhere.join("outside.toml"), folder.join("linked.toml"))
        .expect("a link to a file");
    // A link to a file that is not there yet: writing to it would make the file outside.
    std::os::unix::fs::symlink(elsewhere.join("not-yet.toml"), folder.join("dangling.toml"))
        .expect("a link to nothing");

    for out in [
        "link/escaped.toml",
        "link/deeper/escaped.toml",
        "dangling.toml",
    ] {
        let output = maquette_in(
            Some(&folder),
            &[
                "start-reading",
                "--image",
                "picture.png",
                "--view",
                "front",
                "--reader",
                "made-up reader",
                "--out",
                out,
            ],
        );
        let message = failure(&output);
        assert!(
            message.contains("outside the reference folder"),
            "{out}: {message}"
        );
    }
    let written: Vec<_> = std::fs::read_dir(&elsewhere)
        .expect("the other folder")
        .map(|entry| entry.expect("an entry").file_name())
        .collect();
    assert_eq!(written, ["outside.toml"], "something was written outside");

    // Nothing is read through a link either.
    for reading in ["link/outside.toml", "linked.toml"] {
        let output = maquette_in(Some(&folder), &["check-reading", "--reading", reading]);
        let message = failure(&output);
        assert!(
            message.contains("outside the reference folder"),
            "{reading}: {message}"
        );
    }

    // A link that stays inside the folder is the folder still.
    std::fs::create_dir(folder.join("readings")).expect("made");
    std::os::unix::fs::symlink(folder.join("readings"), folder.join("inside")).expect("a link");
    measure_ok(
        &folder,
        &[
            "start-reading",
            "--image",
            "picture.png",
            "--view",
            "front",
            "--reader",
            "made-up reader",
            "--out",
            "inside/front.toml",
        ],
    );
    assert!(folder.join("readings/front.toml").is_file());
}

#[test]
fn the_reading_instructions_are_printed_and_name_every_landmark_a_reading_asks_for() {
    let instructions = maquette_ok(&["reading-instructions"]);
    let folder = folder_with_picture("readings-instructions");
    let revision = instructions
        .lines()
        .find_map(|line| line.strip_prefix("Revision: "))
        .expect("the instructions state their revision")
        .trim()
        .to_string();
    for (view, more) in [
        ("front", &[][..]),
        ("three-quarter", &["--faces", "right"][..]),
        ("side", &["--faces", "right"][..]),
    ] {
        let template = start(&folder, view, more, &format!("{view}.toml"));
        // A reading says which revision of the instructions it follows.
        assert!(
            template.contains(&format!("instructions = \"{revision}\"")),
            "{template}"
        );
        for name in asked(&template) {
            // The instructions define a pair once, for both sides.
            let general = name
                .replace("-left-", "-<side>-")
                .replace("-right-", "-<side>-");
            assert!(
                instructions.contains(&format!("`{general}`")),
                "the reading instructions do not define `{general}`, which a {view} reading asks for"
            );
        }
    }
    for judged in ["view", "turn", "level", "set aside"] {
        assert!(instructions.contains(judged), "no word on {judged}");
    }
    // The skull is not read; the ears and the temples are.
    assert!(instructions.contains("skull"), "no word on the skull");
}
