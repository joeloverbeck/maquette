//! The record of Jon's check of a reference check sheet: which characters he accepts, which
//! points he marks as off, and where he overrules the readers on whether a drawing is
//! measured.
//!
//! This module hides what the record holds, what makes it complete, and what it does to the
//! readings: which of a drawing's readings are measured after his check, and with which
//! landmarks left out. Jon says his decisions in words and an agent writes them down, so the
//! record is a small TOML file a person can read. `maquette start-jons-check` writes one to
//! fill in from a sheet's description, and `maquette jons-check` says what a record does.
//!
//! The record names each reading with the digest of its file, so that what Jon accepted is
//! those readings and no later ones. It is of one reference each, stays in the reference
//! folder and is never committed.
//!
//! Nothing here judges a drawing. The program reports what the record says and what follows
//! from it by the rules below; the decisions are Jon's.

use std::collections::BTreeSet;

use crate::number::counted;
use crate::readings::{Drawing, Record, view_named};
use crate::toml_text::{quoted, tables, words};
use crate::views::View;
use crate::{Error, Result};

/// How many works a view needs behind it to be a table: a view with fewer is too thin
/// (`docs/foundation/04-evidence-and-references.md`, "Reference drawings", point 6).
pub const WORKS_FOR_A_TABLE: usize = 5;

/// What the record does, by name, as `maquette jons-check` states it.
pub const RULES: &[(&str, &str)] = &[
    (
        "accepted_only",
        "Only the readings of a character Jon accepted are measured. A character he rejected, or has not given a verdict on, gives nothing.",
    ),
    (
        "complete",
        "A measuring command given a record that `maquette jons-check` does not call complete measures nothing by it, and says what the record lacks or gets wrong.",
    ),
    (
        "all_readings",
        "A drawing is measured from all of the readings the record lists for it, as the record rules, or not at all: a command that names only some of them measures nothing.",
    ),
    (
        "as_the_readers_judged",
        "Where Jon gives no ruling on a drawing, it is measured as its readers judged it: from its one reading or its two when each calls the head level and none sets it aside, and not at all when a reader set it aside or the two do not judge the view alike.",
    ),
    (
        "measure",
        "Where Jon rules that a drawing is measured, it is measured as a level head in the view of its card, or in the view he names, from each of its readings that judged that view and placed its landmarks. A reading that set the drawing aside placed nothing, so a drawing both readers set aside must be read again before it is measured.",
    ),
    (
        "set_aside",
        "Where Jon rules that a drawing is set aside, it is not measured, whatever its readers judged. The character's other drawings stand.",
    ),
    (
        "off",
        "A landmark Jon marks as off on a drawing is left out of that drawing, in each of its readings. The drawing is still measured, and what that landmark alone would have given is not given, or is taken from the character's other view where the rules for two views say so. A mark is a landmark's name, or a name with `*` in it for every landmark it fits: `ear-*` is every landmark of both ears.",
    ),
    (
        "off_in_one_reading",
        "A mark may name one reader, where Jon rules that one reader placed a landmark rightly and the other did not: it leaves that landmark out of that reader's reading alone. A landmark the other reading places is then measured from the other reading's point alone, and what is written says so. A landmark only one reading places with no mark of Jon's behind it is still left out.",
    ),
    (
        "proportions_off",
        "A proportion Jon marks as off for a character is not given for that character's work, and its landmarks stay: the eye opening's height can be left out while the opening's top and bottom still give the eye line.",
    ),
    (
        "counted",
        "A work counts for a view when its character is accepted and that view's drawing is measured after the rulings. A view with fewer works than a table needs is reported as too thin.",
    ),
];

/// Jon's verdict on a character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Accepted,
    Rejected,
}

/// Jon's ruling on a drawing, where he overrules its readers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ruling {
    Measure,
    SetAside,
}

/// One reading the record names: its path in the reference folder, and the SHA-256 digest
/// of its file when the record was started.
#[derive(Debug, Clone, PartialEq)]
pub struct Named {
    pub path: String,
    pub sha256: String,
}

/// One drawing of a character, as the record holds it.
#[derive(Debug, Clone)]
pub struct DrawingChecked {
    /// The view of the card the sheet shows it on.
    pub card: View,
    /// Which picture it is, in words.
    pub picture: String,
    pub readings: Vec<Named>,
    pub ruling: Option<Ruling>,
    pub ruling_why: String,
    /// With a ruling to measure: the view Jon says it is, where it is not the card's.
    pub view: Option<View>,
    /// The landmarks Jon marks as off.
    pub off: Vec<Mark>,
}

/// A landmark Jon marks as off on a drawing: a name, or a name with `*` in it, in every
/// reading of the drawing or in one reader's alone.
#[derive(Debug, Clone, PartialEq)]
pub struct Mark {
    pub landmark: String,
    /// The reader, by the label of the reading, whose point alone the mark leaves out.
    /// None for every reading of the drawing.
    pub reader: Option<String>,
}

impl Mark {
    /// Whether the mark leaves a landmark out of a reading by this reader.
    fn leaves_out(&self, reader: &str, name: &str) -> bool {
        self.reader.as_deref().is_none_or(|own| own == reader) && fits(&self.landmark, name)
    }
}

impl std::fmt::Display for Mark {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.reader {
            Some(reader) => write!(formatter, "{} in the reading by {reader}", self.landmark),
            None => formatter.write_str(&self.landmark),
        }
    }
}

/// One character, as the record holds it.
#[derive(Debug, Clone)]
pub struct Checked {
    pub label: String,
    pub verdict: Option<Verdict>,
    pub why: String,
    /// The proportions Jon marks as off for the character, by their names in the list.
    pub proportions_off: Vec<String>,
    pub drawings: Vec<DrawingChecked>,
}

/// Whether a mark fits a landmark's name: the name itself, or with `*` for any run of it.
pub fn fits(mark: &str, name: &str) -> bool {
    let mut parts = mark.split('*');
    let first = parts.next().unwrap_or_default();
    let Some(mut rest) = name.strip_prefix(first) else {
        return false;
    };
    let mut parts: Vec<&str> = parts.collect();
    let Some(last) = parts.pop() else {
        // No `*`: the whole name.
        return rest.is_empty();
    };
    for part in parts {
        match rest.find(part) {
            Some(at) => rest = &rest[at + part.len()..],
            None => return false,
        }
    }
    rest.ends_with(last)
}

fn names(table: &toml::Table, key: &str) -> Vec<String> {
    table
        .get(key)
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|name| name.as_str().map(|name| name.trim().to_string()))
        .collect()
}

/// The marks a drawing's `off` gives: each a landmark's name, or a table that names the
/// `landmark` and the `reader` whose reading alone it is left out of. What is neither is a
/// fault, said with `of`.
fn marks(drawing: &toml::Table, of: &str, faults: &mut Vec<String>) -> Vec<Mark> {
    let entries = drawing
        .get("off")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten();
    let mut marks = Vec::new();
    for (number, entry) in entries.enumerate() {
        let mark = match entry {
            toml::Value::String(landmark) => Some(Mark {
                landmark: landmark.trim().to_string(),
                reader: None,
            }),
            toml::Value::Table(table) => {
                let (landmark, reader) = (words(table, "landmark"), words(table, "reader"));
                (!landmark.is_empty() && !reader.is_empty()).then_some(Mark {
                    landmark,
                    reader: Some(reader),
                })
            }
            _ => None,
        };
        match mark {
            Some(mark) => marks.push(mark),
            None => faults.push(format!(
                "mark {} in the `off` of {of} is not a mark: a mark is a landmark's name, such as \"ear-*\", or {{ landmark = \"brow-line-*\", reader = \"<the reader's label>\" }} for one reader's reading alone",
                number + 1
            )),
        }
    }
    marks
}

/// The record of one check.
#[derive(Debug, Clone)]
pub struct Check {
    /// Which of Jon's looks it was.
    pub look: String,
    pub checked_on: String,
    /// Who wrote the record down from Jon's words.
    pub recorded_by: String,
    /// The sheet description it was started from.
    pub sheet: String,
    pub characters: Vec<Checked>,
    /// What the file gives that is not right in itself.
    faults: Vec<String>,
}

/// One drawing of a sheet, for a record to be started from.
pub struct ToCheck<'a> {
    pub card: View,
    pub picture: String,
    /// Each reading: its path, the digest of its file, and the reading.
    pub readings: Vec<(&'a str, String, &'a Record)>,
}

fn judged(record: &Record) -> String {
    let mut out = format!("a {} view", record.view.name());
    if let Some(faces) = record.faces {
        out.push_str(&format!(" facing {}", faces.words()));
    }
    out.push_str(match record.level {
        Some(true) => ", level",
        Some(false) => ", not level",
        None => ", level not said",
    });
    if record.set_aside {
        out.push_str(", and set it aside");
    }
    out
}

/// A record to fill in: every character of a sheet with its drawings and their readings,
/// and no verdict.
pub fn template(sheet: &str, characters: &[(&str, Vec<ToCheck>)]) -> String {
    let mut out = String::from(
        "\
# The record of Jon's check of a reference check sheet.
#
# Started by `maquette start-jons-check`. Jon says his decisions in words; whoever hears them
# writes them here, and `maquette jons-check` then says what the record does and what it
# still lacks. Only what Jon said is written: nothing here is an agent's judgement.
# The record is private: it stays in the reference folder and is never committed.
#
# - `verdict` is \"accepted\" or \"rejected\", for each character.
# - `ruling` is left empty where Jon goes by the readers. Where he overrules them it is
#   \"measure\" (a drawing a reader set aside is measured after all) or \"set aside\" (a
#   drawing the readers measured is not). With \"measure\", add `view = \"front\"` (or
#   \"three-quarter\" or \"side\") where Jon says the drawing is another view than its card's.
# - `off` lists the landmarks Jon marks as off on that drawing, by the names the sheet gives:
#   each is left out of that drawing, and the drawing is still measured. A name may hold `*`
#   for every landmark it fits: \"ear-*\" is both ears, \"mouth-line-*\" the whole mouth,
#   \"brow-line-*\" both brows. Where Jon says one reader placed a landmark wrongly and the
#   other rightly, the mark names that reader by the label in its reading, and leaves the
#   landmark out of that reading alone: { landmark = \"brow-line-*\", reader = \"<label>\" }.
#   The other reader's point is then measured by itself.
# - `proportions_off` lists the proportions Jon marks as off for the character, by their
#   names in `maquette list-proportions`: each is not given for this work, and its landmarks
#   stay. \"eye_opening_height\" leaves the opening's height out and keeps the eye line.
# - `why` and `ruling_why` are his words, where he gave any.
# - The readings and their digests are what he checked: do not change them. A reading that
#   has changed since is not the reading he accepted.

",
    );
    out.push_str("# Which of Jon's looks this was, such as \"Look 4 of 8\".\nlook = \"\"\n");
    out.push_str("# The day he checked the sheet.\nchecked_on = \"\"\n");
    out.push_str("# Who wrote this record down from his words.\nrecorded_by = \"\"\n");
    out.push_str(&format!(
        "# The sheet description this record was started from.\nsheet = {}\n",
        quoted(sheet)
    ));
    for (label, drawings) in characters {
        out.push_str(&format!(
            "\n[[character]]\nlabel = {}\nverdict = \"\"\nwhy = \"\"\nproportions_off = []\n",
            quoted(label)
        ));
        for drawing in drawings {
            out.push_str(&format!(
                "\n  [[character.drawing]]\n  card = {}\n  picture = {}\n",
                quoted(drawing.card.name()),
                quoted(&drawing.picture)
            ));
            for (_, _, record) in &drawing.readings {
                out.push_str(&format!(
                    "  # {} judged {}.\n",
                    record.reader,
                    judged(record)
                ));
            }
            out.push_str("  ruling = \"\"\n  ruling_why = \"\"\n  off = []\n  readings = [\n");
            for (path, sha256, _) in &drawing.readings {
                out.push_str(&format!(
                    "    {{ path = {}, sha256 = {} }},\n",
                    quoted(path),
                    quoted(sha256)
                ));
            }
            out.push_str("  ]\n");
        }
    }
    out
}

impl Check {
    /// Reads the text of a record. One that is not filled in is still read: what it lacks
    /// is [`Check::review`]'s to say.
    pub fn read(text: &str) -> Result<Check> {
        let file: toml::Table = text.parse().map_err(|error| {
            Error::new(format!(
                "the record of Jon's check is not valid TOML: {error}"
            ))
        })?;
        let mut faults = Vec::new();
        let mut characters = Vec::new();
        for entry in tables(&file, "character") {
            let label = words(entry, "label");
            if label.is_empty() {
                return Err(Error::new(
                    "a [[character]] of the record of Jon's check has no `label`",
                ));
            }
            let verdict = match words(entry, "verdict").as_str() {
                "" => None,
                "accepted" => Some(Verdict::Accepted),
                "rejected" => Some(Verdict::Rejected),
                other => {
                    faults.push(format!(
                        "the verdict on `{label}` is \"{other}\": a verdict is \"accepted\" or \"rejected\""
                    ));
                    None
                }
            };
            let mut drawings = Vec::new();
            for drawing in tables(entry, "drawing") {
                let card = view_named(&words(drawing, "card")).map_err(|error| {
                    Error::new(format!(
                        "a drawing of `{label}` in the record of Jon's check: its `card`: {error}"
                    ))
                })?;
                let of = format!("the {} view of `{label}`", card.name());
                let ruling = match words(drawing, "ruling").as_str() {
                    "" => None,
                    "measure" => Some(Ruling::Measure),
                    "set aside" => Some(Ruling::SetAside),
                    other => {
                        faults.push(format!(
                            "the ruling on {of} is \"{other}\": a ruling is \"measure\" or \"set aside\", or is left empty where Jon goes by the readers"
                        ));
                        None
                    }
                };
                let view = match words(drawing, "view").as_str() {
                    "" => None,
                    named => match view_named(named) {
                        Ok(view) => Some(view),
                        Err(error) => {
                            faults.push(format!("the `view` of {of}: {error}"));
                            None
                        }
                    },
                };
                let off = marks(drawing, &of, &mut faults);
                let readings: Vec<Named> = tables(drawing, "readings")
                    .into_iter()
                    .map(|reading| Named {
                        path: words(reading, "path"),
                        sha256: words(reading, "sha256"),
                    })
                    .collect();
                if readings.is_empty()
                    || readings
                        .iter()
                        .any(|named| named.path.is_empty() || named.sha256.is_empty())
                {
                    return Err(Error::new(format!(
                        "{of} in the record of Jon's check does not name its `readings`, each with its `path` and its `sha256`"
                    )));
                }
                drawings.push(DrawingChecked {
                    card,
                    picture: words(drawing, "picture"),
                    readings,
                    ruling,
                    ruling_why: words(drawing, "ruling_why"),
                    view,
                    off,
                });
            }
            characters.push(Checked {
                label,
                verdict,
                why: words(entry, "why"),
                proportions_off: names(entry, "proportions_off"),
                drawings,
            });
        }
        if characters.is_empty() {
            return Err(Error::new(
                "the record of Jon's check has no [[character]]: `maquette start-jons-check` writes one for each character of a sheet",
            ));
        }
        Ok(Check {
            look: words(&file, "look"),
            checked_on: words(&file, "checked_on"),
            recorded_by: words(&file, "recorded_by"),
            sheet: words(&file, "sheet"),
            characters,
            faults,
        })
    }
}

/// What the record does to one drawing.
#[derive(Debug, Clone)]
pub enum Outcome {
    /// It is measured: its readings taken together, as changed by the rulings and the marks.
    Measured {
        drawing: Box<Drawing>,
        /// How, in words.
        said: String,
    },
    /// It is not measured, and why.
    NotMeasured(String),
}

fn quote(why: &str) -> String {
    if why.is_empty() {
        String::new()
    } else {
        format!(": \"{why}\"")
    }
}

impl DrawingChecked {
    /// The view the drawing counts as: the one Jon names, or its card's.
    pub fn counts_as(&self) -> View {
        self.view.unwrap_or(self.card)
    }

    /// What the record does to the drawing's readings, given as they are in their files and
    /// in the record's order: the rules `as_the_readers_judged`, `measure`, `set_aside` and
    /// `off`.
    pub fn outcome(&self, records: &[Record]) -> Outcome {
        let (mut measured, mut said): (Vec<Record>, String) = match self.ruling {
            Some(Ruling::SetAside) => {
                return Outcome::NotMeasured(format!(
                    "set aside by Jon{}",
                    quote(&self.ruling_why)
                ));
            }
            Some(Ruling::Measure) => {
                let view = self.counts_as();
                let fit: Vec<Record> = records
                    .iter()
                    .filter(|record| record.view == view && !record.placed.is_empty())
                    .cloned()
                    .map(|mut record| {
                        record.set_aside = false;
                        record.level = Some(true);
                        record
                    })
                    .collect();
                if fit.is_empty() {
                    return Outcome::NotMeasured(format!(
                        "Jon rules that it is measured as a {} view{}, and no reading of it places its landmarks as one: it must be read again before it is measured",
                        view.name(),
                        quote(&self.ruling_why)
                    ));
                }
                let readers: Vec<&str> = fit.iter().map(|record| record.reader.as_str()).collect();
                let said = format!(
                    "measured as a {} view by Jon's ruling{}, from the reading by {}{}",
                    view.name(),
                    quote(&self.ruling_why),
                    readers.join(" and by "),
                    match records.len() - fit.len() {
                        0 => "",
                        _ => ", which is the one that places its landmarks as that view",
                    }
                );
                (fit, said)
            }
            None => {
                if let Some(aside) = records
                    .iter()
                    .find(|record| record.set_aside || record.level != Some(true))
                {
                    // What a reading says of an overruling is the reader's word for it: the
                    // ruling counts when this record carries it.
                    return Outcome::NotMeasured(format!(
                        "set aside by {}{}, and this record carries no ruling on it{}",
                        aside.reader,
                        quote(&aside.set_aside_why),
                        if aside.set_aside_overruled {
                            format!(
                                ". The reading says the set-aside was overruled by {}: write `ruling = \"measure\"` in this record if Jon said so",
                                aside.set_aside_overruled_by
                            )
                        } else {
                            String::new()
                        }
                    ));
                }
                if let [first, second] = records
                    && (first.view != second.view || first.faces != second.faces)
                {
                    return Outcome::NotMeasured(
                        "its two readers do not judge the view alike, and Jon gave no ruling on which it is".to_string(),
                    );
                }
                let said = match records.len() {
                    1 => "measured from its one reading, as its reader judged it".to_string(),
                    count => {
                        format!("measured from its {count} readings, as its readers judged it")
                    }
                };
                (records.to_vec(), said)
            }
        };
        // What the marks leave out of each reading: a landmark that one reading still places
        // after them is then that reading's alone, by the rule `off_in_one_reading`.
        let mut marked: Vec<BTreeSet<String>> = Vec::new();
        for record in &mut measured {
            let reader = record.reader.clone();
            let left_out: BTreeSet<String> = record
                .placed
                .keys()
                .filter(|name| self.off.iter().any(|mark| mark.leaves_out(&reader, name)))
                .cloned()
                .collect();
            record.placed.retain(|name, _| !left_out.contains(name));
            marked.push(left_out);
        }
        if !self.off.is_empty() {
            let marks: Vec<String> = self.off.iter().map(Mark::to_string).collect();
            said.push_str(&format!(". Left out by Jon's marks: {}", marks.join(", ")));
        }
        match Drawing::of_marked(&measured, &marked) {
            Ok(drawing) => {
                for words in drawing.left_to_one_words() {
                    said.push_str(&format!(". {}", words.trim_end_matches('.')));
                }
                Outcome::Measured {
                    drawing: Box::new(drawing),
                    said,
                }
            }
            Err(error) => Outcome::NotMeasured(error.to_string()),
        }
    }
}

/// What a record does and what it lacks or gets wrong, worked out once from the readings as
/// they are now. `maquette jons-check` says it, and a measuring command obeys it: each
/// drawing's [`Outcome`] is decided here and nowhere else.
pub struct Review<'a> {
    check: &'a Check,
    /// The record's path in the reference folder.
    path: &'a str,
    /// For each character, in the record's order, what the record does to each drawing.
    outcomes: Vec<Vec<Outcome>>,
    /// What the record lacks or gets wrong. Empty when it is complete.
    pub problems: Vec<String>,
    /// What the record says, for a person: everything but the counts and the problems.
    lines: String,
    /// For each character, its work, and the views measured of it when it is accepted.
    measured_views: Vec<(String, Vec<View>)>,
}

/// A drawing a measuring command measures, as the record rules it.
pub struct Measured<'a> {
    pub character: &'a Checked,
    pub drawing: &'a Drawing,
    /// How it is measured, in words.
    pub said: &'a str,
}

impl Check {
    /// Works out what the record does to each drawing and what it lacks or gets wrong.
    /// `path` is the record's own, for what is said; `reading` gives a reading the record
    /// names as it is now: the digest of its file, and the reading.
    pub fn review<'a>(
        &'a self,
        path: &'a str,
        reading: &dyn Fn(&str) -> Result<(String, Record)>,
    ) -> Review<'a> {
        let mut problems = self.faults.clone();
        for (key, value, what) in [
            (
                "look",
                &self.look,
                "which of Jon's looks this was, such as \"Look 4 of 8\"",
            ),
            (
                "checked_on",
                &self.checked_on,
                "the day he checked the sheet",
            ),
            (
                "recorded_by",
                &self.recorded_by,
                "who wrote the record down from his words",
            ),
        ] {
            if value.is_empty() {
                problems.push(format!("`{key}` is empty: {what}"));
            }
        }
        let or = |value: &str, none: &'static str| -> String {
            if value.is_empty() {
                none.to_string()
            } else {
                value.to_string()
            }
        };
        let mut out = format!(
            "Jon's check `{path}`: {}, checked on {}, recorded by {}.\n",
            or(&self.look, "look not said"),
            or(&self.checked_on, "a day not said"),
            or(&self.recorded_by, "nobody named"),
        );

        let mut outcomes = Vec::new();
        let mut measured_views = Vec::new();
        for character in &self.characters {
            let label = &character.label;
            out.push('\n');
            match character.verdict {
                Some(Verdict::Accepted) => out.push_str(&format!("{label}: accepted")),
                Some(Verdict::Rejected) => out.push_str(&format!("{label}: rejected")),
                None => {
                    out.push_str(&format!("{label}: not yet checked"));
                    problems.push(format!(
                        "`{label}` has no verdict: \"accepted\" or \"rejected\""
                    ));
                }
            }
            if !character.why.is_empty() {
                out.push_str(&format!(": \"{}\"", character.why));
            }
            out.push_str(match character.verdict {
                Some(Verdict::Accepted) => ".\n",
                _ => ". Nothing of it is measured.\n",
            });
            if !character.proportions_off.is_empty() {
                out.push_str(&format!(
                    "  Marked as off by Jon and not given for this work: {}.\n",
                    character.proportions_off.join(", ")
                ));
                match crate::proportions::list() {
                    Ok(list) => {
                        for name in &character.proportions_off {
                            if list.find(name).is_none() {
                                problems.push(format!(
                                    "`{name}` is marked as off for `{label}`, and the list of proportions has no proportion of that name"
                                ));
                            }
                        }
                    }
                    Err(error) => problems.push(error.to_string()),
                }
            }
            let mut views: Vec<View> = Vec::new();
            let mut work = String::new();
            let mut of_character = Vec::new();
            for drawing in &character.drawings {
                let of = format!("the {} view of `{label}`", drawing.card.name());
                let mut records = Vec::new();
                for named in &drawing.readings {
                    match reading(&named.path) {
                        Ok((sha256, record)) => {
                            if sha256 != named.sha256 {
                                problems.push(format!(
                                    "`{}` has changed since the record was started: its digest is {sha256}, and the record names {}. It is not the reading Jon checked",
                                    named.path, named.sha256
                                ));
                            }
                            records.push(record);
                        }
                        Err(error) => problems.push(format!("`{}`: {error}", named.path)),
                    }
                }
                if drawing.view.is_some() && drawing.ruling != Some(Ruling::Measure) {
                    problems.push(format!(
                        "{of} names a `view` and its ruling is not \"measure\": the view is said only where Jon rules that a drawing is measured as another view than its card's"
                    ));
                }
                // A mark names a landmark the drawing's view asks for, placed or not.
                let asked: Vec<String> = records
                    .iter()
                    .flat_map(|record| crate::readings::asked(drawing.counts_as(), record.faces))
                    .map(|asked| asked.name)
                    .collect();
                for mark in &drawing.off {
                    if !asked.iter().any(|name| fits(&mark.landmark, name)) {
                        problems.push(format!(
                            "`{}` is marked as off on {of}, and a {} view asks for no landmark of that name",
                            mark.landmark,
                            drawing.counts_as().name()
                        ));
                    }
                    // A mark on one reader's reading names a reader of the drawing.
                    if let Some(reader) = &mark.reader
                        && !records.iter().any(|record| &record.reader == reader)
                    {
                        let readers: Vec<&str> = records
                            .iter()
                            .map(|record| record.reader.as_str())
                            .collect();
                        problems.push(format!(
                            "`{}` is marked as off on {of} in the reading by `{reader}`, and no reading of it is by that reader: its readers are {}",
                            mark.landmark,
                            readers.join(" and ")
                        ));
                    }
                }
                if let Some(first) = records.first()
                    && work.is_empty()
                {
                    work = first.reference.get("work").cloned().unwrap_or_default();
                }
                let outcome = if records.len() == drawing.readings.len() {
                    drawing.outcome(&records)
                } else {
                    Outcome::NotMeasured("a reading of it could not be read".to_string())
                };
                let words = match &outcome {
                    Outcome::Measured { drawing, said } => {
                        if character.verdict == Some(Verdict::Accepted) {
                            views.push(drawing.view);
                        }
                        said.clone()
                    }
                    Outcome::NotMeasured(why) => format!("not measured: {why}"),
                };
                out.push_str(&format!(
                    "  {}, {}: {words}.\n",
                    drawing.card.name(),
                    or(&drawing.picture, "its picture not named")
                ));
                of_character.push(outcome);
            }
            outcomes.push(of_character);
            // A character with no work named still counts as a work of its own.
            if work.is_empty() {
                work = format!("the work of `{label}`");
            }
            measured_views.push((work, views));
        }
        Review {
            check: self,
            path,
            outcomes,
            problems,
            lines: out,
            measured_views,
        }
    }
}

/// What a count of accepted works is of: a view, or depth from a front view and the view
/// beside it.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Counted {
    View(View),
    Depth(View),
}

impl Counted {
    const ALL: [Counted; 5] = [
        Counted::View(View::Front),
        Counted::View(View::ThreeQuarter),
        Counted::View(View::Side),
        Counted::Depth(View::Side),
        Counted::Depth(View::ThreeQuarter),
    ];

    /// Whether a character whose measured views are these counts for it.
    fn counts(self, measured: &[View]) -> bool {
        match self {
            Counted::View(view) => measured.contains(&view),
            Counted::Depth(beside) => measured.contains(&View::Front) && measured.contains(&beside),
        }
    }

    fn words(self) -> String {
        match self {
            Counted::View(view) => view.name().to_string(),
            Counted::Depth(beside) => format!(
                "depth, from a front and a {} view of one character",
                beside.name()
            ),
        }
    }
}

fn works(count: usize) -> String {
    let thin = if count < WORKS_FOR_A_TABLE {
        format!(": too thin, a view needs {WORKS_FOR_A_TABLE}")
    } else {
        String::new()
    };
    format!("{}{thin}", counted(count, "work"))
}

impl Review<'_> {
    /// Whether the record lacks nothing and gets nothing wrong.
    pub fn complete(&self) -> bool {
        self.problems.is_empty()
    }

    /// What the record says and does, for a person: each character and drawing, the count
    /// of accepted works for each view, and what the record lacks or gets wrong.
    pub fn said(&self) -> String {
        let mut out = self.lines.clone();
        out.push_str(&format!(
            "\nAccepted works for each view, of {} on the sheet:\n",
            counted(self.check.characters.len(), "character")
        ));
        for what in Counted::ALL {
            let mut behind: Vec<&str> = Vec::new();
            for (work, views) in &self.measured_views {
                if what.counts(views) && !behind.contains(&work.as_str()) {
                    behind.push(work);
                }
            }
            out.push_str(&format!("  {}: {}\n", what.words(), works(behind.len())));
        }
        if self.complete() {
            out.push_str("\nThe record is complete.\n");
        } else {
            out.push_str("\nThe record is not complete, so these counts are not final:\n");
            for problem in &self.problems {
                out.push_str(&format!("- {problem}\n"));
            }
        }
        out
    }

    /// The drawing whose readings a measuring command names, as the record rules it: rules
    /// `all_readings` and `accepted_only`. `same` says whether a path the record names and
    /// a path given are one file.
    pub fn measured(
        &self,
        named: &[&str],
        same: &dyn Fn(&str, &str) -> bool,
    ) -> Result<Measured<'_>> {
        let path = self.path;
        let first = named.first().copied().unwrap_or_default();
        let found = self
            .check
            .characters
            .iter()
            .enumerate()
            .find_map(|(at, character)| {
                character
                    .drawings
                    .iter()
                    .enumerate()
                    .find(|(_, drawing)| {
                        drawing
                            .readings
                            .iter()
                            .any(|listed| same(&listed.path, first))
                    })
                    .map(|(drawing_at, drawing)| (at, character, drawing_at, drawing))
            });
        let Some((at, character, drawing_at, drawing)) = found else {
            return Err(Error::new(format!(
                "`{first}` is not a reading of the record of Jon's check `{path}`: with that record, only the readings he checked are measured"
            )));
        };
        let label = &character.label;
        let of = format!("the {} view of `{label}`", drawing.card.name());
        for given in named {
            if !drawing
                .readings
                .iter()
                .any(|listed| same(&listed.path, given))
            {
                return Err(Error::new(format!(
                    "`{given}` is not a reading of the same drawing as `{first}` in the record of Jon's check `{path}`"
                )));
            }
        }
        let not_named: Vec<String> = drawing
            .readings
            .iter()
            .filter(|listed| !named.iter().any(|given| same(&listed.path, given)))
            .map(|listed| format!("`{}`", listed.path))
            .collect();
        if !not_named.is_empty() {
            return Err(Error::new(format!(
                "the record of Jon's check `{path}` lists {} of {of}, and {} is not named: with the record, a drawing is measured from all the readings it lists, as it rules, or not at all. Name them all, with a comma between them",
                counted(drawing.readings.len(), "reading"),
                not_named.join(" and ")
            )));
        }
        match character.verdict {
            Some(Verdict::Accepted) => {}
            Some(Verdict::Rejected) => {
                return Err(Error::new(format!(
                    "Jon rejected the character `{label}`{}, so its readings are not measured",
                    if character.why.is_empty() {
                        String::new()
                    } else {
                        format!(" (\"{}\")", character.why)
                    }
                )));
            }
            None => {
                return Err(Error::new(format!(
                    "the character `{label}` has no verdict in the record of Jon's check `{path}`: it is not yet checked, so its readings are not measured"
                )));
            }
        }
        match &self.outcomes[at][drawing_at] {
            Outcome::Measured { drawing, said } => Ok(Measured {
                character,
                drawing: drawing.as_ref(),
                said,
            }),
            Outcome::NotMeasured(why) => Err(Error::new(format!("{of} is not measured: {why}"))),
        }
    }
}
