//! The one place that knows how proportions and landmarks relate.
//!
//! [`head`] turns a set of proportions into landmarks in space. [`proportions_from_views`]
//! takes landmark positions read from a front view and a three-quarter view back into a set
//! of proportions. Both directions read the same table, [`LANDMARKS`], so they cannot drift
//! apart; the rules that the table cannot state are written once each, below it.
//!
//! Space is measured in units (eye line to chin) along three directions:
//!
//! - *offset*: sideways from the centre line, toward the character's left;
//! - *drop*: downward from the eye line;
//! - *depth*: forward from the eyes, toward where the face points.
//!
//! The head is symmetric: a left and a right landmark share their drop and depth, and their
//! offsets are opposite.

use std::collections::BTreeMap;

use crate::proportions::Set;
use crate::views::View;
use crate::{Error, Result};

use Origin::{Plus, Rise, Rule, Unit, Zero};

/// A named point in space, in units.
#[derive(Debug, Clone, PartialEq)]
pub struct Landmark {
    pub name: String,
    pub offset: f64,
    pub drop: f64,
    pub depth: f64,
}

/// One amount of a named correction: the landmarks the side view draws further back than
/// their depth, and by how much. The landmarks keep their place in space. A head carries it
/// in units, and a view that shows the head carries it in the SVG's own units.
#[derive(Debug, Clone, PartialEq)]
pub struct SetBack {
    /// The correction's name in the list of proportions.
    pub correction: &'static str,
    /// The amount's name there.
    pub amount: &'static str,
    /// How far behind their depth the side view draws the landmarks: in units on a [`Head`],
    /// in SVG units on a view.
    pub by: f64,
    pub landmarks: Vec<String>,
}

/// The correction of the list that sets the lower face back in the side view.
const LOWER_FACE_SET_BACK: &str = "lower_face_set_back";

/// Which landmarks each amount of that correction sets back: the chin with the jaw point
/// beside it, and the mouth line's centre with its two ends. `proportions/list.toml` states
/// this for a person.
const LOWER_FACE_SET_BACK_MOVES: &[(&str, &[&str])] = &[
    (
        "chin",
        &["chin", "face-outline-right-jaw", "face-outline-left-jaw"],
    ),
    (
        "mouth",
        &[
            "mouth-line-centre",
            "mouth-line-right-end",
            "mouth-line-left-end",
        ],
    ),
];

/// A head in space: its landmarks, and the sizes of the forms that are not points.
#[derive(Debug, Clone)]
pub struct Head {
    pub landmarks: Vec<Landmark>,
    /// What the side view draws further back than its depth: the amounts of the named
    /// corrections the set of proportions carries. Empty when it carries none.
    pub side_set_backs: Vec<SetBack>,
    /// Hair top to chin.
    pub head_height: f64,
    pub skull_radius: f64,
    pub iris_width: f64,
    pub iris_height: f64,
    pub hair_half_width: f64,
    pub hair_half_depth: f64,
    pub neck_half_width: f64,
    pub neck_depth: f64,
    pub neck_base_drop: f64,
}

impl Head {
    /// A landmark by name.
    ///
    /// # Panics
    ///
    /// If the head has no such landmark, which is a fault in the program.
    pub fn landmark(&self, name: &str) -> &Landmark {
        self.landmarks
            .iter()
            .find(|landmark| landmark.name == name)
            .unwrap_or_else(|| panic!("the head model has no landmark `{name}`"))
    }
}

/// Where a coordinate of a landmark comes from.
#[derive(Debug, Clone, Copy)]
enum Origin {
    /// On the eye line, on the centre line, or at depth zero.
    Zero,
    /// The chin's drop: the unit itself.
    Unit,
    /// A proportion, as it is.
    Plus(&'static str),
    /// A proportion that is stated as a rise: the drop is its negative.
    Rise(&'static str),
    /// A rule of the head model, stated in `rules`.
    Rule,
}

/// Which view gives a pair's offset when both show it.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Width {
    /// The three-quarter view: the pair turns with the head.
    Turned,
    /// The front view, by the rule for the neck and the shoulder line.
    Front,
}

struct Spec {
    /// `stem` and `point` name a pair as `stem-left-point` and `stem-right-point`.
    /// A landmark of the centre line has no `point` and is named `stem`.
    stem: &'static str,
    point: &'static str,
    pair: bool,
    offset: Origin,
    drop: Origin,
    depth: Origin,
    width: Width,
}

const fn pair(
    stem: &'static str,
    point: &'static str,
    offset: Origin,
    drop: Origin,
    depth: Origin,
) -> Spec {
    Spec {
        stem,
        point,
        pair: true,
        offset,
        drop,
        depth,
        width: Width::Turned,
    }
}

const fn centre(name: &'static str, drop: Origin, depth: Origin) -> Spec {
    Spec {
        stem: name,
        point: "",
        pair: false,
        offset: Origin::Zero,
        drop,
        depth,
        width: Width::Turned,
    }
}

/// Every landmark of the head, and the proportion behind each of its coordinates:
/// its offset, its drop and its depth, in that order.
#[rustfmt::skip]
const LANDMARKS: &[Spec] = &[
    centre("chin", Unit, Plus("chin_depth")),
    centre("hair-top", Rise("hair_top_rise"), Rule),
    centre("skull-centre", Rise("skull_centre_rise"), Plus("skull_centre_depth")),
    centre("skull-front", Rule, Rule),
    centre("centre-line-at-eye-level", Zero, Rule),
    centre("nose-mark-bottom", Plus("nose_mark_bottom_drop"), Plus("nose_mark_bottom_depth")),
    centre("mouth-line-centre", Plus("mouth_centre_drop"), Plus("mouth_centre_depth")),
    pair("face-outline", "top", Plus("face_top_half_width"), Rise("face_top_rise"), Plus("face_top_depth")),
    pair("face-outline", "at-eye-level", Plus("face_half_width_at_eye_level"), Zero, Plus("face_depth_at_eye_level")),
    pair("face-outline", "at-nose-level", Plus("face_half_width_at_nose_level"), Rule, Plus("face_depth_at_nose_level")),
    pair("face-outline", "at-mouth-level", Plus("face_half_width_at_mouth_level"), Rule, Plus("face_depth_at_mouth_level")),
    pair("face-outline", "jaw", Plus("jaw_half_width"), Plus("jaw_drop"), Plus("jaw_depth")),
    pair("jaw-line", "top", Rule, Rule, Rule),
    pair("jaw-line", "at-eye-level", Rule, Zero, Rule),
    pair("jaw-line", "at-nose-level", Rule, Rule, Plus("jaw_line_depth_at_nose_level")),
    pair("jaw-line", "at-mouth-level", Rule, Rule, Plus("jaw_line_depth_at_mouth_level")),
    pair("eye", "outer-corner", Plus("eye_outer_corner_offset"), Plus("eye_outer_corner_drop"), Rule),
    pair("eye", "inner-corner", Plus("eye_inner_corner_offset"), Plus("eye_inner_corner_drop"), Rule),
    pair("eye", "opening-top", Plus("eye_opening_top_offset"), Rule, Rule),
    pair("eye", "opening-bottom", Plus("eye_opening_bottom_offset"), Rule, Rule),
    pair("eye", "opening-centre", Rule, Zero, Zero),
    pair("iris", "centre", Plus("iris_centre_offset"), Plus("iris_centre_drop"), Plus("iris_depth")),
    pair("lid-line", "inner-end", Plus("lid_line_inner_end_offset"), Plus("lid_line_inner_end_drop"), Rule),
    pair("lid-line", "crest", Plus("lid_line_crest_offset"), Rise("lid_line_crest_rise"), Rule),
    pair("lid-line", "outer-end", Plus("lid_line_outer_end_offset"), Rise("lid_line_outer_end_rise"), Rule),
    pair("brow-line", "inner-end", Plus("brow_inner_end_offset"), Rise("brow_inner_end_rise"), Rule),
    pair("brow-line", "outer-end", Plus("brow_outer_end_offset"), Rise("brow_outer_end_rise"), Rule),
    pair("nose-mark", "corner", Plus("nose_mark_corner_offset"), Plus("nose_mark_corner_drop"), Plus("nose_mark_corner_depth")),
    pair("mouth-line", "end", Plus("mouth_end_offset"), Plus("mouth_end_drop"), Plus("mouth_end_depth")),
    pair("ear", "base-outer", Plus("ear_base_outer_offset"), Rise("ear_base_outer_rise"), Plus("ear_base_outer_depth")),
    pair("ear", "base-inner", Plus("ear_base_inner_offset"), Rise("ear_base_inner_rise"), Plus("ear_base_inner_depth")),
    pair("ear", "top", Plus("ear_top_offset"), Rise("ear_top_rise"), Rule),
    pair("ear", "outer-extent", Plus("ear_outer_extent_offset"), Rise("ear_outer_extent_rise"), Rule),
    pair("ear", "inner-extent", Plus("ear_inner_extent_offset"), Rise("ear_inner_extent_rise"), Rule),
    // The shoulder line's width is read from the front view (rule 5 of the list).
    Spec { width: Width::Front, ..pair("shoulder", "point", Plus("shoulder_half_width"), Plus("shoulder_drop"), Plus("shoulder_depth")) },
];

/// The levels at which the face outline has a cheek point and a jaw-line point.
pub const OUTLINE_LEVELS: [&str; 4] = ["top", "at-eye-level", "at-nose-level", "at-mouth-level"];

impl Spec {
    fn key(&self) -> (&'static str, &'static str) {
        (self.stem, self.point)
    }

    fn left(&self) -> String {
        format!("{}-left-{}", self.stem, self.point)
    }

    fn right(&self) -> String {
        format!("{}-right-{}", self.stem, self.point)
    }

    fn label(&self) -> String {
        if self.pair {
            format!(
                "{}-left-{} and {}-right-{}",
                self.stem, self.point, self.stem, self.point
            )
        } else {
            self.stem.to_string()
        }
    }
}

type Key = (&'static str, &'static str);

/// Where the straight line through two points of known offset and depth is, at an offset.
fn depth_on_line(offset: f64, a: (f64, f64), b: (f64, f64)) -> f64 {
    a.1 + (b.1 - a.1) * (offset - a.0) / (b.0 - a.0)
}

/// Turns a set of proportions into a head in space.
pub fn head(set: &Set) -> Result<Head> {
    let direct = |from: Origin| -> Option<f64> {
        match from {
            Zero => Some(0.0),
            Unit => Some(1.0),
            Plus(name) => Some(set.get(name)),
            Rise(name) => Some(-set.get(name)),
            Rule => None,
        }
    };
    let mut at: BTreeMap<Key, [f64; 3]> = BTreeMap::new();
    for spec in LANDMARKS {
        at.insert(
            spec.key(),
            [
                direct(spec.offset).unwrap_or(f64::NAN),
                direct(spec.drop).unwrap_or(f64::NAN),
                direct(spec.depth).unwrap_or(f64::NAN),
            ],
        );
    }
    rules(set, &mut at)?;

    let mut landmarks = Vec::new();
    for spec in LANDMARKS {
        let [offset, drop, depth] = at[&spec.key()];
        // A coordinate the table marks `Rule` and no rule fills is a fault in the program.
        assert!(
            offset.is_finite() && drop.is_finite() && depth.is_finite(),
            "the head model places {} nowhere",
            spec.label()
        );
        if spec.pair {
            landmarks.push(Landmark {
                name: spec.right(),
                offset: -offset,
                drop,
                depth,
            });
            landmarks.push(Landmark {
                name: spec.left(),
                offset,
                drop,
                depth,
            });
        } else {
            landmarks.push(Landmark {
                name: spec.stem.to_string(),
                offset,
                drop,
                depth,
            });
        }
    }

    let mut side_set_backs = Vec::new();
    if let Some(amounts) = set.correction(LOWER_FACE_SET_BACK) {
        for (amount, moved) in LOWER_FACE_SET_BACK_MOVES {
            // An amount the list does not give this correction is a fault in the program.
            let by = *amounts.get(*amount).unwrap_or_else(|| {
                panic!("the list of proportions gives `{LOWER_FACE_SET_BACK}` no amount `{amount}`, which the head model uses")
            });
            side_set_backs.push(SetBack {
                correction: LOWER_FACE_SET_BACK,
                amount,
                by,
                landmarks: moved.iter().map(|name| name.to_string()).collect(),
            });
        }
    }

    Ok(Head {
        landmarks,
        side_set_backs,
        head_height: set.get("hair_top_rise") + 1.0,
        skull_radius: set.get("skull_radius"),
        iris_width: set.get("iris_width"),
        iris_height: set.get("iris_height"),
        hair_half_width: set.get("hair_half_width"),
        hair_half_depth: set.get("hair_half_depth"),
        neck_half_width: set.get("neck_half_width"),
        neck_depth: set.get("neck_depth"),
        neck_base_drop: set.get("neck_base_drop"),
    })
}

/// Checks a share of the chin's full amount: a number from 0 to 1. The one place that says
/// what a share is, for the command line that takes one and for the set worked out at it.
pub fn set_back_share(share: f64) -> Result<f64> {
    if (0.0..=1.0).contains(&share) {
        Ok(share)
    } else {
        Err(Error::new(format!(
            "the share of the lower face's set-back is {share}, and it is a share from 0 to 1"
        )))
    }
}

/// The set carrying the named correction `lower_face_set_back`, with its amounts worked out
/// from the set's own depths as `proportions/list.toml` states their rules.
///
/// `share` is how much of the chin's full amount to take, from 0 to 1. The full amount draws
/// the chin straight below the iris's centre. The mouth's amount then brings the mouth line's
/// centre onto the straight line from the nose mark's bottom to the chin as drawn. Neither
/// amount is below zero: a point that is already there or behind it is left alone.
pub fn with_lower_face_set_back(set: Set, share: f64) -> Result<Set> {
    let share = set_back_share(share)?;
    let chin = (share * (set.get("chin_depth") - set.get("iris_depth"))).max(0.0);
    let nose_drop = set.get("nose_mark_bottom_drop");
    let mouth_drop = set.get("mouth_centre_drop");
    if !(nose_drop < mouth_drop && mouth_drop < 1.0) {
        return Err(Error::new(format!(
            "the mouth line's centre is {mouth_drop} below the eye line, the nose mark's bottom {nose_drop} and the chin 1, so the mouth is not between them and the line from the nose mark to the chin does not pass its height"
        )));
    }
    let nose_depth = set.get("nose_mark_bottom_depth");
    let chin_drawn = set.get("chin_depth") - chin;
    let line_at_mouth =
        nose_depth + (mouth_drop - nose_drop) / (1.0 - nose_drop) * (chin_drawn - nose_depth);
    let mouth = (set.get("mouth_centre_depth") - line_at_mouth).max(0.0);
    set.with_correction(
        LOWER_FACE_SET_BACK,
        BTreeMap::from([("chin".to_string(), chin), ("mouth".to_string(), mouth)]),
    )
}

/// The rules of the head model: every coordinate the table marks `Rule`.
/// `proportions/list.toml` states each of them for a person.
fn rules(set: &Set, at: &mut BTreeMap<Key, [f64; 3]>) -> Result<()> {
    // The eye line is midway between the opening's top and bottom.
    let half_opening = set.get("eye_opening_height") / 2.0;
    at.get_mut(&("eye", "opening-top")).expect("in the table")[1] = -half_opening;
    at.get_mut(&("eye", "opening-bottom"))
        .expect("in the table")[1] = half_opening;

    // Depth zero is midway between the eye's corners; the inner corner stands in front of it.
    let inner = (
        set.get("eye_inner_corner_offset"),
        set.get("eye_wrap_depth") / 2.0,
    );
    let outer = (
        set.get("eye_outer_corner_offset"),
        -set.get("eye_wrap_depth") / 2.0,
    );
    if (outer.0 - inner.0).abs() < 1e-9 {
        return Err(Error::new(
            "the eye's inner and outer corners have the same offset, so the eye has no width",
        ));
    }
    at.get_mut(&("eye", "inner-corner")).expect("in the table")[2] = inner.1;
    at.get_mut(&("eye", "outer-corner")).expect("in the table")[2] = outer.1;
    at.get_mut(&("eye", "opening-centre"))
        .expect("in the table")[0] = (inner.0 + outer.0) / 2.0;

    // The opening's top and bottom, the lid line and the brow line take the depth of the
    // straight line through the eye's two corners.
    for key in [
        ("eye", "opening-top"),
        ("eye", "opening-bottom"),
        ("lid-line", "inner-end"),
        ("lid-line", "crest"),
        ("lid-line", "outer-end"),
        ("brow-line", "inner-end"),
        ("brow-line", "outer-end"),
    ] {
        let point = at.get_mut(&key).expect("in the table");
        point[2] = depth_on_line(point[0], inner, outer);
    }

    // The ear is a flat upright lobe on its two base points.
    let base_outer = at[&("ear", "base-outer")];
    let base_inner = at[&("ear", "base-inner")];
    if (base_outer[0] - base_inner[0]).abs() < 1e-9 {
        return Err(Error::new(
            "the ear's two base points have the same offset, so the ear has no base",
        ));
    }
    for key in [
        ("ear", "top"),
        ("ear", "outer-extent"),
        ("ear", "inner-extent"),
    ] {
        let point = at.get_mut(&key).expect("in the table");
        point[2] = depth_on_line(
            point[0],
            (base_inner[0], base_inner[2]),
            (base_outer[0], base_outer[2]),
        );
    }

    // The nose level is midway between the nose mark's corners and its bottom; the mouth
    // level is the mouth line's centre.
    let nose_level = (set.get("nose_mark_corner_drop") + set.get("nose_mark_bottom_drop")) / 2.0;
    let mouth_level = set.get("mouth_centre_drop");
    at.get_mut(&("face-outline", "at-nose-level"))
        .expect("in the table")[1] = nose_level;
    at.get_mut(&("face-outline", "at-mouth-level"))
        .expect("in the table")[1] = mouth_level;

    // A jaw-line point shares its cheek point's offset and drop. Above the nose level it
    // keeps the depth it has there.
    for level in OUTLINE_LEVELS {
        let cheek = at[&("face-outline", level)];
        let carried = set.get("jaw_line_depth_at_nose_level");
        let point = at.get_mut(&("jaw-line", level)).expect("in the table");
        point[0] = cheek[0];
        point[1] = cheek[1];
        if point[2].is_nan() {
            point[2] = carried;
        }
    }

    // The hair's top is over the middle of the hair.
    at.get_mut(&("hair-top", "")).expect("in the table")[2] = set.get("hair_centre_depth");

    // The centre line follows the skull from its top to the eye line.
    let radius = set.get("skull_radius");
    let rise = set.get("skull_centre_rise");
    let skull_depth = set.get("skull_centre_depth");
    if rise >= radius {
        return Err(Error::new(format!(
            "the skull's centre is {rise} above the eye line and its radius is {radius}, so the skull does not reach the eye line and the centre line cannot follow it there"
        )));
    }
    *at.get_mut(&("skull-front", "")).expect("in the table") = [0.0, -rise, skull_depth + radius];
    at.get_mut(&("centre-line-at-eye-level", ""))
        .expect("in the table")[2] = skull_depth + (radius * radius - rise * rise).sqrt();
    Ok(())
}

/// Landmark positions read from one view: SVG units, x toward the viewer's right, y downward.
#[derive(Debug, Clone, Default)]
pub struct Reading {
    /// How far the head is turned toward the viewer's right. Zero for a front view.
    pub turn_degrees: f64,
    pub points: BTreeMap<String, [f64; 2]>,
}

/// The eye line and the unit of one view, in that view's own SVG units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    pub eye_line_y: f64,
    pub unit: f64,
}

impl Reading {
    fn get(&self, name: &str) -> Option<[f64; 2]> {
        self.points.get(name).copied()
    }

    fn need(&self, name: &str, view: View) -> Result<[f64; 2]> {
        self.get(name).ok_or_else(|| {
            Error::new(format!(
                "the {} view has no position for `{name}`",
                view.name()
            ))
        })
    }

    /// The eye line and the unit as this view shows them: the eye line midway between the
    /// top and the bottom of the eye opening, averaged over the eyes shown, and the unit
    /// from there down to the chin.
    pub fn frame(&self, view: View) -> Result<Frame> {
        let mut levels = Vec::new();
        for side in ["left", "right"] {
            let top = self.get(&format!("eye-{side}-opening-top"));
            let bottom = self.get(&format!("eye-{side}-opening-bottom"));
            if let (Some(top), Some(bottom)) = (top, bottom) {
                levels.push((top[1] + bottom[1]) / 2.0);
            }
        }
        if levels.is_empty() {
            return Err(Error::new(format!(
                "the {} view shows no eye opening with a top and a bottom, so it has no eye line",
                view.name()
            )));
        }
        let eye_line_y = levels.iter().sum::<f64>() / levels.len() as f64;
        let unit = self.need("chin", view)?[1] - eye_line_y;
        if unit <= 0.0 {
            return Err(Error::new(format!(
                "in the {} view the chin is not below the eye line",
                view.name()
            )));
        }
        Ok(Frame { eye_line_y, unit })
    }
}

/// Reads two views of one head back into proportions.
struct Views<'a> {
    front: &'a Reading,
    turned: &'a Reading,
    front_frame: Option<Frame>,
    turned_frame: Frame,
    /// Where depth zero shows sideways in the turned view.
    zero_x: f64,
    cos: f64,
    sin: f64,
}

const FRONT: View = View::Front;
const TURNED: View = View::ThreeQuarter;

impl Views<'_> {
    fn front_frame(&self) -> Result<Frame> {
        self.front_frame.ok_or_else(|| {
            Error::new(
                "the front view shows no eye opening and chin, so nothing can be taken from it",
            )
        })
    }

    fn members(&self, spec: &Spec) -> Vec<String> {
        if spec.pair {
            vec![spec.left(), spec.right()]
        } else {
            vec![spec.stem.to_string()]
        }
    }

    /// Rule 1: a height comes from the three-quarter view, as the mean of the points it
    /// shows. Rule 4: the front view stands in when it shows none.
    fn drop(&self, spec: &Spec) -> Result<f64> {
        let names = self.members(spec);
        let turned: Vec<f64> = names
            .iter()
            .filter_map(|name| self.turned.get(name))
            .map(|point| (point[1] - self.turned_frame.eye_line_y) / self.turned_frame.unit)
            .collect();
        if !turned.is_empty() {
            return Ok(turned.iter().sum::<f64>() / turned.len() as f64);
        }
        let front: Vec<[f64; 2]> = names
            .iter()
            .filter_map(|name| self.front.get(name))
            .collect();
        if front.is_empty() {
            return Err(Error::new(format!(
                "neither view has a position for {}, so its height is unknown",
                spec.label()
            )));
        }
        let frame = self.front_frame()?;
        let drops = front
            .iter()
            .map(|point| (point[1] - frame.eye_line_y) / frame.unit);
        Ok(drops.sum::<f64>() / front.len() as f64)
    }

    fn turned_pair(&self, spec: &Spec) -> (Option<[f64; 2]>, Option<[f64; 2]>) {
        (
            self.turned.get(&spec.left()),
            self.turned.get(&spec.right()),
        )
    }

    fn front_offset(&self, spec: &Spec) -> Result<Option<f64>> {
        match (self.front.get(&spec.left()), self.front.get(&spec.right())) {
            (Some(left), Some(right)) => {
                Ok(Some((left[0] - right[0]) / 2.0 / self.front_frame()?.unit))
            }
            _ => Ok(None),
        }
    }

    fn turned_offset(&self, spec: &Spec) -> Option<f64> {
        match self.turned_pair(spec) {
            (Some(left), Some(right)) => {
                Some((left[0] - right[0]) / 2.0 / self.cos / self.turned_frame.unit)
            }
            _ => None,
        }
    }

    /// Rule 2: an offset comes from the three-quarter view when it shows both points.
    /// Rule 4: otherwise from the front view. Rule 5: the shoulder line's from the front view first.
    fn offset(&self, spec: &Spec) -> Result<f64> {
        let (first, second) = match spec.width {
            Width::Turned => (self.turned_offset(spec), self.front_offset(spec)?),
            Width::Front => (self.front_offset(spec)?, self.turned_offset(spec)),
        };
        first.or(second).ok_or_else(|| {
            Error::new(format!(
                "neither view has positions for both {}, so their offset is unknown",
                spec.label()
            ))
        })
    }

    /// The depth a sideways position in the turned view gives a point of known offset.
    fn depth_at(&self, x: f64, offset: f64) -> f64 {
        ((x - self.zero_x) / self.turned_frame.unit - offset * self.cos) / self.sin
    }

    /// Rule 3: a depth comes from the three-quarter view, from the pair's midpoint or from
    /// the one point it shows and the offset.
    fn depth(&self, spec: &Spec) -> Result<f64> {
        if !spec.pair {
            let point = self.turned.need(spec.stem, TURNED)?;
            return Ok(self.depth_at(point[0], 0.0));
        }
        match self.turned_pair(spec) {
            (Some(left), Some(right)) => Ok(self.depth_at((left[0] + right[0]) / 2.0, 0.0)),
            (Some(left), None) => Ok(self.depth_at(left[0], self.offset(spec)?)),
            (None, Some(right)) => Ok(self.depth_at(right[0], -self.offset(spec)?)),
            (None, None) => Err(Error::new(format!(
                "the three-quarter view has no position for {}, so their depth is unknown",
                spec.label()
            ))),
        }
    }

    /// A height span between two view points of the same eye or form, averaged over the
    /// sides a view shows, in units. The three-quarter view first.
    fn span(&self, stem: &str, low: &str, high: &str, sideways: bool) -> Result<f64> {
        let measure = |reading: &Reading, frame: Frame, shrink: f64| -> Option<f64> {
            let mut spans = Vec::new();
            for side in ["left", "right"] {
                let low = reading.get(&format!("{stem}-{side}-{low}"));
                let high = reading.get(&format!("{stem}-{side}-{high}"));
                if let (Some(low), Some(high)) = (low, high) {
                    let span = if sideways {
                        (high[0] - low[0]).abs() / shrink
                    } else {
                        high[1] - low[1]
                    };
                    spans.push(span / frame.unit);
                }
            }
            if spans.is_empty() {
                None
            } else {
                Some(spans.iter().sum::<f64>() / spans.len() as f64)
            }
        };
        if let Some(span) = measure(self.turned, self.turned_frame, self.cos) {
            return Ok(span);
        }
        if let Some(span) = measure(self.front, self.front_frame()?, 1.0) {
            return Ok(span);
        }
        Err(Error::new(format!(
            "neither view has both `{stem}-…-{low}` and `{stem}-…-{high}` of one side"
        )))
    }
}

fn spec(stem: &str, point: &str) -> &'static Spec {
    LANDMARKS
        .iter()
        .find(|spec| spec.stem == stem && spec.point == point)
        .unwrap_or_else(|| panic!("the head model has no landmark {stem} {point}"))
}

/// The cosine of the turn a three-quarter view shows, from its left and right pairs: for
/// each, how far apart the front view shows it and how far apart the three-quarter view
/// does, each in its own view's unit.
///
/// A level head turned about an upright axis, drawn flat, shows two landmarks that mirror
/// each other across the centre line cos(turn) times as far apart as the front view does,
/// whatever their depth. The one turn for the head is the one that leaves the least squared
/// misfit in spacing over the pairs: the sum of front times three-quarter over the sum of
/// front squared.
///
/// This is the arithmetic only. Which pairs count is each caller's to state, on its own
/// evidence. The result is not a number when no pair has a spacing in the front view, and
/// above 1 when the three-quarter view shows the pairs wider apart: neither is a turn.
pub fn turn_cosine(pairs: &[(f64, f64)]) -> f64 {
    let front_by_turned: f64 = pairs.iter().map(|(front, turned)| front * turned).sum();
    let front_squared: f64 = pairs.iter().map(|(front, _)| front * front).sum();
    front_by_turned / front_squared
}

/// The turn, in degrees, that narrows a spacing to `cosine` of what the front view shows.
/// A ratio outside -1 to 1 is no turn's, and is taken at the nearest end.
pub fn turn_degrees(cosine: f64) -> f64 {
    cosine.clamp(-1.0, 1.0).acos().to_degrees()
}

/// Where the skull of a head taken back from views came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Skull {
    /// A view shows a construction circle, as an underdrawing or a generated armature does:
    /// the skull is that circle.
    Drawn,
    /// Nothing is drawn behind the skull, as in every published drawing: it is built by the
    /// rule of [`skull_by_rule`].
    ByRule,
    /// Neither: no circle is drawn, and the views do not give what the rule builds from.
    NotBuilt,
}

impl Skull {
    pub fn words(self) -> &'static str {
        match self {
            Skull::Drawn => {
                "the construction circle a view draws: its centre and half the height between its top and bottom"
            }
            Skull::ByRule => {
                "built by rule, since no view draws it: centred at the height and the depth of the middle of the ear's base, and reaching the temples"
            }
            Skull::NotBuilt => {
                "not built: no view draws it, and the views do not give the ears' base points and the temples the rule builds it from"
            }
        }
    }
}

/// The three conventions of the skull, in units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SkullBuilt {
    pub centre_rise: f64,
    pub centre_depth: f64,
    pub radius: f64,
}

/// The proportions the skull rule builds from, in the order the rule uses them.
pub const SKULL_RULE_USES: [&str; 7] = [
    "ear_base_outer_rise",
    "ear_base_inner_rise",
    "ear_base_outer_depth",
    "ear_base_inner_depth",
    "face_top_half_width",
    "face_top_rise",
    "face_top_depth",
];

/// Builds the skull where nothing is drawn behind it, from features every reference draws:
/// the ears and the temples. `proportions/list.toml` states the rule for a person.
///
/// - The centre stands at the ears: at the height and at the depth of the middle of the
///   ear's base, midway between its two base points.
/// - The sphere reaches the temples: its radius is the distance in space from that centre to
///   the top end of the face outline.
///
/// The hair's top is not used. `value` gives a proportion by name; the error names the first
/// one the rule needs and does not get.
pub fn skull_by_rule(
    value: impl Fn(&str) -> Option<f64>,
) -> std::result::Result<SkullBuilt, String> {
    let need = |name: &'static str| value(name).ok_or(name.to_string());
    let [
        outer_rise,
        inner_rise,
        outer_depth,
        inner_depth,
        temple_offset,
        temple_rise,
        temple_depth,
    ] = [
        need(SKULL_RULE_USES[0])?,
        need(SKULL_RULE_USES[1])?,
        need(SKULL_RULE_USES[2])?,
        need(SKULL_RULE_USES[3])?,
        need(SKULL_RULE_USES[4])?,
        need(SKULL_RULE_USES[5])?,
        need(SKULL_RULE_USES[6])?,
    ];
    let centre_rise = (outer_rise + inner_rise) / 2.0;
    let centre_depth = (outer_depth + inner_depth) / 2.0;
    let radius = (temple_offset.powi(2)
        + (temple_rise - centre_rise).powi(2)
        + (temple_depth - centre_depth).powi(2))
    .sqrt();
    Ok(SkullBuilt {
        centre_rise,
        centre_depth,
        radius,
    })
}

/// What two views give when they are taken back into proportions.
#[derive(Debug, Clone)]
pub struct Taken {
    /// The proportions the views give.
    pub values: BTreeMap<String, f64>,
    /// The proportions they do not give, and why, in the order they were looked for.
    pub not_given: Vec<(String, String)>,
    pub skull: Skull,
}

/// Takes landmark positions read from a front view and a three-quarter view back into a set
/// of proportions.
///
/// The views may disagree, and either may lack points. The rules for that are numbered in
/// `proportions/list.toml` and applied here: the three-quarter view gives heights, depths and
/// the offsets of the pairs it shows whole; the front view gives what the three-quarter view
/// does not show, and the widths of silhouettes, the neck and the shoulder line.
///
/// Every proportion must be given: a view that hides one is an error. [`take_proportions`]
/// gives what there is.
pub fn proportions_from_views(front: &Reading, three_quarter: &Reading) -> Result<Set> {
    let taken = take_proportions(front, three_quarter)?;
    if let Some((_, why)) = taken.not_given.first() {
        return Err(Error::new(why.clone()));
    }
    Set::from_values(taken.values)
}

/// Takes landmark positions read from a front view and a three-quarter view back into the
/// proportions they give, by the rules of [`proportions_from_views`], and lists the ones they
/// do not give with the reason. A reading of a published drawing hides some: an ear under the
/// hair, a brow under a fringe.
///
/// The skull is the drawn construction circle where a view has one. Where neither view has
/// any of it, it is built by [`skull_by_rule`] from the proportions already taken.
///
/// An error is something no proportion can be taken without: a turn outside 0 to 90 degrees,
/// or a three-quarter view with no eye line, chin or eye corners.
pub fn take_proportions(front: &Reading, three_quarter: &Reading) -> Result<Taken> {
    let turn = three_quarter.turn_degrees;
    if !(turn > 0.0 && turn < 90.0) {
        return Err(Error::new(format!(
            "the three-quarter view is turned {turn} degrees; depth can be taken only from a turn between 0 and 90"
        )));
    }
    let turned_frame = three_quarter.frame(TURNED)?;
    let mut corners = Vec::new();
    for name in [
        "eye-left-inner-corner",
        "eye-left-outer-corner",
        "eye-right-inner-corner",
        "eye-right-outer-corner",
    ] {
        corners.push(three_quarter.need(name, TURNED)?[0]);
    }
    let views = Views {
        front,
        turned: three_quarter,
        front_frame: front.frame(FRONT).ok(),
        turned_frame,
        zero_x: corners.iter().sum::<f64>() / 4.0,
        cos: turn.to_radians().cos(),
        sin: turn.to_radians().sin(),
    };

    let mut values: BTreeMap<String, f64> = BTreeMap::new();
    let mut not_given: Vec<(String, String)> = Vec::new();
    let mut put = |name: &str, value: Result<f64>| match value {
        Ok(value) => {
            values.insert(name.to_string(), value);
        }
        Err(Error(why)) => not_given.push((name.to_string(), why)),
    };

    // A view that has any point of a construction circle draws the skull.
    let skull_drawn = ["skull-centre", "skull-top", "skull-bottom"]
        .iter()
        .any(|name| front.get(name).is_some() || three_quarter.get(name).is_some());

    for spec in LANDMARKS {
        if spec.stem == "skull-centre" && !skull_drawn {
            continue;
        }
        match spec.offset {
            Plus(name) => put(name, views.offset(spec)),
            Rise(_) | Unit => unreachable!("an offset is never a rise or the unit"),
            Zero | Rule => {}
        }
        match spec.drop {
            Plus(name) => put(name, views.drop(spec)),
            Rise(name) => put(name, views.drop(spec).map(|drop| -drop)),
            Zero | Unit | Rule => {}
        }
        match spec.depth {
            Plus(name) => put(name, views.depth(spec)),
            Rise(_) | Unit => unreachable!("a depth is never a rise or the unit"),
            Zero | Rule => {}
        }
    }

    // The proportions the table cannot state.
    put(
        "eye_opening_height",
        views
            .drop(spec("eye", "opening-bottom"))
            .and_then(|bottom| Ok(bottom - views.drop(spec("eye", "opening-top"))?)),
    );
    put(
        "eye_wrap_depth",
        views
            .depth(spec("eye", "inner-corner"))
            .and_then(|inner| Ok(inner - views.depth(spec("eye", "outer-corner"))?)),
    );
    put("iris_height", views.span("iris", "top", "bottom", false));
    put(
        "iris_width",
        views.span("iris", "inner-extent", "outer-extent", true),
    );

    // A drawn skull: half the height of its circle, in the three-quarter view first.
    if skull_drawn {
        let skull = |reading: &Reading, frame: Frame| -> Option<f64> {
            let top = reading.get("skull-top")?;
            let bottom = reading.get("skull-bottom")?;
            Some((bottom[1] - top[1]) / 2.0 / frame.unit)
        };
        let radius = match skull(three_quarter, turned_frame) {
            Some(radius) => Ok(radius),
            None => views.front_frame().and_then(|frame| {
                skull(front, frame).ok_or_else(|| {
                    Error::new(
                        "neither view has `skull-top` and `skull-bottom`, so the skull's radius is unknown",
                    )
                })
            }),
        };
        put("skull_radius", radius);
    }

    // The hair: its half-width from the front view, its depth from how wide the
    // three-quarter view still shows it.
    let half_width = views.front_frame().and_then(|frame| {
        let left = front.need("hair-left-extent", FRONT)?;
        let right = front.need("hair-right-extent", FRONT)?;
        Ok((left[0] - right[0]) / 2.0 / frame.unit)
    });
    let turned_hair = || -> Result<([f64; 2], [f64; 2])> {
        Ok((
            three_quarter.need("hair-left-extent", TURNED)?,
            three_quarter.need("hair-right-extent", TURNED)?,
        ))
    };
    let half_depth = half_width.clone().and_then(|half_width| {
        let (left, right) = turned_hair()?;
        let shown = (left[0] - right[0]) / 2.0 / turned_frame.unit;
        let flat = half_width * views.cos;
        if shown <= flat {
            return Err(Error::new(
                "the hair is no wider in the three-quarter view than a flat shape of its front width would be, so it has no depth to give",
            ));
        }
        Ok((shown * shown - flat * flat).sqrt() / views.sin)
    });
    put("hair_half_width", half_width);
    put("hair_half_depth", half_depth);
    put(
        "hair_centre_depth",
        turned_hair().map(|(left, right)| views.depth_at((left[0] + right[0]) / 2.0, 0.0)),
    );

    // The neck: a column, so its width is the front view's and does not narrow.
    let turned_neck = || -> Result<([f64; 2], [f64; 2])> {
        Ok((
            three_quarter.need("neck-left-base", TURNED)?,
            three_quarter.need("neck-right-base", TURNED)?,
        ))
    };
    put(
        "neck_half_width",
        match (front.get("neck-left-base"), front.get("neck-right-base")) {
            (Some(left), Some(right)) => views
                .front_frame()
                .map(|frame| (left[0] - right[0]) / 2.0 / frame.unit),
            _ => turned_neck().map(|(left, right)| (left[0] - right[0]) / 2.0 / turned_frame.unit),
        },
    );
    put(
        "neck_depth",
        turned_neck().map(|(left, right)| views.depth_at((left[0] + right[0]) / 2.0, 0.0)),
    );
    put(
        "neck_base_drop",
        turned_neck().map(|(left, right)| {
            ((left[1] + right[1]) / 2.0 - turned_frame.eye_line_y) / turned_frame.unit
        }),
    );

    // No circle is drawn: the skull is built by rule from what was taken above.
    let skull = if skull_drawn {
        Skull::Drawn
    } else {
        match skull_by_rule(|name| values.get(name).copied()) {
            Ok(built) => {
                values.insert("skull_centre_rise".to_string(), built.centre_rise);
                values.insert("skull_centre_depth".to_string(), built.centre_depth);
                values.insert("skull_radius".to_string(), built.radius);
                Skull::ByRule
            }
            Err(lacking) => {
                for name in ["skull_radius", "skull_centre_rise", "skull_centre_depth"] {
                    not_given.push((
                        name.to_string(),
                        format!(
                            "no view draws the skull, and the rule that builds it from the ears' base points and the temples lacks `{lacking}`"
                        ),
                    ));
                }
                Skull::NotBuilt
            }
        }
    };

    Ok(Taken {
        values,
        not_given,
        skull,
    })
}
