//! The reading instructions change only with a new revision.
//!
//! A reading names the revision of the instructions it was made under, and a revision's
//! readings are tested on drawings whose positions are known. An edit that keeps the revision
//! would leave readings naming a text they were not made under. So the file's `Revision:` line
//! must have an entry in `docs/agents/reading-instructions.revisions.toml` whose digest is the
//! file's own.

use sha2::{Digest, Sha256};

const INSTRUCTIONS: &str = "docs/agents/reading-instructions.md";
const REVISIONS: &str = "docs/agents/reading-instructions.revisions.toml";

fn repository_file(path: &str) -> Vec<u8> {
    let whole = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    std::fs::read(&whole).unwrap_or_else(|error| panic!("{path}: {error}"))
}

#[test]
fn the_reading_instructions_are_the_text_their_revision_was_published_with() {
    let instructions = repository_file(INSTRUCTIONS);
    let text = std::str::from_utf8(&instructions).expect("the instructions are text");
    let revision = text
        .lines()
        .find_map(|line| line.strip_prefix("Revision: "))
        .unwrap_or_else(|| panic!("{INSTRUCTIONS} has no `Revision:` line"))
        .trim();

    let table: toml::Table = std::str::from_utf8(&repository_file(REVISIONS))
        .expect("the table is text")
        .parse()
        .unwrap_or_else(|error| panic!("{REVISIONS}: {error}"));
    let revisions = table["revisions"]
        .as_table()
        .unwrap_or_else(|| panic!("{REVISIONS} has no [revisions] table"));

    let digest: String = Sha256::digest(&instructions)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let Some(recorded) = revisions.get(revision) else {
        panic!(
            "{INSTRUCTIONS} says it is revision {revision}, which {REVISIONS} does not list. \
             Add it with the file's digest, {digest}, when this revision is published."
        );
    };
    assert_eq!(
        recorded.as_str(),
        Some(digest.as_str()),
        "{INSTRUCTIONS} has changed since revision {revision} was published. Readings name the \
         revision they were made under, so a changed text is a new revision: give the file a \
         new `Revision:` line and add it to {REVISIONS}."
    );
}
