//! The differences between a generated armature and a hand-made one: for every landmark both
//! give in a view, how far the generated position is from the hand-made one, as a share of
//! head height.
//!
//! This module hides how the two are laid over each other, which positions are compared and
//! in what, and the order the differences are numbered in. [`RULES`] states each, and the
//! file it writes carries them. It lists differences and judges none: whether one matters is
//! Jon's look.

use crate::armature::{self, Placed};
use crate::hand_made_views::{self, MARKS_RULE, Marks, Provenance};
use crate::head_model::Reading;
use crate::number::{plain, round3, toml as toml_number};
use crate::toml_text::{quoted, quoted_list};
use crate::{Error, Result};

/// A difference is listed with a number when it is more than this share of head height.
pub const THRESHOLD_PERCENT: f64 = 1.0;

/// The rules of the comparison, by name, as the differences file states them.
pub const RULES: &[(&str, &str)] = &[
    (
        "lay_over",
        "A generated view is laid over its hand-made view by one scale and one shift, the same for every landmark of the view and fitted to none of them. Three things are made to coincide: the unit (eye line to chin), the eye line, and the upright axis the head turns about (through the skull's centre). A position x, y of the landmark file then stands at shift_x + scale * x, shift_y + scale * y in the record's units, where scale is the hand-made unit over the generated one. Nothing is turned or stretched. The generated unit, eye line and axis are the landmark file's [frame]. The hand-made ones are read from the record as the list of proportions defines them: the eye line midway between the top and the bottom of the eye openings, averaged over the eyes the view shows; the unit from there down to the chin; the axis at the skull's centre. So the chin's height and the skull centre's sideways position cannot differ.",
    ),
    (
        "yardstick",
        "Head height is the hand-made front view's, from the hair's top to the chin, in the record's units. Both views are measured against that one figure.",
    ),
    (
        "names",
        "A hand-made position is compared with the generated landmark or view point that the reading rules give its name to; `hand_made_from` gives the record's own names behind each. A plain renaming would be wrong: a lid line and a brow line are the middle of a drawn band, and in a turned view the near side's outline is the jaw line. Where two generated landmarks stand at one position in a view and are compared with the same hand-made position, as the cheek point and the jaw-line point of a level do in a front view, the difference is listed once and `also` names the second.",
    ),
    ("one_direction", MARKS_RULE),
    (
        "distance",
        "Both positions are written to 0.001 unit, in the record's units, and `distance` is between the positions as written. `percent_of_head_height` is the distance over the yardstick. A difference is above 1% when its distance is more than a hundredth of the yardstick, so exactly 1% is not above it.",
    ),
    (
        "order",
        "Differences above 1% are numbered in one run over both views: the front view first, then the three-quarter view, and in each the largest first, with landmarks of equal distance in the order of their names. The differences at or under 1% are listed the same way, without a number.",
    ),
    (
        "without_counterpart",
        "`generated` lists the landmarks and view points of the landmark file that the hand-made view gives no position for. `hand_made` lists the record's landmarks of the view, under the record's own names, that stand behind no compared position: what the reading rules do not read, and what the generated view has no landmark for. A name can stand in both lists: the reading rules do not read the record's centre of an eye opening, which is worked out from positions already compared, nor its drawn centre line, which the armature builds by rule.",
    ),
    (
        "covered",
        "A hand-made position that the head covers in the drawing is compared all the same, and marked `covered_by_the_head`. The record gives it in the drawing's own coordinates. It cannot be seen in the drawing, and no proportion was taken from it.",
    ),
];

/// A file given to the comparison: the path it was named by, and its text.
#[derive(Debug, Clone, Copy)]
pub struct Input<'a> {
    pub path: &'a str,
    pub text: &'a str,
}

/// One landmark both views give, and how far apart they put it.
#[derive(Debug, Clone)]
pub struct Difference {
    /// Its number, when it is above the threshold.
    pub number: Option<usize>,
    pub landmark: String,
    /// Other generated landmarks at the same position, compared with the same hand-made one.
    pub also: Vec<String>,
    pub marks: Marks,
    /// The generated position, laid over, in the record's units.
    pub generated: [f64; 2],
    pub hand_made: [f64; 2],
    /// The record's names behind the hand-made position.
    pub hand_made_from: Vec<String>,
    /// The head covers the hand-made position in the drawing.
    pub covered_by_the_head: bool,
    /// In the record's units.
    pub distance: f64,
    pub percent_of_head_height: f64,
}

/// How a generated view is laid over its hand-made view.
#[derive(Debug, Clone)]
pub struct LayOver {
    pub scale: f64,
    pub shift_x: f64,
    pub shift_y: f64,
    pub generated: Placed,
    pub hand_made_unit: f64,
    pub hand_made_eye_line_y: f64,
    pub hand_made_turn_axis_x: f64,
}

/// The differences of one view.
#[derive(Debug, Clone)]
pub struct ViewDifferences {
    /// `front` or `three-quarter`.
    pub view: &'static str,
    pub landmark_file: String,
    pub lay_over: LayOver,
    /// Numbered, largest first.
    pub above: Vec<Difference>,
    /// Largest first.
    pub at_or_under: Vec<Difference>,
    pub generated_without_counterpart: Vec<String>,
    pub hand_made_without_counterpart: Vec<String>,
}

/// The differences of both views.
#[derive(Debug, Clone)]
pub struct Differences {
    pub record: String,
    /// What the record says the drawings are.
    pub what: String,
    /// The yardstick, in the record's units.
    pub head_height: f64,
    pub front: ViewDifferences,
    pub three_quarter: ViewDifferences,
}

fn above_threshold(distance: f64, head_height: f64) -> bool {
    distance * 100.0 > THRESHOLD_PERCENT * head_height
}

fn compare_view(
    view: &'static str,
    file: Input,
    hand_made: &Reading,
    provenance: &Provenance,
    head_height: f64,
) -> Result<ViewDifferences> {
    let placed = armature::read_placement(file.text)
        .map_err(|error| Error::new(format!("{}: {error}", file.path)))?;
    if placed.view != view {
        return Err(Error::new(format!(
            "{} is the landmark file of a {} view, not of a {view} view",
            file.path, placed.view
        )));
    }
    let generated = armature::read_landmark_file(file.text)
        .map_err(|error| Error::new(format!("{}: {error}", file.path)))?;

    let frame = hand_made.frame(view)?;
    let axis = hand_made
        .points
        .get("skull-centre")
        .ok_or_else(|| Error::new(format!("the record's {view} view has no skull centre")))?[0];
    let scale = frame.unit / placed.unit;
    let lay_over = LayOver {
        scale,
        shift_x: axis - scale * placed.turn_axis_x,
        shift_y: frame.eye_line_y - scale * placed.eye_line_y,
        generated: placed,
        hand_made_unit: frame.unit,
        hand_made_eye_line_y: frame.eye_line_y,
        hand_made_turn_axis_x: axis,
    };

    let mut differences: Vec<Difference> = Vec::new();
    let mut generated_without_counterpart = Vec::new();
    for (name, position) in &generated.points {
        let seen = hand_made.points.get(name);
        let Some(theirs) = seen.or_else(|| provenance.covered_by_head.get(name)) else {
            generated_without_counterpart.push(name.clone());
            continue;
        };
        let laid_over = [
            round3(lay_over.shift_x + scale * position[0]),
            round3(lay_over.shift_y + scale * position[1]),
        ];
        let theirs = [round3(theirs[0]), round3(theirs[1])];
        let from = provenance.read_from.get(name).cloned().unwrap_or_default();
        // One position of the view compared with one hand-made position is one difference.
        if let Some(same) = differences.iter_mut().find(|difference| {
            difference.generated == laid_over && difference.hand_made_from == from
        }) {
            same.also.push(name.clone());
            continue;
        }
        let marks = hand_made_views::marks(name);
        let (sideways, height) = (laid_over[0] - theirs[0], laid_over[1] - theirs[1]);
        let distance = round3(match marks {
            Marks::Position => sideways.hypot(height),
            Marks::Height => height.abs(),
            Marks::Sideways => sideways.abs(),
        });
        differences.push(Difference {
            number: None,
            landmark: name.clone(),
            also: Vec::new(),
            marks,
            generated: laid_over,
            hand_made: theirs,
            hand_made_from: from,
            covered_by_the_head: seen.is_none(),
            distance,
            percent_of_head_height: distance / head_height * 100.0,
        });
    }
    differences.sort_by(|first, second| {
        second
            .distance
            .total_cmp(&first.distance)
            .then_with(|| first.landmark.cmp(&second.landmark))
    });

    let compared: Vec<&String> = differences
        .iter()
        .flat_map(|difference| &difference.hand_made_from)
        .collect();
    let hand_made_without_counterpart = provenance
        .recorded
        .iter()
        .filter(|name| !compared.contains(name))
        .cloned()
        .collect();

    let (above, at_or_under) = differences
        .into_iter()
        .partition(|difference| above_threshold(difference.distance, head_height));
    Ok(ViewDifferences {
        view,
        landmark_file: file.path.to_string(),
        lay_over,
        above,
        at_or_under,
        generated_without_counterpart,
        hand_made_without_counterpart,
    })
}

/// Compares the landmark files of a generated front view and three-quarter view with the
/// record of a hand-made armature's views.
pub fn compare(front: Input, three_quarter: Input, record: Input) -> Result<Differences> {
    let hand_made = hand_made_views::read(record.text)
        .map_err(|error| Error::new(format!("{}: {error}", record.path)))?;
    let point = |name: &str| -> Result<[f64; 2]> {
        hand_made.front.points.get(name).copied().ok_or_else(|| {
            Error::new(format!(
                "{}: the front view has no `{name}`, so there is no head height to measure against",
                record.path
            ))
        })
    };
    let head_height = round3(point("chin")?[1] - point("hair-top")?[1]);
    if head_height <= 0.0 {
        return Err(Error::new(format!(
            "{}: in the front view the chin is not below the hair's top",
            record.path
        )));
    }

    let mut front = compare_view(
        "front",
        front,
        &hand_made.front,
        &hand_made.front_record,
        head_height,
    )?;
    let mut three_quarter = compare_view(
        "three-quarter",
        three_quarter,
        &hand_made.three_quarter,
        &hand_made.three_quarter_record,
        head_height,
    )?;
    let numbered = front.above.iter_mut().chain(three_quarter.above.iter_mut());
    for (index, difference) in numbered.enumerate() {
        difference.number = Some(index + 1);
    }
    Ok(Differences {
        record: record.path.to_string(),
        what: hand_made.what,
        head_height,
        front,
        three_quarter,
    })
}

fn position(value: [f64; 2]) -> String {
    format!(
        "{{ x = {}, y = {} }}",
        toml_number(value[0], 3),
        toml_number(value[1], 3)
    )
}

fn marks_word(marks: Marks) -> &'static str {
    match marks {
        Marks::Position => "position",
        Marks::Height => "height",
        Marks::Sideways => "sideways",
    }
}

impl ViewDifferences {
    fn compared(&self) -> usize {
        self.above.len() + self.at_or_under.len()
    }

    fn file(&self, out: &mut String) {
        let view = self.view;
        out.push_str(&format!("\n[{view}]\n"));
        out.push_str(&format!(
            "landmark_file = {}\n",
            quoted(&self.landmark_file)
        ));

        let lay_over = &self.lay_over;
        out.push_str(
            "\n# A position x, y of the landmark file stands at shift_x + scale * x, shift_y + scale * y.\n",
        );
        out.push_str(&format!("[{view}.lay_over]\n"));
        out.push_str(&format!("scale = {}\n", toml_number(lay_over.scale, 6)));
        out.push_str(&format!("shift_x = {}\n", toml_number(lay_over.shift_x, 3)));
        out.push_str(&format!("shift_y = {}\n", toml_number(lay_over.shift_y, 3)));
        out.push_str(&format!(
            "generated = {{ unit = {}, eye_line_y = {}, turn_axis_x = {} }}\n",
            toml_number(lay_over.generated.unit, 3),
            toml_number(lay_over.generated.eye_line_y, 3),
            toml_number(lay_over.generated.turn_axis_x, 3)
        ));
        out.push_str(&format!(
            "hand_made = {{ unit = {}, eye_line_y = {}, turn_axis_x = {} }}\n",
            toml_number(lay_over.hand_made_unit, 3),
            toml_number(lay_over.hand_made_eye_line_y, 3),
            toml_number(lay_over.hand_made_turn_axis_x, 3)
        ));

        out.push_str(&format!("\n[{view}.counts]\n"));
        out.push_str(&format!("compared = {}\n", self.compared()));
        out.push_str(&format!("above_one_percent = {}\n", self.above.len()));
        out.push_str(&format!(
            "at_or_under_one_percent = {}\n",
            self.at_or_under.len()
        ));

        out.push_str(&format!("\n[{view}.without_counterpart]\n"));
        out.push_str(&format!(
            "generated = {}\n",
            quoted_list(&self.generated_without_counterpart)
        ));
        out.push_str(&format!(
            "hand_made = {}\n",
            quoted_list(&self.hand_made_without_counterpart)
        ));

        for (list, differences) in [
            ("above_one_percent", &self.above),
            ("at_or_under_one_percent", &self.at_or_under),
        ] {
            for difference in differences {
                out.push_str(&format!("\n[[{view}.{list}]]\n"));
                if let Some(number) = difference.number {
                    out.push_str(&format!("number = {number}\n"));
                }
                out.push_str(&format!("landmark = {}\n", quoted(&difference.landmark)));
                if !difference.also.is_empty() {
                    out.push_str(&format!("also = {}\n", quoted_list(&difference.also)));
                }
                out.push_str(&format!(
                    "compared = {}\n",
                    quoted(marks_word(difference.marks))
                ));
                out.push_str(&format!("generated = {}\n", position(difference.generated)));
                out.push_str(&format!("hand_made = {}\n", position(difference.hand_made)));
                out.push_str(&format!(
                    "hand_made_from = {}\n",
                    quoted_list(&difference.hand_made_from)
                ));
                if difference.covered_by_the_head {
                    out.push_str("covered_by_the_head = true\n");
                }
                out.push_str(&format!(
                    "distance = {}\n",
                    toml_number(difference.distance, 3)
                ));
                out.push_str(&format!(
                    "percent_of_head_height = {}\n",
                    toml_number(difference.percent_of_head_height, 3)
                ));
            }
        }
    }

    fn summary(&self, out: &mut String) {
        let largest_under = self
            .at_or_under
            .first()
            .map_or(String::new(), |difference| {
                format!(
                    ", the largest of those {}%",
                    plain_fixed(difference.percent_of_head_height)
                )
            });
        out.push_str(&format!(
            "\n{}: {} compared, {} above 1%, {} at or under{largest_under}\n",
            self.view,
            self.compared(),
            self.above.len(),
            self.at_or_under.len()
        ));
        if self.above.is_empty() {
            return;
        }
        let width = self
            .above
            .iter()
            .map(|difference| difference.landmark.len())
            .max()
            .unwrap_or(0)
            .max("landmark".len());
        out.push_str(&format!(
            "  {:>3}  {:<width$}  {:>18}  {:>18}  {:>8}  {:>8}\n",
            "#", "landmark", "generated", "hand-made", "distance", "share"
        ));
        for difference in &self.above {
            let pair =
                |value: [f64; 2]| format!("{}, {}", plain_fixed(value[0]), plain_fixed(value[1]));
            let only = match difference.marks {
                Marks::Position => "",
                Marks::Height => "  (height only)",
                Marks::Sideways => "  (sideways only)",
            };
            let covered = if difference.covered_by_the_head {
                "  (the head covers the hand-made position)"
            } else {
                ""
            };
            out.push_str(&format!(
                "  {:>3}  {:<width$}  {:>18}  {:>18}  {:>8}  {:>7}%{only}{covered}\n",
                difference.number.unwrap_or(0),
                difference.landmark,
                pair(difference.generated),
                pair(difference.hand_made),
                plain_fixed(difference.distance),
                plain_fixed(difference.percent_of_head_height),
            ));
        }
    }
}

/// A number with three decimals, for a column a person reads down.
fn plain_fixed(value: f64) -> String {
    let rounded = round3(value);
    format!("{rounded:.3}")
}

impl Differences {
    fn above(&self) -> usize {
        self.front.above.len() + self.three_quarter.above.len()
    }

    /// The differences file: TOML that a person reads and the review sheet is composed from.
    pub fn file(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "\
# The differences between a generated armature and a hand-made one, written by
# `maquette differences-from-hand-made-views`. For every landmark both give in a view: where
# the generated view puts it, where the hand-made view puts it, and how far apart those are
# as a share of head height. Every position is in the record's units, the generated ones
# laid over as [rules] says.
#
# This file lists differences and judges none. Whether a difference matters is Jon's look.
# The numbers of the differences above 1% are the numbers the review sheet puts on the drawing.

",
        );
        out.push_str(&format!("record = {}\n", quoted(&self.record)));
        out.push_str(&format!("what = {}\n", quoted(&self.what)));
        out.push_str(&format!(
            "threshold_percent = {}\n",
            toml_number(THRESHOLD_PERCENT, 3)
        ));
        out.push_str(&format!(
            "compared = {}\n",
            self.front.compared() + self.three_quarter.compared()
        ));
        out.push_str(&format!("above_one_percent = {}\n", self.above()));

        out.push_str("\n[yardstick]\n");
        out.push_str(&format!(
            "head_height = {}  # the hand-made front view, hair top to chin\n",
            toml_number(self.head_height, 3)
        ));
        out.push_str(&format!(
            "one_percent = {}\n",
            toml_number(self.head_height / 100.0, 3)
        ));

        out.push_str("\n[rules]\n");
        for (name, rule) in RULES {
            out.push_str(&format!("{name} = {}\n", quoted(rule)));
        }

        self.front.file(&mut out);
        self.three_quarter.file(&mut out);
        out
    }

    /// The numbered list, for a person at a terminal.
    pub fn summary(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "Differences between the generated armature and the hand-made views of {}\n",
            self.record
        ));
        out.push_str(&format!(
            "Head height {} (the hand-made front view, hair top to chin), so 1% is {} units.\n",
            plain(self.head_height, 3),
            plain(self.head_height / 100.0, 3)
        ));
        out.push_str(
            "Positions are x, y in the record's units, the generated ones laid over the hand-made view.\n",
        );
        out.push_str(&format!(
            "{} differences above 1%, numbered. Whether one matters is Jon's look.\n",
            self.above()
        ));
        self.front.summary(&mut out);
        self.three_quarter.summary(&mut out);
        out
    }
}
