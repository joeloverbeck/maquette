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

/// References are kept outside every repository, so that none can be committed. A scratch
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
