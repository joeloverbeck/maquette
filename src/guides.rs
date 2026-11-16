//! Builds the named guides of a view from its landmarks.
//!
//! The inventory is the one issue #3 lists: the skull, the face outline, the centre line, the
//! three level lines, each eye's opening, lid line and iris, the brow lines, the nose mark, the
//! mouth line, the ears, the neck, the hair volume and the shoulder line. Nothing the painter
//! owns is here: no pupil, no hair shape, no torso, no marking, no form of the nose.
//!
//! Every anchor of a guide is a named point: a landmark, which is a point in space, or a view
//! point, which belongs to one view only (a silhouette's extent, a crossing). Each
//! construction is stated for a person in `proportions/list.toml`.

use std::collections::BTreeMap;

use crate::head_model::{Head, OUTLINE_LEVELS};
use crate::number::round3;
use crate::views::Seen;

/// How a guide is stroked. Every guide is a stroke with no fill.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Paint {
    /// A form of the head.
    Form,
    /// A heavier line: the lid.
    Heavy,
    /// A construction line or shape: nothing of the figure's own.
    Construction,
    /// The hair volume's envelope.
    Envelope,
}

/// One stretch of a path, from one anchor to the next.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Stretch {
    Straight,
    /// A cubic curve with a handle leaving the first anchor and one arriving at the second.
    Curve {
        leave: [f64; 2],
        arrive: [f64; 2],
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    /// Its anchors are its right, bottom, left and top points, as VectorCraft reads a circle.
    Circle { centre: String, r: f64 },
    /// Its anchors are its right, bottom, left and top points.
    Ellipse { centre: String, rx: f64, ry: f64 },
    /// From its first anchor to its second.
    Line,
    /// Straight through its anchors.
    Polyline,
    /// Through its anchors, one stretch between each two; a closed path has one more stretch,
    /// back to its first anchor.
    Path {
        stretches: Vec<Stretch>,
        closed: bool,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Guide {
    pub name: String,
    /// The group the guide sits in, inside the armature's layer.
    pub group: Option<&'static str>,
    pub paint: Paint,
    pub shape: Shape,
    /// The names of the guide's anchors, in order.
    pub anchors: Vec<String>,
}

/// The guides of one view, in drawing order, with the view points they introduce.
#[derive(Debug, Clone)]
pub struct Guides {
    pub guides: Vec<Guide>,
    /// Points that belong to this view only, in the order they were made.
    pub view_points: Vec<(String, [f64; 2])>,
    positions: BTreeMap<String, [f64; 2]>,
}

/// The layer every guide is in.
pub const LAYER: &str = "armature";

impl Guides {
    /// Where a landmark or a view point of this view is.
    ///
    /// # Panics
    ///
    /// If the view has no such point, which is a fault in the program.
    pub fn position(&self, name: &str) -> [f64; 2] {
        match self.positions.get(name) {
            Some(position) => *position,
            None => panic!("the view has no point `{name}`"),
        }
    }

    fn view_point(&mut self, name: String, x: f64, y: f64) -> String {
        let position = [round3(x), round3(y)];
        self.view_points.push((name.clone(), position));
        self.positions.insert(name.clone(), position);
        name
    }

    fn add(
        &mut self,
        name: &str,
        group: Option<&'static str>,
        paint: Paint,
        shape: Shape,
        anchors: Vec<String>,
    ) {
        self.guides.push(Guide {
            name: name.to_string(),
            group,
            paint,
            shape,
            anchors,
        });
    }

    fn positions_of(&self, names: &[String]) -> Vec<[f64; 2]> {
        names.iter().map(|name| self.position(name)).collect()
    }

    /// A smooth path through named anchors.
    fn smooth_path(&self, names: &[String], directions: &[Option<[f64; 2]>]) -> Shape {
        Shape::Path {
            stretches: smooth(&self.positions_of(names), directions),
            closed: false,
        }
    }
}

/// The ratio of a handle to a radius that makes a cubic curve a quarter of an ellipse.
const QUARTER: f64 = 0.552_284_749_8;

fn unit_vector(vector: [f64; 2]) -> [f64; 2] {
    let length = vector[0].hypot(vector[1]);
    if length == 0.0 {
        [0.0, 0.0]
    } else {
        [vector[0] / length, vector[1] / length]
    }
}

/// A smooth curve through points. At each point the curve runs parallel to the line joining
/// the point's two neighbours, or along a stated direction; at an end it starts toward its
/// neighbour. Each handle is a third as long as its stretch, so the curve cannot overshoot.
fn smooth(points: &[[f64; 2]], directions: &[Option<[f64; 2]>]) -> Vec<Stretch> {
    let last = points.len() - 1;
    let direction = |index: usize| -> [f64; 2] {
        let before = points[index.saturating_sub(1)];
        let after = points[(index + 1).min(last)];
        let along = [after[0] - before[0], after[1] - before[1]];
        match directions.get(index).copied().flatten() {
            Some(stated) => {
                // A stated direction is a line; the curve follows it the way it is going.
                if stated[0] * along[0] + stated[1] * along[1] < 0.0 {
                    [-stated[0], -stated[1]]
                } else {
                    stated
                }
            }
            None => unit_vector(along),
        }
    };
    (0..last)
        .map(|index| {
            let from = points[index];
            let to = points[index + 1];
            let third = (to[0] - from[0]).hypot(to[1] - from[1]) / 3.0;
            let leaving = direction(index);
            let arriving = direction(index + 1);
            Stretch::Curve {
                leave: [
                    round3(from[0] + leaving[0] * third),
                    round3(from[1] + leaving[1] * third),
                ],
                arrive: [
                    round3(to[0] - arriving[0] * third),
                    round3(to[1] - arriving[1] * third),
                ],
            }
        })
        .collect()
}

const UPRIGHT: Option<[f64; 2]> = Some([0.0, 1.0]);
const LEVEL: Option<[f64; 2]> = Some([1.0, 0.0]);

fn cubic(from: [f64; 2], leave: [f64; 2], arrive: [f64; 2], to: [f64; 2], t: f64) -> [f64; 2] {
    let s = 1.0 - t;
    let weights = [s * s * s, 3.0 * s * s * t, 3.0 * s * t * t, t * t * t];
    [
        weights[0] * from[0] + weights[1] * leave[0] + weights[2] * arrive[0] + weights[3] * to[0],
        weights[0] * from[1] + weights[1] * leave[1] + weights[2] * arrive[1] + weights[3] * to[1],
    ]
}

/// The lowest point where an upright line at `x` crosses a path of curves.
fn lowest_crossing(points: &[[f64; 2]], stretches: &[Stretch], x: f64) -> Option<f64> {
    const STEPS: usize = 64;
    let mut lowest: Option<f64> = None;
    for (index, stretch) in stretches.iter().enumerate() {
        let (from, to) = (points[index], points[index + 1]);
        let (leave, arrive) = match stretch {
            Stretch::Curve { leave, arrive } => (*leave, *arrive),
            Stretch::Straight => (from, to),
        };
        let at = |t: f64| cubic(from, leave, arrive, to, t);
        for step in 0..STEPS {
            let mut low = step as f64 / STEPS as f64;
            let mut high = (step + 1) as f64 / STEPS as f64;
            let (start, end) = (at(low)[0] - x, at(high)[0] - x);
            if start == 0.0 || start * end < 0.0 {
                for _ in 0..48 {
                    let middle = (low + high) / 2.0;
                    if (at(low)[0] - x) * (at(middle)[0] - x) <= 0.0 {
                        high = middle;
                    } else {
                        low = middle;
                    }
                }
                let y = at((low + high) / 2.0)[1];
                lowest = Some(lowest.map_or(y, |known: f64| known.max(y)));
            }
        }
    }
    lowest
}

/// Builds every guide of a view.
pub fn build(head: &Head, seen: &Seen) -> Guides {
    let mut guides = Guides {
        guides: Vec::new(),
        view_points: Vec::new(),
        positions: seen
            .landmarks
            .iter()
            .map(|landmark| (landmark.name.clone(), [landmark.x, landmark.y]))
            .collect(),
    };
    let names =
        |list: &[&str]| -> Vec<String> { list.iter().map(|name| name.to_string()).collect() };

    // Hair volume: a half-ellipse dome from the hair's top to the height of the skull's
    // centre, then straight sides to the height of the chin.
    {
        let top = guides.position("hair-top");
        let base_y = guides.position("skull-centre")[1];
        let end_y = guides.position("chin")[1];
        let half = seen.round_half_width(head.hair_half_width, head.hair_half_depth);
        let (right, left) = (top[0] - half, top[0] + half);
        let anchors = vec![
            guides.view_point("hair-right-end".to_string(), right, end_y),
            guides.view_point("hair-right-extent".to_string(), right, base_y),
            "hair-top".to_string(),
            guides.view_point("hair-left-extent".to_string(), left, base_y),
            guides.view_point("hair-left-end".to_string(), left, end_y),
        ];
        let dome = base_y - top[1];
        let stretches = vec![
            Stretch::Straight,
            Stretch::Curve {
                leave: [right, round3(base_y - QUARTER * dome)],
                arrive: [round3(top[0] - QUARTER * half), top[1]],
            },
            Stretch::Curve {
                leave: [round3(top[0] + QUARTER * half), top[1]],
                arrive: [left, round3(base_y - QUARTER * dome)],
            },
            Stretch::Straight,
        ];
        guides.add(
            "hair-volume",
            None,
            Paint::Envelope,
            Shape::Path {
                stretches,
                closed: false,
            },
            anchors,
        );
    }

    // Skull: the same circle in every view.
    {
        let centre = guides.position("skull-centre");
        let r = seen.length(head.skull_radius);
        let anchors = vec![
            guides.view_point("skull-left-extent".to_string(), centre[0] + r, centre[1]),
            guides.view_point("skull-bottom".to_string(), centre[0], centre[1] + r),
            guides.view_point("skull-right-extent".to_string(), centre[0] - r, centre[1]),
            guides.view_point("skull-top".to_string(), centre[0], centre[1] - r),
        ];
        guides.add(
            "skull",
            None,
            Paint::Construction,
            Shape::Circle {
                centre: "skull-centre".to_string(),
                r,
            },
            anchors,
        );
    }

    // Face outline: at each level, on each side, through whichever of the cheek point and the
    // jaw-line point stands further out in this view.
    let outline_point = |guides: &Guides, side: &str, level: &str| -> String {
        let cheek = format!("face-outline-{side}-{level}");
        let jaw_line = format!("jaw-line-{side}-{level}");
        let (cheek_x, jaw_line_x) = (guides.position(&cheek)[0], guides.position(&jaw_line)[0]);
        let jaw_line_is_further_out = if side == "left" {
            jaw_line_x > cheek_x
        } else {
            jaw_line_x < cheek_x
        };
        if jaw_line_is_further_out {
            jaw_line
        } else {
            cheek
        }
    };
    let (outline_anchors, outline_stretches) = {
        let mut anchors = Vec::new();
        for level in OUTLINE_LEVELS {
            anchors.push(outline_point(&guides, "left", level));
        }
        anchors.push("face-outline-left-jaw".to_string());
        anchors.push("chin".to_string());
        anchors.push("face-outline-right-jaw".to_string());
        for level in OUTLINE_LEVELS.iter().rev() {
            anchors.push(outline_point(&guides, "right", level));
        }
        let stretches = smooth(&guides.positions_of(&anchors), &[]);
        guides.add(
            "face-outline",
            None,
            Paint::Form,
            Shape::Path {
                stretches: stretches.clone(),
                closed: false,
            },
            anchors.clone(),
        );
        (anchors, stretches)
    };

    // Centre line: along the skull from its top to its front, a quarter of an ellipse, then
    // smoothly through the eye line, the nose mark's bottom, the mouth line's centre and the chin.
    {
        let anchors = names(&[
            "skull-top",
            "skull-front",
            "centre-line-at-eye-level",
            "nose-mark-bottom",
            "mouth-line-centre",
            "chin",
        ]);
        let points = guides.positions_of(&anchors);
        let (top, front) = (points[0], points[1]);
        let mut stretches = vec![Stretch::Curve {
            leave: [round3(top[0] + QUARTER * (front[0] - top[0])), top[1]],
            arrive: [front[0], round3(front[1] - QUARTER * (front[1] - top[1]))],
        }];
        stretches.extend(smooth(&points[1..], &[UPRIGHT]));
        guides.add(
            "centre-line",
            None,
            Paint::Construction,
            Shape::Path {
                stretches,
                closed: false,
            },
            anchors,
        );
    }

    // Level lines: each spans the face outline at its height.
    for (name, level) in [
        ("eye-level-line", "at-eye-level"),
        ("nose-level-line", "at-nose-level"),
        ("mouth-level-line", "at-mouth-level"),
    ] {
        let anchors = vec![
            outline_point(&guides, "right", level),
            outline_point(&guides, "left", level),
        ];
        guides.add(name, None, Paint::Construction, Shape::Line, anchors);
    }

    // Eyes: the opening, the lid line and the iris of each.
    for (side, group) in [("right", "eye-right"), ("left", "eye-left")] {
        let point = |name: &str| format!("eye-{side}-{name}");
        let opening = vec![
            point("outer-corner"),
            point("opening-top"),
            point("inner-corner"),
            point("opening-bottom"),
        ];
        let positions = guides.positions_of(&opening);
        let mut stretches = smooth(&positions[0..3], &[]);
        stretches.extend(smooth(&[positions[2], positions[3], positions[0]], &[]));
        guides.add(
            &format!("eye-opening-{side}"),
            Some(group),
            Paint::Form,
            Shape::Path {
                stretches,
                closed: true,
            },
            opening,
        );

        let lid = vec![
            format!("lid-line-{side}-inner-end"),
            format!("lid-line-{side}-crest"),
            format!("lid-line-{side}-outer-end"),
        ];
        let shape = guides.smooth_path(&lid, &[]);
        guides.add(
            &format!("lid-line-{side}"),
            Some(group),
            Paint::Heavy,
            shape,
            lid,
        );

        // The iris is a flat disc that faces forward: its width narrows with the turn.
        let centre_name = format!("iris-{side}-centre");
        let centre = guides.position(&centre_name);
        let rx = seen.flat_length(head.iris_width / 2.0);
        let ry = seen.length(head.iris_height / 2.0);
        let outward = if side == "left" { rx } else { -rx };
        let outer = guides.view_point(
            format!("iris-{side}-outer-extent"),
            centre[0] + outward,
            centre[1],
        );
        let inner = guides.view_point(
            format!("iris-{side}-inner-extent"),
            centre[0] - outward,
            centre[1],
        );
        let bottom = guides.view_point(format!("iris-{side}-bottom"), centre[0], centre[1] + ry);
        let top = guides.view_point(format!("iris-{side}-top"), centre[0], centre[1] - ry);
        let anchors = if side == "left" {
            vec![outer, bottom, inner, top]
        } else {
            vec![inner, bottom, outer, top]
        };
        guides.add(
            &format!("iris-{side}"),
            Some(group),
            Paint::Form,
            Shape::Ellipse {
                centre: centre_name,
                rx,
                ry,
            },
            anchors,
        );
    }

    for side in ["right", "left"] {
        let anchors = vec![
            format!("brow-line-{side}-inner-end"),
            format!("brow-line-{side}-outer-end"),
        ];
        guides.add(
            &format!("brow-line-{side}"),
            None,
            Paint::Form,
            Shape::Line,
            anchors,
        );
    }

    guides.add(
        "nose-mark",
        None,
        Paint::Form,
        Shape::Polyline,
        names(&[
            "nose-mark-right-corner",
            "nose-mark-bottom",
            "nose-mark-left-corner",
        ]),
    );
    guides.add(
        "mouth-line",
        None,
        Paint::Form,
        Shape::Polyline,
        names(&[
            "mouth-line-right-end",
            "mouth-line-centre",
            "mouth-line-left-end",
        ]),
    );

    // Ears: a lobe on its two base points, upright at its extents and level at its top.
    for side in ["right", "left"] {
        let anchors: Vec<String> = [
            "base-outer",
            "outer-extent",
            "top",
            "inner-extent",
            "base-inner",
        ]
        .iter()
        .map(|point| format!("ear-{side}-{point}"))
        .collect();
        let shape = guides.smooth_path(&anchors, &[None, UPRIGHT, LEVEL, UPRIGHT, None]);
        guides.add(&format!("ear-{side}"), None, Paint::Form, shape, anchors);
    }

    // Neck: an upright round column, as wide in every view. Each line starts at the lowest
    // point where it crosses the face outline. A line that passes outside the face starts
    // where it crosses the lower half of the skull, and one that misses both starts at the
    // chin's height.
    {
        let middle = seen.x_at(head.neck_depth);
        let half = seen.length(head.neck_half_width);
        let base_y = seen.y_at(head.neck_base_drop);
        let chin_y = guides.position("chin")[1];
        let outline = guides.positions_of(&outline_anchors);
        let skull = guides.position("skull-centre");
        let radius = seen.length(head.skull_radius);
        for (side, x) in [("right", middle - half), ("left", middle + half)] {
            let under_skull = || {
                let across = x - skull[0];
                (across.abs() < radius)
                    .then(|| skull[1] + (radius * radius - across * across).sqrt())
            };
            let start_y = lowest_crossing(&outline, &outline_stretches, x)
                .or_else(under_skull)
                .unwrap_or(chin_y);
            let anchors = vec![
                guides.view_point(format!("neck-{side}-top"), x, start_y),
                guides.view_point(format!("neck-{side}-base"), x, base_y),
            ];
            guides.add(
                &format!("neck-{side}"),
                Some("neck"),
                Paint::Form,
                Shape::Line,
                anchors,
            );
        }
    }

    guides.add(
        "shoulder-line",
        None,
        Paint::Form,
        Shape::Line,
        names(&["shoulder-right-point", "shoulder-left-point"]),
    );

    guides
}
