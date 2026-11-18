//! A whole set of proportions made from one structure reference, for a head to be drawn: the
//! numbers one character's readings give, as they stand, and another set's own numbers where
//! they give none.
//!
//! This module hides which source each proportion of such a set takes, and how the set says
//! so. The rules every whole set made for a head shares, that an ear and the neck are taken
//! whole and that the jaw point hangs on the chin, are the table's ([`crate::table`]).
//!
//! A set made here reproduces one character's proportions. It is worked out from readings of
//! one reference, and like them it is kept in the reference folder and never committed.

use crate::measuring::Proportions;
use crate::proportions::{self, Kind};
use crate::table::{self, Source, Table};
use crate::toml_text::{quoted, quoted_lines};
use crate::{Error, Result};

/// How a whole set is made from a structure reference, by name, as the set's file states it.
pub const RULES: &[(&str, &str)] = &[
    (
        "the_structure_reference",
        "A proportion the structure reference's readings give takes their number as it stands. It is one reference's number, drawn on purpose, and is averaged with nothing.",
    ),
    (
        "unseen_from_the_table",
        "Where a table is named, a proportion that a front view does not show, as the list of proportions says, and that the structure reference does not give, takes the table's middle value where the table gives it a valid range. A proportion a front view shows is never taken from the table, so the front view drawn is the reference's own with the borrowed set's numbers in its gaps.",
    ),
    (
        "borrowed",
        "Every other proportion is borrowed from the set named as `borrowed_from`, as it stands there unless the rule `hangs_on` places it: one the structure reference does not give, one named in `borrowed_also`, and a convention, which is never measured.",
    ),
    (
        "taken_whole",
        "An ear and the neck are each taken whole, from one source. An ear is one form built on its two base points, and the neck one column: its width, where it ends and how far back it stands belong together, and a width from one source on a depth from another sets the head forward on a post. So unless one source gives every one of a form's proportions, all of them are borrowed. The points of the neck's profile, where its front and its back meet the head and end, stand apart from the column: each is the structure reference's where it gives it, since the front's top is where the underside of the jaw turns down into the neck, and is borrowed otherwise. Taken whole with a borrowed column that has none, they would be left out, and the jaw would run straight from the chin to its angle, which the reference does not draw. Each line of the profile, the front and the back, is taken whole from one source, both its ends, or borrowed whole: one end from one source and the other from another draws a line that is neither's.",
    ),
    (
        "hangs_on",
        "A borrowed point that hangs on a point from another source keeps its offset from it. The jaw point hangs on the chin: it is borrowed as how far the borrowed set's jaw point stands from that set's own chin, and is placed from this set's chin. Every chin is one unit below the eye line and on the centre line, so the jaw point's height and half-width come over as they stand; its depth is this set's chin's depth plus the borrowed offset.",
    ),
    (
        "no_correction",
        "The set carries no correction, whatever the set it borrows from carries. A correction's amounts are worked out from a set's own depths, and whether a head is drawn with one is Jon's to say.",
    ),
];

/// The starts of the names of the points of the neck's profile, which stand apart from the
/// neck's column in the rule `taken_whole`.
const NECK_PROFILE: &[&str] = &["neck_front_", "neck_back_"];

/// The lines of the neck's profile, each by the start of its two ends' names, with what it is
/// called: the rule `taken_whole`.
const NECK_LINES: &[(&str, &str)] = &[
    ("neck_front_", "the neck's front line"),
    ("neck_back_", "the neck's back line"),
];

/// Takes each line of the neck's profile from one source, both its ends, or borrows it whole:
/// one end from one source and the other from another, or left out, draws a line that is
/// neither's.
fn whole_lines(
    mut made: Vec<(String, Option<f64>, Source)>,
    borrowed: &proportions::Set,
) -> Vec<(String, Option<f64>, Source)> {
    for (start, line) in NECK_LINES {
        let ends: Vec<&(String, Option<f64>, Source)> = made
            .iter()
            .filter(|(name, _, _)| name.starts_with(start))
            .collect();
        let one_source = ends.iter().all(|(_, value, source)| {
            value.is_some() && std::mem::discriminant(source) == std::mem::discriminant(&ends[0].2)
        });
        if one_source {
            continue;
        }
        let why = format!(
            "{line} is taken whole, from both its ends, and the structure reference does not give both"
        );
        for (name, value, source) in &mut made {
            if name.starts_with(start) && !source.is_borrowed() && !source.is_left_out() {
                (*value, *source) = table::borrow(borrowed, name, why.clone());
            }
        }
    }
    made
}

/// A whole set of proportions made from one structure reference, with where each number comes
/// from.
#[derive(Debug, Clone)]
pub struct SetFromReference {
    reference: Proportions,
    /// What the table named for what a front view does not show was built from, where one was.
    table: Option<String>,
    borrowed_from: String,
    borrowed_also: Vec<String>,
    sources: Vec<(String, Source)>,
    set: proportions::Set,
    rule_would_set_back: Option<[f64; 2]>,
}

/// Makes a whole set from what a structure reference's readings give: the rules of [`RULES`].
/// `table` stands in, where it is given, for what a front view does not show; `borrowed` is
/// the set whose numbers stand in for the rest, and `borrowed_from` what the file calls it;
/// `also` names proportions borrowed in place of the reference's own, each by its name or by
/// the start of the names of several, ending in `_`.
pub fn set(
    reference: &Proportions,
    table: Option<&Table>,
    borrowed_from: &str,
    borrowed: &proportions::Set,
    also: &[String],
) -> Result<SetFromReference> {
    let list = proportions::list()?;
    for named in also {
        if !list
            .proportions
            .iter()
            .any(|proportion| named_by(named, &proportion.name))
        {
            return Err(Error::new(format!(
                "`{named}`, named to be borrowed, names no proportion of the list: name a proportion, or the start of several names ending in `_`, such as `brow_`. `maquette list-proportions` gives every name"
            )));
        }
    }
    let borrow = |name: &str, why: &str| table::borrow(borrowed, name, why.to_string());
    let made = list
        .proportions
        .iter()
        .map(|proportion| {
            let name = proportion.name.as_str();
            let unseen_from_table = table
                .filter(|_| !proportion.front_shows())
                .and_then(|table| table.middle(name));
            let (value, source) = if proportion.kind == Kind::Convention {
                borrow(name, "a convention, which is never measured")
            } else if also.iter().any(|named| named_by(named, name)) {
                borrow(
                    name,
                    "named in `borrowed_also`, in place of the structure reference's own",
                )
            } else if let Some(value) = reference.taken.values.get(name) {
                (Some(*value), Source::Reference)
            } else if let Some((middle, characters)) = unseen_from_table {
                (Some(middle), Source::Table(characters))
            } else {
                borrow(name, "the structure reference does not give it")
            };
            (name.to_string(), value, source)
        })
        .collect::<Vec<_>>();
    let made = whole_lines(made, borrowed);
    // The points of the neck's profile stand apart from its column: the rule `taken_whole`.
    let finished = table::finish(made, borrowed, "the structure reference does", NECK_PROFILE)?;
    Ok(SetFromReference {
        reference: reference.clone(),
        table: table.map(|table| table.built_from().to_string()),
        borrowed_from: borrowed_from.to_string(),
        borrowed_also: also.to_vec(),
        sources: finished.sources,
        set: finished.set,
        rule_would_set_back: finished.rule_would_set_back,
    })
}

/// Whether `named`, a proportion's name or the start of several ending in `_`, names `name`.
fn named_by(named: &str, name: &str) -> bool {
    match named.ends_with('_') {
        true => name.starts_with(named),
        false => name == named,
    }
}

impl SetFromReference {
    fn count(&self, of: impl Fn(&Source) -> bool) -> usize {
        self.sources.iter().filter(|(_, source)| of(source)).count()
    }

    /// The set as the text of a file that `maquette draw` reads.
    pub fn file(&self) -> Result<String> {
        let mut opening = String::from(
            "\
# A whole set of proportions made from one structure reference, by
# `maquette set-from-structure-reference`: the numbers one character's readings give, as they
# stand, and another set's own numbers for the rest. [sources] says which each is.
#
# It is one head to be drawn and looked at, and no proportion table. It reproduces one
# character's proportions: it is worked out from readings of one reference and is never
# committed. `maquette draw` reads this file as it reads any set of proportions, and
# `maquette list-proportions` defines each proportion.
",
        );
        opening.push_str("\n[source]\n");
        opening.push_str(&format!(
            "views = {}\n",
            quoted_lines(&self.reference.views)
        ));
        opening.push_str(&self.reference.read.file());
        for (view, digest) in &self.reference.digests {
            opening.push_str(&format!("{view}_sha256 = {}\n", quoted(digest)));
        }
        opening.push_str(&format!(
            "depth_from = {}\n",
            quoted(self.reference.depth_from.words())
        ));
        if let Some(table) = &self.table {
            opening.push_str(&format!(
                "table = {}\n",
                quoted(&format!("built from {table}"))
            ));
        }
        opening.push_str(&format!(
            "borrowed_from = {}\n",
            quoted(&self.borrowed_from)
        ));
        if !self.borrowed_also.is_empty() {
            opening.push_str(&format!(
                "borrowed_also = {}\n",
                quoted_lines(&self.borrowed_also)
            ));
        }
        opening.push_str(&format!(
            "from_the_structure_reference = {}\nfrom_the_table = {}\nborrowed = {}\nleft_out = {}\n",
            self.count(|source| matches!(source, Source::Reference)),
            self.count(|source| matches!(source, Source::Table(_))),
            self.count(Source::is_borrowed),
            self.count(Source::is_left_out)
        ));
        opening.push_str(&format!(
            "correction = {}\n",
            quoted(&table::correction_words(self.rule_would_set_back))
        ));
        opening.push_str("\n[rules]\n");
        for (name, text) in RULES {
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
        let mut out = format!(
            "A set of {} proportions: {} from the structure reference, {} from the table's middle values, {} borrowed from `{}`.{}\n{}\n",
            self.sources.len() - left_out,
            self.count(|source| matches!(source, Source::Reference)),
            self.count(|source| matches!(source, Source::Table(_))),
            self.count(Source::is_borrowed),
            self.borrowed_from,
            match left_out {
                0 => String::new(),
                left_out => format!(
                    " {} left out: a set may leave them out, and neither the structure reference nor `{}` gives them.",
                    crate::number::counted(left_out, "proportion"),
                    self.borrowed_from
                ),
            },
            table::correction_words(self.rule_would_set_back)
        );
        out.push_str(&self.reference.read.summary());
        out
    }
}
