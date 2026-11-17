//! The read-back and the hard case, live, in whichever VectorCraft is installed.
//!
//! Every test here is ignored by `cargo test`, so that a clean clone and GitHub Actions,
//! which have no VectorCraft, need none of them. Run them by hand where it is installed:
//!
//! ```sh
//! cargo test --test read_back_in_vectorcraft -- --ignored
//! ```
//!
//! Where it is not installed they fail and say so: a test that is asked for and cannot run
//! has not passed. A pass here says that VectorCraft keeps what the landmark files say. It
//! does not say an armature looks right, and it accepts nothing.

mod support;

use support::{draw, path_text};

const HER_PROPORTIONS: &str = "data/saffi/proportions.toml";

/// Runs a command that needs VectorCraft, and gives what it printed when it passed.
fn in_vectorcraft(arguments: &[&str]) -> String {
    let output = support::maquette(arguments);
    let printed = String::from_utf8_lossy(&output.stdout).into_owned();
    let message = String::from_utf8_lossy(&output.stderr);
    match output.status.code() {
        Some(0) => printed,
        Some(2) => panic!("this test needs VectorCraft, and {message}"),
        _ => panic!(
            "`maquette {}` did not pass:\n{printed}{message}",
            arguments.join(" ")
        ),
    }
}

#[test]
#[ignore = "needs vectorcraft-cli; run with -- --ignored where VectorCraft is installed"]
fn all_three_of_saffis_views_pass_the_oracle() {
    let folder = draw("saffi-read-back", HER_PROPORTIONS, &[]);
    for view in ["front", "three-quarter", "side"] {
        let landmarks = folder.join(format!("{view}.landmarks.toml"));
        let printed = in_vectorcraft(&["read-back", "--landmarks", path_text(&landmarks)]);
        assert!(
            printed.contains("(headless MCP server), run now"),
            "{printed}"
        );
        assert!(printed.contains("PASSED"), "{printed}");
        println!("{printed}");
    }
}

#[test]
#[ignore = "needs vectorcraft-cli; run with -- --ignored where VectorCraft is installed"]
fn saffis_three_quarter_view_holds_the_hard_case() {
    let folder = draw("saffi-hard-case", HER_PROPORTIONS, &[]);
    let landmarks = folder.join("three-quarter.landmarks.toml");
    let printed = in_vectorcraft(&["read-back-hard-case", "--landmarks", path_text(&landmarks)]);
    assert!(printed.contains("HELD: all 10 lines"), "{printed}");
    // The two spoiled copies were failed, not passed.
    assert_eq!(
        printed.matches("the oracle failed it").count(),
        2,
        "{printed}"
    );
    println!("{printed}");
}

#[test]
#[ignore = "needs vectorcraft-cli; run with -- --ignored where VectorCraft is installed"]
fn the_made_up_views_pass_the_oracle_and_the_hard_case() {
    let folder = draw("made-up-read-back", support::MADE_UP, &[]);
    for view in ["front", "three-quarter", "side"] {
        let landmarks = folder.join(format!("{view}.landmarks.toml"));
        let printed = in_vectorcraft(&["read-back", "--landmarks", path_text(&landmarks)]);
        assert!(printed.contains("PASSED"), "{printed}");
    }
    let landmarks = folder.join("three-quarter.landmarks.toml");
    let printed = in_vectorcraft(&["read-back-hard-case", "--landmarks", path_text(&landmarks)]);
    assert!(printed.contains("HELD: all 10 lines"), "{printed}");
}

#[test]
#[ignore = "needs vectorcraft-cli; run with -- --ignored where VectorCraft is installed"]
fn a_front_view_is_refused_as_a_hard_case() {
    let folder = draw("front-hard-case", support::MADE_UP, &[]);
    let landmarks = folder.join("front.landmarks.toml");
    let output = support::maquette(&["read-back-hard-case", "--landmarks", path_text(&landmarks)]);
    assert_eq!(output.status.code(), Some(1));
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(
        message.contains("The hard case is a three-quarter view"),
        "{message}"
    );
}
