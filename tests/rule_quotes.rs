//! The quotations in `docs/agents/rule-quotes.md` are the constitution's rules, byte for byte.
//!
//! A brief that quotes a current-revision rule exactly, naming the revision, stands in for the
//! recipient's reading of that rule (`CLAUDE.md`). A quotation that has drifted from the rule
//! would stand in for a rule that does not exist.

const CONSTITUTION: &str = "docs/principles/principles-for-repos.md";
const QUOTES: &str = "docs/agents/rule-quotes.md";

fn repository_text(path: &str) -> String {
    let whole = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    std::fs::read_to_string(&whole).unwrap_or_else(|error| panic!("{path}: {error}"))
}

/// Every rule in a Markdown text: its ID, and its whole text from its `### ` heading up to the
/// next heading of a rule or a section, without the blank lines that end it.
fn rules(text: &str) -> Vec<(String, String)> {
    let mut found: Vec<(String, Vec<&str>)> = Vec::new();
    let mut inside = false;
    for line in text.lines() {
        if let Some(heading) = line.strip_prefix("### ") {
            let id = heading.split_whitespace().next().unwrap_or("").to_string();
            found.push((id, vec![line]));
            inside = true;
        } else if line.starts_with("## ") || line.starts_with("# ") {
            inside = false;
        } else if inside {
            found.last_mut().expect("a rule is open").1.push(line);
        }
    }
    found
        .into_iter()
        .map(|(id, mut lines)| {
            while lines.last() == Some(&"") {
                lines.pop();
            }
            (id, lines.join("\n"))
        })
        .collect()
}

#[test]
fn every_quoted_rule_is_the_constitutions_rule_at_its_revision() {
    let constitution = repository_text(CONSTITUTION);
    let quotes = repository_text(QUOTES);

    let revision_of = |text: &str, prefix: &str, path: &str| -> String {
        text.lines()
            .find_map(|line| line.strip_prefix(prefix))
            .unwrap_or_else(|| panic!("{path} has no line beginning `{prefix}`"))
            .trim()
            .to_string()
    };
    let current = revision_of(&constitution, "**Constitution revision:** ", CONSTITUTION);
    let quoted_at = revision_of(&quotes, "Constitution revision: ", QUOTES);
    assert_eq!(
        quoted_at, current,
        "{QUOTES} quotes revision {quoted_at}, and the constitution is at {current}: quote the rules again from it"
    );

    let rules_now = rules(&constitution);
    let quoted = rules(&quotes);
    assert!(!quoted.is_empty(), "{QUOTES} quotes no rule");
    let mut differ = Vec::new();
    for (id, text) in &quoted {
        match rules_now.iter().find(|(rule, _)| rule == id) {
            Some((_, rule)) if rule == text => {}
            Some(_) => differ.push(format!("{id} differs from the constitution's text")),
            None => differ.push(format!("{id} is not a rule of the constitution")),
        }
    }
    assert!(differ.is_empty(), "{QUOTES}:\n{}", differ.join("\n"));
}
