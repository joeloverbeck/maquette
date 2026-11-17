//! The proportion table, and the setting of a set of proportions against one.
//!
//! This module hides what a table is built from, what it states of each proportion, when a
//! proportion has a valid range, and what a set of proportions is told when it is set against
//! one. `maquette table-from-readings` builds a table from the record of Jon's check, and
//! `maquette set-against-table` sets a set of proportions against a table.
//!
//! It also hides how a whole set of proportions is made from a table for a head to be drawn:
//! the table's middle values where it has a range, the average of the works where two to
//! four give a proportion, and another set's own numbers for the rest, each marked as what it
//! is. `maquette set-from-table` writes one.
//!
//! A table is a proposal until Jon has accepted it. Nothing here judges a head: the program
//! reports what the accepted readings give.

use std::collections::BTreeMap;

use crate::jons_check::{Accepted, Outcome, Review, WORKS_FOR_A_TABLE};
use crate::known_positions::{NOT_A_READER, dimension};
use crate::measuring::{self, Given};
use crate::number::{counted, median, toml as toml_number};
use crate::proportions::{self, Kind};
use crate::readings::Drawing;
use crate::toml_text::{quoted, quoted_lines, tables, words};
use crate::views::View;
use crate::{Error, Result};

/// What a table is and does, by name, as the table's own file states it.
pub const RULES: &[(&str, &str)] = &[
    (
        "built_from",
        "A table is built from the record of Jon's check and from nothing else: the readings of each character he accepted, each drawing as the record rules it, with what he marked as off left out. A drawing that is set aside or not called level is left out, and so is a character he rejected.",
    ),
    (
        "a_work",
        "Each accepted character is one work, and a table counts works. Heights and widths are taken from a work's front view, so a work with no measured front view gives nothing. Depth is taken from its side view where one is measured, and otherwise from its three-quarter view at the turn the readings show.",
    ),
    (
        "values",
        "For each proportion: the number of works that give it and, where it has a range, the lowest, the middle and the highest value among them. The middle is the median: the middle value in order, or midway between the two middle values of an even number of works. Every value is in units, a length over the height from the eye line down to the chin.",
    ),
    (
        "floor",
        "A proportion with five works or more behind it is given: its valid range runs from its lowest value to its highest. A proportion with fewer is too thin: the table says how many works stand behind it and why the others do not give it, and states no number, because the values of one to four works are those works' own numbers. It has no range, and nothing is set against it. The floor is held proportion by proportion. In a view that has five works, a proportion that hair or a mark of Jon's leaves with fewer is too thin all the same: the floor is there so that no range is one hand's habit, and a range is backed only by the works that show that feature. A view with fewer than five works is reported as too thin and is not a table.",
    ),
    (
        "conventions",
        "A convention is built by a stated rule and is never a measurement: it has no valid range, however many works the rule could be run on, and the table states no number for it.",
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
    /// Five works or more: its lowest and highest value are its valid range.
    Given,
    /// One to four works: what they give is stated and is no range.
    TooThin,
    /// No work gives it.
    NoWork,
    /// Built by rule and never measured: no range.
    Convention,
}

impl Standing {
    fn of(kind: Kind, works: usize) -> Standing {
        match (kind, works) {
            (Kind::Convention, _) => Standing::Convention,
            (Kind::Measured, 0) => Standing::NoWork,
            (Kind::Measured, works) if works < WORKS_FOR_A_TABLE => Standing::TooThin,
            (Kind::Measured, _) => Standing::Given,
        }
    }

    pub fn word(self) -> &'static str {
        match self {
            Standing::Given => "given",
            Standing::TooThin => "too thin",
            Standing::NoWork => "no work",
            Standing::Convention => "convention",
        }
    }

    fn named(word: &str) -> Option<Standing> {
        [
            Standing::Given,
            Standing::TooThin,
            Standing::NoWork,
            Standing::Convention,
        ]
        .into_iter()
        .find(|standing| standing.word() == word)
    }
}

/// One work a table was measured on, as the table lists it.
#[derive(Debug, Clone)]
struct Work {
    /// What its readings say the reference is, field by field.
    reference: BTreeMap<String, String>,
    /// The views measured of it.
    views: Vec<View>,
    /// Where its depth is taken from, in words.
    depth_from: &'static str,
}

/// One proportion of a table.
#[derive(Debug, Clone)]
struct Line {
    name: String,
    kind: Kind,
    /// The values the works give, in order from the lowest.
    values: Vec<f64>,
    /// Why the other works do not give it, each reason with the number of works it is said of.
    not_given: Vec<(String, usize)>,
}

impl Line {
    fn standing(&self) -> Standing {
        Standing::of(self.kind, self.values.len())
    }
}

/// A proportion table as it is built from the record of Jon's check.
#[derive(Debug, Clone)]
pub struct Table {
    /// The record it was built from, as [`Review::named`] says it.
    built_from: String,
    works: Vec<Work>,
    /// The number of accepted works for each view and each route to depth.
    views: Vec<(String, usize)>,
    /// What was accepted and gives nothing, in words: a drawing that is not measured.
    left_out: Vec<String>,
    lines: Vec<Line>,
}

/// The fields of a reference a table lists a work by. Two are left in the readings: where a
/// picture was taken from, since an address can name a character and a table is written to
/// be published, and the reader's own account of the manner it is drawn in, which is one
/// reading's.
const WORK_FIELDS: [&str; 5] = ["work", "designer", "age_band", "sex", "species"];

/// The one measured drawing of a view among a character's, or none. Two are refused: which
/// of them gives the character's proportions is not decided anywhere.
fn of_view<'a>(accepted: &Accepted<'a>, view: View) -> Result<Option<(Vec<&'a str>, &'a Drawing)>> {
    let mut found = None;
    for (checked, outcome) in &accepted.drawings {
        let Outcome::Measured { drawing, .. } = outcome else {
            continue;
        };
        if drawing.view != view {
            continue;
        }
        if found.is_some() {
            return Err(Error::new(format!(
                "the character `{}` has two measured {} views in the record of Jon's check: a table takes one drawing of a view from a work, and which of the two it is has not been decided",
                accepted.character.label,
                view.name()
            )));
        }
        let paths: Vec<&str> = checked
            .readings
            .iter()
            .map(|named| named.path.as_str())
            .collect();
        found = Some((paths, drawing.as_ref()));
    }
    Ok(found)
}

/// Builds a table from a record of Jon's check that is complete: the rules `built_from`,
/// `a_work`, `values` and `floor`.
pub fn build(review: &Review) -> Result<Table> {
    if !review.complete() {
        return Err(Error::new(format!(
            "the record of Jon's check is not complete, so no table is built from it: {}",
            review.problems.join("; ")
        )));
    }
    let list = proportions::list()?;
    let mut works: Vec<Work> = Vec::new();
    let mut left_out = Vec::new();
    // For each proportion, the values the works give and why the others do not.
    let mut values: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
    let mut not_given: BTreeMap<&str, Vec<(String, usize)>> = BTreeMap::new();

    for accepted in review.accepted() {
        let label = &accepted.character.label;
        let of_character = |error: Error| Error::new(format!("the character `{label}`: {error}"));
        for (checked, outcome) in &accepted.drawings {
            match outcome {
                Outcome::NotMeasured(_) => left_out.push(format!(
                    "the {} view of `{label}` is not measured",
                    checked.counts_as().name()
                )),
                Outcome::Measured { drawing, .. } => {
                    if drawing.readers.iter().any(|reader| reader == NOT_A_READER) {
                        return Err(Error::new(format!(
                            "the {} view of `{label}` is not a reading: it is the positions of a record of known positions, written as a reading, and nobody read them from a drawing. A table is measured on readings of references. What is known beforehand, as a comparison case's numbers are, never enters one",
                            drawing.view.name()
                        )));
                    }
                }
            }
        }
        let Some((front_paths, front)) = of_view(&accepted, View::Front)? else {
            left_out.push(format!(
                "`{label}` has no measured front view, and heights and widths are the front view's: nothing is taken from it"
            ));
            continue;
        };
        let side = of_view(&accepted, View::Side)?;
        let three_quarter = of_view(&accepted, View::ThreeQuarter)?;
        let front_named = front_paths.join(",");
        let front_given = Given {
            named: &front_named,
            drawing: front,
        };
        let mut views = vec![View::Front];
        let (mut taken, depth_from) = match (side, three_quarter) {
            (Some((paths, side)), three_quarter) => {
                views.push(View::Side);
                if three_quarter.is_some() {
                    left_out.push(format!(
                        "the three-quarter view of `{label}` is not used: its depth is taken from its side view"
                    ));
                }
                let named = paths.join(",");
                let side = Given {
                    named: &named,
                    drawing: side,
                };
                (
                    measuring::proportions(&front_given, None, Some(&side))
                        .map_err(of_character)?,
                    "its side view",
                )
            }
            (None, Some((paths, three_quarter))) => {
                views.push(View::ThreeQuarter);
                let named = paths.join(",");
                let three_quarter = Given {
                    named: &named,
                    drawing: three_quarter,
                };
                (
                    measuring::proportions(&front_given, Some(&three_quarter), None)
                        .map_err(of_character)?,
                    "its three-quarter view, at the turn the readings show",
                )
            }
            (None, None) => (
                measuring::proportions(&front_given, None, None).map_err(of_character)?,
                "no view: it gives no depth",
            ),
        };
        taken.leave_out(&accepted.character.proportions_off);

        let reference = accepted.reference.clone();
        let said = |field: &str| reference.get(field).cloned().unwrap_or_default();
        if said("work").is_empty() {
            return Err(Error::new(format!(
                "the readings of `{label}` do not say which work the drawing is from: a table lists the works it was measured on"
            )));
        }
        if works
            .iter()
            .any(|work| work.reference.get("work") == reference.get("work"))
        {
            return Err(Error::new(format!(
                "`{label}` is a second accepted character of the work \"{}\": a batch takes one character from a work, and a table counts works",
                said("work")
            )));
        }
        for proportion in &list.proportions {
            let name = proportion.name.as_str();
            if let Some(value) = taken.taken.values.get(name) {
                values.entry(name).or_default().push(*value);
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
        works.push(Work {
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
            values.sort_by(f64::total_cmp);
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
        built_from: review.named(),
        works,
        views: review.counts(),
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
# number of works whose accepted readings give it and, where five or more do, the lowest, the
# middle and the highest value among them. `maquette list-proportions` defines each proportion.
#
# It is a proposal until Jon has accepted it. [rules] says what it is built from, when a
# proportion has a valid range, and what the table does not say.
",
        );
        out.push_str("\n[table]\n");
        out.push_str(&format!(
            "built_from = {}\n",
            quoted(&format!("the record of Jon's check {}", self.built_from))
        ));
        out.push_str(&format!("works = {}\n", self.works.len()));
        out.push_str(&format!("works_for_a_table = {WORKS_FOR_A_TABLE}\n"));
        out.push_str(&format!(
            "proportions = {}\ngiven = {}\ntoo_thin = {}\nno_work = {}\nconventions = {}\n",
            self.lines.len(),
            self.count(Standing::Given),
            self.count(Standing::TooThin),
            self.count(Standing::NoWork),
            self.count(Standing::Convention)
        ));
        if !self.left_out.is_empty() {
            out.push_str(&format!(
                "# Accepted and not measured. `maquette jons-check` says why.\nleft_out = {}\n",
                quoted_lines(&self.left_out)
            ));
        }

        out.push_str("\n[rules]\n");
        for (name, text) in RULES {
            out.push_str(&format!("{name} = {}\n", quoted(text)));
        }

        out.push_str(
            "\n# The accepted works behind each view, and behind depth, which takes two views of one\n# character. A view with fewer than five is too thin, and is not a table.\n",
        );
        for (what, count) in &self.views {
            out.push_str(&format!(
                "[[view]]\nview = {}\nworks = {count}\nstanding = {}\n",
                quoted(what),
                quoted(if *count < WORKS_FOR_A_TABLE {
                    "too thin: not a table"
                } else {
                    "a table"
                })
            ));
        }

        out.push_str(
            "\n# The works the table was measured on, as their readings name them. Where each picture\n# was taken from is in its readings, in the reference folder.\n",
        );
        for work in &self.works {
            out.push_str("[[work]]\n");
            for field in WORK_FIELDS {
                if let Some(said) = work.reference.get(field) {
                    out.push_str(&format!("{field} = {}\n", quoted(said)));
                }
            }
            let views: Vec<String> = work.views.iter().map(|view| quoted(view.name())).collect();
            out.push_str(&format!("views = [{}]\n", views.join(", ")));
            out.push_str(&format!("depth_from = {}\n", quoted(work.depth_from)));
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
            out.push_str(&format!("works = {}\n", line.values.len()));
            out.push_str(&format!("standing = {}\n", quoted(line.standing().word())));
            // Only a range is stated in numbers: the values of a few works are those works'
            // own numbers.
            if let (Standing::Given, Some(lowest), Some(highest)) =
                (line.standing(), line.values.first(), line.values.last())
            {
                out.push_str(&format!("lowest = {}\n", six(*lowest)));
                out.push_str(&format!("middle = {}\n", six(median(&line.values))));
                out.push_str(&format!("highest = {}\n", six(*highest)));
            }
            if !line.not_given.is_empty() {
                let reasons: Vec<String> = line
                    .not_given
                    .iter()
                    .map(|(why, count)| format!("{}: {why}", counted(*count, "work")))
                    .collect();
                out.push_str(&format!("not_given_by = {}\n", quoted_lines(&reasons)));
            }
        }
        out
    }

    /// What was built, in a few lines, for a person.
    pub fn summary(&self) -> String {
        let mut out = format!(
            "A table of {} proportions from {}, by the record of Jon's check {}.\n",
            self.lines.len(),
            counted(self.works.len(), "work"),
            self.built_from
        );
        out.push_str(&format!(
            "  {} given: five works or more, with a valid range.\n  {} too thin: one to four works, and no range.\n  {} with no work.\n  {}, built by rule and never measured: no range.\n",
            self.count(Standing::Given),
            self.count(Standing::TooThin),
            self.count(Standing::NoWork),
            counted(self.count(Standing::Convention), "convention")
        ));
        out.push_str("Accepted works for each view:\n");
        for (what, count) in &self.views {
            out.push_str(&format!("  {what}: {}\n", crate::jons_check::works(*count)));
        }
        for left_out in &self.left_out {
            out.push_str(&format!("Left out: {left_out}.\n"));
        }
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
        "A proportion with a valid range takes the table's middle value: the median of the works that give it.",
    ),
    (
        "a_thin_average",
        "A proportion that two to four works give takes the average of those works, and is marked as thin: it has no range. It is the average and not the middle, because the middle of three works is one work's own number.",
    ),
    (
        "borrowed",
        "Every other proportion is borrowed from the set named as `borrowed_from`, as it stands there unless the rule `hangs_on` places it: one that a single work gives, since that is one reference's number; one that no work gives; and a convention, which is never measured.",
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
/// names, with what a proportion borrowed for that reason says: the rule `taken_whole`.
const TAKEN_WHOLE: &[(&str, &str)] = &[
    (
        "ear_",
        "the ear is taken whole, and the works do not give all of it",
    ),
    (
        "neck_",
        "the neck is taken whole, and the works do not give all of it",
    ),
];

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

/// Where a proportion of a set made from a table comes from.
#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    /// The table's middle value, with the works behind it.
    Table(usize),
    /// The average of two to four works.
    ThinAverage(usize),
    /// The borrowed set's own number, and why.
    Borrowed(&'static str),
    /// Borrowed as an offset from the point it hangs on, by the rule `hangs_on`: how it
    /// hangs, and why it is borrowed at all.
    BorrowedOffset(&'static str, &'static str),
    /// Left out of the set: a proportion a set may leave out, which the works do not give to
    /// draw and the borrowed set leaves out too. Why it would have been borrowed.
    LeftOut(&'static str),
}

impl Source {
    fn is_borrowed(&self) -> bool {
        matches!(self, Source::Borrowed(_) | Source::BorrowedOffset(..))
    }

    /// The source as the set's file says it. What stands before the colon is the kind, which
    /// [`Source::kind_of`] reads back.
    fn words(&self) -> String {
        match self {
            Source::Table(works) => format!("the table: the middle of {works} works"),
            Source::ThinAverage(works) => format!("a thin average: of {works} works"),
            Source::Borrowed(why) => format!("borrowed: {why}"),
            Source::BorrowedOffset(how, why) => format!("borrowed: {how}; {why}"),
            Source::LeftOut(why) => {
                format!("left out: {why}, and the set it would be borrowed from leaves it out")
            }
        }
    }

    /// The kind of source a set's file states for a proportion: "the table", "a thin
    /// average", "borrowed" or "left out". None for anything else.
    pub fn kind_of(words: &str) -> Option<&'static str> {
        let kind = words.split(':').next().unwrap_or_default().trim();
        ["the table", "a thin average", "borrowed", "left out"]
            .into_iter()
            .find(|known| *known == kind)
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
    /// `borrowed` is the set whose numbers stand in where the works give none to draw, and
    /// `borrowed_from` is what the file calls it.
    pub fn set_with(
        &self,
        borrowed_from: &str,
        borrowed: &proportions::Set,
    ) -> Result<SetFromTable> {
        // A borrowed number, or none where the borrowed set leaves out a proportion that a set
        // may leave out: it is then left out of this set too.
        let borrow = |name: &str, why: &'static str| -> (Option<f64>, Source) {
            match borrowed.given(name) {
                Some(value) => (Some(value), Source::Borrowed(why)),
                None => (None, Source::LeftOut(why)),
            }
        };
        let mut made: Vec<(String, Option<f64>, Source)> = self
            .lines
            .iter()
            .map(|line| {
                let works = line.values.len();
                let (value, source) = match line.standing() {
                    Standing::Given => (Some(median(&line.values)), Source::Table(works)),
                    Standing::TooThin if works >= 2 => (
                        Some(line.values.iter().sum::<f64>() / works as f64),
                        Source::ThinAverage(works),
                    ),
                    Standing::TooThin => borrow(
                        &line.name,
                        "one work gives it, and one reference's number is not drawn",
                    ),
                    Standing::NoWork => borrow(&line.name, "no work gives it"),
                    Standing::Convention => {
                        borrow(&line.name, "a convention, which is never measured")
                    }
                };
                (line.name.clone(), value, source)
            })
            .collect();
        // The rule `taken_whole`. A proportion left out counts as one the works do not give.
        let not_from_the_works =
            |source: &Source| matches!(source, Source::Borrowed(_) | Source::LeftOut(_));
        for (start, why) in TAKEN_WHOLE {
            let whole = |name: &str| name.starts_with(start);
            if made
                .iter()
                .any(|(name, _, source)| whole(name) && not_from_the_works(source))
            {
                for (name, value, source) in &mut made {
                    if whole(name) && !not_from_the_works(source) {
                        (*value, *source) = borrow(name, why);
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
            if let Some((_, Some(value), source)) =
                made.iter_mut().find(|(known, _, _)| known == name)
                && let Source::Borrowed(why) = *source
            {
                // The borrowed number, moved by as far as this set's anchor stands from the
                // borrowed set's: the point keeps its offset from what it hangs on.
                *value += anchored.unwrap_or(0.0);
                *source = Source::BorrowedOffset(how, why);
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
        Ok(SetFromTable {
            built_from: self.built_from.clone(),
            borrowed_from: borrowed_from.to_string(),
            sources: made
                .into_iter()
                .map(|(name, _, source)| (name, source))
                .collect(),
            set,
            rule_would_set_back,
        })
    }
}

impl SetFromTable {
    fn count(&self, of: impl Fn(&Source) -> bool) -> usize {
        self.sources.iter().filter(|(_, source)| of(source)).count()
    }

    /// What the set says of a correction: that it carries none, and what the list's rule
    /// would give on its own depths.
    fn correction(&self) -> String {
        let carried = "The set carries no correction.";
        match self.rule_would_set_back {
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

    /// The set as the text of a file that `maquette draw` reads.
    pub fn file(&self) -> Result<String> {
        let mut opening = String::from(
            "\
# A whole set of proportions made from a proportion table, by `maquette set-from-table`: the
# table's middle values where it has a range, the average of the works where two to four give
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
            quoted(&format!(
                "built from the record of Jon's check {}",
                self.built_from
            ))
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
            self.count(|source| matches!(source, Source::LeftOut(_)))
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
        let left_out = self.count(|source| matches!(source, Source::LeftOut(_)));
        format!(
            "A set of {} proportions: {} from the table's middle values, {} thin averages of two to four works, {} borrowed from `{}`.{}\n{}\n",
            self.sources.len() - left_out,
            self.count(|source| matches!(source, Source::Table(_))),
            self.count(|source| matches!(source, Source::ThinAverage(_))),
            self.count(Source::is_borrowed),
            self.borrowed_from,
            match left_out {
                0 => String::new(),
                left_out => format!(
                    " {} left out: a set may leave them out, and neither the works nor `{}` give them.",
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
        "A proportion has no range to be set against when the table has fewer than five works behind it, or none, or when it is a convention. What the few works give stands beside a thin one, and is no verdict.",
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
    works: usize,
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
/// states it. A file that calls a proportion given without five works and a range behind it
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
        let works = entry
            .get("works")
            .and_then(toml::Value::as_integer)
            .and_then(|works| usize::try_from(works).ok())
            .ok_or_else(|| {
                not_a_table(format!(
                    "`{name}` does not say how many works stand behind it"
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
        // What the file calls a proportion is what its kind and its count of works make it.
        if standing != Standing::of(proportion.kind, works) {
            return Err(not_a_table(format!(
                "it calls `{name}` \"{}\" with {} behind it, and a {} proportion with that many is \"{}\"",
                standing.word(),
                counted(works, "work"),
                proportion.kind.word(),
                Standing::of(proportion.kind, works).word()
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
                works,
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
                    standing: Standing::NoWork,
                    works: 0,
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
    works: usize,
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
                "too thin: {} behind it, and a range needs {WORKS_FOR_A_TABLE}",
                match stated.works {
                    1 => "1 work stands".to_string(),
                    works => format!("{works} works stand"),
                }
            )),
            (Standing::NoWork | Standing::Given, _) => {
                Verdict::NoRange("no work gives it".to_string())
            }
        };
        lines.push(Against {
            name: name.clone(),
            value,
            verdict,
            works: stated.works,
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
            out.push_str(&format!("works = {}\n", line.works));
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
