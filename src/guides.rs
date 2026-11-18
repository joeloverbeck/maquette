//! Builds the named guides of a view from its landmarks.
//!
//! The inventory is the one issue #3 lists: the skull, the face outline, the centre line, the
//! three level lines, each eye's opening, lid line and iris, the brow lines, the nose mark, the
//! mouth line, the ears, the neck, the hair volume and the shoulder line. Nothing the painter
//! owns is here: no pupil, no hair shape, no torso, no marking, no form of the nose.
//!
//! Every point a guide passes through, each an anchor of its element in the SVG, is a named
//! point: a landmark, which is a point in space, or a view point, which belongs to one view
//! only (a silhouette's extent, a crossing). Each construction is stated for a person in
//! `proportions/list.toml`.
//!
//! The side view shows fewer guides: the head hides the eye, the brow and the ear of the far
//! side; the centre line is the front of the face outline and is not drawn twice; and the
//! neck's two lines are its back and its front. It has constructions of its own for the guides
//! that a plain projection would draw at no width: the iris and the shoulder line.

use std::collections::BTreeMap;

use crate::head_model::{BRIDGE, FOREHEAD, Head, OUTLINE_LEVELS, jaw_angle};
use crate::number::round3;
use crate::views::{Seen, View};

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
    /// The landmarks and view points the guide passes through, in order. Each is one anchor
    /// of its element in the SVG.
    pub passes_through: Vec<String>,
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
        passes_through: Vec<String>,
    ) {
        self.guides.push(Guide {
            name: name.to_string(),
            group,
            paint,
            shape,
            passes_through,
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

/// Where a straight line crosses a path of curves. The line holds one coordinate at `value`:
/// `along` is 0 for an upright line at an x and 1 for a level line at a y. Of the crossings,
/// the one whose other coordinate is greatest: the lowest on an upright line, the furthest
/// toward the viewer's right on a level one.
fn furthest_crossing(
    points: &[[f64; 2]],
    stretches: &[Stretch],
    along: usize,
    value: f64,
) -> Option<f64> {
    const STEPS: usize = 64;
    let mut furthest: Option<f64> = None;
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
            let (start, end) = (at(low)[along] - value, at(high)[along] - value);
            if start == 0.0 || start * end < 0.0 {
                for _ in 0..48 {
                    let middle = (low + high) / 2.0;
                    if (at(low)[along] - value) * (at(middle)[along] - value) <= 0.0 {
                        high = middle;
                    } else {
                        low = middle;
                    }
                }
                let other = at((low + high) / 2.0)[1 - along];
                furthest = Some(furthest.map_or(other, |known: f64| known.max(other)));
            }
        }
    }
    furthest
}

/// The lowest point where an upright line at `x` crosses a path of curves.
fn lowest_crossing(points: &[[f64; 2]], stretches: &[Stretch], x: f64) -> Option<f64> {
    furthest_crossing(points, stretches, 0, x)
}

/// Where a neck line at `x` starts: at the lowest point where it crosses the face outline. A
/// line that passes outside the face starts where it crosses the lower half of the skull, and
/// one that misses both starts at the chin's height.
fn neck_line_start(
    x: f64,
    outline: &[[f64; 2]],
    outline_stretches: &[Stretch],
    skull: [f64; 2],
    radius: f64,
    chin_y: f64,
) -> f64 {
    let under_skull = || {
        let across = x - skull[0];
        (across.abs() < radius).then(|| skull[1] + (radius * radius - across * across).sqrt())
    };
    lowest_crossing(outline, outline_stretches, x)
        .or_else(under_skull)
        .unwrap_or(chin_y)
}

fn names(list: &[&str]) -> Vec<String> {
    list.iter().map(|name| name.to_string()).collect()
}

/// The opening and the lid line of one eye, which every view builds alike from the eye's
/// landmarks as that view shows them.
fn eye_opening_and_lid_line(guides: &mut Guides, side: &str, group: &'static str) {
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
}

/// The brow line of one side: straight between its two ends.
fn brow_line(guides: &mut Guides, side: &str) {
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

/// The ear of one side: a lobe on its two base points, upright at its extents and level at
/// its top.
fn ear(guides: &mut Guides, side: &str) {
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

/// The side every guide of the side view is named for: the near one, the character's right.
const NEAR: &str = "right";

/// Builds every guide of the side view. `proportions/list.toml` states each construction.
fn build_side(head: &Head, seen: &Seen) -> Guides {
    // The nose point of a turned view: the tip of the nose, or the nose mark's bottom where
    // the two are one point.
    let nose_point = head.nose_point();
    let mut guides = Guides {
        guides: Vec::new(),
        view_points: Vec::new(),
        positions: seen
            .landmarks
            .iter()
            .map(|landmark| (landmark.name.clone(), [landmark.x, landmark.y]))
            .collect(),
    };

    // Hair volume: the same dome, as deep as the hair. Behind the head its side runs straight
    // down to the height of the chin; in front it stops at the dome's base, where a side would
    // cross the face.
    {
        let top = guides.position("hair-top");
        let base_y = guides.position("skull-centre")[1];
        let end_y = guides.position("chin")[1];
        let half = seen.round_half_width(head.hair_half_width, head.hair_half_depth);
        let (back, front) = (top[0] - half, top[0] + half);
        let anchors = vec![
            guides.view_point("hair-back-end".to_string(), back, end_y),
            guides.view_point("hair-back-extent".to_string(), back, base_y),
            "hair-top".to_string(),
            guides.view_point("hair-front-extent".to_string(), front, base_y),
        ];
        let dome = base_y - top[1];
        let stretches = vec![
            Stretch::Straight,
            Stretch::Curve {
                leave: [back, round3(base_y - QUARTER * dome)],
                arrive: [round3(top[0] - QUARTER * half), top[1]],
            },
            Stretch::Curve {
                leave: [round3(top[0] + QUARTER * half), top[1]],
                arrive: [front, round3(base_y - QUARTER * dome)],
            },
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

    // Skull: the same circle. Its point toward the viewer's right is the landmark the centre
    // line passes through at the height of the skull's centre.
    {
        let centre = guides.position("skull-centre");
        let r = seen.length(head.skull_radius);
        let anchors = vec![
            "skull-front".to_string(),
            guides.view_point("skull-bottom".to_string(), centre[0], centre[1] + r),
            guides.view_point("skull-back".to_string(), centre[0] - r, centre[1]),
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

    // Face outline: down the centre line's landmarks from the skull's front to the chin, then
    // back and up through the jaw-line points. Where the set gives the points of the profile,
    // the front runs through the forehead and the bridge, and the underside of the jaw runs
    // back from the chin through the top of the neck's front and the angle of the jaw.
    let near_angle = jaw_angle(NEAR);
    let mut outline_anchors = vec!["skull-front".to_string()];
    if head.has(FOREHEAD) {
        outline_anchors.push(FOREHEAD.to_string());
    }
    outline_anchors.extend(names(&[BRIDGE, nose_point, "mouth-line-centre", "chin"]));
    let chin_at = outline_anchors.len() - 1;
    for under in ["neck-front-top", near_angle.as_str()] {
        if head.has(under) {
            outline_anchors.push(under.to_string());
        }
    }
    for level in OUTLINE_LEVELS.iter().rev() {
        outline_anchors.push(format!("jaw-line-{NEAR}-{level}"));
    }
    let outline = guides.positions_of(&outline_anchors);
    // The chin is the outline's lowest point, where it turns from running down the face to
    // running back under the jaw: the curve runs level there, and does not sag below it on its
    // way back to a neck that meets the jaw above the chin's height.
    let mut directions = vec![None; outline.len()];
    directions[chin_at] = LEVEL;
    // The underside of the jaw runs on from the chin through the top of the neck's front, so
    // the curve keeps that line's direction there and turns up toward the ear after it.
    if outline_anchors.get(chin_at + 1).map(String::as_str) == Some("neck-front-top") {
        let (chin, throat) = (outline[chin_at], outline[chin_at + 1]);
        directions[chin_at + 1] = Some(unit_vector([throat[0] - chin[0], throat[1] - chin[1]]));
    }
    let outline_stretches = smooth(&outline, &directions);
    guides.add(
        "face-outline",
        None,
        Paint::Form,
        Shape::Path {
            stretches: outline_stretches.clone(),
            closed: false,
        },
        outline_anchors,
    );

    // Level lines: each spans the face outline at its height, from the jaw line to the front.
    {
        let nose_y = guides.position(&format!("jaw-line-{NEAR}-at-nose-level"))[1];
        // The front of the outline is its stretches down to the chin.
        let front_x = furthest_crossing(
            &outline[..=chin_at],
            &outline_stretches[..chin_at],
            1,
            nose_y,
        )
        .unwrap_or_else(|| guides.position(nose_point)[0]);
        let nose_front = guides.view_point(
            "face-outline-front-at-nose-level".to_string(),
            front_x,
            nose_y,
        );
        for (name, level, front) in [
            (
                "eye-level-line",
                "at-eye-level",
                "centre-line-at-eye-level".to_string(),
            ),
            ("nose-level-line", "at-nose-level", nose_front),
            (
                "mouth-level-line",
                "at-mouth-level",
                "mouth-line-centre".to_string(),
            ),
        ] {
            let anchors = vec![format!("jaw-line-{NEAR}-{level}"), front];
            guides.add(name, None, Paint::Construction, Shape::Line, anchors);
        }
    }

    // The near eye: the same opening and lid line at their depths. The iris is a flat disc
    // that faces forward, so it is seen edge on: an upright line from its top to its bottom.
    {
        let group = "eye-right";
        eye_opening_and_lid_line(&mut guides, NEAR, group);
        let centre = guides.position(&format!("iris-{NEAR}-centre"));
        let ry = seen.length(head.iris_height / 2.0);
        let anchors = vec![
            guides.view_point(format!("iris-{NEAR}-top"), centre[0], centre[1] - ry),
            guides.view_point(format!("iris-{NEAR}-bottom"), centre[0], centre[1] + ry),
        ];
        guides.add(
            &format!("iris-{NEAR}"),
            Some(group),
            Paint::Form,
            Shape::Line,
            anchors,
        );
    }

    brow_line(&mut guides, NEAR);

    // The nose mark and the mouth line: the near half of each, from its end to the centre line.
    guides.add(
        "nose-mark",
        None,
        Paint::Form,
        Shape::Line,
        vec![format!("nose-mark-{NEAR}-corner"), nose_point.to_string()],
    );
    guides.add(
        "mouth-line",
        None,
        Paint::Form,
        Shape::Line,
        vec![
            format!("mouth-line-{NEAR}-end"),
            "mouth-line-centre".to_string(),
        ],
    );

    ear(&mut guides, NEAR);

    // Neck: the same upright round column. Its two lines are its back and its front. Where
    // the set gives a point of the profile at an end of a line, the line runs through it.
    {
        let middle = seen.x_at(head.neck_depth);
        let half = seen.length(head.neck_half_width);
        let base_y = seen.y_at(head.neck_base_drop);
        let chin_y = guides.position("chin")[1];
        let skull = guides.position("skull-centre");
        let radius = seen.length(head.skull_radius);
        for (end, x) in [("back", middle - half), ("front", middle + half)] {
            let (top, base) = (format!("neck-{end}-top"), format!("neck-{end}-base"));
            let top = match head.has(&top) {
                true => top,
                false => {
                    let start_y =
                        neck_line_start(x, &outline, &outline_stretches, skull, radius, chin_y);
                    guides.view_point(top, x, start_y)
                }
            };
            let base = match head.has(&base) {
                true => base,
                false => guides.view_point(base, x, base_y),
            };
            let anchors = vec![top, base];
            guides.add(
                &format!("neck-{end}"),
                Some("neck"),
                Paint::Form,
                Shape::Line,
                anchors,
            );
        }
    }

    // Shoulder line: seen end on, both shoulder points stand at one place. So that it shows,
    // it is drawn as a level tick through that place, half the neck's width to each side.
    {
        let point = guides.position(&format!("shoulder-{NEAR}-point"));
        let half = seen.length(head.neck_half_width);
        let anchors = vec![
            guides.view_point(
                "shoulder-line-back-end".to_string(),
                point[0] - half,
                point[1],
            ),
            guides.view_point(
                "shoulder-line-front-end".to_string(),
                point[0] + half,
                point[1],
            ),
        ];
        guides.add("shoulder-line", None, Paint::Form, Shape::Line, anchors);
    }

    guides
}

/// Builds every guide of a view.
pub fn build(head: &Head, seen: &Seen) -> Guides {
    if seen.view == View::Side {
        return build_side(head, seen);
    }
    // The three-quarter view's centre line and nose mark run through the nose point of a
    // turned view, the tip of the nose. The front view's run through the nose mark's bottom:
    // the base of the nose is what a front view marks.
    let nose_point = if seen.view == View::ThreeQuarter {
        head.nose_point()
    } else {
        "nose-mark-bottom"
    };
    let mut guides = Guides {
        guides: Vec::new(),
        view_points: Vec::new(),
        positions: seen
            .landmarks
            .iter()
            .map(|landmark| (landmark.name.clone(), [landmark.x, landmark.y]))
            .collect(),
    };

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
        // Where the set gives the forehead, the centre line runs through it below the skull's
        // front.
        let mut anchors = names(&["skull-top", "skull-front"]);
        if head.has(FOREHEAD) {
            anchors.push(FOREHEAD.to_string());
        }
        anchors.extend(names(&[BRIDGE, nose_point, "mouth-line-centre", "chin"]));
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
        eye_opening_and_lid_line(&mut guides, side, group);

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
        brow_line(&mut guides, side);
    }

    guides.add(
        "nose-mark",
        None,
        Paint::Form,
        Shape::Polyline,
        names(&[
            "nose-mark-right-corner",
            nose_point,
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

    for side in ["right", "left"] {
        ear(&mut guides, side);
    }

    // Neck: an upright round column, as wide in every view.
    {
        let middle = seen.x_at(head.neck_depth);
        let half = seen.length(head.neck_half_width);
        let base_y = seen.y_at(head.neck_base_drop);
        let chin_y = guides.position("chin")[1];
        let outline = guides.positions_of(&outline_anchors);
        let skull = guides.position("skull-centre");
        let radius = seen.length(head.skull_radius);
        for (side, x) in [("right", middle - half), ("left", middle + half)] {
            let start_y = neck_line_start(x, &outline, &outline_stretches, skull, radius, chin_y);
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
