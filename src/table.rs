//! The proportion table, and the setting of a set of proportions against one.
//!
//! This module hides what a table is built from, what it states of each proportion, when a
//! proportion has a valid range, and what a set of proportions is told when it is set against
//! one. `maquette table-from-readings` builds a table from the record of Jon's check, and
//! `maquette set-against-table` sets a set of proportions against a table.
//!
//! It also hides how a whole set of proportions is made from a table for a head to be drawn:
//! the table's middle values where it has a range, the average of the characters where two to
//! four give a proportion, and another set's own numbers for the rest, each marked as what it
//! is. `maquette set-from-table` writes one.
//!
//! A table is a proposal until Jon has accepted it. Nothing here judges a head: the program
//! reports what the accepted readings give.

use std::collections::BTreeMap;

use crate::jons_check::{CHARACTERS_FOR_A_TABLE, Count, Outcome, Review, Works, counts};
use crate::known_positions::{NOT_A_READER, dimension};
use crate::measuring::{self, Given, Proportions};
use crate::number::{counted, median, toml as toml_number};
use crate::proportions::{self, Kind};
use crate::readings::{self, Drawing};
use crate::toml_text::{quoted, quoted_lines, tables, words};
use crate::views::View;
use crate::{Error, Result};

/// What a table is and does, by name, as the table's own file states it.
pub const RULES: &[(&str, &str)] = &[
    (
        "built_from",
        "A table is built from the records of Jon's checks and from nothing else: the readings of each character he accepted, each drawing as its record rules it, with what he marked as off left out. A drawing that is set aside or not called level is left out, and so is a character he rejected. A record names its readings from the folder it is in, so the records of several batches, each in its own folder, can build one table.",
    ),
    (
        "read_again",
        "A label that several records name is one character, and its readings must say the same reference in each; a character accepted in one record and rejected in another builds no table. One drawing may be read again under a later revision of the reading instructions, which asks for points the earlier did not: the earlier readings, which Jon accepted first, give every proportion they give, and the later ones only what the earlier were not asked for. Two measured drawings of a view that are two pictures, or one picture read twice under one revision, build no table.",
    ),
    (
        "a_character",
        "A table counts characters: one drawing of each accepted character in each view. Heights and widths are taken from a character's front view, so a character with no measured front view gives nothing. Depth is taken from its side view where one is measured, and otherwise from its three-quarter view at the turn the readings show.",
    ),
    (
        "works",
        "Beside each count of characters stands the number of works they come from, and how many of them the work that gives the most gives, so that a range resting on one hand can be seen: several characters of one work's sheets are drawn by one hand or a few. A work is what the characters' readings name as their work, word for word, with its designer where it is known. Characters whose work is not known are counted under the words their readings give, so they may join drawings by different hands, or part drawings by one.",
    ),
    (
        "values",
        "For each proportion: the number of characters that give it and, where it has a range, the lowest, the middle and the highest value among them. The middle is the median: the middle value in order, or midway between the two middle values of an even number of characters. Every value is in units, a length over the height from the eye line down to the chin.",
    ),
    (
        "floor",
        "A proportion with five characters or more behind it is given: its valid range runs from its lowest value to its highest. A proportion with fewer is too thin: the table says how many characters stand behind it and why the others do not give it, and states no number, because the values of one to four characters are those characters' own numbers. It has no range, and nothing is set against it. The floor is held proportion by proportion. In a view that has five characters, a proportion that hair or a mark of Jon's leaves with fewer is too thin all the same, because a range is backed only by the characters that show that feature. A view with fewer than five characters is reported as too thin and is not a table. The floor counts characters and not works; how many works stand behind a range is stated beside it, and is not a floor.",
    ),
    (
        "age_bands",
        "Where the characters are of more than one age band, as their readings give it, each proportion also states each band's own count of characters and, where five or more of a band give it, that band's own lowest, middle and highest value, with whether the ranges of every two bands overlap. The table's own range is of every character, whatever its band. Where the ranges overlap, that proportion does not tell the bands apart on the characters measured, and where they do not, it does. Neither tests whether the bands differ beyond those characters: ranges that overlap may still be of bands that differ, and ranges that do not may rest on few characters of few works.",
    ),
    (
        "conventions",
        "A convention is built by a stated rule and is never a measurement: it has no valid range, however many characters the rule could be run on, and the table states no number for it.",
    ),
    (
        "separate_ranges",
        "Each range is separate. The table does not rule on which combinations of proportions go together: a head with every proportion inside its range may still be one that no reference shows.",
    ),
    (
        "single_references",
        "A lowest value and a highest value are each one reference's number. The table does not tie them to a work.",
    ),
    (
        "comparison_case",
        "No comparison case's numbers are in a table. The command that builds one takes the record of Jon's check and no set of proportions, reads only from the reference folder, and refuses what is not a reader's reading of a drawing.",
    ),
];

fn rule(name: &str) -> &'static str {
    RULES
        .iter()
        .find(|(known, _)| *known == name)
        .map(|(_, text)| *text)
        .unwrap_or_else(|| panic!("the table has no rule named {name}"))
}

/// Where a proportion stands in a table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Standing {
    /// Five characters or more: its lowest and highest value are its valid range.
    Given,
    /// One to four characters: what they give is stated and is no range.
    TooThin,
    /// No character gives it.
    NoCharacter,
    /// Built by rule and never measured: no range.
    Convention,
}

impl Standing {
    fn of(kind: Kind, characters: usize) -> Standing {
        match (kind, characters) {
            (Kind::Convention, _) => Standing::Convention,
            (Kind::Measured, 0) => Standing::NoCharacter,
            (Kind::Measured, characters) if characters < CHARACTERS_FOR_A_TABLE => {
                Standing::TooThin
            }
            (Kind::Measured, _) => Standing::Given,
        }
    }

    pub fn word(self) -> &'static str {
        match self {
            Standing::Given => "given",
            Standing::TooThin => "too thin",
            Standing::NoCharacter => "no character",
            Standing::Convention => "convention",
        }
    }

    fn named(word: &str) -> Option<Standing> {
        [
            Standing::Given,
            Standing::TooThin,
            Standing::NoCharacter,
            Standing::Convention,
        ]
        .into_iter()
        .find(|standing| standing.word() == word)
    }
}

/// One character a table was measured on, as the table lists it.
#[derive(Debug, Clone)]
struct Character {
    label: String,
    /// What its readings say the reference is, field by field.
    reference: BTreeMap<String, String>,
    /// The views it gives its proportions from: its front view, and the view depth is from.
    /// The counts of each view are of these.
    views: Vec<View>,
    /// Where its depth is taken from, in words.
    depth_from: &'static str,
}

impl Character {
    fn said(&self, field: &str) -> &str {
        self.reference.get(field).map_or("", String::as_str)
    }

    fn work(&self) -> &str {
        self.said("work")
    }
}

/// One character's value of a proportion.
#[derive(Debug, Clone)]
struct Value {
    value: f64,
    /// The work it comes from, as the character's readings name it.
    work: String,
    /// The character's age band, as its readings give it.
    band: String,
}

/// The lowest, the middle and the highest of some values, in order from the lowest, where
/// they are enough to be a range: five or more of a measured proportion. None otherwise: the
/// values of a few characters are those characters' own numbers.
fn range(kind: Kind, numbers: &[f64]) -> Option<[f64; 3]> {
    match (
        Standing::of(kind, numbers.len()),
        numbers.first(),
        numbers.last(),
    ) {
        (Standing::Given, Some(lowest), Some(highest)) => {
            Some([*lowest, median(numbers), *highest])
        }
        _ => None,
    }
}

/// One proportion of a table.
#[derive(Debug, Clone)]
struct Line {
    name: String,
    kind: Kind,
    /// The values the characters give, in order from the lowest.
    values: Vec<Value>,
    /// Why the other characters do not give it, each reason with the number of characters
    /// it is said of.
    not_given: Vec<(String, usize)>,
}

impl Line {
    fn standing(&self) -> Standing {
        Standing::of(self.kind, self.values.len())
    }

    fn numbers(&self) -> Vec<f64> {
        self.values.iter().map(|value| value.value).collect()
    }

    fn works(&self) -> Works {
        Works::of(self.values.iter().map(|value| value.work.as_str()))
    }

    /// The values the characters of one age band give, in order from the lowest.
    fn of_band(&self, band: &str) -> Vec<f64> {
        self.values
            .iter()
            .filter(|value| value.band == band)
            .map(|value| value.value)
            .collect()
    }

    /// Each band's lowest and highest value, where it has a range: the rule `age_bands`.
    fn band_ranges(&self, bands: &[String]) -> Vec<Option<[f64; 2]>> {
        bands
            .iter()
            .map(|band| {
                range(self.kind, &self.of_band(band)).map(|[lowest, _, highest]| [lowest, highest])
            })
            .collect()
    }

    /// Whether the ranges of every two bands overlap, where two bands or more have a range of
    /// their own: the rule `age_bands`. None where fewer do.
    fn bands_overlap(&self, bands: &[String]) -> Option<bool> {
        let ranges: Vec<[f64; 2]> = self.band_ranges(bands).into_iter().flatten().collect();
        (ranges.len() >= 2).then(|| {
            ranges
                .iter()
                .all(|a| ranges.iter().all(|b| a[0] <= b[1] && b[0] <= a[1]))
        })
    }
}

/// A proportion table as it is built from the record of Jon's check.
#[derive(Debug, Clone)]
pub struct Table {
    /// The record it was built from, as [`Review::named`] says it.
    built_from: String,
    characters: Vec<Character>,
    /// The number of accepted characters for each view and each route to depth.
    views: Vec<Count>,
    /// What was accepted and gives nothing, in words: a drawing that is not measured.
    left_out: Vec<String>,
    lines: Vec<Line>,
    /// The age bands of the characters, with how many characters each has: adult first,
    /// then teenage, then any other in order.
    bands: Vec<(String, usize)>,
}

/// The age bands of some characters, with how many characters each has.
fn bands_of(characters: &[Character]) -> Vec<(String, usize)> {
    let mut bands: Vec<(String, usize)> = Vec::new();
    for character in characters {
        let band = character.said("age_band");
        match bands.iter_mut().find(|(known, _)| known == band) {
            Some((_, count)) => *count += 1,
            None => bands.push((band.to_string(), 1)),
        }
    }
    let rank = |band: &str| match band {
        "adult" => 0,
        "teenage" => 1,
        _ => 2,
    };
    bands.sort_by(|(a, _), (b, _)| rank(a).cmp(&rank(b)).then_with(|| a.cmp(b)));
    bands
}

/// The fields of a reference a table lists a character by. Two are left in the readings:
/// where a picture was taken from, since an address can name a character and a table is
/// written to be published, and the reader's own account of the manner it is drawn in, which
/// is one reading's.
const CHARACTER_FIELDS: [&str; 5] = ["work", "designer", "age_band", "sex", "species"];

fn view_counts(characters: &[Character]) -> Vec<Count> {
    let given: Vec<(&str, &[View])> = characters
        .iter()
        .map(|character| (character.work(), character.views.as_slice()))
        .collect();
    counts(&given)
}

/// A character as the records of Jon's checks give it, gathered by its label: a label that
/// several records name is one character, such as one whose side view a later batch read
/// again.
struct Gathered<'a> {
    label: &'a str,
    reference: &'a BTreeMap<String, String>,
    /// The proportions Jon marked as off for it, in any record.
    proportions_off: Vec<String>,
    /// Each drawing measured of it, with the paths of its readings, in the records' order.
    measured: Vec<(String, &'a Drawing)>,
}

/// The characters the records give, each gathered by its label, and what was accepted and
/// gives nothing: the rule `read_again`. A character rejected in one record and accepted in
/// another, or one label whose readings say two different references, is refused.
fn gathered<'a>(reviews: &'a [Review]) -> Result<(Vec<Gathered<'a>>, Vec<String>)> {
    let mut gathered: Vec<Gathered<'a>> = Vec::new();
    let mut left_out = Vec::new();
    let mut not_measured = Vec::new();
    for (at, review) in reviews.iter().enumerate() {
        for accepted in review.accepted() {
            let label = accepted.character.label.as_str();
            if let Some(other) = reviews
                .iter()
                .enumerate()
                .find(|(other, review)| *other != at && review.rejected().contains(&label))
            {
                return Err(Error::new(format!(
                    "the character `{label}` is accepted in the record of Jon's check {} and rejected in the record {}: which verdict stands is not decided, so no table is built",
                    review.named(),
                    other.1.named()
                )));
            }
            let mut measured = Vec::new();
            for (checked, outcome) in &accepted.drawings {
                match outcome {
                    Outcome::NotMeasured(_) => not_measured.push((label, checked.counts_as(), at)),
                    Outcome::Measured { drawing, .. } => {
                        if drawing.readers.iter().any(|reader| reader == NOT_A_READER) {
                            return Err(Error::new(format!(
                                "the {} view of `{label}` is not a reading: it is the positions of a record of known positions, written as a reading, and nobody read them from a drawing. A table is measured on readings of references. What is known beforehand, as a comparison case's numbers are, never enters one",
                                drawing.view.name()
                            )));
                        }
                        let paths: Vec<&str> = checked
                            .readings
                            .iter()
                            .map(|named| named.path.as_str())
                            .collect();
                        measured.push((paths.join(","), drawing.as_ref()));
                    }
                }
            }
            match gathered.iter_mut().find(|known| known.label == label) {
                Some(known) => {
                    if let Some(field) = CHARACTER_FIELDS
                        .into_iter()
                        .find(|field| known.reference.get(*field) != accepted.reference.get(*field))
                    {
                        let said = |reference: &BTreeMap<String, String>| {
                            reference.get(field).cloned().unwrap_or_default()
                        };
                        return Err(Error::new(format!(
                            "the label `{label}` is named in two records whose readings say two things of it: its {field} is \"{}\" in one and \"{}\" in the other. A label names one character in every record",
                            said(known.reference),
                            said(accepted.reference)
                        )));
                    }
                    for name in &accepted.character.proportions_off {
                        if !known.proportions_off.contains(name) {
                            known.proportions_off.push(name.clone());
                        }
                    }
                    known.measured.extend(measured);
                }
                None => gathered.push(Gathered {
                    label,
                    reference: accepted.reference,
                    proportions_off: accepted.character.proportions_off.clone(),
                    measured,
                }),
            }
        }
    }
    // A drawing not measured in one record whose view another record measures: the readings
    // of that record give the view.
    for (label, view, at) in not_measured {
        let elsewhere = gathered
            .iter()
            .find(|known| known.label == label)
            .is_some_and(|known| {
                known
                    .measured
                    .iter()
                    .any(|(_, drawing)| drawing.view == view)
            });
        left_out.push(match elsewhere {
            true => format!(
                "the {} view of `{label}` is not measured by the record of Jon's check {}, and another record's readings of it are",
                view.name(),
                reviews[at].named()
            ),
            false => format!("the {} view of `{label}` is not measured", view.name()),
        });
    }
    Ok((gathered, left_out))
}

/// The drawings of a view that give a character's proportions: none, the one measured, or
/// one drawing read again under later revisions of the reading instructions, the earliest
/// first: the rule `read_again`. Two drawings of a view that are two pictures, or one
/// picture read twice under one revision, are refused: which gives the character's
/// proportions is not decided.
fn of_view<'a>(character: &Gathered<'a>, view: View) -> Result<Vec<(String, &'a Drawing)>> {
    let mut found: Vec<(String, &'a Drawing)> = character
        .measured
        .iter()
        .filter(|(_, drawing)| drawing.view == view)
        .cloned()
        .collect();
    let label = character.label;
    if let Some((_, first)) = found.first()
        && found
            .iter()
            .any(|(_, other)| other.picture.sha256 != first.picture.sha256)
    {
        return Err(Error::new(format!(
            "the character `{label}` has two measured {} views of two pictures: a table takes one drawing of a character in each view, and which of the two it is has not been decided",
            view.name()
        )));
    }
    found.sort_by(|(_, a), (_, b)| readings::revisions_in_order(&a.instructions, &b.instructions));
    if found
        .windows(2)
        .any(|pair| pair[0].1.instructions == pair[1].1.instructions)
    {
        return Err(Error::new(format!(
            "the {} view of the character `{label}` is measured twice under one revision of the reading instructions: a drawing read again gives what its earlier readings were not asked for, and which of two readings under one revision gives it has not been decided",
            view.name()
        )));
    }
    Ok(found)
}

/// The proportions of the profile a drawing's readings were not asked for, by the revision
/// of the reading instructions they follow: the rule `read_again`.
fn not_asked(drawing: &Drawing) -> Vec<&'static str> {
    let asked: Vec<String> =
        readings::asked_under(drawing.view, drawing.faces, &drawing.instructions)
            .into_iter()
            .map(|asked| asked.name)
            .collect();
    crate::head_model::profile_not_asked(&asked)
}

/// The records a table is built from, as the table names them: which look each is of, and
/// the day he checked.
fn records_named(reviews: &[Review]) -> String {
    let named: Vec<String> = reviews.iter().map(Review::named).collect();
    match named.as_slice() {
        [one] => format!("the record of Jon's check {one}"),
        [first @ .., last] => format!(
            "the records of Jon's checks {}, and {last}",
            first.join(", ")
        ),
        [] => "no record".to_string(),
    }
}

/// Builds a table from records of Jon's check that are complete: the rules `built_from`,
/// `a_character`, `values` and `floor`.
pub fn build(reviews: &[Review]) -> Result<Table> {
    if reviews.is_empty() {
        return Err(Error::new(
            "no record of Jon's check is named, and a table is built from the records of his checks and from nothing else",
        ));
    }
    for review in reviews {
        if !review.complete() {
            return Err(Error::new(format!(
                "the record of Jon's check is not complete, so no table is built from it: {}",
                review.problems.join("; ")
            )));
        }
    }
    let list = proportions::list()?;
    let mut characters: Vec<Character> = Vec::new();
    let (gathered, mut left_out) = gathered(reviews)?;
    // For each proportion, the values the characters give and why the others do not.
    let mut values: BTreeMap<&str, Vec<Value>> = BTreeMap::new();
    let mut not_given: BTreeMap<&str, Vec<(String, usize)>> = BTreeMap::new();

    for character in &gathered {
        let label = character.label;
        let of_character = |error: Error| Error::new(format!("the character `{label}`: {error}"));
        let fronts = of_view(character, View::Front)?;
        let Some((front_named, front)) = fronts.first() else {
            left_out.push(format!(
                "`{label}` has no measured front view, and heights and widths are the front view's: nothing is taken from it"
            ));
            continue;
        };
        let sides = of_view(character, View::Side)?;
        let three_quarters = of_view(character, View::ThreeQuarter)?;
        // Depth is taken from the side view where one is measured, and otherwise from the
        // three-quarter view.
        let (beside, depth_from) = match (sides.is_empty(), three_quarters.is_empty()) {
            (false, three_quarter_too) => {
                if !three_quarter_too {
                    left_out.push(format!(
                        "the three-quarter view of `{label}` is not used: its depth is taken from its side view"
                    ));
                }
                (Some((View::Side, &sides)), "its side view")
            }
            (true, false) => (
                Some((View::ThreeQuarter, &three_quarters)),
                "its three-quarter view, at the turn the readings show",
            ),
            (true, true) => (None, "no view: it gives no depth"),
        };
        let take = |(front_named, front): &(String, &Drawing),
                    beside_drawing: Option<&(String, &Drawing)>|
         -> Result<Proportions> {
            let front = Given {
                named: front_named,
                drawing: front,
            };
            let beside_given = beside_drawing.map(|(named, drawing)| Given { named, drawing });
            match (beside.map(|(view, _)| view), beside_given.as_ref()) {
                (Some(View::Side), Some(side)) => measuring::proportions(&front, None, Some(side)),
                (Some(View::ThreeQuarter), Some(turned)) => {
                    measuring::proportions(&front, Some(turned), None)
                }
                _ => measuring::proportions(&front, None, None),
            }
            .map_err(of_character)
        };
        let first_beside = beside.and_then(|(_, drawings)| drawings.first());
        let mut taken = take(&(front_named.clone(), *front), first_beside)?;
        // A drawing read again under a later revision gives what the earlier readings of it
        // were not asked for, and nothing else.
        for later in fronts.iter().skip(1) {
            taken.add_what_is_missing(&take(later, first_beside)?, &not_asked(front));
        }
        if let Some((_, drawings)) = beside {
            let earliest = drawings[0].1;
            for later in drawings.iter().skip(1) {
                taken.add_what_is_missing(
                    &take(&(front_named.clone(), *front), Some(later))?,
                    &not_asked(earliest),
                );
            }
        }
        taken.leave_out(&character.proportions_off);
        let mut views = vec![View::Front];
        if let Some((view, _)) = beside {
            views.push(view);
        }

        let reference = character.reference.clone();
        let work = reference.get("work").cloned().unwrap_or_default();
        if work.is_empty() {
            return Err(Error::new(format!(
                "the readings of `{label}` do not say which work the drawing is from: a table says how many works stand behind each count of characters, so the readings of a character name its work, or say that it is not known"
            )));
        }
        for proportion in &list.proportions {
            let name = proportion.name.as_str();
            if let Some(value) = taken.taken.values.get(name) {
                values.entry(name).or_default().push(Value {
                    value: *value,
                    work: work.clone(),
                    band: reference.get("age_band").cloned().unwrap_or_default(),
                });
            }
        }
        for (name, why) in &taken.taken.not_given {
            let Some(proportion) = list.find(name) else {
                continue;
            };
            let reasons = not_given.entry(proportion.name.as_str()).or_default();
            match reasons.iter_mut().find(|(known, _)| known == why) {
                Some((_, count)) => *count += 1,
                None => reasons.push((why.clone(), 1)),
            }
        }
        characters.push(Character {
            label: label.to_string(),
            reference,
            views,
            depth_from,
        });
    }

    let lines = list
        .proportions
        .iter()
        .map(|proportion| {
            let mut values = values.remove(proportion.name.as_str()).unwrap_or_default();
            values.sort_by(|a, b| a.value.total_cmp(&b.value));
            Line {
                name: proportion.name.clone(),
                kind: proportion.kind,
                values,
                not_given: not_given
                    .remove(proportion.name.as_str())
                    .unwrap_or_default(),
            }
        })
        .collect();
    Ok(Table {
        built_from: records_named(reviews),
        bands: bands_of(&characters),
        views: view_counts(&characters),
        characters,
        left_out,
        lines,
    })
}

fn six(value: f64) -> String {
    toml_number(value, 6)
}

impl Table {
    fn count(&self, standing: Standing) -> usize {
        self.lines
            .iter()
            .filter(|line| line.standing() == standing)
            .count()
    }

    /// The table as the text of its file.
    pub fn file(&self) -> String {
        let mut out = String::from(
            "\
# A proportion table, by `maquette table-from-readings`: for each proportion of the list, the
# number of characters whose accepted readings give it, the works they come from and, where
# five or more characters do, the lowest, the middle and the highest value among them.
# `maquette list-proportions` defines each proportion.
#
# It is a proposal until Jon has accepted it. [rules] says what it is built from, what it
# counts, when a proportion has a valid range, and what the table does not say.
",
        );
        out.push_str("\n[table]\n");
        out.push_str(&format!("built_from = {}\n", quoted(&self.built_from)));
        let works = Works::of(self.characters.iter().map(Character::work));
        out.push_str(&format!("characters = {}\n", self.characters.len()));
        out.push_str(&format!("works = {}\n", works.works));
        out.push_str(&format!(
            "characters_for_a_table = {CHARACTERS_FOR_A_TABLE}\n"
        ));
        out.push_str(&format!(
            "proportions = {}\ngiven = {}\ntoo_thin = {}\nno_character = {}\nconventions = {}\n",
            self.lines.len(),
            self.count(Standing::Given),
            self.count(Standing::TooThin),
            self.count(Standing::NoCharacter),
            self.count(Standing::Convention)
        ));
        if !self.left_out.is_empty() {
            out.push_str(&format!(
                "# Accepted and not measured. `maquette jons-check` says why.\nleft_out = {}\n",
                quoted_lines(&self.left_out)
            ));
        }
        let bands: Vec<String> = self
            .bands
            .iter()
            .map(|(band, characters)| {
                format!("{{ band = {}, characters = {characters} }}", quoted(band))
            })
            .collect();
        out.push_str(&format!("bands = [{}]\n", bands.join(", ")));

        out.push_str("\n[rules]\n");
        for (name, text) in RULES {
            out.push_str(&format!("{name} = {}\n", quoted(text)));
        }

        out.push_str(
            "\n# The accepted characters whose view the table takes, for each view and for depth, which\n# takes two views of one character, with the works they come from. A view with fewer than\n# five characters is too thin, and is not a table.\n",
        );
        for count in &self.views {
            out.push_str(&format!(
                "[[view]]\nview = {}\ncharacters = {}\nworks = {}\nmost_from_one_work = {}\nstanding = {}\n",
                quoted(&count.what),
                count.characters,
                count.works.works,
                count.works.most_from_one,
                quoted(if count.characters < CHARACTERS_FOR_A_TABLE {
                    "too thin: not a table"
                } else {
                    "a table"
                })
            ));
        }

        out.push_str(
            "\n# The characters the table was measured on, as their readings name their work and its\n# designer. Where each picture was taken from is in its readings, in the reference folder.\n",
        );
        for character in &self.characters {
            out.push_str("[[character]]\n");
            out.push_str(&format!("label = {}\n", quoted(&character.label)));
            for field in CHARACTER_FIELDS {
                if let Some(said) = character.reference.get(field) {
                    out.push_str(&format!("{field} = {}\n", quoted(said)));
                }
            }
            let views: Vec<String> = character
                .views
                .iter()
                .map(|view| quoted(view.name()))
                .collect();
            out.push_str(&format!("views = [{}]\n", views.join(", ")));
            out.push_str(&format!("depth_from = {}\n", quoted(character.depth_from)));
        }

        out.push_str(
            "\n# Every proportion of the list, in the list's order. Only a proportion whose standing\n# is \"given\" has a valid range.\n",
        );
        for line in &self.lines {
            out.push_str("[[proportion]]\n");
            out.push_str(&format!("name = {}\n", quoted(&line.name)));
            out.push_str(&format!("kind = {}\n", quoted(line.kind.word())));
            out.push_str(&format!(
                "dimension = {}\n",
                quoted(dimension(&line.name).trim_end_matches('s'))
            ));
            out.push_str(&format!("characters = {}\n", line.values.len()));
            let works = line.works();
            out.push_str(&format!(
                "works = {}\nmost_from_one_work = {}\n",
                works.works, works.most_from_one
            ));
            out.push_str(&format!("standing = {}\n", quoted(line.standing().word())));
            // Only a range is stated in numbers: the values of a few characters are those
            // characters' own numbers.
            if let Some([lowest, middle, highest]) = range(line.kind, &line.numbers()) {
                out.push_str(&format!("lowest = {}\n", six(lowest)));
                out.push_str(&format!("middle = {}\n", six(middle)));
                out.push_str(&format!("highest = {}\n", six(highest)));
            }
            if !line.not_given.is_empty() {
                let reasons: Vec<String> = line
                    .not_given
                    .iter()
                    .map(|(why, count)| format!("{}: {why}", counted(*count, "character")))
                    .collect();
                out.push_str(&format!("not_given_by = {}\n", quoted_lines(&reasons)));
            }
            out.push_str(&self.bands_of_line(line));
        }
        out
    }

    /// The names of the table's age bands, in its order.
    fn band_names(&self) -> Vec<String> {
        self.bands.iter().map(|(band, _)| band.clone()).collect()
    }

    /// Each age band's own count and range of a proportion, where the characters are of more
    /// than one band, and whether the bands' ranges overlap: the rule `age_bands`.
    fn bands_of_line(&self, line: &Line) -> String {
        if self.bands.len() < 2 {
            return String::new();
        }
        let names = self.band_names();
        let mut out = String::new();
        if let Some(overlap) = line.bands_overlap(&names) {
            out.push_str(&format!("bands_overlap = {overlap}\n"));
        }
        let bands: Vec<String> = names
            .iter()
            .map(|band| {
                let numbers = line.of_band(band);
                let stated = match range(line.kind, &numbers) {
                    Some([lowest, middle, highest]) => format!(
                        ", lowest = {}, middle = {}, highest = {}",
                        six(lowest),
                        six(middle),
                        six(highest)
                    ),
                    None => String::new(),
                };
                format!(
                    "{{ band = {}, characters = {}{stated} }}",
                    quoted(band),
                    numbers.len()
                )
            })
            .collect();
        out.push_str(&format!("bands = [{}]\n", bands.join(", ")));
        out
    }

    /// What the bands' ranges show, for a person: in how many proportions each band has a
    /// range of its own, and in which of them the ranges do not overlap.
    fn bands_said(&self) -> String {
        if self.bands.len() < 2 {
            return String::new();
        }
        let names = self.band_names();
        let counts: Vec<String> = self
            .bands
            .iter()
            .map(|(band, characters)| format!("{band}, {}", counted(*characters, "character")))
            .collect();
        let compared: Vec<(&str, bool)> = self
            .lines
            .iter()
            .filter_map(|line| Some((line.name.as_str(), line.bands_overlap(&names)?)))
            .collect();
        let apart: Vec<&str> = compared
            .iter()
            .filter(|(_, overlap)| !overlap)
            .map(|(name, _)| *name)
            .collect();
        let apart_said = match apart.as_slice() {
            [] => "overlap in every one: on these characters no proportion tells the bands apart."
                .to_string(),
            names => format!(
                "overlap in {}, and do not overlap in {}: {}. Those are the proportions that tell the bands apart on these characters.",
                compared.len() - names.len(),
                names.len(),
                names.join(", ")
            ),
        };
        format!(
            "Age bands, side by side: {}. Every band has a range of its own in {}; there the bands' ranges {apart_said}\n",
            counts.join("; "),
            counted(compared.len(), "proportion")
        )
    }

    /// What was built, in a few lines, for a person.
    pub fn summary(&self) -> String {
        let works = Works::of(self.characters.iter().map(Character::work));
        let mut out = format!(
            "A table of {} proportions from {} of {}, by {}.\n",
            self.lines.len(),
            counted(self.characters.len(), "character"),
            counted(works.works, "work"),
            self.built_from
        );
        out.push_str(&format!(
            "  {} given: five characters or more, with a valid range.\n  {} too thin: one to four characters, and no range.\n  {} with no character.\n  {}, built by rule and never measured: no range.\n",
            self.count(Standing::Given),
            self.count(Standing::TooThin),
            self.count(Standing::NoCharacter),
            counted(self.count(Standing::Convention), "convention")
        ));
        out.push_str("Accepted characters for each view:\n");
        for count in &self.views {
            out.push_str(&format!("  {}: {}\n", count.what, count.said()));
        }
        for left_out in &self.left_out {
            out.push_str(&format!("Left out: {left_out}.\n"));
        }
        out.push_str(&self.bands_said());
        out.push_str(&format!(
            "{}\n{}\n",
            rule("separate_ranges"),
            rule("single_references")
        ));
        out.push_str("It is a proposal until Jon has accepted it.\n");
        out
    }
}

// ---------------------------------------------------------------- a whole set from a table

/// How a whole set of proportions is made from a table, by name, as the set's file states it.
pub const SET_RULES: &[(&str, &str)] = &[
    (
        "the_table",
        "A proportion with a valid range takes the table's middle value: the median of the characters that give it.",
    ),
    (
        "a_thin_average",
        "A proportion that two to four characters give takes the average of those characters, and is marked as thin: it has no range. It is the average and not the middle, because the middle of three characters is one character's own number.",
    ),
    (
        "borrowed",
        "Every other proportion is borrowed from the set named as `borrowed_from`, as it stands there unless the rule `hangs_on` places it: one that a single character gives, since that is one reference's number; one that no character gives; and a convention, which is never measured.",
    ),
    (
        "taken_whole",
        "An ear and the neck are each taken whole. An ear is one form built on its two base points, and the neck one column: its width, where it ends and how far back it stands belong together, and a width from one source on a depth from another sets the head forward on a post. So unless the table and the thin averages give every one of a form's proportions, all of them are borrowed.",
    ),
    (
        "hangs_on",
        "A borrowed point that hangs on a measured one keeps its offset from it. The jaw point hangs on the chin: it is borrowed as how far the borrowed set's jaw point stands from that set's own chin, in height, in half-width and in depth, and is placed from this set's chin. Every chin is one unit below the eye line and on the centre line, so the jaw point's height and half-width come over as they stand; its depth is this set's chin's depth plus the borrowed offset. Without this a jaw point borrowed as it stands is set in front of or behind a chin that was measured elsewhere, and the chin of a turned view is lopsided.",
    ),
    (
        "no_correction",
        "The set carries no correction, whatever the set it borrows from carries. A correction's amounts are worked out from a set's own depths, and whether a head is drawn with one is Jon's to say.",
    ),
];

/// The forms that are taken whole from one source, each by the start of its proportions'
/// names, with what it is called: the rule `taken_whole`.
const TAKEN_WHOLE: &[(&str, &str)] = &[("ear_", "the ear"), ("neck_", "the neck")];

/// How the jaw point hangs on the chin, as a proportion borrowed that way says it.
const ON_THE_CHIN: &str = "as its offset from the chin, on which the jaw point hangs";

/// The borrowed points that hang on another point, proportion by proportion: the rule
/// `hangs_on`. Each names the proportion of the point it hangs on that it keeps its offset
/// from, or none where that point stands at the same place in every head, and how it hangs.
///
/// The jaw point hangs on the chin. Every chin is one unit below the eye line and on the
/// centre line, so the jaw point's height and half-width keep their offset as they stand.
/// The chin's depth differs from head to head, so the jaw point's depth moves with it.
const HANGS_ON: &[(&str, Option<&str>, &str)] = &[
    ("jaw_drop", None, ON_THE_CHIN),
    ("jaw_half_width", None, ON_THE_CHIN),
    ("jaw_depth", Some("chin_depth"), ON_THE_CHIN),
];

/// Where a proportion of a whole set made for a head to be drawn comes from: a set made from a
/// table, or from one structure reference.
#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    /// The table's middle value, with the characters behind it.
    Table(usize),
    /// The average of two to four characters.
    ThinAverage(usize),
    /// The number the structure reference's readings give, as it stands.
    Reference,
    /// The borrowed set's own number, and why.
    Borrowed(String),
    /// Borrowed as an offset from the point it hangs on, by the rule `hangs_on`: how it
    /// hangs, and why it is borrowed at all.
    BorrowedOffset(&'static str, String),
    /// Left out of the set: a proportion a set may leave out, which nothing gives to draw and
    /// the borrowed set leaves out too. Why it would have been borrowed.
    LeftOut(String),
}

impl Source {
    pub(crate) fn is_borrowed(&self) -> bool {
        matches!(self, Source::Borrowed(_) | Source::BorrowedOffset(..))
    }

    pub(crate) fn is_left_out(&self) -> bool {
        matches!(self, Source::LeftOut(_))
    }

    /// Which source gives the number, for the rule `taken_whole`: the table's characters, the
    /// structure reference, or the borrowed set. None for a proportion left out.
    fn giver(&self) -> Option<u8> {
        match self {
            Source::Table(_) | Source::ThinAverage(_) => Some(0),
            Source::Reference => Some(1),
            Source::Borrowed(_) | Source::BorrowedOffset(..) => Some(2),
            Source::LeftOut(_) => None,
        }
    }

    /// The source as the set's file says it. What stands before the colon is the kind, which
    /// [`Source::kind_of`] reads back.
    pub(crate) fn words(&self) -> String {
        match self {
            Source::Table(characters) => {
                format!("the table: the middle of {characters} characters")
            }
            Source::ThinAverage(characters) => {
                format!("a thin average: of {characters} characters")
            }
            Source::Reference => "the structure reference: as its readings give it".to_string(),
            Source::Borrowed(why) => format!("borrowed: {why}"),
            Source::BorrowedOffset(how, why) => format!("borrowed: {how}; {why}"),
            Source::LeftOut(why) => {
                format!("left out: {why}, and the set it would be borrowed from leaves it out")
            }
        }
    }

    /// The kind of source a set's file states for a proportion: "the table", "a thin
    /// average", "the structure reference", "borrowed" or "left out". None for anything else.
    pub fn kind_of(words: &str) -> Option<&'static str> {
        let kind = words.split(':').next().unwrap_or_default().trim();
        [
            "the table",
            "a thin average",
            "the structure reference",
            "borrowed",
            "left out",
        ]
        .into_iter()
        .find(|known| *known == kind)
    }
}

/// A borrowed number, or none where the borrowed set leaves out a proportion that a set may
/// leave out: it is then left out of the set being made too.
pub(crate) fn borrow(
    borrowed: &proportions::Set,
    name: &str,
    why: String,
) -> (Option<f64>, Source) {
    match borrowed.given(name) {
        Some(value) => (Some(value), Source::Borrowed(why)),
        None => (None, Source::LeftOut(why)),
    }
}

/// A whole set made for a head to be drawn, once the rules every such set shares are applied.
pub(crate) struct Finished {
    /// Each proportion's source, in the list's order.
    pub sources: Vec<(String, Source)>,
    pub set: proportions::Set,
    /// What the list's rule for `lower_face_set_back` would set back on the set's own depths
    /// at a share of 1: the chin and the mouth, in units. None where the rule cannot be run.
    pub rule_would_set_back: Option<[f64; 2]>,
}

/// Applies the rules `taken_whole` and `hangs_on` to a whole set being made, each proportion
/// with its number and its source, and makes the set. `givers` says who did not give all of a
/// form that is borrowed whole: "the characters do", "the structure reference does".
pub(crate) fn finish(
    mut made: Vec<(String, Option<f64>, Source)>,
    borrowed: &proportions::Set,
    givers: &str,
) -> Result<Finished> {
    // The rule `taken_whole`: a form whose numbers come from more than one source is borrowed
    // whole. A proportion left out does not take a form from the borrowed set: what a head
    // drawn without it is drawn with, the list's `when_left_out`, rests on the form's other
    // proportions alone.
    for (start, form) in TAKEN_WHOLE {
        let whole = |name: &str| name.starts_with(start);
        let mut sources: Vec<u8> = made
            .iter()
            .filter(|(name, _, _)| whole(name))
            .filter_map(|(_, _, source)| source.giver())
            .collect();
        sources.sort_unstable();
        sources.dedup();
        if sources.len() > 1 {
            let why = format!("{form} is taken whole, and {givers} not give all of it");
            for (name, value, source) in &mut made {
                if whole(name) && !source.is_borrowed() && !source.is_left_out() {
                    (*value, *source) = borrow(borrowed, name, why.clone());
                }
            }
        }
    }
    // The rule `hangs_on`.
    for (name, anchor, how) in HANGS_ON {
        let anchored = anchor.and_then(|anchor| {
            let (_, at, _) = made.iter().find(|(known, _, _)| known == anchor)?;
            Some((*at)? - borrowed.get(anchor))
        });
        if let Some((_, Some(value), source)) = made.iter_mut().find(|(known, _, _)| known == name)
            && let Source::Borrowed(why) = source
        {
            // The borrowed number, moved by as far as this set's anchor stands from the
            // borrowed set's: the point keeps its offset from what it hangs on.
            *value += anchored.unwrap_or(0.0);
            *source = Source::BorrowedOffset(how, std::mem::take(why));
        }
    }
    let set = proportions::Set::from_values(
        made.iter()
            .filter_map(|(name, value, _)| Some((name.clone(), (*value)?)))
            .collect(),
    )?;
    const SET_BACK: &str = "lower_face_set_back";
    let rule_would_set_back = crate::head_model::with_lower_face_set_back(set.clone(), 1.0)
        .ok()
        .and_then(|with| {
            let amounts = with.correction(SET_BACK)?;
            Some([*amounts.get("chin")?, *amounts.get("mouth")?])
        });
    Ok(Finished {
        sources: made
            .into_iter()
            .map(|(name, _, source)| (name, source))
            .collect(),
        set,
        rule_would_set_back,
    })
}

/// What a whole set made for a head to be drawn says of a correction: that it carries none,
/// and what the list's rule would give on its own depths.
pub(crate) fn correction_words(rule_would_set_back: Option<[f64; 2]>) -> String {
    let carried = "The set carries no correction.";
    match rule_would_set_back {
        Some([chin, mouth]) if chin == 0.0 && mouth == 0.0 => format!(
            "{carried} Run on this set's own depths, the list's rule for `lower_face_set_back` sets nothing back at any share: the chin does not stand in front of the iris's centre, and the mouth does not stand in front of the line from the nose mark's bottom to the chin."
        ),
        Some([chin, mouth]) => format!(
            "{carried} Run on this set's own depths, the list's rule for `lower_face_set_back` would set the chin back by {} of a unit at a share of 1, and the mouth by {}. That is not carried.",
            six(chin),
            six(mouth)
        ),
        None => format!(
            "{carried} The list's rule for `lower_face_set_back` cannot be run on this set: its mouth does not stand between its nose mark and its chin."
        ),
    }
}

/// A whole set of proportions made from a table, with where each number comes from.
#[derive(Debug, Clone)]
pub struct SetFromTable {
    built_from: String,
    borrowed_from: String,
    /// Each proportion's source, in the list's order.
    sources: Vec<(String, Source)>,
    set: proportions::Set,
    /// What the list's rule for `lower_face_set_back` would set back on this set's own
    /// depths at a share of 1: the chin and the mouth, in units. None where the rule cannot
    /// be run on the set.
    rule_would_set_back: Option<[f64; 2]>,
}

impl Table {
    /// Makes a whole set of proportions from the table: the rules of [`SET_RULES`].
    /// `borrowed` is the set whose numbers stand in where the characters give none to draw, and
    /// `borrowed_from` is what the file calls it.
    pub fn set_with(
        &self,
        borrowed_from: &str,
        borrowed: &proportions::Set,
    ) -> Result<SetFromTable> {
        let borrow = |name: &str, why: &str| borrow(borrowed, name, why.to_string());
        let made: Vec<(String, Option<f64>, Source)> = self
            .lines
            .iter()
            .map(|line| {
                let numbers = line.numbers();
                let characters = numbers.len();
                let (value, source) = match line.standing() {
                    Standing::Given => (Some(median(&numbers)), Source::Table(characters)),
                    Standing::TooThin if characters >= 2 => (
                        Some(numbers.iter().sum::<f64>() / characters as f64),
                        Source::ThinAverage(characters),
                    ),
                    Standing::TooThin => borrow(
                        &line.name,
                        "one character gives it, and one reference's number is not drawn",
                    ),
                    Standing::NoCharacter => borrow(&line.name, "no character gives it"),
                    Standing::Convention => {
                        borrow(&line.name, "a convention, which is never measured")
                    }
                };
                (line.name.clone(), value, source)
            })
            .collect();
        let finished = finish(made, borrowed, "the characters do")?;
        Ok(SetFromTable {
            built_from: self.built_from.clone(),
            borrowed_from: borrowed_from.to_string(),
            sources: finished.sources,
            set: finished.set,
            rule_would_set_back: finished.rule_would_set_back,
        })
    }

    /// What the table was built from, in words.
    pub fn built_from(&self) -> &str {
        &self.built_from
    }

    /// A proportion's middle value and the characters behind it, where the table gives it a
    /// valid range. None where it is too thin, given by no character, a convention, or not in
    /// the table.
    pub fn middle(&self, name: &str) -> Option<(f64, usize)> {
        let line = self.lines.iter().find(|line| line.name == name)?;
        let numbers = line.numbers();
        matches!(line.standing(), Standing::Given).then(|| (median(&numbers), numbers.len()))
    }
}

impl SetFromTable {
    fn count(&self, of: impl Fn(&Source) -> bool) -> usize {
        self.sources.iter().filter(|(_, source)| of(source)).count()
    }

    /// What the set says of a correction: that it carries none, and what the list's rule
    /// would give on its own depths.
    fn correction(&self) -> String {
        correction_words(self.rule_would_set_back)
    }

    /// The set as the text of a file that `maquette draw` reads.
    pub fn file(&self) -> Result<String> {
        let mut opening = String::from(
            "\
# A whole set of proportions made from a proportion table, by `maquette set-from-table`: the
# table's middle values where it has a range, the average of the characters where two to four give
# a proportion, and another set's own numbers for the rest. [sources] says which each is.
#
# It is one head to be drawn and looked at. It is no proportion table and no part of one, and
# what it borrows is not measured on any reference. `maquette draw` reads this file as it
# reads any set of proportions, and `maquette list-proportions` defines each proportion.
",
        );
        opening.push_str("\n[source]\n");
        opening.push_str(&format!(
            "table = {}\n",
            quoted(&format!("built from {}", self.built_from))
        ));
        opening.push_str(&format!(
            "borrowed_from = {}\n",
            quoted(&self.borrowed_from)
        ));
        opening.push_str(&format!(
            "from_the_table = {}\nthin_averages = {}\nborrowed = {}\nleft_out = {}\n",
            self.count(|source| matches!(source, Source::Table(_))),
            self.count(|source| matches!(source, Source::ThinAverage(_))),
            self.count(Source::is_borrowed),
            self.count(Source::is_left_out)
        ));
        opening.push_str(&format!("correction = {}\n", quoted(&self.correction())));
        opening.push_str("\n[rules]\n");
        for (name, text) in SET_RULES {
            opening.push_str(&format!("{name} = {}\n", quoted(text)));
        }
        opening.push_str("\n# Where each proportion comes from, in the list's order.\n[sources]\n");
        for (name, source) in &self.sources {
            opening.push_str(&format!("{name} = {}\n", quoted(&source.words())));
        }
        self.set.write(&opening)
    }

    /// What was made, in a few lines, for a person.
    pub fn summary(&self) -> String {
        let left_out = self.count(Source::is_left_out);
        format!(
            "A set of {} proportions: {} from the table's middle values, {} thin averages of two to four characters, {} borrowed from `{}`.{}\n{}\n",
            self.sources.len() - left_out,
            self.count(|source| matches!(source, Source::Table(_))),
            self.count(|source| matches!(source, Source::ThinAverage(_))),
            self.count(Source::is_borrowed),
            self.borrowed_from,
            match left_out {
                0 => String::new(),
                left_out => format!(
                    " {} left out: a set may leave them out, and neither the characters nor `{}` give them.",
                    counted(left_out, "proportion"),
                    self.borrowed_from
                ),
            },
            self.correction()
        )
    }
}

// ---------------------------------------------------------------- a set against a table

/// What a set of proportions is told when it is set against a table, by name, as the file
/// of the comparison states it.
pub const AGAINST_RULES: &[(&str, &str)] = &[
    (
        "inside",
        "A proportion is inside its valid range when its value is not below the table's lowest value and not above its highest. A value on either end is inside.",
    ),
    (
        "outside",
        "A proportion is outside when it is below the lowest value or above the highest. How far beyond is stated in units, and as a share of the set's own head height, from the hair's top to the chin.",
    ),
    (
        "no_range",
        "A proportion has no range to be set against when the table has fewer than five characters behind it, or none, or when it is a convention. What the few characters give stands beside a thin one, and is no verdict.",
    ),
    (
        "a_finding",
        "Outside is a finding, not a fault, and inside is no praise: the ranges are separate and do not rule on which combinations go together. Nothing in the set and nothing in the table is changed to bring a set inside.",
    ),
    (
        "corrections",
        "A named correction a set carries is no proportion, and a table has no range for it.",
    ),
];

/// One proportion as a table's file states it.
#[derive(Debug, Clone)]
struct Stated {
    standing: Standing,
    characters: usize,
    /// The lowest, the middle and the highest value, where any work gives it.
    values: Option<[f64; 3]>,
}

fn number_of(table: &toml::Table, key: &str) -> Option<f64> {
    match table.get(key) {
        Some(toml::Value::Float(number)) => Some(*number),
        Some(toml::Value::Integer(number)) => Some(*number as f64),
        _ => None,
    }
}

/// Reads a table's file: what it was built from, and each proportion of the list as it
/// states it. A file that calls a proportion given without five characters and a range behind it
/// is refused: a range is never taken from a thin proportion, whatever a file says.
fn read_table(path: &str, text: &str) -> Result<(String, BTreeMap<String, Stated>)> {
    let not_a_table = |why: String| {
        Error::new(format!(
            "{path} is not a proportion table as `maquette table-from-readings` writes one: {why}"
        ))
    };
    let file: toml::Table = text
        .parse()
        .map_err(|error| not_a_table(format!("it is not valid TOML: {error}")))?;
    let Some(head) = file.get("table").and_then(toml::Value::as_table) else {
        return Err(not_a_table("it has no [table]".to_string()));
    };
    let list = proportions::list()?;
    let mut stated = BTreeMap::new();
    for entry in tables(&file, "proportion") {
        let name = words(entry, "name");
        let Some(proportion) = list.find(&name) else {
            return Err(not_a_table(format!(
                "it has an entry `{name}`, and the list of proportions has no proportion of that name"
            )));
        };
        let Some(standing) = Standing::named(&words(entry, "standing")) else {
            return Err(not_a_table(format!(
                "the standing of `{name}` is \"{}\"",
                words(entry, "standing")
            )));
        };
        let characters = entry
            .get("characters")
            .and_then(toml::Value::as_integer)
            .and_then(|characters| usize::try_from(characters).ok())
            .ok_or_else(|| {
                not_a_table(format!(
                    "`{name}` does not say how many characters stand behind it"
                ))
            })?;
        let values = match (
            number_of(entry, "lowest"),
            number_of(entry, "middle"),
            number_of(entry, "highest"),
        ) {
            (Some(lowest), Some(middle), Some(highest)) => Some([lowest, middle, highest]),
            _ => None,
        };
        // What the file calls a proportion is what its kind and its count of characters make it.
        if standing != Standing::of(proportion.kind, characters) {
            return Err(not_a_table(format!(
                "it calls `{name}` \"{}\" with {} behind it, and a {} proportion with that many is \"{}\"",
                standing.word(),
                counted(characters, "character"),
                proportion.kind.word(),
                Standing::of(proportion.kind, characters).word()
            )));
        }
        if standing == Standing::Given
            && !values.is_some_and(|[lowest, _, highest]| lowest <= highest)
        {
            return Err(not_a_table(format!(
                "it calls `{name}` given and states no lowest and highest value for it"
            )));
        }
        stated.insert(
            name,
            Stated {
                standing,
                characters,
                values,
            },
        );
    }
    // A table built before the list had a proportion that a set may leave out has no entry
    // for it, and no work of that table gives it.
    for proportion in &list.proportions {
        if proportion.optional() && !stated.contains_key(&proportion.name) {
            stated.insert(
                proportion.name.clone(),
                Stated {
                    standing: Standing::NoCharacter,
                    characters: 0,
                    values: None,
                },
            );
        }
    }
    if let Some(missing) = list
        .proportions
        .iter()
        .find(|proportion| !stated.contains_key(&proportion.name))
    {
        return Err(not_a_table(format!(
            "it has no entry for `{}`, a proportion of the list",
            missing.name
        )));
    }
    Ok((words(head, "built_from"), stated))
}

/// What a set's value is told against a table's proportion.
#[derive(Debug, Clone, PartialEq)]
pub enum Verdict {
    Inside,
    /// Below the lowest value or above the highest, and how far beyond, in units.
    Outside {
        above: bool,
        beyond: f64,
    },
    /// No range to be set against, and why.
    NoRange(String),
}

/// One proportion of a set, set against a table.
#[derive(Debug, Clone)]
struct Against {
    name: String,
    value: f64,
    verdict: Verdict,
    characters: usize,
    values: Option<[f64; 3]>,
}

/// A set of proportions set against a table.
#[derive(Debug, Clone)]
pub struct SetAgainst {
    set: String,
    table: String,
    built_from: String,
    /// The set's own head height, where it gives the hair's top.
    head_height: Option<f64>,
    lines: Vec<Against>,
    not_in_the_set: Vec<String>,
    corrections: Vec<String>,
}

/// Sets a set of proportions against a table: the rules `inside`, `outside` and `no_range`.
/// The set need not be whole: a proportion it does not give is listed. Nothing is written
/// back into either.
pub fn set_against(
    set_path: &str,
    set_text: &str,
    table_path: &str,
    table_text: &str,
) -> Result<SetAgainst> {
    let (built_from, stated) = read_table(table_path, table_text)?;
    let list = proportions::list()?;
    let file: toml::Table = set_text
        .parse()
        .map_err(|error| Error::new(format!("{set_path}: not valid TOML: {error}")))?;
    let given = file
        .get("proportions")
        .and_then(toml::Value::as_table)
        .ok_or_else(|| Error::new(format!("{set_path} has no [proportions] table")))?;
    if let Some(unknown) = given.keys().find(|name| list.find(name).is_none()) {
        return Err(Error::new(format!(
            "{set_path} gives `{unknown}`, and the list of proportions has no proportion of that name. `maquette list-proportions` defines each."
        )));
    }
    let mut lines = Vec::new();
    let mut not_in_the_set = Vec::new();
    for proportion in &list.proportions {
        let name = &proportion.name;
        if !given.contains_key(name) {
            not_in_the_set.push(name.clone());
            continue;
        }
        let value = number_of(given, name)
            .filter(|value| value.is_finite())
            .ok_or_else(|| {
                Error::new(format!(
                    "{set_path}: the proportion `{name}` is not a number"
                ))
            })?;
        let stated = &stated[name];
        let verdict = match (stated.standing, stated.values) {
            (Standing::Given, Some([lowest, _, highest])) => {
                if value < lowest {
                    Verdict::Outside {
                        above: false,
                        beyond: lowest - value,
                    }
                } else if value > highest {
                    Verdict::Outside {
                        above: true,
                        beyond: value - highest,
                    }
                } else {
                    Verdict::Inside
                }
            }
            (Standing::Convention, _) => Verdict::NoRange(
                "a convention: it is built by rule, never measured, and has no valid range"
                    .to_string(),
            ),
            (Standing::TooThin, _) => Verdict::NoRange(format!(
                "too thin: {} behind it, and a range needs {CHARACTERS_FOR_A_TABLE}",
                match stated.characters {
                    1 => "1 character stands".to_string(),
                    characters => format!("{characters} characters stand"),
                }
            )),
            (Standing::NoCharacter | Standing::Given, _) => {
                Verdict::NoRange("no character gives it".to_string())
            }
        };
        lines.push(Against {
            name: name.clone(),
            value,
            verdict,
            characters: stated.characters,
            values: stated.values,
        });
    }
    let corrections = file
        .get("corrections")
        .and_then(toml::Value::as_table)
        .map(|corrections| corrections.keys().cloned().collect())
        .unwrap_or_default();
    Ok(SetAgainst {
        set: set_path.to_string(),
        table: table_path.to_string(),
        built_from,
        head_height: number_of(given, "hair_top_rise").map(crate::head_model::head_height),
        lines,
        not_in_the_set,
        corrections,
    })
}

impl SetAgainst {
    fn count(&self, of: impl Fn(&Verdict) -> bool) -> usize {
        self.lines.iter().filter(|line| of(&line.verdict)).count()
    }

    fn percent(&self, beyond: f64) -> Option<f64> {
        self.head_height
            .map(|head_height| 100.0 * beyond / head_height)
    }

    /// The comparison as the text of a file.
    pub fn file(&self) -> String {
        let mut out = String::from(
            "\
# A set of proportions set against a proportion table, by `maquette set-against-table`: each
# proportion of the set as inside or outside its valid range, or as having no range to be set
# against. [rules] says what each verdict means. It reports and changes nothing.
",
        );
        out.push_str("\n[source]\n");
        out.push_str(&format!("proportions = {}\n", quoted(&self.set)));
        out.push_str(&format!("table = {}\n", quoted(&self.table)));
        out.push_str(&format!(
            "table_built_from = {}\n",
            quoted(&self.built_from)
        ));
        if let Some(head_height) = self.head_height {
            out.push_str(&format!("head_height = {}\n", six(head_height)));
        }
        if !self.corrections.is_empty() {
            let named: Vec<String> = self.corrections.iter().map(|name| quoted(name)).collect();
            out.push_str(&format!(
                "corrections_not_set_against = [{}]\n",
                named.join(", ")
            ));
        }
        out.push_str("\n[summary]\n");
        out.push_str(&format!("set_against = {}\n", self.lines.len()));
        out.push_str(&format!(
            "inside = {}\noutside = {}\nno_range = {}\nnot_in_the_set = {}\n",
            self.count(|verdict| *verdict == Verdict::Inside),
            self.count(|verdict| matches!(verdict, Verdict::Outside { .. })),
            self.count(|verdict| matches!(verdict, Verdict::NoRange(_))),
            self.not_in_the_set.len()
        ));
        if !self.not_in_the_set.is_empty() {
            let named: Vec<String> = self
                .not_in_the_set
                .iter()
                .map(|name| quoted(name))
                .collect();
            out.push_str(&format!(
                "# The proportions of the list the set does not give.\nnot_given = [{}]\n",
                named.join(", ")
            ));
        }
        out.push_str("\n[rules]\n");
        for (name, text) in AGAINST_RULES {
            out.push_str(&format!("{name} = {}\n", quoted(text)));
        }
        out.push_str("\n# Every proportion the set gives, in the list's order.\n");
        for line in &self.lines {
            out.push_str("[[proportion]]\n");
            out.push_str(&format!("name = {}\n", quoted(&line.name)));
            out.push_str(&format!("value = {}\n", six(line.value)));
            match &line.verdict {
                Verdict::Inside => out.push_str("verdict = \"inside\"\n"),
                Verdict::Outside { above, beyond } => {
                    out.push_str("verdict = \"outside\"\n");
                    out.push_str(&format!(
                        "side = {}\n",
                        quoted(if *above {
                            "above the highest value"
                        } else {
                            "below the lowest value"
                        })
                    ));
                    out.push_str(&format!("beyond = {}\n", six(*beyond)));
                    if let Some(percent) = self.percent(*beyond) {
                        out.push_str(&format!(
                            "beyond_percent_of_head_height = {}\n",
                            toml_number(percent, 4)
                        ));
                    }
                }
                Verdict::NoRange(why) => {
                    out.push_str("verdict = \"no range\"\n");
                    out.push_str(&format!("why = {}\n", quoted(why)));
                }
            }
            out.push_str(&format!("characters = {}\n", line.characters));
            if let Some([lowest, middle, highest]) = line.values {
                out.push_str(&format!(
                    "lowest = {}\nmiddle = {}\nhighest = {}\n",
                    six(lowest),
                    six(middle),
                    six(highest)
                ));
            }
        }
        out
    }

    /// What was found, in a few lines, for a person: the counts, and each proportion outside
    /// its range with how far.
    pub fn summary(&self) -> String {
        let outside = self.count(|verdict| matches!(verdict, Verdict::Outside { .. }));
        let mut out = format!(
            "Of the {} the set gives: {} inside their valid range, {outside} outside, {} with no range to be set against.\n",
            counted(self.lines.len(), "proportion"),
            self.count(|verdict| *verdict == Verdict::Inside),
            self.count(|verdict| matches!(verdict, Verdict::NoRange(_))),
        );
        if outside > 0 {
            out.push_str("Outside:\n");
        }
        for line in &self.lines {
            let (Verdict::Outside { above, beyond }, Some([lowest, _, highest])) =
                (&line.verdict, line.values)
            else {
                continue;
            };
            let (side, end) = if *above {
                ("above the highest value", highest)
            } else {
                ("below the lowest value", lowest)
            };
            out.push_str(&format!(
                "  {}: {} is {side} {} by {} units",
                line.name,
                six(line.value),
                six(end),
                six(*beyond)
            ));
            if let Some(percent) = self.percent(*beyond) {
                out.push_str(&format!(
                    ", {}% of head height",
                    crate::number::plain(percent, 2)
                ));
            }
            out.push('\n');
        }
        if !self.not_in_the_set.is_empty() {
            out.push_str(&format!(
                "The set does not give {} of the list.\n",
                counted(self.not_in_the_set.len(), "proportion")
            ));
        }
        out.push_str(&format!(
            "{}\n",
            AGAINST_RULES
                .iter()
                .find(|(name, _)| *name == "a_finding")
                .map(|(_, text)| *text)
                .unwrap_or_default()
        ));
        out
    }
}
