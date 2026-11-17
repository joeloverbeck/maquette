//! The repository holds nothing that must never be committed.
//!
//! The repository is public, the reference drawings are other people's, and the reference
//! folder is private (`docs/foundation/04-evidence-and-references.md`, README.md "The
//! reference folder"). So among the tracked files there is:
//!
//! - no image outside `design/`, which holds the designs made by hand and the sheets Jon saw;
//! - no reading outside `tests/fixtures/`, where a reading would be a made-up one;
//! - in no text file, the path of a home directory or the name of the private reference
//!   folder.
//!
//! The tracked files are taken from `git ls-files`. The words searched for are put together
//! here from pieces, so that this file does not hold them itself.

use std::process::Command;

const IMAGES: &[&str] = &["png", "jpg", "jpeg", "webp", "gif"];

/// The beginnings of a home directory's path, each followed by a user's name.
fn home_directories() -> [String; 3] {
    [
        ["/", "home", "/"].concat(),
        ["/", "Users", "/"].concat(),
        ["C:", "\\", "Users", "\\"].concat(),
    ]
}

/// The name of the private reference folder on Jon's machine.
fn reference_folder_name() -> String {
    ["maquette", "references"].join("-")
}

fn tracked_files() -> Vec<String> {
    let output = Command::new("git")
        .args(["ls-files", "-z"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("git runs: this test reads the tracked files from it");
    assert!(
        output.status.success(),
        "`git ls-files` failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("the tracked paths are text")
        .split('\0')
        .filter(|path| !path.is_empty())
        .map(str::to_string)
        .collect()
}

/// Where in `text` a home directory's path begins: one of the beginnings followed by a letter,
/// a digit or an underscore, which starts a user's name.
fn home_directory_in(text: &str) -> Option<String> {
    for beginning in home_directories() {
        for (at, _) in text.match_indices(&beginning) {
            let after = &text[at + beginning.len()..];
            if after
                .chars()
                .next()
                .is_some_and(|first| first.is_ascii_alphanumeric() || first == '_')
            {
                let shown: String = text[at..].chars().take(40).collect();
                return Some(shown);
            }
        }
    }
    None
}

#[test]
fn no_tracked_file_is_an_image_a_reading_or_a_private_path() {
    let mut found = Vec::new();
    for path in tracked_files() {
        let lower = path.to_ascii_lowercase();
        let extension = lower
            .rsplit_once('.')
            .map_or("", |(_, extension)| extension);
        if IMAGES.contains(&extension) && !path.starts_with("design/") {
            found.push(format!(
                "{path}: an image outside design/. A reference image or a crop of one is never committed."
            ));
        }
        if lower.ends_with(".reading.toml") && !path.starts_with("tests/fixtures/") {
            found.push(format!(
                "{path}: a reading outside tests/fixtures/. Readings stay in the reference folder."
            ));
        }
        let whole = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(&path);
        // A tracked file that is a link, or deleted in the working tree, is not read here.
        let Ok(bytes) = std::fs::read(&whole) else {
            continue;
        };
        let Ok(text) = std::str::from_utf8(&bytes) else {
            continue;
        };
        if let Some(shown) = home_directory_in(text) {
            found.push(format!(
                "{path}: the path of a home directory, `{shown}`. Name a path from the repository, or describe it."
            ));
        }
        if text.contains(&reference_folder_name()) {
            found.push(format!(
                "{path}: the name of the private reference folder. Nothing committed names it."
            ));
        }
    }
    assert!(
        found.is_empty(),
        "the repository holds what must never be committed:\n{}",
        found.join("\n")
    );
}

#[test]
fn the_search_for_a_home_directory_finds_one_and_nothing_else() {
    let user = ["/", "home", "/", "someone/work"].concat();
    assert!(home_directory_in(&format!("a path {user}")).is_some());
    let mac = ["/", "Users", "/", "someone"].concat();
    assert!(home_directory_in(&mac).is_some());
    let windows = ["C:", "\\", "Users", "\\", "someone"].concat();
    assert!(home_directory_in(&windows).is_some());
    assert!(home_directory_in("installed at ~/.local/bin/maquette").is_none());
    assert!(home_directory_in(&["/", "home", "/"].concat()).is_none());
}
