//! Shows a head in space from the front, from the three-quarter turn and from the side.
//!
//! A view is a flat projection of the same points: the head is level and turns about the
//! upright axis through the skull's centre, so a landmark keeps its height in every view, and
//! its sideways position is its offset times the cosine of the turn plus its depth times the
//! sine. The side view is the same turn carried to 90 degrees: it shows a landmark's depth as
//! its sideways position, and it faces the viewer's right. There is no perspective.
//!
//! A departure from this is a named correction with an amount, which the list of proportions
//! defines and a set of proportions carries. There is one: in the side view only, the mouth
//! and the chin are drawn further back than their depths. A landmark it moves keeps its depth
//! and says beside it where the view draws it. A set that carries no correction is drawn flat.

use crate::head_model::Head;
use crate::number::round3;
use crate::{Error, Result};

/// How far the three-quarter view is turned, toward the viewer's right, in degrees.
///
/// Pinned from Saffi's hand-made three-quarter view, the one Jon accepted. The spacing of 18
/// left and right pairs there, against the front view, gives 28.96 degrees, and 27.85 without
/// the three pairs that have a covered point (`data/saffi/views-compared.toml`). The wide pairs
/// one by one support 27.9 to 29.4 degrees. This is the only place the turn is written.
pub const THREE_QUARTER_TURN_DEGREES: f64 = 29.0;

/// How far the side view is turned: the three-quarter view's turn carried to a full profile.
pub const SIDE_TURN_DEGREES: f64 = 90.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Front,
    ThreeQuarter,
    Side,
}

impl View {
    /// The views the generator writes.
    pub const ALL: [View; 3] = [View::Front, View::ThreeQuarter, View::Side];

    /// The view's name, as files and landmark files carry it.
    pub fn name(self) -> &'static str {
        match self {
            View::Front => "front",
            View::ThreeQuarter => "three-quarter",
            View::Side => "side",
        }
    }

    /// The turn toward the viewer's right, so that the character's right is the near side.
    pub fn turn_degrees(self) -> f64 {
        match self {
            View::Front => 0.0,
            View::ThreeQuarter => THREE_QUARTER_TURN_DEGREES,
            View::Side => SIDE_TURN_DEGREES,
        }
    }
}

/// Where and how large a head is drawn on the page. It is no part of a set of proportions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Placement {
    pub width: f64,
    pub height: f64,
    /// The length of one unit, eye line to chin, in SVG units.
    pub unit: f64,
    pub eye_line_y: f64,
    /// Where the upright axis the head turns about stands in each view.
    pub front_axis_x: f64,
    pub three_quarter_axis_x: f64,
    pub side_axis_x: f64,
}

impl Default for Placement {
    fn default() -> Placement {
        Placement {
            width: 1000.0,
            height: 1000.0,
            unit: 200.0,
            eye_line_y: 400.0,
            front_axis_x: 500.0,
            three_quarter_axis_x: 500.0,
            side_axis_x: 500.0,
        }
    }
}

impl Placement {
    /// Rounds the placement to what a file can carry, and refuses one that cannot be drawn.
    pub fn checked(self) -> Result<Placement> {
        let placement = Placement {
            width: round3(self.width),
            height: round3(self.height),
            unit: round3(self.unit),
            eye_line_y: round3(self.eye_line_y),
            front_axis_x: round3(self.front_axis_x),
            three_quarter_axis_x: round3(self.three_quarter_axis_x),
            side_axis_x: round3(self.side_axis_x),
        };
        for (name, value) in [
            ("page width", placement.width),
            ("page height", placement.height),
            ("unit", placement.unit),
        ] {
            if !(value.is_finite() && value > 0.0) {
                return Err(Error::new(format!(
                    "the {name} must be above zero, and it is {value}"
                )));
            }
        }
        for (name, value) in [
            ("eye line", placement.eye_line_y),
            ("front view's axis", placement.front_axis_x),
            ("three-quarter view's axis", placement.three_quarter_axis_x),
            ("side view's axis", placement.side_axis_x),
        ] {
            if !value.is_finite() {
                return Err(Error::new(format!("the {name} is not a finite number")));
            }
        }
        Ok(placement)
    }
}

/// A landmark as one view shows it, in SVG units.
#[derive(Debug, Clone, PartialEq)]
pub struct Placed {
    pub name: String,
    pub x: f64,
    pub y: f64,
    /// How far the point stands in front of the eyes, in SVG units: depth zero is the midpoint
    /// between the two eye openings' centres, as the list of proportions defines it. It is the
    /// same in every view.
    pub depth: f64,
    /// Set when a named correction moves the landmark in this view: the depth the view draws
    /// it at, in SVG units, which is where its `x` stands. Its `depth` is still its place in
    /// space.
    pub drawn_at_depth: Option<f64>,
}

/// One amount of a named correction as a view carries it.
#[derive(Debug, Clone, PartialEq)]
pub struct SetBack {
    /// The correction's name in the list of proportions.
    pub correction: &'static str,
    /// The amount's name there.
    pub amount: &'static str,
    /// How far behind their depth the view draws the landmarks, in SVG units.
    pub by: f64,
    pub landmarks: Vec<String>,
}

/// A head as one view shows it.
#[derive(Debug, Clone)]
pub struct Seen {
    pub view: View,
    pub turn_degrees: f64,
    pub width: f64,
    pub height: f64,
    pub unit: f64,
    pub eye_line_y: f64,
    pub turn_axis_x: f64,
    /// The depth the upright axis the head turns about stands at, in SVG units: the skull's
    /// centre, behind the eyes.
    pub turn_axis_depth: f64,
    /// Hair top to chin, in SVG units.
    pub head_height: f64,
    pub landmarks: Vec<Placed>,
    /// The amounts of the named corrections this view carries. Only the side view carries any.
    pub set_backs: Vec<SetBack>,
    cos: f64,
    sin: f64,
    axis_depth: f64,
}

impl Seen {
    /// A landmark by name.
    ///
    /// # Panics
    ///
    /// If the view has no such landmark, which is a fault in the program.
    pub fn landmark(&self, name: &str) -> &Placed {
        self.landmarks
            .iter()
            .find(|landmark| landmark.name == name)
            .unwrap_or_else(|| panic!("the view has no landmark `{name}`"))
    }

    /// A length that no turn foreshortens, such as a height or the radius of a round form.
    pub fn length(&self, units: f64) -> f64 {
        round3(units * self.unit)
    }

    /// The height on the page of a drop below the eye line.
    pub fn y_at(&self, drop: f64) -> f64 {
        self.eye_line_y + round3(drop * self.unit)
    }

    /// Where a point of the centre plane at a depth shows sideways.
    pub fn x_at(&self, depth: f64) -> f64 {
        self.turn_axis_x + round3((depth - self.axis_depth) * self.sin * self.unit)
    }

    /// How wide a flat span that faces forward shows.
    pub fn flat_length(&self, units: f64) -> f64 {
        round3(units * self.cos * self.unit)
    }

    /// How far to each side a form shows that is an ellipse seen from above, with a
    /// half-width across the head and a half-depth along it.
    pub fn round_half_width(&self, half_width: f64, half_depth: f64) -> f64 {
        round3((half_width * self.cos).hypot(half_depth * self.sin) * self.unit)
    }
}

/// Shows a head from one view.
pub fn show(head: &Head, view: View, placement: &Placement) -> Seen {
    let turn = view.turn_degrees();
    // A front view is exactly flat on and a side view exactly side on: no rounding of a
    // cosine may move a point.
    let (sin, cos) = if turn == 0.0 {
        (0.0, 1.0)
    } else if turn == 90.0 {
        (1.0, 0.0)
    } else {
        turn.to_radians().sin_cos()
    };
    let axis_depth = head.landmark("skull-centre").depth;
    let turn_axis_x = match view {
        View::Front => placement.front_axis_x,
        View::ThreeQuarter => placement.three_quarter_axis_x,
        View::Side => placement.side_axis_x,
    };
    // The named corrections set landmarks back in the side view and in no other.
    let set_backs: &[crate::head_model::SetBack] = if view == View::Side {
        &head.side_set_backs
    } else {
        &[]
    };
    let landmarks = head
        .landmarks
        .iter()
        .map(|landmark| {
            let set_back = set_backs
                .iter()
                .find(|set_back| set_back.landmarks.contains(&landmark.name))
                .map(|set_back| set_back.by);
            // Where the view draws the point: its depth, less a correction's amount.
            let drawn_depth = landmark.depth - set_back.unwrap_or(0.0);
            Placed {
                name: landmark.name.clone(),
                x: turn_axis_x
                    + round3(
                        (landmark.offset * cos + (drawn_depth - axis_depth) * sin) * placement.unit,
                    ),
                y: placement.eye_line_y + round3(landmark.drop * placement.unit),
                depth: round3(landmark.depth * placement.unit),
                drawn_at_depth: set_back.map(|_| round3(drawn_depth * placement.unit)),
            }
        })
        .collect();
    let set_backs = set_backs
        .iter()
        .map(|set_back| SetBack {
            correction: set_back.correction,
            amount: set_back.amount,
            by: round3(set_back.by * placement.unit),
            landmarks: set_back.landmarks.clone(),
        })
        .collect();
    Seen {
        view,
        turn_degrees: turn,
        width: placement.width,
        height: placement.height,
        unit: placement.unit,
        eye_line_y: placement.eye_line_y,
        turn_axis_x,
        turn_axis_depth: round3(axis_depth * placement.unit),
        head_height: round3(head.head_height * placement.unit),
        landmarks,
        set_backs,
        cos,
        sin,
        axis_depth,
    }
}
