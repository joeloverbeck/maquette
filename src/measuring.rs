//! What readings say when they are set against each other: the agreement between two readings
//! of one drawing, the turn a three-quarter drawing shows, the fit of a character's views to
//! one head, and the proportions the readings give.
//!
//! This module hides which positions are compared and in what, which left and right pairs
//! say how far a head is turned, and what "the views fit one head" means as one figure.
//! [`RULES`] states each, and every file written here carries the rules it used. It reports
//! and judges nothing: whether a drawing counts, and whether a point is off, is Jon's check.
//!
//! Every share is of head height, hair top to chin. A disagreement or a misfit is listed as
//! above 1% when it is more than a hundredth of that, as a difference is.

use std::collections::BTreeMap;

use crate::differences::THRESHOLD_PERCENT;
use crate::hand_made_views::{Marks, marks};
use crate::head_model::{self, Reading as Positions, Taken};
use crate::number::{round3, toml as toml_number};
use crate::readings::{Drawing, Faces, Record, other_revision, two_readings};
use crate::toml_text::{position, quoted, quoted_lines, quoted_list};
use crate::views::View;
use crate::{Error, Result};

/// The rules of measuring, by name, as the files written here state them.
pub const RULES: &[(&str, &str)] = &[
    (
        "one_direction",
        "Two readings of a drawing are compared landmark by landmark, in pixels of the picture. The top and the bottom of an iris and the hair's top say how far a form reaches, and a reader may put one anywhere along a level edge: they are compared in height only. A side extent of the hair, of an iris or of an ear, where a reader may put the point anywhere along an upright edge, and a crossing of the face outline with the eye line, the nose level or the mouth level, are compared sideways only. Every other landmark is compared as a position, by the straight distance.",
    ),
    (
        "yardstick_of_two_readings",
        "Head height is the chin's height minus the hair top's, the mean of the two readings, in pixels of the picture. A disagreement is above 1% when its distance is more than a hundredth of it, so exactly 1% is not above it.",
    ),
    (
        "two_readings",
        "Where a view is given by its two readings, a landmark stands midway between the two positions. A landmark only one of them places is left out.",
    ),
    (
        "facing",
        "A head that faces the viewer's left is mirrored before it is measured: the picture is flipped left to right, and left and right exchange in every name. The head model is symmetric, so nothing else changes, and every turned view is then turned toward the viewer's right.",
    ),
    (
        "frame",
        "Each view is measured in its own unit and from its own eye line: the eye line midway between the top and the bottom of the eye openings, averaged over the eyes the view shows, and the unit from there down to the chin. So two drawings of different sizes are compared, and nothing is compared in pixels across views.",
    ),
    (
        "yardstick_of_views",
        "Head height is the front view's, hair top to chin, in units. Where the front view places no hair top it is the three-quarter view's, and with no three-quarter view the side view's.",
    ),
    (
        "turn",
        "A level head turned about an upright axis, drawn flat, shows a left and right pair cos(turn) times as far apart as the front view does, whatever its depth. Each pair gives a turn of its own, acos(three-quarter spacing over front spacing), and the turn of the head is the one that leaves the least squared misfit in spacing over all the pairs: its cosine is the sum of front spacing times three-quarter spacing over the sum of front spacing squared. A pair's misfit is its three-quarter spacing minus its front spacing times that cosine. The reader's own estimate of the turn is listed beside it and is not used.",
    ),
    (
        "pairs_counted",
        "A pair is counted when both views place both of its points, and by what it is, before anything is fitted: its two points must each mark a place sideways. Those are the corners of the eye opening, the iris centres, the ends of the lid line and of the brow line, the nose mark's corners, the mouth line's ends and the ears' base points. Not counted: a top, a bottom or a crest (of the eye opening, the lid line or the ear), which is where an outline runs level, so that its sideways position is the least certain thing about it; the face outline, the hair's extents and the ears' side extents, which are silhouettes that show a different point of the head in each view; the neck, a column whose width does not narrow as it turns; and the shoulders, which belong to the body and may be turned differently.",
    ),
    (
        "fit",
        "The fit of a character's views to one head is the largest misfit, as a share of head height. One level, symmetric head turned about an upright axis shows three things alike, and each is a kind of misfit. Level: a landmark stands at one height in every view, so its misfit is the difference between its highest and its lowest height over the views that place it, in units. Spacing: a counted pair's misfit against the turn, as above. Centre: in the front view the centre-line landmarks and the midpoints of the counted pairs stand on one upright line, which is taken as the mean of their sideways positions; the misfit is how far one stands from it. A misfit is the whole disagreement between two views. A spacing's belongs to the pair: each of its points is off by half of it. The fit is zero when the views are projections of one such head. With no three-quarter view there is no turn, and no spacing is compared: a front view and a side view are set against each other in height, each in its own unit, and the front view against its own centre line.",
    ),
    (
        "fit_left_out",
        "Left out of the fit by what they are: the neck and the shoulders, which belong to the body; in height, the side extents and the face outline's crossings with a level, whose height is the level's own; and a side view's depths, which no command compares with a three-quarter view's yet.",
    ),
];

fn rule(name: &str) -> &'static str {
    RULES
        .iter()
        .find(|(known, _)| *known == name)
        .map(|(_, rule)| *rule)
        .unwrap_or_else(|| panic!("measuring has no rule `{name}`"))
}

fn rules_table(names: &[&str]) -> String {
    let mut out = String::from("\n# The rules behind these numbers.\n[rules]\n");
    for name in names {
        out.push_str(&format!("{name} = {}\n", quoted(rule(name))));
    }
    out
}

/// The line of a file's `[readings]` or `[source]` that says which of the readings follow a
/// revision of the reading instructions that is not the program's. Nothing when none does.
fn other_instructions(notes: &[String]) -> String {
    if notes.is_empty() {
        String::new()
    } else {
        format!("other_instructions = {}\n", quoted_lines(notes))
    }
}

fn above_threshold(distance: f64, head_height: f64) -> bool {
    distance * 100.0 > THRESHOLD_PERCENT * head_height
}

/// What a landmark of a reading is compared in: the `one_direction` rule. A side extent says
/// how far a form reaches to a side, and marks no height.
pub fn compared_in(landmark: &str) -> Marks {
    if landmark.ends_with("-extent") {
        Marks::Sideways
    } else {
        marks(landmark)
    }
}

fn percent(value: f64) -> String {
    toml_number(value, 3)
}

// ---------------------------------------------------------------- agreement

/// One landmark both readings place, and how far apart they put it.
#[derive(Debug, Clone)]
pub struct Compared {
    /// Its number, when the disagreement is above 1%.
    pub number: Option<usize>,
    pub landmark: String,
    pub marks: Marks,
    pub first: [f64; 2],
    pub second: [f64; 2],
    /// In pixels of the picture.
    pub distance: f64,
    pub percent_of_head_height: f64,
}

/// The agreement between two readings of one drawing.
#[derive(Debug, Clone)]
pub struct Agreement {
    pub first_path: String,
    pub second_path: String,
    pub first: Record,
    pub second: Record,
    /// The yardstick, in pixels. None when a reading places no hair top or no chin.
    pub head_height: Option<f64>,
    /// Numbered, largest first.
    pub above: Vec<Compared>,
    /// Largest first.
    pub at_or_under: Vec<Compared>,
    pub placed_by_first_only: Vec<String>,
    pub placed_by_second_only: Vec<String>,
    /// What the two readers judge differently: what it is, and what each says.
    pub judged_differently: Vec<(String, String, String)>,
}

fn head_height_of(record: &Record) -> Option<f64> {
    let height = record.placed.get("chin")?[1] - record.placed.get("hair-top")?[1];
    (height > 0.0).then_some(height)
}

/// Sets two readings of one drawing against each other.
pub fn agreement(
    first_path: &str,
    first: &Record,
    second_path: &str,
    second: &Record,
) -> Result<Agreement> {
    // Not `same_drawing`: two readers who judge the view differently are still set against
    // each other, and what they judge differently is listed.
    two_readings(first, second)?;
    let mut judged_differently = Vec::new();
    let mut judged = |what: &str, first: String, second: String| {
        if first != second {
            judged_differently.push((what.to_string(), first, second));
        }
    };
    let faces = |record: &Record| match record.faces {
        Some(faces) => faces.words().to_string(),
        None => "not said".to_string(),
    };
    let said = |value: Option<bool>| match value {
        Some(true) => "true".to_string(),
        Some(false) => "false".to_string(),
        None => "not said".to_string(),
    };
    judged(
        "view",
        first.view.name().to_string(),
        second.view.name().to_string(),
    );
    judged("faces", faces(first), faces(second));
    judged("level", said(first.level), said(second.level));
    judged(
        "set_aside",
        first.set_aside.to_string(),
        second.set_aside.to_string(),
    );
    let turn = |record: &Record| match record.turn_degrees {
        Some(turn) => toml_number(turn, 1),
        None => "not said".to_string(),
    };
    judged("turn_degrees", turn(first), turn(second));

    let head_height = match (head_height_of(first), head_height_of(second)) {
        (Some(first), Some(second)) => Some((first + second) / 2.0),
        _ => None,
    };
    let mut compared = Vec::new();
    let mut placed_by_first_only = Vec::new();
    for (name, at) in &first.placed {
        let Some(other) = second.placed.get(name) else {
            placed_by_first_only.push(name.clone());
            continue;
        };
        let Some(head_height) = head_height else {
            return Err(Error::new(
                "the two readings cannot be compared: one of them places no hair top or no chin, so there is no head height to state a disagreement in",
            ));
        };
        let marks = compared_in(name);
        let distance = marks.distance(*at, *other);
        compared.push(Compared {
            number: None,
            landmark: name.clone(),
            marks,
            first: *at,
            second: *other,
            distance,
            percent_of_head_height: distance / head_height * 100.0,
        });
    }
    let placed_by_second_only = second
        .placed
        .keys()
        .filter(|name| !first.placed.contains_key(*name))
        .cloned()
        .collect();
    compared.sort_by(|a, b| {
        b.distance
            .total_cmp(&a.distance)
            .then_with(|| a.landmark.cmp(&b.landmark))
    });
    let (mut above, at_or_under): (Vec<Compared>, Vec<Compared>) = compared
        .into_iter()
        .partition(|compared| above_threshold(compared.distance, head_height.unwrap_or(1.0)));
    for (index, compared) in above.iter_mut().enumerate() {
        compared.number = Some(index + 1);
    }
    Ok(Agreement {
        first_path: first_path.to_string(),
        second_path: second_path.to_string(),
        first: first.clone(),
        second: second.clone(),
        head_height,
        above,
        at_or_under,
        placed_by_first_only,
        placed_by_second_only,
        judged_differently,
    })
}

impl Agreement {
    fn compared(&self) -> usize {
        self.above.len() + self.at_or_under.len()
    }

    fn largest(&self) -> Option<&Compared> {
        self.above.first().or(self.at_or_under.first())
    }

    /// The largest disagreement, as a share of head height. None when nothing was compared.
    pub fn largest_percent(&self) -> Option<f64> {
        self.largest()
            .map(|compared| compared.percent_of_head_height)
    }

    fn mean_percent(&self) -> f64 {
        let all = self.above.iter().chain(&self.at_or_under);
        let sum: f64 = all.map(|compared| compared.percent_of_head_height).sum();
        if self.compared() == 0 {
            0.0
        } else {
            sum / self.compared() as f64
        }
    }

    /// The agreement as the text of its file.
    pub fn file(&self) -> String {
        let mut out = String::from(
            "\
# The agreement between two readings of one drawing, by `maquette agreement-of-readings`:
# for every landmark both place, how far apart they put it, as a share of head height.
# It lists disagreements and judges none: which reading is right, if either, is Jon's check.
# It is worked out from readings of one reference and is never committed.
",
        );
        out.push_str("\n[drawing]\n");
        out.push_str(&format!("file = {}\n", quoted(&self.first.picture.file)));
        out.push_str(&format!(
            "sha256 = {}\n",
            quoted(&self.first.picture.sha256)
        ));
        out.push_str("\n[readings]\n");
        out.push_str(&format!("first = {}\n", quoted(&self.first_path)));
        out.push_str(&format!("first_reader = {}\n", quoted(&self.first.reader)));
        out.push_str(&format!("second = {}\n", quoted(&self.second_path)));
        out.push_str(&format!(
            "second_reader = {}\n",
            quoted(&self.second.reader)
        ));
        out.push_str(&other_instructions(&self.other_instructions()));
        if let Some(head_height) = self.head_height {
            out.push_str("\n[yardstick]\n");
            out.push_str(&format!(
                "head_height = {}  # pixels of the picture\n",
                toml_number(head_height, 3)
            ));
        }
        out.push_str("\n[summary]\n");
        out.push_str(&format!("compared = {}\n", self.compared()));
        out.push_str(&format!("above_1_percent = {}\n", self.above.len()));
        if let Some(largest) = self.largest() {
            out.push_str(&format!(
                "largest_percent_of_head_height = {}\n",
                percent(largest.percent_of_head_height)
            ));
            out.push_str(&format!("largest = {}\n", quoted(&largest.landmark)));
            out.push_str(&format!(
                "mean_percent_of_head_height = {}\n",
                percent(self.mean_percent())
            ));
        }
        out.push_str(&format!(
            "placed_by_first_only = {}\n",
            quoted_list(&self.placed_by_first_only)
        ));
        out.push_str(&format!(
            "placed_by_second_only = {}\n",
            quoted_list(&self.placed_by_second_only)
        ));
        if !self.judged_differently.is_empty() {
            out.push_str("\n# What the two readers judge differently.\n");
        }
        for (what, first, second) in &self.judged_differently {
            out.push_str("[[judged_differently]]\n");
            out.push_str(&format!("what = {}\n", quoted(what)));
            out.push_str(&format!("first = {}\n", quoted(first)));
            out.push_str(&format!("second = {}\n", quoted(second)));
        }
        for (list, entries) in [("above", &self.above), ("at_or_under", &self.at_or_under)] {
            if !entries.is_empty() {
                out.push_str(&format!(
                    "\n# Disagreements {} 1% of head height, largest first.\n",
                    if list == "above" {
                        "above"
                    } else {
                        "at or under"
                    }
                ));
            }
            for compared in entries {
                out.push_str(&format!("[[{list}]]\n"));
                if let Some(number) = compared.number {
                    out.push_str(&format!("number = {number}\n"));
                }
                out.push_str(&format!("landmark = {}\n", quoted(&compared.landmark)));
                out.push_str(&format!(
                    "compared_in = {}\n",
                    quoted(compared.marks.word())
                ));
                out.push_str(&format!("first = {}\n", position(compared.first)));
                out.push_str(&format!("second = {}\n", position(compared.second)));
                out.push_str(&format!(
                    "distance = {}\n",
                    toml_number(compared.distance, 3)
                ));
                out.push_str(&format!(
                    "percent_of_head_height = {}\n",
                    percent(compared.percent_of_head_height)
                ));
            }
        }
        out.push_str(&rules_table(&[
            "one_direction",
            "yardstick_of_two_readings",
        ]));
        out
    }

    /// The agreement in a few lines, for a person.
    pub fn summary(&self) -> String {
        let mut out = format!(
            "Two readings of `{}`, by {} and by {}.\n",
            self.first.picture.file, self.first.reader, self.second.reader
        );
        match self.largest() {
            Some(largest) => {
                out.push_str(&format!(
                    "{} of {} landmarks both place are more than 1% of head height apart. The largest disagreement is {}% ({}); the mean is {}%.\n",
                    self.above.len(),
                    self.compared(),
                    crate::number::plain(largest.percent_of_head_height, 3),
                    largest.landmark,
                    crate::number::plain(self.mean_percent(), 3),
                ));
            }
            None => out.push_str("No landmark is placed by both.\n"),
        }
        for compared in &self.above {
            out.push_str(&format!(
                "  {}. {}  {}%\n",
                compared.number.unwrap_or_default(),
                compared.landmark,
                crate::number::plain(compared.percent_of_head_height, 3)
            ));
        }
        for (who, names) in [
            (&self.first.reader, &self.placed_by_first_only),
            (&self.second.reader, &self.placed_by_second_only),
        ] {
            if !names.is_empty() {
                out.push_str(&format!("Placed by {who} only: {}.\n", names.join(", ")));
            }
        }
        for (what, first, second) in &self.judged_differently {
            out.push_str(&format!(
                "Judged differently: {what} is {first} for {} and {second} for {}.\n",
                self.first.reader, self.second.reader
            ));
        }
        for note in self.other_instructions() {
            out.push_str(&format!("{note}\n"));
        }
        out
    }

    /// What is said when the two readings follow a revision of the reading instructions
    /// that is not the program's: they follow one, or they are not compared.
    fn other_instructions(&self) -> Vec<String> {
        other_revision("The drawing", &self.first.instructions)
            .into_iter()
            .collect()
    }
}

// ---------------------------------------------------------------- turn

/// The left and right pairs whose spacing says how far a head is turned: each marks a
/// feature, so both views show the same two points of the head.
///
/// The arithmetic of the turn is the head model's, shared with the record of the hand-made
/// drawings. The list of pairs is not shared, and is not meant to be: this one is of the
/// landmarks a reader places on a picture, narrowed on what a reader can place sideways,
/// and that one (`hand_made_drawings`) is of positions taken from the nodes of a vector
/// drawing. Each is stated on its own evidence, and a change to one is no reason to change
/// the other.
const PAIRS_COUNTED: &[(&str, &str)] = &[
    ("eye", "outer-corner"),
    ("eye", "inner-corner"),
    ("iris", "centre"),
    ("lid-line", "inner-end"),
    ("lid-line", "outer-end"),
    ("brow-line", "inner-end"),
    ("brow-line", "outer-end"),
    ("nose-mark", "corner"),
    ("mouth-line", "end"),
    ("ear", "base-outer"),
    ("ear", "base-inner"),
];

/// The landmarks of the centre line that mark a place.
const ON_CENTRE_LINE: &[&str] = &["chin", "nose-mark-bottom", "mouth-line-centre"];

/// One view of a character, with the frame it is measured in.
struct Seen {
    view: View,
    positions: Positions,
    frame: head_model::Frame,
}

impl Seen {
    fn of(drawing: &Drawing, view: View) -> Result<Seen> {
        if drawing.view != view {
            return Err(Error::new(format!(
                "the reading of `{}` is of a {} view, and it is named where a {} view belongs",
                drawing.picture.file,
                drawing.view.name(),
                view.name()
            )));
        }
        let positions = drawing.positions()?;
        let frame = positions.frame(view)?;
        Ok(Seen {
            view,
            positions,
            frame,
        })
    }

    fn at(&self, name: &str) -> Option<[f64; 2]> {
        self.positions.points.get(name).copied()
    }

    /// How far apart the view shows a pair, in units, the character's left toward the
    /// viewer's right.
    fn spacing(&self, stem: &str, point: &str) -> Option<f64> {
        let left = self.at(&format!("{stem}-left-{point}"))?;
        let right = self.at(&format!("{stem}-right-{point}"))?;
        Some((left[0] - right[0]) / self.frame.unit)
    }

    /// A landmark's height below the eye line, in units.
    fn drop(&self, name: &str) -> Option<f64> {
        Some((self.at(name)?[1] - self.frame.eye_line_y) / self.frame.unit)
    }

    fn head_height(&self) -> Option<f64> {
        Some(self.drop("chin")? - self.drop("hair-top")?)
    }
}

/// Head height in units: the front view's, or the three-quarter view's where the front view
/// places no hair top.
fn head_height(front: &Seen, turned: &Seen) -> Result<f64> {
    front
        .head_height()
        .or(turned.head_height())
        .filter(|height| *height > 0.0)
        .ok_or_else(|| {
            Error::new(
                "neither view places the hair's top, so there is no head height to state a share of",
            )
        })
}

/// One pair, and the turn it gives by itself.
#[derive(Debug, Clone)]
pub struct Pair {
    pub name: String,
    /// In units.
    pub front_spacing: f64,
    pub three_quarter_spacing: f64,
    pub turn_degrees: f64,
    /// Three-quarter spacing minus front spacing times the fitted cosine, in units.
    pub misfit: f64,
    pub misfit_percent_of_head_height: f64,
}

/// The turn a three-quarter drawing shows, against the front view of the same character.
#[derive(Debug, Clone)]
pub struct Turn {
    pub degrees: f64,
    pub cosine: f64,
    pub faces: Faces,
    /// Widest in the front view first.
    pub pairs: Vec<Pair>,
    /// The counted pairs that one of the views does not place whole.
    pub not_placed: Vec<String>,
    pub stated_by_the_readers: Vec<f64>,
    /// In units.
    pub head_height: f64,
    pub front: String,
    pub three_quarter: String,
    pub read: ReadFrom,
}

fn turn_of(front: &Seen, turned: &Seen, drawing: &Drawing) -> Result<Turn> {
    let head_height = head_height(front, turned)?;
    let mut spacings = Vec::new();
    let mut not_placed = Vec::new();
    for (stem, point) in PAIRS_COUNTED {
        let name = format!("{stem}-{point}");
        match (front.spacing(stem, point), turned.spacing(stem, point)) {
            (Some(front), Some(turned)) => spacings.push((name, front, turned)),
            _ => not_placed.push(name),
        }
    }
    if spacings.is_empty() {
        return Err(Error::new(
            "no left and right pair is placed whole in both views, so the turn cannot be worked out. The eye corners, the brow ends, the mouth ends and the others are listed by `maquette help`",
        ));
    }
    // A pair is two places. One that the front view shows with no spacing has nothing for
    // the three-quarter view's spacing to have narrowed from, and would give no number.
    let no_spacing: Vec<&str> = spacings
        .iter()
        .filter(|(_, front, _)| *front <= 0.0)
        .map(|(name, _, _)| name.as_str())
        .collect();
    if !no_spacing.is_empty() {
        return Err(Error::new(format!(
            "the front view shows no spacing between the two points of {}: it puts both at one place sideways, or the character's left toward the viewer's left of the right. A turn is worked out from how far a spacing narrows, so none can be worked out from these readings",
            no_spacing.join(", ")
        )));
    }
    let cosine = head_model::turn_cosine(
        &spacings
            .iter()
            .map(|(_, front, turned)| (*front, *turned))
            .collect::<Vec<_>>(),
    );
    if cosine > 1.0 {
        return Err(Error::new(format!(
            "the three-quarter view shows its left and right pairs wider apart than the front view does, {} times as far, each view in its own unit. A turned head shows them closer together, so no turn gives this and none is reported: the drawings are not two views of one head, or a view is misjudged or misread",
            crate::number::plain(cosine, 4)
        )));
    }
    let degrees = head_model::turn_degrees;
    let mut pairs: Vec<Pair> = spacings
        .into_iter()
        .map(|(name, front_spacing, three_quarter_spacing)| {
            let misfit = three_quarter_spacing - front_spacing * cosine;
            Pair {
                name,
                front_spacing,
                three_quarter_spacing,
                turn_degrees: degrees(three_quarter_spacing / front_spacing),
                misfit,
                misfit_percent_of_head_height: misfit / head_height * 100.0,
            }
        })
        .collect();
    pairs.sort_by(|a, b| {
        b.front_spacing
            .total_cmp(&a.front_spacing)
            .then_with(|| a.name.cmp(&b.name))
    });
    Ok(Turn {
        degrees: degrees(cosine),
        cosine,
        faces: drawing.faces.unwrap_or(Faces::ViewersRight),
        pairs,
        not_placed,
        stated_by_the_readers: drawing.stated_turns.clone(),
        head_height,
        front: String::new(),
        three_quarter: String::new(),
        read: ReadFrom::default(),
    })
}

/// A view as a measuring command was given it: the path or paths it was named by, and its
/// one reading or two.
pub struct Given<'a> {
    pub named: &'a str,
    pub drawing: &'a Drawing,
}

/// Which readings something was worked out from, as its file says it.
#[derive(Debug, Clone, Default)]
pub struct ReadFrom {
    /// One line for each view: its picture, and who read it.
    pub lines: Vec<String>,
    /// One line for each view that was read under a revision of the reading instructions
    /// that is not the program's own.
    pub other_instructions: Vec<String>,
}

impl ReadFrom {
    fn of(views: &[(&str, &Given)]) -> ReadFrom {
        let lines = views
            .iter()
            .map(|(view, given)| {
                let mut line = format!(
                    "The {view} view, `{}`: {}.",
                    given.drawing.picture.file,
                    given.drawing.read_by()
                );
                if !given.drawing.placed_by_one.is_empty() {
                    line.push_str(&format!(
                        " Placed by one reading only and left out: {}.",
                        given.drawing.placed_by_one.join(", ")
                    ));
                }
                line
            })
            .collect();
        let other_instructions = views
            .iter()
            .filter_map(|(view, given)| {
                other_revision(&format!("The {view} view"), &given.drawing.instructions)
            })
            .collect();
        ReadFrom {
            lines,
            other_instructions,
        }
    }

    /// The lines of a file's `[readings]` or `[source]`.
    fn file(&self) -> String {
        format!(
            "read = {}\n{}",
            quoted_lines(&self.lines),
            other_instructions(&self.other_instructions)
        )
    }

    /// What a person is told beside the result: only what was not read by the instructions
    /// as they stand.
    fn summary(&self) -> String {
        self.other_instructions
            .iter()
            .map(|note| format!("{note}\n"))
            .collect()
    }
}

/// Works out the turn a three-quarter drawing shows from the front view of the same
/// character.
pub fn turn(front: &Given, three_quarter: &Given) -> Result<Turn> {
    let seen_front = Seen::of(front.drawing, View::Front)?;
    let seen_turned = Seen::of(three_quarter.drawing, View::ThreeQuarter)?;
    let mut turn = turn_of(&seen_front, &seen_turned, three_quarter.drawing)?;
    turn.front = front.named.to_string();
    turn.three_quarter = three_quarter.named.to_string();
    turn.read = ReadFrom::of(&[("front", front), ("three-quarter", three_quarter)]);
    Ok(turn)
}

impl Turn {
    /// The turn as the text of its file.
    pub fn file(&self) -> String {
        let mut out = String::from(
            "\
# The turn a three-quarter drawing shows, by `maquette turn-from-readings`: how far the head
# is turned from the front view of the same character, worked out from how far the spacing of
# its left and right pairs narrows. Spacings are in units, eye line to chin, each view in its own.
# It is worked out from readings of one reference and is never committed.
",
        );
        out.push_str("\n[readings]\n");
        out.push_str(&format!("front = {}\n", quoted(&self.front)));
        out.push_str(&format!(
            "three_quarter = {}\n",
            quoted(&self.three_quarter)
        ));
        out.push_str(&self.read.file());
        out.push_str("\n[turn]\n");
        out.push_str(&format!("degrees = {}\n", toml_number(self.degrees, 3)));
        out.push_str(&format!("cosine = {}\n", toml_number(self.cosine, 5)));
        out.push_str(&format!("faces = {}\n", quoted(self.faces.words())));
        out.push_str(&format!("pairs = {}\n", self.pairs.len()));
        let stated: Vec<String> = self
            .stated_by_the_readers
            .iter()
            .map(|turn| toml_number(*turn, 3))
            .collect();
        out.push_str(&format!(
            "stated_by_the_readers = [{}]  # each reader's own estimate, not used\n",
            stated.join(", ")
        ));
        out.push_str(&format!(
            "head_height = {}  # in units\n",
            toml_number(self.head_height, 5)
        ));
        out.push_str(&format!(
            "pairs_not_placed_in_both_views = {}\n",
            quoted_list(&self.not_placed)
        ));
        out.push_str(
            "\n# Each pair, widest in the front view first, and the turn it gives by itself.\n",
        );
        for pair in &self.pairs {
            out.push_str("[[pair]]\n");
            out.push_str(&format!("pair = {}\n", quoted(&pair.name)));
            out.push_str(&format!(
                "front_spacing = {}\n",
                toml_number(pair.front_spacing, 5)
            ));
            out.push_str(&format!(
                "three_quarter_spacing = {}\n",
                toml_number(pair.three_quarter_spacing, 5)
            ));
            out.push_str(&format!(
                "turn_degrees = {}\n",
                toml_number(pair.turn_degrees, 3)
            ));
            out.push_str(&format!(
                "misfit_percent_of_head_height = {}\n",
                percent(pair.misfit_percent_of_head_height)
            ));
        }
        out.push_str(&rules_table(&[
            "frame",
            "turn",
            "pairs_counted",
            "yardstick_of_views",
            "two_readings",
            "facing",
        ]));
        out
    }

    /// The turn in a few lines, for a person.
    pub fn summary(&self) -> String {
        let plain = crate::number::plain;
        let mut out = format!(
            "The three-quarter view is turned {} degrees toward {} (cosine {}), over {} pairs.\n",
            plain(self.degrees, 2),
            self.faces.words(),
            plain(self.cosine, 4),
            self.pairs.len()
        );
        if !self.stated_by_the_readers.is_empty() {
            let stated: Vec<String> = self
                .stated_by_the_readers
                .iter()
                .map(|turn| plain(*turn, 1))
                .collect();
            out.push_str(&format!(
                "The readers' own estimates: {} degrees.\n",
                stated.join(" and ")
            ));
        }
        let mut turns: Vec<f64> = self.pairs.iter().map(|pair| pair.turn_degrees).collect();
        turns.sort_by(f64::total_cmp);
        out.push_str(&format!(
            "The pairs one by one give {} to {} degrees.\n",
            plain(turns[0], 2),
            plain(turns[turns.len() - 1], 2)
        ));
        let worst = self
            .pairs
            .iter()
            .max_by(|a, b| a.misfit.abs().total_cmp(&b.misfit.abs()))
            .expect("at least one pair");
        out.push_str(&format!(
            "The largest misfit in spacing is {}% of head height ({}).\n",
            plain(worst.misfit_percent_of_head_height.abs(), 3),
            worst.name
        ));
        out.push_str(&self.read.summary());
        out
    }
}

// ---------------------------------------------------------------- fit

/// One place where the views do not show one head.
#[derive(Debug, Clone)]
pub struct Misfit {
    /// `level`, `spacing` or `centre`.
    pub kind: &'static str,
    /// The landmark, or the pair without its side.
    pub landmark: String,
    /// What disagrees, in words.
    pub what: String,
    pub percent_of_head_height: f64,
}

/// The fit of a character's views to one head.
#[derive(Debug, Clone)]
pub struct Fit {
    pub percent_of_head_height: f64,
    /// Largest first.
    pub misfits: Vec<Misfit>,
    /// The turn the three-quarter view shows. None when no three-quarter view was read.
    pub turn: Option<Turn>,
    /// Head height in units: the yardstick of every misfit.
    pub head_height: f64,
    pub views: Vec<String>,
    pub read: ReadFrom,
}

/// Whether a landmark's height says where a feature of the head is.
fn level_comparable(name: &str) -> bool {
    let of_the_body = name.starts_with("neck-") || name.starts_with("shoulder-");
    // The jaw line is the face outline under its other name: listed once.
    let named_twice = name.starts_with("jaw-line-");
    !of_the_body && !named_twice && compared_in(name) != Marks::Sideways
}

/// Works out the fit of a character's views to one head: its front view with a
/// three-quarter view, a side view or both. A side view is compared in height.
pub fn fit(front: &Given, three_quarter: Option<&Given>, side: Option<&Given>) -> Result<Fit> {
    if three_quarter.is_none() && side.is_none() {
        return Err(Error::new(
            "a fit sets a character's views against each other, so it needs a three-quarter view or a side view beside the front view",
        ));
    }
    let seen_front = Seen::of(front.drawing, View::Front)?;
    let seen_turned = match three_quarter {
        Some(three_quarter) => Some(Seen::of(three_quarter.drawing, View::ThreeQuarter)?),
        None => None,
    };
    let seen_side = match side {
        Some(side) => Some(Seen::of(side.drawing, View::Side)?),
        None => None,
    };
    let mut turn = match (three_quarter, &seen_turned) {
        (Some(three_quarter), Some(seen_turned)) => {
            let mut turn = turn_of(&seen_front, seen_turned, three_quarter.drawing)?;
            turn.front = front.named.to_string();
            turn.three_quarter = three_quarter.named.to_string();
            Some(turn)
        }
        _ => None,
    };
    let head_height = match &turn {
        Some(turn) => turn.head_height,
        None => [Some(&seen_front), seen_side.as_ref()]
            .into_iter()
            .flatten()
            .find_map(Seen::head_height)
            .filter(|height| *height > 0.0)
            .ok_or_else(|| {
                Error::new(
                    "neither view places the hair's top, so there is no head height to state a share of",
                )
            })?,
    };
    let share = |units: f64| round3(units / head_height * 100.0);
    let plain = crate::number::plain;
    let mut misfits = Vec::new();

    // Level: one height in every view.
    let mut views: Vec<&Seen> = vec![&seen_front];
    views.extend(seen_turned.as_ref());
    views.extend(seen_side.as_ref());
    let mut names: Vec<&String> = views
        .iter()
        .flat_map(|seen| seen.positions.points.keys())
        .filter(|name| level_comparable(name))
        .collect();
    names.sort();
    names.dedup();
    for name in names {
        let drops: Vec<(View, f64)> = views
            .iter()
            .filter_map(|seen| Some((seen.view, seen.drop(name)?)))
            .collect();
        if drops.len() < 2 {
            continue;
        }
        let highest = drops
            .iter()
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .expect("two views");
        let lowest = drops
            .iter()
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .expect("two views");
        misfits.push(Misfit {
            kind: "level",
            landmark: name.clone(),
            what: format!(
                "{} units below the eye line in the {} view and {} in the {} view",
                plain(highest.1, 4),
                highest.0.name(),
                plain(lowest.1, 4),
                lowest.0.name()
            ),
            percent_of_head_height: share(lowest.1 - highest.1),
        });
    }

    // Spacing: each counted pair against the turn, where a three-quarter view shows one.
    for (pair, turn) in turn
        .iter()
        .flat_map(|turn| turn.pairs.iter().map(move |pair| (pair, turn)))
    {
        misfits.push(Misfit {
            kind: "spacing",
            landmark: pair.name.clone(),
            what: format!(
                "{} units apart in the three-quarter view, where the front view's {} and the turn give {}",
                plain(pair.three_quarter_spacing, 4),
                plain(pair.front_spacing, 4),
                plain(pair.front_spacing * turn.cosine, 4)
            ),
            percent_of_head_height: share(pair.misfit.abs()),
        });
    }

    // Centre: one upright centre line in the front view.
    let mut across: Vec<(String, f64)> = Vec::new();
    for name in ON_CENTRE_LINE {
        if let Some(at) = seen_front.at(name) {
            across.push((name.to_string(), at[0]));
        }
    }
    for (stem, point) in PAIRS_COUNTED {
        let left = seen_front.at(&format!("{stem}-left-{point}"));
        let right = seen_front.at(&format!("{stem}-right-{point}"));
        if let (Some(left), Some(right)) = (left, right) {
            across.push((format!("{stem}-{point}"), (left[0] + right[0]) / 2.0));
        }
    }
    if across.len() > 1 {
        let centre = across.iter().map(|(_, x)| x).sum::<f64>() / across.len() as f64;
        for (name, x) in &across {
            let off = (x - centre) / seen_front.frame.unit;
            misfits.push(Misfit {
                kind: "centre",
                landmark: name.clone(),
                what: format!(
                    "{} units toward the viewer's {} of the front view's centre line",
                    plain(off.abs(), 4),
                    if off >= 0.0 { "right" } else { "left" }
                ),
                percent_of_head_height: share(off.abs()),
            });
        }
    }

    misfits.sort_by(|a, b| {
        b.percent_of_head_height
            .total_cmp(&a.percent_of_head_height)
            .then_with(|| a.kind.cmp(b.kind))
            .then_with(|| a.landmark.cmp(&b.landmark))
    });
    let mut given = vec![("front", front)];
    if let Some(three_quarter) = three_quarter {
        given.push(("three-quarter", three_quarter));
        if let Some(turn) = &mut turn {
            turn.read = ReadFrom::of(&given);
        }
    }
    if let Some(side) = side {
        given.push(("side", side));
    }
    Ok(Fit {
        percent_of_head_height: misfits
            .first()
            .map(|misfit| misfit.percent_of_head_height)
            .unwrap_or(0.0),
        misfits,
        turn,
        head_height,
        views: given
            .iter()
            .map(|(view, given)| format!("{view}: {}", given.named))
            .collect(),
        read: ReadFrom::of(&given),
    })
}

impl Fit {
    /// The fit as the text of its file.
    pub fn file(&self) -> String {
        let mut out = String::from(
            "\
# The fit of a character's views to one head, by `maquette fit-of-views`: as one figure, how
# far the views are from being drawings of one level, symmetric head, as a share of head height.
# It lists misfits and judges none: whether the views count as one character's is Jon's check.
# It is worked out from readings of one reference and is never committed.
",
        );
        out.push_str("\n[readings]\n");
        out.push_str(&format!("views = {}\n", quoted_lines(&self.views)));
        out.push_str(&self.read.file());
        out.push_str("\n[fit]\n");
        out.push_str(&format!(
            "percent_of_head_height = {}\n",
            percent(self.percent_of_head_height)
        ));
        if let Some(worst) = self.misfits.first() {
            out.push_str(&format!("worst_kind = {}\n", quoted(worst.kind)));
            out.push_str(&format!("worst = {}\n", quoted(&worst.landmark)));
        }
        for kind in ["level", "spacing", "centre"] {
            let of_kind: Vec<&Misfit> = self
                .misfits
                .iter()
                .filter(|misfit| misfit.kind == kind)
                .collect();
            out.push_str(&format!("{kind}_compared = {}\n", of_kind.len()));
            if let Some(worst) = of_kind.first() {
                out.push_str(&format!(
                    "{kind}_largest_percent_of_head_height = {}\n",
                    percent(worst.percent_of_head_height)
                ));
            }
        }
        out.push_str(&format!(
            "head_height = {}  # in units: what a misfit is a share of\n",
            toml_number(self.head_height, 5)
        ));
        if let Some(turn) = &self.turn {
            out.push_str("\n# The turn the spacings were set against.\n[turn]\n");
            out.push_str(&format!("degrees = {}\n", toml_number(turn.degrees, 3)));
            out.push_str(&format!("cosine = {}\n", toml_number(turn.cosine, 5)));
            out.push_str(&format!("faces = {}\n", quoted(turn.faces.words())));
            out.push_str(&format!(
                "head_height = {}  # in units\n",
                toml_number(turn.head_height, 5)
            ));
        }
        out.push_str("\n# Every misfit, largest first.\n");
        for misfit in &self.misfits {
            out.push_str("[[misfit]]\n");
            out.push_str(&format!("kind = {}\n", quoted(misfit.kind)));
            out.push_str(&format!("landmark = {}\n", quoted(&misfit.landmark)));
            out.push_str(&format!("what = {}\n", quoted(&misfit.what)));
            out.push_str(&format!(
                "percent_of_head_height = {}\n",
                percent(misfit.percent_of_head_height)
            ));
        }
        out.push_str(&rules_table(&[
            "fit",
            "fit_left_out",
            "frame",
            "turn",
            "pairs_counted",
            "yardstick_of_views",
            "two_readings",
            "facing",
        ]));
        out
    }

    /// The fit in a few lines, for a person.
    pub fn summary(&self) -> String {
        let plain = crate::number::plain;
        let mut out = format!(
            "The views fit one head to within {}% of head height.\n",
            plain(self.percent_of_head_height, 3)
        );
        for misfit in self
            .misfits
            .iter()
            .filter(|misfit| misfit.percent_of_head_height > THRESHOLD_PERCENT)
        {
            out.push_str(&format!(
                "  {}%  {} of {}: {}\n",
                plain(misfit.percent_of_head_height, 3),
                misfit.kind,
                misfit.landmark,
                misfit.what
            ));
        }
        match &self.turn {
            Some(turn) => out.push_str(&format!(
                "The three-quarter view is turned {} degrees toward {}.\n",
                plain(turn.degrees, 2),
                turn.faces.words()
            )),
            None => out.push_str(
                "No three-quarter view was read: the front view and the side view are set against each other in height, and the front view against its own centre line.\n",
            ),
        }
        out.push_str(&self.read.summary());
        out
    }
}

// ---------------------------------------------------------------- the skull rule

/// Runs the skull rule on a set of proportions and sets what it builds against the set's
/// own skull, part by part, as text. It tests the rule on a head whose skull is drawn; it
/// changes nothing and tunes nothing.
pub fn skull_rule_on(set: &crate::proportions::Set, named: &str) -> Result<String> {
    let lacking =
        |lacking: String| Error::new(format!("the set of proportions has no `{lacking}`"));
    let built = head_model::skull_by_rule(|name| Some(set.get(name))).map_err(lacking)?;
    let head_height = head_model::head_height(set.get("hair_top_rise"));
    let (rise, depth, radius) = (
        set.get("skull_centre_rise"),
        set.get("skull_centre_depth"),
        set.get("skull_radius"),
    );
    // The radius part alone: the same distance to the temple, from the set's own centre.
    let radius_alone = head_model::skull_radius_to_temple(|name| Some(set.get(name)), rise, depth)
        .map_err(lacking)?;
    let mut out = String::from(
        "\
# The skull rule run on a set of proportions, by `maquette skull-by-rule`, and set against the
# set's own skull, part by part. The rule builds a skull where nothing is drawn: its centre at
# the height and the depth of the middle of the ear's base, and its radius the distance in
# space from that centre to the temple. This tests the rule; it tunes nothing to the set.
# Every number is in units, eye line to chin. A difference is the rule's minus the set's.
",
    );
    out.push_str(&format!("set = {}\n", quoted(named)));
    out.push_str(&format!("head_height = {}\n", toml_number(head_height, 6)));
    out.push_str(&format!(
        "hair_top_rise = {}\n",
        toml_number(set.get("hair_top_rise"), 6)
    ));
    out.push_str("\n# What the rule builds from.\n[built_from]\n");
    for name in head_model::SKULL_RULE_USES {
        out.push_str(&format!("{name} = {}\n", toml_number(set.get(name), 6)));
    }
    for (name, what, by_rule, the_sets) in [
        (
            "centre_rise",
            "The centre's height above the eye line: the middle of the ear's base.",
            built.centre_rise,
            rise,
        ),
        (
            "centre_depth",
            "The centre's depth: the middle of the ear's base.",
            built.centre_depth,
            depth,
        ),
        (
            "radius",
            "The radius as the whole rule gives it: from the rule's own centre to the temple.",
            built.radius,
            radius,
        ),
        (
            "radius_from_the_sets_centre",
            "The radius part of the rule alone: from the set's own centre to the temple.",
            radius_alone,
            radius,
        ),
        (
            "top_rise",
            "How high the skull's top stands above the eye line: the centre's rise plus the radius. The hair's top is at hair_top_rise.",
            built.centre_rise + built.radius,
            rise + radius,
        ),
    ] {
        out.push_str(&format!("\n[part.{name}]\n"));
        out.push_str(&format!("what = {}\n", quoted(what)));
        out.push_str(&format!("by_rule = {}\n", toml_number(by_rule, 6)));
        out.push_str(&format!("the_sets = {}\n", toml_number(the_sets, 6)));
        out.push_str(&format!(
            "difference = {}\n",
            toml_number(by_rule - the_sets, 6)
        ));
        out.push_str(&format!(
            "percent_of_head_height = {}\n",
            toml_number((by_rule - the_sets).abs() / head_height * 100.0, 4)
        ));
    }
    Ok(out)
}

// ---------------------------------------------------------------- proportions

/// Where a character's depth is taken from: the view read beside the front view.
#[derive(Debug, Clone)]
pub enum DepthFrom {
    /// No view: a front view read alone shows no depth.
    NoView,
    /// A three-quarter view, at the turn the readings show.
    ThreeQuarter(Turn),
    /// A side view, which faces this way.
    Side(Faces),
}

impl DepthFrom {
    fn words(&self) -> &'static str {
        match self {
            DepthFrom::NoView => {
                "no view: a front view was read alone, and it shows heights and widths and no depth"
            }
            DepthFrom::ThreeQuarter(_) => {
                "the three-quarter view, at the turn the readings themselves show"
            }
            DepthFrom::Side(_) => {
                "the side view, which shows a point at its depth; each view is measured in its own unit"
            }
        }
    }
}

/// The proportions a character's readings give.
#[derive(Debug, Clone)]
pub struct Proportions {
    pub taken: Taken,
    pub depth_from: DepthFrom,
    pub views: Vec<String>,
    pub read: ReadFrom,
    pub digests: BTreeMap<String, String>,
}

/// Refuses a drawing that is named where another view belongs.
fn of_view(given: &Given, view: View) -> Result<()> {
    if given.drawing.view == view {
        return Ok(());
    }
    Err(Error::new(format!(
        "the reading of `{}` is of a {} view, and it is named where a {} view belongs",
        given.drawing.picture.file,
        given.drawing.view.name(),
        view.name()
    )))
}

/// Takes the readings of a character back into proportions: of its front view alone, which
/// gives heights and widths; with a three-quarter view, at the turn the readings themselves
/// show; or with a side view, which shows depth as it is.
pub fn proportions(
    front: &Given,
    three_quarter: Option<&Given>,
    side: Option<&Given>,
) -> Result<Proportions> {
    of_view(front, View::Front)?;
    let front_positions = front.drawing.positions()?;
    let mut given = vec![("front", front)];
    let (taken, depth_from) = match (three_quarter, side) {
        (Some(_), Some(_)) => {
            return Err(Error::new(
                "depth is taken from a three-quarter view or from a side view, not both: taking a character's three views together is not built. Name the side view where the character has one, which is the first route to depth",
            ));
        }
        (Some(three_quarter), None) => {
            let turn = turn(front, three_quarter)?;
            let mut turned_positions = three_quarter.drawing.positions()?;
            turned_positions.turn_degrees = turn.degrees;
            given.push(("three-quarter", three_quarter));
            (
                head_model::take_proportions(&front_positions, &turned_positions)?,
                DepthFrom::ThreeQuarter(turn),
            )
        }
        (None, Some(side)) => {
            of_view(side, View::Side)?;
            given.push(("side", side));
            (
                head_model::take_from(
                    &front_positions,
                    head_model::Beside::Side(&side.drawing.positions()?),
                )?,
                DepthFrom::Side(side.drawing.faces.unwrap_or(Faces::ViewersRight)),
            )
        }
        (None, None) => (
            head_model::take_from(&front_positions, head_model::Beside::Nothing)?,
            DepthFrom::NoView,
        ),
    };
    Ok(Proportions {
        taken,
        views: given
            .iter()
            .map(|(view, given)| format!("{view}: {}", given.named))
            .collect(),
        read: ReadFrom::of(&given),
        digests: given
            .iter()
            .map(|(view, given)| (view.replace('-', "_"), given.drawing.picture.sha256.clone()))
            .collect(),
        depth_from,
    })
}

impl Proportions {
    /// The proportions as the text of a file: a set of proportions when the readings give
    /// every one, and otherwise the ones they give with the rest listed and why.
    pub fn file(&self) -> Result<String> {
        let list = crate::proportions::list()?;
        let mut out = String::from(
            "\
# The proportions a character's readings give, by `maquette proportions-from-readings`: the
# readings of its front view, alone or with a three-quarter view or a side view, taken back
# into proportions by the head model. [source] says which views were read and where depth is from.
# It is worked out from readings of one reference and is never committed.
#
# `maquette list-proportions` defines each proportion. When every proportion is given, this
# file is a set of proportions and `maquette draw` reads it.
",
        );
        out.push_str("\n[source]\n");
        out.push_str(&format!("views = {}\n", quoted_lines(&self.views)));
        out.push_str(&self.read.file());
        for (view, digest) in &self.digests {
            out.push_str(&format!("{view}_sha256 = {}\n", quoted(digest)));
        }
        out.push_str(&format!(
            "depth_from = {}\n",
            quoted(self.depth_from.words())
        ));
        match &self.depth_from {
            DepthFrom::ThreeQuarter(turn) => {
                out.push_str(&format!(
                    "turn_degrees = {}  # worked out from the readings, over {} pairs\n",
                    toml_number(turn.degrees, 3),
                    turn.pairs.len()
                ));
                out.push_str(&format!("faces = {}\n", quoted(turn.faces.words())));
            }
            DepthFrom::Side(faces) => {
                out.push_str(&format!("faces = {}\n", quoted(faces.words())));
            }
            DepthFrom::NoView => {}
        }
        out.push_str(&format!("skull = {}\n", quoted(self.taken.skull.words())));
        match &self.depth_from {
            DepthFrom::ThreeQuarter(_) => out.push_str(&format!(
                "disagreement_rules = {}\n",
                quoted_lines(list.disagreement_rules())
            )),
            DepthFrom::Side(_) => out.push_str(&format!(
                "side_view_rules = {}\n",
                quoted_lines(list.side_view_rules())
            )),
            DepthFrom::NoView => {}
        }
        out.push_str(&format!("given = {}\n", self.taken.values.len()));
        out.push_str(&format!("not_given = {}\n", self.taken.not_given.len()));
        if !self.taken.not_given.is_empty() {
            out.push_str("\n# The proportions the readings do not give, and why.\n[not_given]\n");
            for (name, why) in &self.taken.not_given {
                out.push_str(&format!("{name} = {}\n", quoted(why)));
            }
        }
        out.push_str("\n[proportions]\n");
        for proportion in &list.proportions {
            if let Some(value) = self.taken.values.get(&proportion.name) {
                out.push_str(&format!(
                    "{} = {}\n",
                    proportion.name,
                    toml_number(*value, 6)
                ));
            }
        }
        Ok(out)
    }

    /// What was taken, in a few lines, for a person.
    pub fn summary(&self) -> String {
        let mut out = format!(
            "The readings give {} of {} proportions, {}.\n",
            self.taken.values.len(),
            self.taken.values.len() + self.taken.not_given.len(),
            match &self.depth_from {
                DepthFrom::ThreeQuarter(turn) => format!(
                    "at a turn of {} degrees",
                    crate::number::plain(turn.degrees, 2)
                ),
                DepthFrom::Side(faces) => format!(
                    "with depth from the side view, which faces {}",
                    faces.words()
                ),
                DepthFrom::NoView => {
                    "from the front view alone: heights and widths, and no depth".to_string()
                }
            }
        );
        out.push_str(&format!("The skull: {}.\n", self.taken.skull.words()));
        for (name, why) in &self.taken.not_given {
            out.push_str(&format!("  not given: {name}: {why}\n"));
        }
        out.push_str(&self.read.summary());
        out
    }
}
