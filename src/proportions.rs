//! The list of named proportions, and the reading and checking of a set of proportions.
//!
//! The list itself is `proportions/list.toml`, built into the program. This module is the only
//! reader of that file, so a definition is written once and read by a person and by the
//! program in the same place.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::{Error, Result};

const LIST_TEXT: &str = include_str!("../proportions/list.toml");

/// Whether a proportion is taken from drawn features or built by a stated rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Measured,
    Convention,
}

impl Kind {
    pub fn word(self) -> &'static str {
        match self {
            Kind::Measured => "measured",
            Kind::Convention => "convention",
        }
    }
}

/// One entry of the list.
#[derive(Debug, Clone)]
pub struct Proportion {
    pub name: String,
    pub kind: Kind,
    /// A span that must be above zero for a head to be drawn.
    pub positive: bool,
    pub what: String,
    pub front: String,
    pub three_quarter: String,
    /// How a convention is built where nothing is drawn behind it. A measured proportion
    /// has none.
    pub rule: Option<String>,
}

/// The list of proportions with the statements that go with it.
#[derive(Debug, Clone)]
pub struct List {
    pub proportions: Vec<Proportion>,
    statements: Vec<(String, Vec<(String, String)>)>,
    disagreement_rules: Vec<String>,
    constructions: Vec<(String, String)>,
}

fn text_of(table: &toml::Table, key: &str, context: &str) -> Result<String> {
    table
        .get(key)
        .and_then(toml::Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| {
            Error::new(format!(
                "the list of proportions: {context} has no text `{key}`"
            ))
        })
}

fn tables<'a>(table: &'a toml::Table, key: &str) -> Vec<&'a toml::Table> {
    table
        .get(key)
        .and_then(toml::Value::as_array)
        .map(|entries| entries.iter().filter_map(toml::Value::as_table).collect())
        .unwrap_or_default()
}

fn parse_list(text: &str) -> Result<List> {
    let table: toml::Table = text.parse().map_err(|error| {
        Error::new(format!(
            "the list of proportions is not valid TOML: {error}"
        ))
    })?;

    let mut proportions = Vec::new();
    for entry in tables(&table, "proportion") {
        let name = text_of(entry, "name", "a proportion")?;
        let context = format!("the proportion `{name}`");
        let kind = match text_of(entry, "kind", &context)?.as_str() {
            "measured" => Kind::Measured,
            "convention" => Kind::Convention,
            other => {
                return Err(Error::new(format!(
                    "the list of proportions: {context} is marked `{other}`, neither measured nor convention"
                )));
            }
        };
        let positive = match text_of(entry, "sign", &context)?.as_str() {
            "positive" => true,
            "any" => false,
            other => {
                return Err(Error::new(format!(
                    "the list of proportions: {context} has the sign `{other}`, neither positive nor any"
                )));
            }
        };
        if proportions
            .iter()
            .any(|known: &Proportion| known.name == name)
        {
            return Err(Error::new(format!(
                "the list of proportions names `{name}` twice"
            )));
        }
        // A convention is built by a stated rule, and only a convention is.
        let rule = match (kind, entry.contains_key("rule")) {
            (Kind::Convention, _) => Some(text_of(entry, "rule", &context)?),
            (Kind::Measured, false) => None,
            (Kind::Measured, true) => {
                return Err(Error::new(format!(
                    "the list of proportions: {context} is measured and states a rule; only a convention is built by rule"
                )));
            }
        };
        proportions.push(Proportion {
            kind,
            positive,
            what: text_of(entry, "what", &context)?,
            front: text_of(entry, "front", &context)?,
            three_quarter: text_of(entry, "three_quarter", &context)?,
            rule,
            name,
        });
    }

    let mut statements = Vec::new();
    for (key, title) in [
        ("unit", "Unit"),
        ("eye_line", "Eye line"),
        ("head_height", "Head height"),
        ("depth", "Depth"),
    ] {
        let section = table
            .get(key)
            .and_then(toml::Value::as_table)
            .ok_or_else(|| Error::new(format!("the list of proportions has no [{key}]")))?;
        let context = format!("[{key}]");
        let mut lines = Vec::new();
        let definition = text_of(section, "definition", &context)?;
        let heading = match section.get("name").and_then(toml::Value::as_str) {
            Some(name) => format!("{title}: {name}"),
            None => format!("{title}: {definition}"),
        };
        if section.contains_key("name") {
            lines.push(("Definition".to_string(), definition));
        }
        for (field, label) in [
            ("why", "Why"),
            ("front", "Front view"),
            ("three_quarter", "Three-quarter view"),
            ("side", "Side view"),
        ] {
            if let Some(value) = section.get(field).and_then(toml::Value::as_str) {
                lines.push((label.to_string(), value.to_string()));
            }
        }
        statements.push((heading, lines));
    }

    let words = table
        .get("words")
        .and_then(toml::Value::as_table)
        .ok_or_else(|| Error::new("the list of proportions has no [words]"))?;
    let mut word_lines = Vec::new();
    for (word, meaning) in words {
        let meaning = meaning.as_str().ok_or_else(|| {
            Error::new(format!(
                "the list of proportions: the word `{word}` has no text"
            ))
        })?;
        word_lines.push((word.replace('_', " "), meaning.to_string()));
    }
    statements.push(("Words".to_string(), word_lines));

    let mut disagreement_rules = Vec::new();
    for entry in tables(&table, "disagreement_rule") {
        disagreement_rules.push(text_of(entry, "rule", "a disagreement rule")?);
    }
    let mut constructions = Vec::new();
    for entry in tables(&table, "construction") {
        constructions.push((
            text_of(entry, "guide", "a construction")?,
            text_of(entry, "rule", "a construction")?,
        ));
    }

    Ok(List {
        proportions,
        statements,
        disagreement_rules,
        constructions,
    })
}

/// The list of proportions, read once from the text built into the program.
pub fn list() -> Result<&'static List> {
    static LIST: OnceLock<Result<List>> = OnceLock::new();
    LIST.get_or_init(|| parse_list(LIST_TEXT))
        .as_ref()
        .map_err(Clone::clone)
}

impl List {
    pub fn find(&self, name: &str) -> Option<&Proportion> {
        self.proportions
            .iter()
            .find(|proportion| proportion.name == name)
    }

    /// The rules for views that disagree, as the list states them.
    pub fn disagreement_rules(&self) -> &[String] {
        &self.disagreement_rules
    }

    /// The whole list as plain text for a person.
    pub fn describe(&self) -> String {
        let mut out = String::new();
        for (heading, lines) in &self.statements {
            out.push_str(heading);
            out.push('\n');
            for (label, value) in lines {
                out.push_str(&format!("  {label}: {value}\n"));
            }
            out.push('\n');
        }
        out.push_str("Where two views disagree:\n");
        for (index, rule) in self.disagreement_rules.iter().enumerate() {
            out.push_str(&format!("  {}. {rule}\n", index + 1));
        }
        out.push_str("\nHow the guides are built in a view:\n");
        for (guide, rule) in &self.constructions {
            out.push_str(&format!("  {guide}: {rule}\n"));
        }
        let conventions = self
            .proportions
            .iter()
            .filter(|proportion| proportion.kind == Kind::Convention)
            .count();
        out.push_str(&format!(
            "\n{} proportions: {} measured, {} conventions.\n",
            self.proportions.len(),
            self.proportions.len() - conventions,
            conventions
        ));
        for proportion in &self.proportions {
            out.push_str(&format!(
                "\n{}  [{}]\n",
                proportion.name,
                proportion.kind.word()
            ));
            out.push_str(&format!("  What: {}\n", proportion.what));
            out.push_str(&format!("  Front view: {}\n", proportion.front));
            out.push_str(&format!(
                "  Three-quarter view: {}\n",
                proportion.three_quarter
            ));
            if let Some(rule) = &proportion.rule {
                out.push_str(&format!("  By rule: {rule}\n"));
            }
        }
        out
    }
}

/// A set of proportions: one value for every proportion of the list, and nothing else.
#[derive(Debug, Clone, PartialEq)]
pub struct Set {
    values: BTreeMap<String, f64>,
}

impl Set {
    /// Checks values against the list: every proportion once, no other name, every number
    /// finite, and every span that must be above zero above zero.
    pub fn from_values(values: BTreeMap<String, f64>) -> Result<Set> {
        let list = list()?;
        let missing: Vec<&str> = list
            .proportions
            .iter()
            .filter(|proportion| !values.contains_key(&proportion.name))
            .map(|proportion| proportion.name.as_str())
            .collect();
        if !missing.is_empty() {
            return Err(Error::new(format!(
                "the set of proportions lacks {}: {}. `maquette list-proportions` defines each.",
                count(missing.len(), "proportion"),
                missing.join(", ")
            )));
        }
        let unknown: Vec<&str> = values
            .keys()
            .filter(|name| list.find(name).is_none())
            .map(String::as_str)
            .collect();
        if !unknown.is_empty() {
            return Err(Error::new(format!(
                "the set of proportions has {} the list does not: {}",
                count(unknown.len(), "name"),
                unknown.join(", ")
            )));
        }
        for proportion in &list.proportions {
            let value = values[&proportion.name];
            if !value.is_finite() {
                return Err(Error::new(format!(
                    "the proportion `{}` is not a finite number",
                    proportion.name
                )));
            }
            if proportion.positive && value <= 0.0 {
                return Err(Error::new(format!(
                    "the proportion `{}` is {value}, and it must be above zero",
                    proportion.name
                )));
            }
        }
        Ok(Set { values })
    }

    /// Reads a set of proportions from the text of its file: a `[proportions]` table of
    /// names and numbers. Other tables, such as a note of where the set came from, are left alone.
    pub fn read(text: &str) -> Result<Set> {
        let table: toml::Table = text.parse().map_err(|error| {
            Error::new(format!("the set of proportions is not valid TOML: {error}"))
        })?;
        let entries = table
            .get("proportions")
            .and_then(toml::Value::as_table)
            .ok_or_else(|| Error::new("the set of proportions has no [proportions] table"))?;
        let mut values = BTreeMap::new();
        for (name, value) in entries {
            let number = match value {
                toml::Value::Float(number) => *number,
                toml::Value::Integer(number) => *number as f64,
                _ => {
                    return Err(Error::new(format!(
                        "the proportion `{name}` is not a number"
                    )));
                }
            };
            values.insert(name.clone(), number);
        }
        Set::from_values(values)
    }

    /// The value of a proportion of the list.
    ///
    /// # Panics
    ///
    /// If the list has no such proportion: the head model and the list then disagree, which
    /// is a fault in the program and not in a set of proportions.
    pub fn get(&self, name: &str) -> f64 {
        match self.values.get(name) {
            Some(value) => *value,
            None => panic!("the list of proportions has no `{name}`, which the head model uses"),
        }
    }

    /// The set as the text of its file, in the list's order, under the opening lines given.
    pub fn write(&self, opening: &str) -> Result<String> {
        let mut out = String::from(opening);
        if !out.is_empty() && !out.ends_with('\n') {
            out.push('\n');
        }
        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str("[proportions]\n");
        for proportion in &list()?.proportions {
            out.push_str(&format!(
                "{} = {}\n",
                proportion.name,
                crate::number::toml(self.values[&proportion.name], 6)
            ));
        }
        Ok(out)
    }
}

fn count(number: usize, noun: &str) -> String {
    if number == 1 {
        format!("1 {noun}")
    } else {
        format!("{number} {noun}s")
    }
}
