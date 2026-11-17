//! Takes landmark positions from the hand-made drawings maquette relies on, and writes the
//! records of them that `data/saffi/` holds.
//!
//! The drawings are Saffi's, made by hand in mudsong's portrait study: the two grey S2
//! underdrawings her comparison case was first recorded from, and the two final drawings
//! whose ears and lids Jon ruled it takes instead. They are private. This module is the one
//! place that says which node of which shape is which landmark in each of them, and it is
//! arithmetic on the files' own coordinates: nothing is estimated from a rendered picture.
//!
//! It began as a throwaway program kept beside the private copies of the underdrawings, which
//! wrote `hand-made-views.toml` and `views-compared.toml` for ticket #4. It was moved here so
//! that a drawing is read with a command and not with one-off searches, and the part that
//! writes those two files is that program's, unchanged: where the drawings are present the
//! command writes the committed files again byte for byte.
//!
//! A drawing is read only when its SHA-256 digest is the recorded one. The checks inside the
//! readings (`assert!`) guard against a slip in this module: a drawing with the recorded
//! digest cannot fail them, so one that fires is a fault in the program.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::armature::File;
use crate::drawing::{self, Axis, Drawing, Elem, P, Seg, Shape, Sub, covers};
use crate::number;
use crate::{Error, Result};

/// One hand-made drawing maquette relies on: its file's name where it is kept, and the
/// SHA-256 digest that is its only identity, because no commit names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pinned {
    pub file: &'static str,
    pub sha256: &'static str,
}

/// The S2 underdrawing, front view.
pub const FRONT: Pinned = Pinned {
    file: "s2-base-underdrawing-A-front.svg",
    sha256: "34ee7b92a8422031a20f7d2f195d6952d16e1099de3002f5fffcb6697a57f76f",
};
/// The S2 underdrawing, three-quarter view.
pub const THREE_QUARTER: Pinned = Pinned {
    file: "s2-base-underdrawing-A.svg",
    sha256: "c50bb0529f254fe12ac0af7b3f6175ae175df14b61d24f16bfec54afcc2efb02",
};
/// The front of the S6 model sheet.
pub const FINAL_FRONT: Pinned = Pinned {
    file: "s6-front.svg",
    sha256: "15d6fe02177427db7b0ae956411b97295f250d3328554f598134bb8e91c8136e",
};
/// The S8 final rig, a three-quarter view.
pub const FINAL_THREE_QUARTER: Pinned = Pinned {
    file: "s8-final-rig.svg",
    sha256: "23251a2e5c196738514f324cec1c25c171fb0785112044d45168541d060d304a",
};

const FRONT_FILE: &str = FRONT.file;
const FRONT_SHA: &str = FRONT.sha256;
const TQ_FILE: &str = THREE_QUARTER.file;
const TQ_SHA: &str = THREE_QUARTER.sha256;

/// A drawing whose shapes are looked up by address.
struct Doc(Drawing);

impl Doc {
    /// The one shape under an address.
    fn get(&self, addr: &str) -> Elem {
        let found = self.0.find(addr);
        assert!(
            found.len() == 1,
            "element {addr} found {} times",
            found.len()
        );
        found[0].clone()
    }

    fn subs(&self, addr: &str) -> Vec<Sub> {
        match self.get(addr).shape {
            Shape::Path(subs) => subs,
            _ => panic!("{addr} is not a path or a line"),
        }
    }
}

fn axis_of(letter: char) -> Axis {
    if letter == 'x' { Axis::X } else { Axis::Y }
}

fn r3(v: f64) -> f64 {
    number::round3(v)
}

/// A number as the data files write it: rounded to `places`, trailing zeros dropped, one
/// decimal kept.
fn num(v: f64, places: usize) -> String {
    number::toml(v, places)
}

#[derive(Clone, Debug)]
struct Lm {
    name: String,
    x: f64,
    y: f64,
    kind: &'static str,
    element: String,
    how: String,
    under: Vec<String>,
}

struct View {
    doc: Doc,
    lms: Vec<Lm>,
    index: BTreeMap<String, usize>,
    sections: Vec<(usize, String)>,
}

fn node_label(subs: &[Sub], s: usize, k: usize) -> String {
    if subs.len() > 1 {
        format!("subpath {s}, node {k}")
    } else {
        format!("node {k}")
    }
}

fn seg_label(subs: &[Sub], s: usize, k: usize) -> String {
    let sub = &subs[s];
    let pre = if subs.len() > 1 {
        format!("subpath {s}, ")
    } else {
        String::new()
    };
    if k == sub.segs.len() {
        format!("{pre}closing edge (node {k} to node 0)")
    } else {
        format!("{pre}segment {k} (node {k} to node {})", k + 1)
    }
}

impl View {
    fn new(doc: Doc) -> View {
        View {
            doc,
            lms: vec![],
            index: BTreeMap::new(),
            sections: vec![],
        }
    }

    fn section(&mut self, title: &str) {
        self.sections.push((self.lms.len(), title.to_string()));
    }

    fn add(&mut self, name: &str, p: P, kind: &'static str, element: &str, how: String) {
        assert!(
            !self.index.contains_key(name),
            "landmark {name} defined twice"
        );
        self.index.insert(name.to_string(), self.lms.len());
        self.lms.push(Lm {
            name: name.into(),
            x: r3(p.x),
            y: r3(p.y),
            kind,
            element: element.into(),
            how,
            under: vec![],
        });
    }

    fn get(&self, name: &str) -> &Lm {
        &self.lms[*self
            .index
            .get(name)
            .unwrap_or_else(|| panic!("no landmark {name}"))]
    }

    fn has(&self, name: &str) -> bool {
        self.index.contains_key(name)
    }

    /// An on-curve node exactly as the path data gives it.
    fn node_kind(&mut self, name: &str, addr: &str, s: usize, k: usize, kind: &'static str) {
        let subs = self.doc.subs(addr);
        let p = subs[s].nodes[k];
        self.add(name, p, kind, addr, node_label(&subs, s, k));
    }

    fn node(&mut self, name: &str, addr: &str, k: usize) {
        self.node_kind(name, addr, 0, k, "node");
    }

    /// The extreme of a whole outline along one axis: the best of its nodes and of the points
    /// where a cubic segment's derivative along that axis is zero.
    fn extreme(&mut self, name: &str, addr: &str, axis: char, want_max: bool, kind: &'static str) {
        let subs = self.doc.subs(addr);
        let val = |p: P| if axis == 'x' { p.x } else { p.y };
        let mut best: Option<(f64, P, String)> = None;
        let mut consider = |p: P, how: String| {
            let v = val(p);
            let better = match &best {
                None => true,
                Some((bv, _, _)) => {
                    if want_max {
                        v > bv + 1e-9
                    } else {
                        v < bv - 1e-9
                    }
                }
            };
            if better {
                best = Some((v, p, how));
            }
        };
        for (s, sub) in subs.iter().enumerate() {
            for (k, n) in sub.nodes.iter().enumerate() {
                consider(*n, node_label(&subs, s, k));
            }
        }
        for (s, sub) in subs.iter().enumerate() {
            for (k, seg) in sub.segs.iter().enumerate() {
                for t in seg.stationary(axis_of(axis)) {
                    consider(
                        seg.at(t),
                        format!(
                            "{}, where d{axis}/dt = 0 at t = {:.4}",
                            seg_label(&subs, s, k),
                            t
                        ),
                    );
                }
            }
        }
        let (_, p, how) = best.unwrap();
        let word = match (axis, want_max) {
            ('y', false) => "highest point of the outline",
            ('y', true) => "lowest point of the outline",
            ('x', false) => "the outline's furthest point toward the viewer's left",
            _ => "the outline's furthest point toward the viewer's right",
        };
        self.add(name, p, kind, addr, format!("{word}: {how}"));
    }

    /// One interior stationary point of one segment (a local extreme, not the whole outline's).
    fn local(&mut self, name: &str, addr: &str, k: usize, axis: char, kind: &'static str) {
        let subs = self.doc.subs(addr);
        let seg = subs[0].seg(k);
        let ts = seg.stationary(axis_of(axis));
        assert!(
            ts.len() == 1,
            "{name}: expected one stationary point, found {}",
            ts.len()
        );
        let how = format!(
            "{}, where d{axis}/dt = 0 at t = {:.4}",
            seg_label(&subs, 0, k),
            ts[0]
        );
        self.add(name, seg.at(ts[0]), kind, addr, how);
    }

    /// Where one segment of an outline crosses the height of a level line.
    fn cross_level(&mut self, name: &str, addr: &str, k: usize, level_from: &str) {
        let y = self.get(level_from).y;
        let subs = self.doc.subs(addr);
        let seg = subs[0].seg(k);
        let ts = seg.roots(&|p: P| p.y - y);
        assert!(
            ts.len() == 1,
            "{name}: expected one crossing, found {}",
            ts.len()
        );
        let how = format!(
            "where {} crosses y = {} (t = {:.4})",
            seg_label(&subs, 0, k),
            num(y, 3),
            ts[0]
        );
        self.add(name, seg.at(ts[0]), "crossing", addr, how);
    }

    /// Where one segment of an outline crosses a circle.
    fn cross_circle(&mut self, name: &str, addr: &str, k: usize, circle: &str) {
        let Shape::Circle { c, r } = self.doc.get(circle).shape else {
            panic!("{circle} is not a circle")
        };
        let subs = self.doc.subs(addr);
        let seg = subs[0].seg(k);
        let ts = seg.roots(&|p: P| (p.x - c.x).powi(2) + (p.y - c.y).powi(2) - r * r);
        assert!(
            ts.len() == 1,
            "{name}: expected one crossing, found {}",
            ts.len()
        );
        let how = format!(
            "where {} crosses the circle {circle} (t = {:.4})",
            seg_label(&subs, 0, k),
            ts[0]
        );
        self.add(name, seg.at(ts[0]), "crossing", addr, how);
    }

    /// Where a straight edge of one outline crosses a segment of another.
    fn cross_edge(&mut self, name: &str, edge_addr: &str, edge_k: usize, addr: &str, k: usize) {
        let esubs = self.doc.subs(edge_addr);
        let Seg::Line(a, b) = esubs[0].seg(edge_k) else {
            panic!("{name}: the edge is not straight")
        };
        let subs = self.doc.subs(addr);
        let seg = subs[0].seg(k);
        let ts = seg.roots(&|p: P| (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x));
        assert!(
            ts.len() == 1,
            "{name}: expected one crossing, found {}",
            ts.len()
        );
        let p = seg.at(ts[0]);
        let along = ((p.x - a.x) * (b.x - a.x) + (p.y - a.y) * (b.y - a.y))
            / ((b.x - a.x).powi(2) + (b.y - a.y).powi(2));
        assert!(
            (0.0..=1.0).contains(&along),
            "{name}: the crossing lies off the edge"
        );
        let how = format!(
            "where the straight {} crosses {addr}, {} (t = {:.4})",
            seg_label(&esubs, 0, edge_k),
            seg_label(&subs, 0, k),
            ts[0]
        );
        self.add(name, p, "crossing", edge_addr, how);
    }

    /// A point midway between recorded landmarks: sideways between `a` and `b`, in height between `c` and `d`.
    fn centre(&mut self, name: &str, a: &str, b: &str, c: &str, d: &str) {
        let p = P {
            x: 0.5 * (self.get(a).x + self.get(b).x),
            y: 0.5 * (self.get(c).y + self.get(d).y),
        };
        let element = self.get(a).element.clone();
        let how = if a == c && b == d {
            format!("midway between {a} and {b}")
        } else {
            format!("midway between {a} and {b} sideways, and between {c} and {d} in height")
        };
        self.add(name, p, "centre", &element, how);
    }

    fn circle(&mut self, prefix: &str, addr: &str) {
        let Shape::Circle { c, r } = self.doc.get(addr).shape else {
            panic!("{addr} is not a circle")
        };
        self.add(
            &format!("{prefix}_centre"),
            c,
            "attribute",
            addr,
            "the circle's cx and cy".into(),
        );
        self.add(
            &format!("{prefix}_top"),
            P { x: c.x, y: c.y - r },
            "highest",
            addr,
            "cy minus r".into(),
        );
        self.add(
            &format!("{prefix}_bottom"),
            P { x: c.x, y: c.y + r },
            "lowest",
            addr,
            "cy plus r".into(),
        );
        self.add(
            &format!("{prefix}_right_extent"),
            P { x: c.x - r, y: c.y },
            "side_extent",
            addr,
            "cx minus r".into(),
        );
        self.add(
            &format!("{prefix}_left_extent"),
            P { x: c.x + r, y: c.y },
            "side_extent",
            addr,
            "cx plus r".into(),
        );
    }

    /// For every landmark, the filled elements painted later that cover it.
    fn find_cover(&mut self) {
        for i in 0..self.lms.len() {
            let lm = self.lms[i].clone();
            if lm.kind == "centre" {
                continue; // a point between landmarks is not on drawn geometry
            }
            let own = self.doc.get(&lm.element).order;
            let p = P { x: lm.x, y: lm.y };
            let mut under = vec![];
            for e in &self.doc.0.elems {
                if e.order > own && e.filled && covers(&e.shape, p) {
                    under.push(e.addr.clone());
                }
            }
            self.lms[i].under = under;
        }
    }

    /// For every landmark, whether the skull, painted later, covers it. A final drawing is
    /// painted: clothing, gear and hair lie over much of it, and an armature carries none of
    /// those, so only the skull is tested.
    fn find_cover_by_skull(&mut self) {
        let skull = self.doc.get(SKULL);
        for i in 0..self.lms.len() {
            let lm = self.lms[i].clone();
            if lm.kind == "centre" {
                continue;
            }
            let own = self.doc.get(&lm.element).order;
            if skull.order > own && covers(&skull.shape, P { x: lm.x, y: lm.y }) {
                self.lms[i].under = vec![SKULL.to_string()];
            }
        }
    }

    /// The highest point of one edge of a band: of the segments `first` to `last` of its
    /// outline, and of the nodes they join.
    fn highest_of_edge(&mut self, name: &str, addr: &str, first: usize, last: usize, edge: &str) {
        let subs = self.doc.subs(addr);
        let sub = &subs[0];
        let mut best: Option<(P, String)> = None;
        let mut consider = |p: P, how: String| {
            if best.as_ref().is_none_or(|(known, _)| p.y < known.y - 1e-9) {
                best = Some((p, how));
            }
        };
        for k in first..=last + 1 {
            consider(sub.nodes[k], node_label(&subs, 0, k));
        }
        for k in first..=last {
            let seg = sub.segs[k];
            for t in seg.stationary(Axis::Y) {
                consider(
                    seg.at(t),
                    format!(
                        "{}, where dy/dt = 0 at t = {:.4}",
                        seg_label(&subs, 0, k),
                        t
                    ),
                );
            }
        }
        let (p, how) = best.expect("an edge has a node");
        self.add(
            name,
            p,
            "highest",
            addr,
            format!("highest point of the band's {edge} edge, segments {first} to {last}: {how}"),
        );
    }
}

/// The skull's construction circle, under the address every one of the drawings gives it.
const SKULL: &str = "face/cranium";

// ---------------------------------------------------------------- which node is which

/// Node numbers of an eye opening (the `white` shape).
struct OpeningIdx {
    outer: usize,
    inner: usize,
}

/// Node numbers of a brow shape.
struct BrowIdx {
    outer_top: usize,
    inner_top: usize,
    inner_bottom: usize,
    outer_bottom: usize,
}

/// Node numbers of an ear's outline (the `back` shape). `side_is_inner` says which flank node 2 sits on.
struct EarIdx {
    base_outer: usize,
    base_inner: usize,
    side_is_inner: bool,
}

/// `side` is "left" or "right", the character's own; `g` is the group's letter in the file.
fn eye(v: &mut View, side: &str, g: &str, o: OpeningIdx) {
    let iris = format!("eye_{g}/iris");
    let pupil = format!("eye_{g}/pupil");
    let lid = format!("eye_{g}/lid_upper");
    // The character's left is toward the viewer's right, so "outer" is larger x for a left eye.
    let outer_is_max = side == "left";
    opening(v, side, g, o);

    let i = |part: &str| format!("iris_{side}_{part}");
    v.extreme(&i("top"), &iris, 'y', false, "highest");
    v.extreme(&i("bottom"), &iris, 'y', true, "lowest");
    v.extreme(&i("outer_extent"), &iris, 'x', outer_is_max, "side_extent");
    eye_rest(v, side, &iris, &pupil, &lid, outer_is_max);
}

/// An eye opening, the `white` shape: its two corners, its top, its bottom and its centre.
fn opening(v: &mut View, side: &str, g: &str, o: OpeningIdx) {
    let white = format!("eye_{g}/white");
    let outer_is_max = side == "left";
    let n = |part: &str| format!("eye_{side}_{part}");

    v.node(&n("outer_corner"), &white, o.outer);
    v.node(&n("inner_corner"), &white, o.inner);
    v.extreme(&n("opening_top"), &white, 'y', false, "highest");
    v.extreme(&n("opening_bottom"), &white, 'y', true, "lowest");
    v.centre(
        &n("opening_centre"),
        &n("outer_corner"),
        &n("inner_corner"),
        &n("opening_top"),
        &n("opening_bottom"),
    );
    // The corners must be the opening's sideways extremes, or the centre above would be wrong.
    {
        let subs = v.doc.subs(&white);
        let xs: Vec<f64> = subs[0]
            .segs
            .iter()
            .flat_map(|s| {
                s.stationary(Axis::X)
                    .into_iter()
                    .map(|t| s.at(t).x)
                    .collect::<Vec<_>>()
            })
            .chain(subs[0].nodes.iter().map(|p| p.x))
            .collect();
        let (lo, hi) = (
            xs.iter().cloned().fold(f64::MAX, f64::min),
            xs.iter().cloned().fold(f64::MIN, f64::max),
        );
        let (oc, ic) = (v.get(&n("outer_corner")).x, v.get(&n("inner_corner")).x);
        assert!(
            (oc.min(ic) - lo).abs() < 1e-9 && (oc.max(ic) - hi).abs() < 1e-9,
            "{side} eye: a corner is not an extreme"
        );
        assert!(
            (oc > ic) == outer_is_max,
            "{side} eye: outer and inner corners are swapped"
        );
    }
}

/// The rest of an underdrawing's eye, after its iris's top, bottom and outer extent: the
/// iris's inner extent and centre, the pupil and the lid line.
fn eye_rest(v: &mut View, side: &str, iris: &str, pupil: &str, lid: &str, outer_is_max: bool) {
    let (iris, pupil, lid) = (iris.to_string(), pupil.to_string(), lid.to_string());
    let i = |part: &str| format!("iris_{side}_{part}");
    v.extreme(&i("inner_extent"), &iris, 'x', !outer_is_max, "side_extent");
    v.centre(
        &i("centre"),
        &i("outer_extent"),
        &i("inner_extent"),
        &i("top"),
        &i("bottom"),
    );

    let p = |part: &str| format!("pupil_{side}_{part}");
    v.node(&p("top"), &pupil, 0);
    v.node(&p("bottom"), &pupil, 1);
    v.centre(
        &p("centre"),
        &p("top"),
        &p("bottom"),
        &p("top"),
        &p("bottom"),
    );
    assert!(
        v.get(&p("top")).y < v.get(&p("bottom")).y && v.get(&p("top")).x == v.get(&p("bottom")).x
    );

    let l = |part: &str| format!("lid_line_{side}_{part}");
    v.node(&l("inner_lower"), &lid, 0);
    v.node(&l("lower_mid"), &lid, 1);
    v.node(&l("outer_lower"), &lid, 2);
    v.node(&l("outer_tip"), &lid, 3);
    v.node(&l("upper_mid"), &lid, 4);
    v.node(&l("inner_upper"), &lid, 5);
    assert!(
        (v.get(&l("outer_tip")).x > v.get(&l("inner_lower")).x) == outer_is_max,
        "{side} lid: ends are swapped"
    );
    assert!(
        v.get(&l("upper_mid")).y < v.get(&l("lower_mid")).y
            && v.get(&l("inner_upper")).y < v.get(&l("inner_lower")).y
    );
}

fn brow(v: &mut View, side: &str, g: &str, b: BrowIdx) {
    let addr = format!("head/brow_{g}");
    let n = |part: &str| format!("brow_{side}_{part}");
    v.node(&n("outer_top"), &addr, b.outer_top);
    v.node(&n("inner_top"), &addr, b.inner_top);
    v.node(&n("inner_bottom"), &addr, b.inner_bottom);
    v.node(&n("outer_bottom"), &addr, b.outer_bottom);
    v.extreme(&n("highest"), &addr, 'y', false, "highest");
    let outer_is_max = side == "left";
    assert!(
        (v.get(&n("outer_top")).x > v.get(&n("inner_top")).x) == outer_is_max,
        "{side} brow: ends are swapped"
    );
    assert!((v.get(&n("outer_bottom")).x > v.get(&n("inner_bottom")).x) == outer_is_max);
    assert!(
        v.get(&n("outer_top")).y < v.get(&n("outer_bottom")).y
            && v.get(&n("inner_top")).y < v.get(&n("inner_bottom")).y
    );
}

fn ear(v: &mut View, side: &str, g: &str, e: EarIdx) {
    let addr = format!("ear_{g}/back");
    let n = |part: &str| format!("ear_{side}_{part}");
    let outer_is_max = side == "left";
    v.node(&n("base_outer"), &addr, e.base_outer);
    v.node(&n("upper_node"), &addr, 1);
    v.node_kind(
        &n(if e.side_is_inner {
            "inner_side_node"
        } else {
            "outer_side_node"
        }),
        &addr,
        0,
        2,
        "outline_node",
    );
    v.node(&n("base_inner"), &addr, e.base_inner);
    v.extreme(&n("top"), &addr, 'y', false, "highest");
    v.extreme(&n("outer_extent"), &addr, 'x', outer_is_max, "side_extent");
    v.extreme(&n("inner_extent"), &addr, 'x', !outer_is_max, "side_extent");
    assert!(
        (v.get(&n("base_outer")).x > v.get(&n("base_inner")).x) == outer_is_max,
        "{side} ear: bases are swapped"
    );
}

fn common_head(v: &mut View) {
    v.section("Head height runs from hair_top to chin.");
    v.extreme("hair_top", "head/hair_front", 'y', false, "highest");
    v.extreme("chin", "face/jaw", 'y', true, "lowest");

    v.section("Skull: the construction circle. It is drawn twice, as a filled shape and as a line, with the same numbers.");
    v.circle("skull", "face/cranium");
    {
        let (Shape::Circle { c: a, r: ra }, Shape::Circle { c: b, r: rb }) = (
            v.doc.get("face/cranium").shape,
            v.doc.get("construction/cranium_circle").shape,
        ) else {
            panic!("skull circles missing")
        };
        assert!(
            a.x == b.x && a.y == b.y && ra == rb,
            "the two skull circles differ"
        );
    }

    v.section("Level lines: the two ends of each construction line. Their heights are the levels; their lengths mark nothing.");
    for (s, name) in ["eye", "nose", "mouth"].iter().enumerate() {
        v.node_kind(
            &format!("{name}_level_line_right_end"),
            "construction/levels",
            s,
            0,
            "guide_end",
        );
        v.node_kind(
            &format!("{name}_level_line_left_end"),
            "construction/levels",
            s,
            1,
            "guide_end",
        );
        let (a, b) = (
            v.get(&format!("{name}_level_line_right_end")).clone(),
            v.get(&format!("{name}_level_line_left_end")).clone(),
        );
        assert!(
            a.x < b.x && a.y == b.y,
            "level line {name} is not level or runs the other way"
        );
    }
}

fn common_features(v: &mut View) {
    v.section("Nose mark: the nose shape and the line below it.");
    v.node("nose_mark_right_corner", "nose/leather", 0);
    v.node("nose_mark_left_corner", "nose/leather", 1);
    v.extreme("nose_mark_top", "nose/leather", 'y', false, "highest");
    v.node("nose_mark_bottom", "nose/leather", 2);
    v.node("philtrum_bottom", "nose/philtrum", 1);
    {
        let top = v.doc.subs("nose/philtrum")[0].nodes[0];
        let b = v.get("nose_mark_bottom");
        assert!(
            top.x == b.x && top.y == b.y,
            "the philtrum does not start at the nose's bottom"
        );
        assert!(v.get("nose_mark_right_corner").x < v.get("nose_mark_left_corner").x);
        let subs = v.doc.subs("nose/leather");
        let lowest = subs[0].nodes.iter().map(|p| p.y).fold(f64::MIN, f64::max);
        assert!(lowest == b.y, "the nose's bottom node is not its lowest");
    }

    v.section("Mouth line.");
    v.node("mouth_line_right_end", "mouth/line", 0);
    v.node("mouth_line_centre", "mouth/line", 1);
    v.node("mouth_line_left_end", "mouth/line", 2);
    assert!(v.get("mouth_line_right_end").x < v.get("mouth_line_left_end").x);

    v.section("Hair volume: only the extents of the drawn hair. Its fringe and locks are the painter's and are not recorded. Several lock tips share the lowest height; hair_bottom is the first of them in path order.");
    v.extreme(
        "hair_right_extent",
        "head/hair_front",
        'x',
        false,
        "side_extent",
    );
    v.extreme(
        "hair_left_extent",
        "head/hair_front",
        'x',
        true,
        "side_extent",
    );
    v.extreme("hair_bottom", "head/hair_front", 'y', true, "lowest");
}

fn build_front(doc: Doc) -> View {
    let mut v = View::new(doc);
    common_head(&mut v);

    v.section("Face outline: the `jaw` shape. Its top is closed by straight edges inside the skull circle, which are not recorded.");
    for (k, name) in [
        (0, "right_node_1"),
        (1, "right_node_2"),
        (2, "right_node_3"),
        (6, "left_node_1"),
        (5, "left_node_2"),
        (4, "left_node_3"),
    ] {
        v.node_kind(
            &format!("face_outline_{name}"),
            "face/jaw",
            0,
            k,
            "outline_node",
        );
    }
    v.local(
        "face_outline_right_widest",
        "face/jaw",
        0,
        'x',
        "side_extent",
    );
    v.local(
        "face_outline_left_widest",
        "face/jaw",
        5,
        'x',
        "side_extent",
    );
    v.cross_circle(
        "face_outline_right_meets_skull",
        "face/jaw",
        0,
        "face/cranium",
    );
    v.cross_circle(
        "face_outline_left_meets_skull",
        "face/jaw",
        5,
        "face/cranium",
    );
    v.cross_level(
        "face_outline_right_at_eye_level",
        "face/jaw",
        0,
        "eye_level_line_right_end",
    );
    v.cross_level(
        "face_outline_left_at_eye_level",
        "face/jaw",
        5,
        "eye_level_line_right_end",
    );
    v.cross_level(
        "face_outline_right_at_nose_level",
        "face/jaw",
        0,
        "nose_level_line_right_end",
    );
    v.cross_level(
        "face_outline_left_at_nose_level",
        "face/jaw",
        5,
        "nose_level_line_right_end",
    );
    v.cross_level(
        "face_outline_right_at_mouth_level",
        "face/jaw",
        1,
        "mouth_level_line_right_end",
    );
    v.cross_level(
        "face_outline_left_at_mouth_level",
        "face/jaw",
        4,
        "mouth_level_line_right_end",
    );

    v.section("Centre line: a straight construction line in this view.");
    v.node("centre_line_top", "construction/centre_line", 0);
    v.node("centre_line_bottom", "construction/centre_line", 1);
    v.cross_level(
        "centre_line_at_eye_level",
        "construction/centre_line",
        0,
        "eye_level_line_right_end",
    );
    v.cross_level(
        "centre_line_at_nose_level",
        "construction/centre_line",
        0,
        "nose_level_line_right_end",
    );
    v.cross_level(
        "centre_line_at_mouth_level",
        "construction/centre_line",
        0,
        "mouth_level_line_right_end",
    );

    v.section("Left eye: opening (the `white` shape), iris as drawn (the lid already cuts its top), pupil, and the lid line (the `lid_upper` shape).");
    eye(&mut v, "left", "L", OpeningIdx { outer: 0, inner: 2 });
    v.section("Right eye.");
    eye(&mut v, "right", "R", OpeningIdx { outer: 0, inner: 2 });

    v.section("Brows: each is a filled shape with four corners.");
    brow(
        &mut v,
        "left",
        "L",
        BrowIdx {
            outer_top: 0,
            inner_top: 1,
            inner_bottom: 2,
            outer_bottom: 3,
        },
    );
    brow(
        &mut v,
        "right",
        "R",
        BrowIdx {
            outer_top: 0,
            inner_top: 1,
            inner_bottom: 2,
            outer_bottom: 3,
        },
    );

    common_features(&mut v);

    v.section("Centre line at the heights of the features that stand on it.");
    v.cross_level(
        "centre_line_at_nose_mark_top",
        "construction/centre_line",
        0,
        "nose_mark_top",
    );
    v.cross_level(
        "centre_line_at_nose_mark_bottom",
        "construction/centre_line",
        0,
        "nose_mark_bottom",
    );
    v.cross_level(
        "centre_line_at_philtrum_bottom",
        "construction/centre_line",
        0,
        "philtrum_bottom",
    );

    v.section(
        "Ears: the `back` shape of each. `outer` is away from the centre line, `inner` toward it.",
    );
    ear(
        &mut v,
        "left",
        "L",
        EarIdx {
            base_outer: 0,
            base_inner: 3,
            side_is_inner: true,
        },
    );
    ear(
        &mut v,
        "right",
        "R",
        EarIdx {
            base_outer: 0,
            base_inner: 3,
            side_is_inner: true,
        },
    );

    v.section("Neck and shoulder line: taken from the `torso` shape, which has no separate neck or shoulder element.");
    v.node_kind("neck_right_top", "body/torso", 0, 0, "outline_node");
    v.cross_edge("neck_right_meets_jaw", "body/torso", 0, "face/jaw", 1);
    v.node("neck_right_base", "body/torso", 1);
    v.node("shoulder_right_point", "body/torso", 3);
    v.node_kind("neck_left_top", "body/torso", 0, 9, "outline_node");
    v.cross_edge("neck_left_meets_jaw", "body/torso", 8, "face/jaw", 4);
    v.node("neck_left_base", "body/torso", 8);
    v.node("shoulder_left_point", "body/torso", 6);

    v.find_cover();
    v
}

fn build_three_quarter(doc: Doc) -> View {
    let mut v = View::new(doc);
    common_head(&mut v);

    v.section("Face outline: the `jaw` shape. The left side is the far side. On the near side the drawn outline stops at node 8; above it the shape is closed by straight edges that are not recorded.");
    for (k, name) in [
        (8, "right_node_1"),
        (7, "right_node_2"),
        (6, "right_node_3"),
    ] {
        v.node_kind(
            &format!("face_outline_{name}"),
            "face/jaw",
            0,
            k,
            "outline_node",
        );
    }
    for (k, name) in [
        (0, "left_node_1"),
        (1, "left_node_2"),
        (2, "left_node_3"),
        (3, "left_node_4"),
        (4, "left_node_5"),
    ] {
        v.node_kind(
            &format!("face_outline_{name}"),
            "face/jaw",
            0,
            k,
            "outline_node",
        );
    }
    v.local("face_outline_left_cheek", "face/jaw", 1, 'x', "side_extent");
    v.cross_circle(
        "face_outline_right_meets_skull",
        "face/jaw",
        7,
        "face/cranium",
    );
    v.cross_circle(
        "face_outline_left_meets_skull",
        "face/jaw",
        1,
        "face/cranium",
    );
    v.cross_level(
        "face_outline_left_at_eye_level",
        "face/jaw",
        0,
        "eye_level_line_right_end",
    );
    v.cross_level(
        "face_outline_right_at_nose_level",
        "face/jaw",
        7,
        "nose_level_line_right_end",
    );
    v.cross_level(
        "face_outline_left_at_nose_level",
        "face/jaw",
        2,
        "nose_level_line_right_end",
    );
    v.cross_level(
        "face_outline_right_at_mouth_level",
        "face/jaw",
        7,
        "mouth_level_line_right_end",
    );
    v.cross_level(
        "face_outline_left_at_mouth_level",
        "face/jaw",
        3,
        "mouth_level_line_right_end",
    );

    v.section(
        "Centre line: a curved construction line in this view, with four nodes between its ends.",
    );
    v.node("centre_line_top", "construction/centre_line", 0);
    for k in 1..=4 {
        v.node_kind(
            &format!("centre_line_node_{k}"),
            "construction/centre_line",
            0,
            k,
            "outline_node",
        );
    }
    v.node("centre_line_bottom", "construction/centre_line", 5);
    v.cross_level(
        "centre_line_at_eye_level",
        "construction/centre_line",
        1,
        "eye_level_line_right_end",
    );
    v.cross_level(
        "centre_line_at_nose_level",
        "construction/centre_line",
        3,
        "nose_level_line_right_end",
    );
    v.cross_level(
        "centre_line_at_mouth_level",
        "construction/centre_line",
        3,
        "mouth_level_line_right_end",
    );

    v.section("Left eye, on the far side: opening (the `white` shape), iris as drawn (the lid already cuts its top), pupil, and the lid line (the `lid_upper` shape).");
    eye(&mut v, "left", "L", OpeningIdx { outer: 2, inner: 0 });
    v.section("Right eye, on the near side.");
    eye(&mut v, "right", "R", OpeningIdx { outer: 0, inner: 2 });

    v.section("Brows: each is a filled shape with four corners.");
    brow(
        &mut v,
        "left",
        "L",
        BrowIdx {
            outer_top: 1,
            inner_top: 0,
            inner_bottom: 3,
            outer_bottom: 2,
        },
    );
    brow(
        &mut v,
        "right",
        "R",
        BrowIdx {
            outer_top: 0,
            inner_top: 1,
            inner_bottom: 2,
            outer_bottom: 3,
        },
    );

    common_features(&mut v);

    v.section("Centre line at the heights of the features that stand on it.");
    v.cross_level(
        "centre_line_at_nose_mark_top",
        "construction/centre_line",
        2,
        "nose_mark_top",
    );
    v.cross_level(
        "centre_line_at_nose_mark_bottom",
        "construction/centre_line",
        3,
        "nose_mark_bottom",
    );
    v.cross_level(
        "centre_line_at_philtrum_bottom",
        "construction/centre_line",
        3,
        "philtrum_bottom",
    );

    v.section("Ears: the `back` shape of each. `outer` is away from the centre line, `inner` toward it. The left ear is the far one and is painted first, behind the head.");
    ear(
        &mut v,
        "left",
        "L",
        EarIdx {
            base_outer: 3,
            base_inner: 0,
            side_is_inner: false,
        },
    );
    ear(
        &mut v,
        "right",
        "R",
        EarIdx {
            base_outer: 0,
            base_inner: 3,
            side_is_inner: true,
        },
    );

    v.section("Neck and shoulder line: taken from the `torso` shape, which has no separate neck or shoulder element. The near edge of the neck is the shape's closing edge.");
    v.node_kind("neck_right_top", "body/torso", 0, 10, "outline_node");
    v.cross_edge("neck_right_meets_jaw", "body/torso", 10, "face/jaw", 7);
    v.node("neck_right_base", "body/torso", 0);
    v.node("shoulder_right_point", "body/torso", 2);
    v.node_kind("neck_left_top", "body/torso", 0, 9, "outline_node");
    v.cross_edge("neck_left_meets_jaw", "body/torso", 8, "face/jaw", 5);
    v.node("neck_left_base", "body/torso", 8);
    v.node("shoulder_left_point", "body/torso", 5);

    v.find_cover();
    v
}

// ---------------------------------------------------------------- writing the positions

fn q(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

fn head_height(v: &View) -> f64 {
    r3(v.get("chin").y - v.get("hair_top").y)
}

fn write_view(out: &mut String, key: &str, title: &str, file: &str, sha: &str, v: &View) {
    let Shape::Circle { r, .. } = v.doc.get("face/cranium").shape else {
        unreachable!()
    };
    writeln!(
        out,
        "\n# ---------------------------------------------------------------- {title}\n"
    )
    .unwrap();
    writeln!(out, "[views.{key}]").unwrap();
    writeln!(out, "private_copy = {}", q(file)).unwrap();
    writeln!(out, "sha256 = {}", q(sha)).unwrap();
    writeln!(
        out,
        "head_height = {}  # chin.y minus hair_top.y",
        num(head_height(v), 3)
    )
    .unwrap();
    writeln!(out, "skull_radius = {}", num(r, 3)).unwrap();
    writeln!(out, "landmark_count = {}", v.lms.len()).unwrap();
    writeln!(out, "\n[views.{key}.landmarks]").unwrap();
    for (i, lm) in v.lms.iter().enumerate() {
        for (at, text) in &v.sections {
            if *at == i {
                writeln!(out, "\n# {text}").unwrap();
            }
        }
        let mut line = format!(
            "{} = {{ x = {}, y = {}, kind = {}, element = {}, how = {}",
            lm.name,
            num(lm.x, 3),
            num(lm.y, 3),
            q(lm.kind),
            q(&lm.element),
            q(&lm.how)
        );
        if !lm.under.is_empty() {
            let list: Vec<String> = lm.under.iter().map(|u| q(u)).collect();
            write!(line, ", under = [{}]", list.join(", ")).unwrap();
        }
        line.push_str(" }");
        writeln!(out, "{line}").unwrap();
    }
}

fn write_positions(front: &View, tq: &View) -> String {
    let mut out = String::new();
    out.push_str(
        r#"# Saffi's hand-made views as numbers: landmark positions in her front and three-quarter views.
#
# Written by a derivation program from two private drawings; see README.md beside this file.
# Do not edit by hand: rerun the derivation where the private copies exist.
# These numbers are the comparison case. They never enter a proportion table.

[source]
what = "Two hand-made grey underdrawings of Saffi's bust, a front view and a three-quarter view, drawn in mudsong's portrait study on 2026-11-15."
owner = "Jon. Saffi is his own character."
held_in = "mudsong's study folder, which Git ignores there, so the SHA-256 digest of each file is its only identity. Private copies are kept in maquette's reference folder, outside every repository."
read_on = 2026-11-16
approval = "Jon approved one read of these drawings on 2026-11-16."
taken_by = "Arithmetic on the files' own coordinates: path nodes, shape attributes, and curve arithmetic on cubic segments. Nothing was estimated from a rendered picture."

[coordinates]
units = "SVG user units of the source files. Both have viewBox 0 0 1000 1000 at 1000 by 1000 pixels, so one unit is one pixel of their renders."
x = "grows toward the viewer's right"
y = "grows downward"
transforms = "None. Neither file has a transform on any element, so the numbers are the files' own."
rounding = "Positions are rounded to 0.001 unit. Node and attribute positions are exact."
left_right = "Left and right in a name are the character's own. Her right is toward the viewer's left in both views."
strokes = "A position is on a shape's geometric outline or a line's centre. Stroke width is ignored."

[kinds]
node = "An on-curve node, exactly as the path data gives it. It marks a point of a feature."
outline_node = "An on-curve node of a silhouette or guide line that marks no feature of its own. It is not the same point of the head in the other view."
attribute = "Read from a shape's attributes."
guide_end = "An end of a straight construction line."
highest = "The top of an outline: the least y among its nodes and the points where a cubic segment's dy/dt is zero."
lowest = "The bottom of an outline, found the same way."
side_extent = "The sideways extreme of an outline, found like its top with dx/dt in place of dy/dt, or the one point of a named segment where dx/dt is zero. It is a silhouette point: another view shows a different point of the head there."
crossing = "Where a segment crosses a level line's height, the skull circle or another shape's straight edge, found by bisection on the curve's parameter t."
centre = "Midway between other recorded landmarks, as its `how` says."

[fields]
element = "The source element, as the id of its enclosing group, a slash, and its name in the file."
how = "Which node or segment, and the arithmetic. Nodes and segments are numbered from 0 in path order; segment k runs from node k to node k + 1."
under = "Filled shapes painted later that cover the point, so it cannot be seen in the drawing. Absent when nothing covers it. Strokes are not counted, a point within 0.02 unit of a shape's edge counts as uncovered, and a `centre` is not tested."

# What the hand-made views give for each guide an armature carries, and what they do not give.
[guides]
skull = "A construction circle of the same radius in both views, drawn once as a filled shape and once as a line. It is a rule of construction and not a drawn feature: its top and both sides lie under the hair."
face_outline = "The `jaw` shape. Front view: both sides from y = 330 down to the chin, the stretch from the top to past the eye level lying under the hair. Three-quarter view: the far side from y = 337 down to the chin, and the near side from the nose level down. Above the nose level the near side has nothing drawn."
centre_line = "A straight line in the front view and a curve in the three-quarter view, from the height of the skull's top to the chin."
level_lines = "Three straight lines for the eyes, the nose and the mouth, in both views."
eye_opening = "The `white` shape of each eye, in both views."
lid_line = "The `lid_upper` shape of each eye, in both views. It is a band with thickness and not a line, and its tilt is written nowhere: it follows from the recorded ends."
iris = "The `iris` shape of each eye, in both views, as drawn: the lid already cuts its top, so the whole disc is not there. The pupil is a slit, recorded by its top, bottom and centre."
brow_lines = "The `brow` shape of each side, in both views. It is a band with four corners and not a line."
nose_mark = "The `leather` shape and the `philtrum` line below it, in both views."
mouth_line = "The `line` path of the mouth, in both views: two ends and a node between them."
ears = "The `back` shape of each ear, in both views. In the three-quarter view the far ear is painted behind the head, and its two base points are covered."
neck = "No element of its own: its edges are edges of the `torso` shape. Front view: both edges, straight and upright. Three-quarter view: a short far edge, and for the near edge only the shape's straight closing edge."
hair_volume = "Nothing as a plain envelope. The drawn hair has a fringe and locks. Only its top, its two side extents and its lowest lock tip are recorded."
shoulder_line = "No element of its own. The two shoulder points and the two points where the neck ends are nodes of the `torso` outline."
depth = "Nothing. There is no hand-made side view, and no position in either view says how far forward a feature sits."

# Drawn in the files and left out, because no guide above uses them.
[not_recorded]
lid_lower = "The stroke under each eye."
lip_mark = "The stroke under the mouth line."
ear_inner = "The lighter shape inside each ear."
hair_over_ear = "The band of hair across the base of an ear."
hair_fringe = "Every node of the hair's fringe and locks."
torso = "The torso outline between the neck and the shoulder points, and everything below the shoulder points."
jaw_closing_edges = "The straight edges that close the `jaw` shape inside the skull circle."
control_points = "Every off-curve control point of every path. No path data is recorded."
"#,
    );
    {
        // The statements above that carry numbers or cover are checked here against the positions.
        for v in [front, tq] {
            for name in ["skull_top", "skull_right_extent", "skull_left_extent"] {
                assert!(
                    v.get(name).under.contains(&"head/hair_front".to_string()),
                    "{name} is not under the hair"
                );
            }
            for (a, b) in [("eye", 366.0), ("nose", 450.0), ("mouth", 500.0)] {
                assert!(v.get(&format!("{a}_level_line_right_end")).y == b);
            }
        }
        assert!(
            front.get("face_outline_right_node_1").y == 330.0
                && front.get("face_outline_left_node_1").y == 330.0
        );
        for name in [
            "face_outline_right_node_1",
            "face_outline_left_node_1",
            "face_outline_right_at_eye_level",
            "face_outline_left_at_eye_level",
        ] {
            assert!(!front.get(name).under.is_empty(), "{name} is not covered");
        }
        assert!(
            front
                .get("face_outline_right_at_nose_level")
                .under
                .is_empty()
                && front
                    .get("face_outline_left_at_nose_level")
                    .under
                    .is_empty()
        );
        assert!(
            tq.get("face_outline_left_node_1").y == 337.0
                && tq.get("face_outline_left_node_1").under.is_empty()
        );
        assert!(
            tq.get("face_outline_right_node_1").y == tq.get("nose_level_line_right_end").y
                && !tq.has("face_outline_right_at_eye_level")
        );
        assert!(
            !tq.get("ear_left_base_outer").under.is_empty()
                && !tq.get("ear_left_base_inner").under.is_empty()
        );
        let Shape::Circle { r: rf, .. } = front.doc.get("face/cranium").shape else {
            unreachable!()
        };
        let Shape::Circle { r: rt, .. } = tq.doc.get("face/cranium").shape else {
            unreachable!()
        };
        assert!(rf == rt);
    }
    write_view(
        &mut out,
        "front",
        "Front view",
        FRONT_FILE,
        FRONT_SHA,
        front,
    );
    write_view(
        &mut out,
        "three_quarter",
        "Three-quarter view",
        TQ_FILE,
        TQ_SHA,
        tq,
    );
    out
}

// ---------------------------------------------------------------- comparing the views

const POINT_KINDS: [&str; 2] = ["node", "centre"];
const LEVEL_KINDS: [&str; 6] = [
    "node",
    "centre",
    "highest",
    "lowest",
    "attribute",
    "guide_end",
];

/// Pairs left out of the turn, and why. Decided by what the landmark is, before any fitting.
const NOT_COUNTED: [(&str, &str); 5] = [
    ("pupil_top", "same sideways position as pupil_centre"),
    ("pupil_bottom", "same sideways position as pupil_centre"),
    (
        "lid_line_upper_mid",
        "same sideways position as lid_line_lower_mid",
    ),
    (
        "eye_opening_centre",
        "the mean of the two corners of the opening, which are both counted",
    ),
    (
        "neck_base",
        "the neck is a column: both its edges are silhouettes, and a column's width does not narrow as it turns",
    ),
];

struct Pair {
    name: String,
    body: bool,
    front: f64,
    tq: f64,
    covered: Vec<String>,
}

fn pairs(front: &View, tq: &View) -> Vec<Pair> {
    let mut out = vec![];
    for lm in &front.lms {
        if !lm.name.contains("_right") {
            continue;
        }
        let right = lm.name.clone();
        let left = right.replace("_right", "_left");
        if !(front.has(&left) && tq.has(&left) && tq.has(&right)) {
            continue;
        }
        let all = [
            front.get(&left),
            front.get(&right),
            tq.get(&left),
            tq.get(&right),
        ];
        if !all.iter().all(|l| POINT_KINDS.contains(&l.kind)) {
            continue;
        }
        let name = right.replace("_right", "");
        if NOT_COUNTED.iter().any(|(n, _)| *n == name) {
            continue;
        }
        let mut covered = vec![];
        for (view, l) in [
            ("front", all[0]),
            ("front", all[1]),
            ("three-quarter", all[2]),
            ("three-quarter", all[3]),
        ] {
            if !l.under.is_empty() {
                covered.push(format!("{} in the {view} view", l.name));
            }
        }
        out.push(Pair {
            body: name.starts_with("neck_") || name.starts_with("shoulder_"),
            front: r3(all[0].x - all[1].x),
            tq: r3(all[2].x - all[3].x),
            name,
            covered,
        });
    }
    out
}

fn fit(ps: &[&Pair]) -> f64 {
    let ft: f64 = ps.iter().map(|p| p.front * p.tq).sum();
    let ff: f64 = ps.iter().map(|p| p.front * p.front).sum();
    ft / ff
}

fn deg(cosine: f64) -> f64 {
    cosine.clamp(-1.0, 1.0).acos().to_degrees()
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        0.5 * (v[n / 2 - 1] + v[n / 2])
    }
}

/// The centre line's sideways position at one height.
fn centre_line_x(v: &View, y: f64) -> f64 {
    let subs = v.doc.subs("construction/centre_line");
    let mut xs: Vec<f64> = vec![];
    for seg in &subs[0].segs {
        for t in seg.roots(&|p: P| p.y - y) {
            let x = seg.at(t).x;
            if !xs.iter().any(|o| (o - x).abs() < 1e-6) {
                xs.push(x);
            }
        }
    }
    assert!(
        xs.len() == 1,
        "the centre line crosses y = {y} {} times",
        xs.len()
    );
    r3(xs[0])
}

fn write_compared(front: &View, tq: &View) -> (String, String) {
    let hh = head_height(front);
    let hh_tq = head_height(tq);
    let pct = |d: f64| 100.0 * d.abs() / hh;
    let mut out = String::new();
    let mut log = String::new();

    out.push_str(
        r#"# Saffi's hand-made views compared: the turn of the three-quarter view, and where the two
# views do not describe one head.
#
# Every number here follows from the positions in hand-made-views.toml by the arithmetic
# stated beside it, so it can be recomputed from that file alone. See README.md.
# These numbers are the comparison case. They never enter a proportion table.

"#,
    );

    writeln!(out, "[yardstick]").unwrap();
    writeln!(
        out,
        "head_height = {}  # front view, hair_top to chin; every share below is of this",
        num(hh, 3)
    )
    .unwrap();
    writeln!(out, "head_height_three_quarter = {}", num(hh_tq, 3)).unwrap();
    writeln!(out, "one_percent = {}", num(hh / 100.0, 3)).unwrap();

    // Is the front view symmetric about its centre line?
    let cl = front.get("centre_line_top").x;
    let (mut n_sym, mut worst_side, mut worst_level) = (0, 0.0f64, 0.0f64);
    let mut uneven: Vec<String> = vec![];
    for lm in &front.lms {
        if lm.name.contains("_right") {
            let left = lm.name.replace("_right", "_left");
            if front.has(&left) {
                let l = front.get(&left);
                n_sym += 1;
                let (side, level) = (r3((0.5 * (l.x + lm.x) - cl).abs()), r3((l.y - lm.y).abs()));
                worst_side = worst_side.max(side);
                worst_level = worst_level.max(level);
                if side > 0.0 || level > 0.0 {
                    uneven.push(lm.name.replace("_right", ""));
                }
            }
        }
    }
    writeln!(out, "\n# Every left and right landmark of the front view, set against its mirror twin about the centre line.").unwrap();
    writeln!(out, "[front_symmetry]").unwrap();
    writeln!(out, "centre_line_x = {}", num(cl, 3)).unwrap();
    writeln!(out, "pairs_checked = {n_sym}").unwrap();
    writeln!(out, "largest_sideways_offset = {}", num(worst_side, 3)).unwrap();
    writeln!(out, "largest_level_offset = {}", num(worst_level, 3)).unwrap();
    let list: Vec<String> = uneven.iter().map(|u| q(u)).collect();
    writeln!(out, "pairs_not_mirrored = [{}]", list.join(", ")).unwrap();

    // The turn.
    let all = pairs(front, tq);
    let head: Vec<&Pair> = all.iter().filter(|p| !p.body).collect();
    let body: Vec<&Pair> = all.iter().filter(|p| p.body).collect();
    let seen: Vec<&Pair> = head
        .iter()
        .cloned()
        .filter(|p| p.covered.is_empty())
        .collect();
    let cosine = fit(&head);
    let turn = deg(cosine);
    let cosine_seen = fit(&seen);
    let turns: Vec<f64> = head.iter().map(|p| deg(p.tq / p.front)).collect();
    let lo = turns.iter().cloned().fold(f64::MAX, f64::min);
    let hi = turns.iter().cloned().fold(f64::MIN, f64::max);

    out.push_str(
        r#"
# The turn. A level head turns about an upright axis and is drawn flat, with no perspective.
# Two landmarks that mirror each other across the centre line then stand cos(turn) times as
# far apart as in the front view, whatever their depth. So each pair gives a turn of its own:
# acos(three-quarter spacing / front spacing).
#
# One turn for the whole head is the one that leaves the smallest squared misfit in spacing
# over all the head's pairs: cosine = sum(front * three_quarter) / sum(front * front).
# A pair is every left and right landmark of the head, of kind `node` or `centre`, recorded in
# both views. The pairs left out are listed with the reason, and the shoulders follow separately.
# No pair was dropped for its result.
"#,
    );
    writeln!(out, "[turn]").unwrap();
    writeln!(out, "direction = \"Toward the viewer's right. The character's right is the near side: her left eye is the narrower one and the nose stands to the viewer's right of the skull's centre.\"").unwrap();
    writeln!(out, "pairs = {}", head.len()).unwrap();
    writeln!(out, "cosine = {}", num(cosine, 4)).unwrap();
    writeln!(out, "degrees = {}", num(turn, 2)).unwrap();
    writeln!(out, "lowest_pair_degrees = {}", num(lo, 2)).unwrap();
    writeln!(out, "highest_pair_degrees = {}", num(hi, 2)).unwrap();
    writeln!(
        out,
        "median_pair_degrees = {}",
        num(median(turns.clone()), 2)
    )
    .unwrap();
    writeln!(
        out,
        "# The same fit over only the pairs whose four points are all uncovered in the drawings."
    )
    .unwrap();
    writeln!(out, "uncovered_pairs = {}", seen.len()).unwrap();
    writeln!(out, "uncovered_cosine = {}", num(cosine_seen, 4)).unwrap();
    writeln!(out, "uncovered_degrees = {}", num(deg(cosine_seen), 2)).unwrap();
    writeln!(out, "\n[turn.not_counted]").unwrap();
    for (n, why) in NOT_COUNTED {
        writeln!(out, "{n} = {}", q(why)).unwrap();
    }
    {
        // direction, checked rather than assumed
        let far_eye = tq.get("eye_left_outer_corner").x - tq.get("eye_left_inner_corner").x;
        let near_eye = tq.get("eye_right_inner_corner").x - tq.get("eye_right_outer_corner").x;
        assert!(
            far_eye < near_eye && tq.get("nose_mark_bottom").x > tq.get("skull_centre").x,
            "the turn is not toward the viewer's right"
        );
    }

    writeln!(
        log,
        "head height: front {}  three-quarter {}",
        num(hh, 3),
        num(hh_tq, 3)
    )
    .unwrap();
    writeln!(
        log,
        "landmarks: front {}  three-quarter {}",
        front.lms.len(),
        tq.lms.len()
    )
    .unwrap();
    writeln!(log, "turn: {} deg (cos {}) over {} pairs; pairs span {} to {}, median {}; uncovered only: {} deg over {}", num(turn, 2), num(cosine, 4), head.len(), num(lo, 2), num(hi, 2), num(median(turns.clone()), 2), num(deg(cosine_seen), 2), seen.len()).unwrap();

    let write_pair = |out: &mut String, log: &mut String, header: &str, p: &Pair| {
        let ratio = p.tq / p.front;
        let residual = p.tq - p.front * cosine;
        writeln!(out, "\n{header}").unwrap();
        if header.starts_with("[[") {
            writeln!(out, "name = {}", q(&p.name)).unwrap();
        }
        writeln!(out, "front_spacing = {}", num(p.front, 3)).unwrap();
        writeln!(out, "three_quarter_spacing = {}", num(p.tq, 3)).unwrap();
        writeln!(out, "ratio = {}", num(ratio, 4)).unwrap();
        writeln!(out, "turn_degrees = {}", num(deg(ratio), 2)).unwrap();
        writeln!(out, "misfit = {}", num(residual, 3)).unwrap();
        writeln!(out, "percent_of_head_height = {}", num(pct(residual), 2)).unwrap();
        writeln!(out, "above_one_percent = {}", pct(residual) > 1.0).unwrap();
        if !p.covered.is_empty() {
            let list: Vec<String> = p.covered.iter().map(|c| q(c)).collect();
            writeln!(out, "covered = [{}]", list.join(", ")).unwrap();
        }
        writeln!(
            log,
            "  {:<24} {:>8} {:>8}  ratio {}  turn {:>6}  misfit {:>7} = {:>5}%{}{}",
            p.name,
            num(p.front, 3),
            num(p.tq, 3),
            num(ratio, 4),
            num(deg(ratio), 2),
            num(residual, 3),
            num(pct(residual), 2),
            if pct(residual) > 1.0 {
                "  ABOVE 1%"
            } else {
                ""
            },
            if p.covered.is_empty() {
                ""
            } else {
                "  (covered)"
            }
        )
        .unwrap();
    };

    writeln!(out, "\n# One entry for each pair of the head. `misfit` is the three-quarter spacing minus the front spacing\n# times the fitted cosine: how far the pair's spacing is from what one head at the fitted turn would show.").unwrap();
    for p in &head {
        write_pair(&mut out, &mut log, "[[turn.pair]]", p);
    }
    writeln!(out, "\n# The shoulders: the spacing of the two shoulder points, kept out of the turn because they belong\n# to the body. `misfit` is against the head's fitted turn: what the spacing would have to lose for\n# the shoulders to turn as far as the head.").unwrap();
    writeln!(log, "shoulders:").unwrap();
    assert!(body.len() == 1 && body[0].name == "shoulder_point");
    for p in &body {
        write_pair(&mut out, &mut log, "[shoulders]", p);
    }

    // Levels.
    let mut diffs: Vec<(String, f64, f64)> = vec![];
    let mut compared = 0;
    for lm in &front.lms {
        if tq.has(&lm.name) {
            let other = tq.get(&lm.name);
            if LEVEL_KINDS.contains(&lm.kind) && LEVEL_KINDS.contains(&other.kind) {
                compared += 1;
                if (other.y - lm.y).abs() > 1e-9 {
                    diffs.push((lm.name.clone(), lm.y, other.y));
                }
            }
        }
    }
    diffs.sort_by(|a, b| {
        (b.2 - b.1)
            .abs()
            .partial_cmp(&(a.2 - a.1).abs())
            .unwrap()
            .then(a.0.cmp(&b.0))
    });
    out.push_str(
        r#"
# Levels. A level head turning about an upright axis keeps every point at its height, and the
# top and bottom of an outline stay the top and bottom. So every landmark recorded in both views
# under one name, except side extents, crossings and outline nodes, should have one y.
"#,
    );
    writeln!(out, "[levels]").unwrap();
    writeln!(out, "compared = {compared}").unwrap();
    writeln!(out, "equal = {}", compared - diffs.len()).unwrap();
    writeln!(out, "different = {}", diffs.len()).unwrap();
    writeln!(
        out,
        "above_one_percent = {}",
        diffs.iter().filter(|d| pct(d.2 - d.1) > 1.0).count()
    )
    .unwrap();
    writeln!(log, "levels: {compared} compared, {} differ", diffs.len()).unwrap();
    for (name, yf, yt) in &diffs {
        let d = r3(yt - yf);
        writeln!(out, "\n[[levels.difference]]").unwrap();
        writeln!(out, "landmark = {}", q(name)).unwrap();
        writeln!(out, "front_y = {}", num(*yf, 3)).unwrap();
        writeln!(out, "three_quarter_y = {}", num(*yt, 3)).unwrap();
        writeln!(
            out,
            "difference = {}  # positive is lower in the three-quarter view",
            num(d, 3)
        )
        .unwrap();
        writeln!(out, "percent_of_head_height = {}", num(pct(d), 2)).unwrap();
        writeln!(out, "above_one_percent = {}", pct(d) > 1.0).unwrap();
        let mut cov = vec![];
        if !front.get(name).under.is_empty() {
            cov.push("front");
        }
        if !tq.get(name).under.is_empty() {
            cov.push("three_quarter");
        }
        if !cov.is_empty() {
            let list: Vec<String> = cov.iter().map(|c| q(c)).collect();
            writeln!(out, "covered_in = [{}]", list.join(", ")).unwrap();
        }
        writeln!(
            log,
            "  {:<28} {:>8} {:>8}  {:>7} = {:>5}%{}{}",
            name,
            num(*yf, 3),
            num(*yt, 3),
            num(d, 3),
            num(pct(d), 2),
            if pct(d) > 1.0 { "  ABOVE 1%" } else { "" },
            if cov.is_empty() {
                String::new()
            } else {
                format!("  (covered: {})", cov.join(", "))
            }
        )
        .unwrap();
    }

    // The centre line against the features that lie on it.
    out.push_str(
        r#"
# The centre line. In the front view the skull's top, the nose mark, the mouth's centre and the
# chin all stand on the centre line. On one head they still do after a turn: the centre line is
# where the middle of the head shows. `offset` is the feature's x minus the centre line's x at
# the feature's height. The centre line starts at the height of the skull's top in both views.
"#,
    );
    writeln!(log, "centre line:").unwrap();
    let mut entry = |out: &mut String, name: &str, on_line: &str, f: f64, t: f64| {
        let d = r3(t - f);
        writeln!(out, "\n[[centre_line.offset]]").unwrap();
        writeln!(out, "feature = {}", q(name)).unwrap();
        writeln!(out, "centre_line_at = {}", q(on_line)).unwrap();
        writeln!(out, "front_offset = {}", num(f, 3)).unwrap();
        writeln!(
            out,
            "three_quarter_offset = {}  # positive is toward the viewer's right",
            num(t, 3)
        )
        .unwrap();
        writeln!(out, "percent_of_head_height = {}", num(pct(d), 2)).unwrap();
        writeln!(out, "above_one_percent = {}", pct(d) > 1.0).unwrap();
        writeln!(
            log,
            "  {:<28} front {:>6}  three-quarter {:>7} = {:>5}%{}",
            name,
            num(f, 3),
            num(t, 3),
            num(pct(d), 2),
            if pct(d) > 1.0 { "  ABOVE 1%" } else { "" }
        )
        .unwrap();
    };
    assert!(
        front.get("centre_line_top").y == front.get("skull_top").y
            && tq.get("centre_line_top").y == tq.get("skull_top").y
    );
    for (name, on_line) in [
        ("skull_top", "centre_line_top"),
        ("nose_mark_top", "centre_line_at_nose_mark_top"),
        ("nose_mark_bottom", "centre_line_at_nose_mark_bottom"),
        ("philtrum_bottom", "centre_line_at_philtrum_bottom"),
        ("mouth_line_centre", "centre_line_at_mouth_level"),
        ("chin", "centre_line_bottom"),
    ] {
        let off = |v: &View| {
            let (f, c) = (v.get(name), v.get(on_line));
            assert!(f.y == c.y, "{name} and {on_line} are at different heights");
            // the recorded crossing must agree with the curve searched over all its segments
            assert!((c.x - centre_line_x(v, f.y)).abs() < 1e-9);
            r3(f.x - c.x)
        };
        entry(&mut out, name, on_line, off(front), off(tq));
    }

    (out, log)
}

// ---------------------------------------------------------------- the final drawings

/// The lids of one eye of a final drawing: the opening they leave, which is the `white`
/// shape, and the lid line, which is the dark `lashes` band along the upper lid. The band's
/// outline has the same eight nodes in all four eyes: its lower edge runs from node 0 at the
/// inner end to node 3 at the outer end, a stroke joins that to the outer tip at node 4, and
/// its upper edge runs back from there to node 7.
fn lids(v: &mut View, side: &str, g: &str, o: OpeningIdx) {
    let lashes = format!("eye_{g}/lashes");
    let outer_is_max = side == "left";
    opening(v, side, g, o);

    let l = |part: &str| format!("lid_line_{side}_{part}");
    v.node(&l("inner_lower"), &lashes, 0);
    v.highest_of_edge(&l("lower_mid"), &lashes, 0, 2, "lower");
    v.node(&l("outer_lower"), &lashes, 3);
    v.node(&l("outer_tip"), &lashes, 4);
    v.highest_of_edge(&l("upper_mid"), &lashes, 4, 6, "upper");
    v.node(&l("inner_upper"), &lashes, 7);
    assert!(
        v.doc.subs(&lashes)[0].nodes.len() == 8,
        "{side} lid: the band does not have eight nodes"
    );
    assert!(
        (v.get(&l("outer_tip")).x > v.get(&l("inner_lower")).x) == outer_is_max,
        "{side} lid: ends are swapped"
    );
    assert!(
        v.get(&l("upper_mid")).y < v.get(&l("lower_mid")).y
            && v.get(&l("inner_upper")).y < v.get(&l("inner_lower")).y
    );
}

fn build_final_front(doc: Doc) -> View {
    let mut v = View::new(doc);
    v.section("Left eye: the opening (the `white` shape) and the lid line (the `lashes` band along the upper lid).");
    lids(&mut v, "left", "L", OpeningIdx { outer: 0, inner: 3 });
    v.section("Right eye.");
    lids(&mut v, "right", "R", OpeningIdx { outer: 0, inner: 3 });
    v.section(
        "Ears: the `back` shape of each. `outer` is away from the centre line, `inner` toward it.",
    );
    ear(
        &mut v,
        "left",
        "L",
        EarIdx {
            base_outer: 0,
            base_inner: 3,
            side_is_inner: true,
        },
    );
    ear(
        &mut v,
        "right",
        "R",
        EarIdx {
            base_outer: 0,
            base_inner: 3,
            side_is_inner: true,
        },
    );
    v.find_cover_by_skull();
    v
}

fn build_final_three_quarter(doc: Doc) -> View {
    let mut v = View::new(doc);
    v.section("Left eye, on the far side: the opening (the `white` shape) and the lid line (the `lashes` band along the upper lid).");
    lids(&mut v, "left", "L", OpeningIdx { outer: 3, inner: 0 });
    v.section("Right eye, on the near side.");
    lids(&mut v, "right", "R", OpeningIdx { outer: 0, inner: 3 });
    v.section("Ears: the `back` shape of each. `outer` is away from the centre line, `inner` toward it. The left ear is the far one and is painted first, behind the head.");
    ear(
        &mut v,
        "left",
        "L",
        EarIdx {
            base_outer: 3,
            base_inner: 0,
            side_is_inner: false,
        },
    );
    ear(
        &mut v,
        "right",
        "R",
        EarIdx {
            base_outer: 0,
            base_inner: 3,
            side_is_inner: true,
        },
    );
    v.find_cover_by_skull();
    v
}

/// The lowest point of the `jaw` shape: the chin.
fn chin_of(doc: &Doc) -> P {
    let mut lowest: Option<P> = None;
    for sub in doc.subs("face/jaw") {
        let stationary = sub
            .segs
            .iter()
            .flat_map(|seg| seg.stationary(Axis::Y).into_iter().map(|t| seg.at(t)));
        for p in sub.nodes.iter().copied().chain(stationary) {
            if lowest.is_none_or(|known| p.y > known.y) {
                lowest = Some(p);
            }
        }
    }
    lowest.expect("the jaw has a node")
}

/// A final drawing's skull circle and chin, which must be the underdrawing's: the two
/// drawings then stand in one frame, and a position of the one can replace a position of
/// the other.
fn frame_of(final_view: &View, underdrawing: &View) -> (P, f64, P) {
    let Shape::Circle { c, r } = final_view.doc.get(SKULL).shape else {
        panic!("{SKULL} is not a circle")
    };
    let chin = chin_of(&final_view.doc);
    let (centre, top, theirs) = (
        underdrawing.get("skull_centre"),
        underdrawing.get("skull_top"),
        underdrawing.get("chin"),
    );
    assert!(
        c.x == centre.x && c.y == centre.y && r3(c.y - r) == top.y,
        "the final drawing's skull is not the underdrawing's"
    );
    assert!(
        r3(chin.x) == theirs.x && r3(chin.y) == theirs.y,
        "the final drawing's chin is not the underdrawing's"
    );
    (c, r, chin)
}

fn write_final_view(
    out: &mut String,
    key: &str,
    title: &str,
    pinned: Pinned,
    stage: &str,
    v: &View,
    underdrawing: &View,
) {
    let (c, r, chin) = frame_of(v, underdrawing);
    writeln!(
        out,
        "\n# ---------------------------------------------------------------- {title}\n"
    )
    .unwrap();
    writeln!(out, "[views.{key}]").unwrap();
    writeln!(out, "file = {}", q(pinned.file)).unwrap();
    writeln!(out, "sha256 = {}", q(pinned.sha256)).unwrap();
    writeln!(out, "stage = {}", q(stage)).unwrap();
    writeln!(
        out,
        "# The skull circle and the chin, which are the S2 underdrawing's exactly: the two drawings stand in one frame."
    )
    .unwrap();
    writeln!(
        out,
        "skull_centre = {{ x = {}, y = {} }}",
        num(c.x, 3),
        num(c.y, 3)
    )
    .unwrap();
    writeln!(out, "skull_radius = {}", num(r, 3)).unwrap();
    writeln!(
        out,
        "chin = {{ x = {}, y = {} }}",
        num(chin.x, 3),
        num(chin.y, 3)
    )
    .unwrap();
    writeln!(out, "landmark_count = {}", v.lms.len()).unwrap();
    writeln!(out, "\n[views.{key}.landmarks]").unwrap();
    for (i, lm) in v.lms.iter().enumerate() {
        for (at, text) in &v.sections {
            if *at == i {
                writeln!(out, "\n# {text}").unwrap();
            }
        }
        let mut line = format!(
            "{} = {{ x = {}, y = {}, kind = {}, element = {}, how = {}",
            lm.name,
            num(lm.x, 3),
            num(lm.y, 3),
            q(lm.kind),
            q(&lm.element),
            q(&lm.how)
        );
        if !lm.under.is_empty() {
            let list: Vec<String> = lm.under.iter().map(|u| q(u)).collect();
            write!(line, ", under = [{}]", list.join(", ")).unwrap();
        }
        line.push_str(" }");
        writeln!(out, "{line}").unwrap();
    }

    writeln!(
        out,
        "\n# How far each of these stands from the S2 underdrawing's landmark of the same name: this drawing's\n# position minus that one's. Positive dx is toward the viewer's right, positive dy is lower."
    )
    .unwrap();
    writeln!(out, "[views.{key}.moved_from_s2]").unwrap();
    for lm in &v.lms {
        assert!(
            underdrawing.has(&lm.name),
            "the underdrawing has no {} to replace",
            lm.name
        );
        let was = underdrawing.get(&lm.name);
        writeln!(
            out,
            "{} = {{ dx = {}, dy = {} }}",
            lm.name,
            num(r3(lm.x - was.x), 3),
            num(r3(lm.y - was.y), 3)
        )
        .unwrap();
    }
}

/// The record of the final drawings: the ears and the lids of each view, and where the two
/// views do not agree on them.
fn write_final(front: &View, tq: &View, s2_front: &View, s2_tq: &View) -> (String, String) {
    let hh = head_height(s2_front);
    let pct = |d: f64| 100.0 * d.abs() / hh;
    let mut out = String::new();
    let mut log = String::new();
    out.push_str(
        r#"# Saffi's final drawings as numbers: the ears and the lids of her front view and her
# three-quarter view, which her comparison case takes in place of the S2 underdrawings'.
#
# Written by `maquette record-hand-made-drawings` from two private drawings; see README.md
# beside this file. Do not edit by hand: run the command again where the drawings are.
# These numbers are the comparison case. They never enter a proportion table.

[source]
what = "The front of the S6 model sheet and the S8 final rig, a three-quarter view: two finished drawings of Saffi's bust, made by hand in mudsong's portrait study on 2026-11-16 over the construction of the S2 underdrawings."
owner = "Jon. Saffi is his own character."
held_in = "mudsong's study folder, which Git ignores there, so the SHA-256 digest of each file is its only identity. They are recorded where they stay: no private copy is kept."
read_on = 2026-11-17
ruling = "Jon, 2026-11-16: Saffi's comparison case takes the larger ears and the hooded upper lids of her final drawings, not those of the grey S2 underdrawing. Everything else stays as recorded from S2."
taken_by = "Arithmetic on the files' own coordinates: path nodes, shape attributes, and curve arithmetic on cubic segments. Nothing was estimated from a rendered picture."

[supersedes]
record = "hand-made-views.toml"
rule = "A landmark of a view here replaces the landmark of the same name in the same view of that record. Each view's ears and lids come from its own drawing, so nothing is worked out through the turn."
ears = "Each ear whole: the `back` shape's two base points, its top, its two side extents and its two other nodes."
lids = "Each eye's lids: the opening they leave, which is the `white` shape (its two corners, its top, its bottom and its centre), and the lid line, which is the dark `lashes` band along the upper lid. The S2 underdrawings draw the lid line as the `lid_upper` shape; in these drawings that shape is a blink lid with no opacity, and the band is `lashes`."
kept_from_s2 = "Everything else. The iris and the pupil stay as S2 has them: the final front view sets its pupils 3 units further out on each side than the S2 front view, and the final three-quarter view keeps the S2 pupils, so taking the front view's would make the two final views disagree. An S2 iris therefore stands 3 to 4.4 units above the top of a final opening: the hooded lid covers that much of it."

[coordinates]
units = "SVG user units of the source files. Both have viewBox 0 0 1000 1000 at 1000 by 1000 pixels, as the S2 underdrawings have."
frame = "Each drawing's skull circle and chin are its S2 underdrawing's exactly, which the command checks before it writes. So a position here and a position of hand-made-views.toml in the same view stand in one frame."
transforms = "None. Neither file has a transform on any element, so the numbers are the files' own."
rounding = "Positions are rounded to 0.001 unit. Node positions are exact."
left_right = "Left and right in a name are the character's own. Her right is toward the viewer's left in both views."

[kinds]
node = "An on-curve node, exactly as the path data gives it. It marks a point of a feature."
outline_node = "An on-curve node of a silhouette that marks no feature of its own. It is not the same point of the head in the other view."
highest = "The top of an outline, or of one edge of the lid band: the least y among its nodes and the points where a cubic segment's dy/dt is zero."
lowest = "The bottom of an outline, found the same way."
side_extent = "The sideways extreme of an outline, found like its top with dx/dt in place of dy/dt. It is a silhouette point: another view shows a different point of the head there."
centre = "Midway between other recorded landmarks, as its `how` says."

[fields]
element = "The source element, as the id of its enclosing group, a slash, and its name in the file."
how = "Which node or segment, and the arithmetic. Nodes and segments are numbered from 0 in path order; segment k runs from node k to node k + 1."
under = "Present when the skull, painted later, covers the point, so it cannot be seen in the drawing. Only the skull is tested: these drawings are painted, and the clothing, gear and hair over them are no part of an armature. A `centre` is not tested."
"#,
    );
    write_final_view(
        &mut out,
        "front",
        "Front view",
        FINAL_FRONT,
        "S6, the model sheet's front",
        front,
        s2_front,
    );
    write_final_view(
        &mut out,
        "three_quarter",
        "Three-quarter view",
        FINAL_THREE_QUARTER,
        "S8, the final rig",
        tq,
        s2_tq,
    );

    // Where the two final views do not agree, on what this record holds.
    out.push_str(
        r#"
# ---------------------------------------------------------------- The two views compared

# Levels. A level head turning about an upright axis keeps every point at its height. So every
# landmark above recorded in both views under one name, except side extents and outline nodes,
# should have one y. Shares are of the S2 front view's head height, as in views-compared.toml.
# These differences are reported and not corrected.
"#,
    );
    let mut diffs: Vec<(String, f64, f64)> = Vec::new();
    let mut compared = 0;
    for lm in &front.lms {
        if tq.has(&lm.name) {
            let other = tq.get(&lm.name);
            if LEVEL_KINDS.contains(&lm.kind) && LEVEL_KINDS.contains(&other.kind) {
                compared += 1;
                if (other.y - lm.y).abs() > 1e-9 {
                    diffs.push((lm.name.clone(), lm.y, other.y));
                }
            }
        }
    }
    diffs.sort_by(|a, b| {
        (b.2 - b.1)
            .abs()
            .total_cmp(&(a.2 - a.1).abs())
            .then(a.0.cmp(&b.0))
    });
    writeln!(out, "[levels]").unwrap();
    writeln!(
        out,
        "head_height = {}  # the S2 front view, hair_top to chin",
        num(hh, 3)
    )
    .unwrap();
    writeln!(out, "compared = {compared}").unwrap();
    writeln!(out, "equal = {}", compared - diffs.len()).unwrap();
    writeln!(out, "different = {}", diffs.len()).unwrap();
    writeln!(
        out,
        "above_one_percent = {}",
        diffs.iter().filter(|d| pct(d.2 - d.1) > 1.0).count()
    )
    .unwrap();
    writeln!(
        log,
        "final drawings: front {} landmarks, three-quarter {}; levels: {compared} compared, {} differ",
        front.lms.len(),
        tq.lms.len(),
        diffs.len()
    )
    .unwrap();
    for (name, yf, yt) in &diffs {
        let d = r3(yt - yf);
        writeln!(out, "\n[[levels.difference]]").unwrap();
        writeln!(out, "landmark = {}", q(name)).unwrap();
        writeln!(out, "front_y = {}", num(*yf, 3)).unwrap();
        writeln!(out, "three_quarter_y = {}", num(*yt, 3)).unwrap();
        writeln!(
            out,
            "difference = {}  # positive is lower in the three-quarter view",
            num(d, 3)
        )
        .unwrap();
        writeln!(out, "percent_of_head_height = {}", num(pct(d), 2)).unwrap();
        writeln!(out, "above_one_percent = {}", pct(d) > 1.0).unwrap();
        if !tq.get(name).under.is_empty() {
            writeln!(out, "covered_in = [\"three_quarter\"]").unwrap();
        }
        writeln!(
            log,
            "  {:<28} {:>8} {:>8}  {:>7} = {:>5}%{}",
            name,
            num(*yf, 3),
            num(*yt, 3),
            num(d, 3),
            num(pct(d), 2),
            if pct(d) > 1.0 { "  ABOVE 1%" } else { "" }
        )
        .unwrap();
    }

    // Spacings. The turn each left and right pair gives, as views-compared.toml gives it for S2.
    out.push_str(
        r#"
# Spacings. Two landmarks that mirror each other across the centre line stand cos(turn) times as
# far apart in the three-quarter view as in the front view, so each pair gives a turn of its own.
# One entry for each left and right pair of kind `node` or `centre` recorded in both views.
# `covered` names the points of a pair that the skull covers.
"#,
    );
    for p in pairs(front, tq) {
        let ratio = p.tq / p.front;
        writeln!(out, "\n[[spacing.pair]]").unwrap();
        writeln!(out, "name = {}", q(&p.name)).unwrap();
        writeln!(out, "front_spacing = {}", num(p.front, 3)).unwrap();
        writeln!(out, "three_quarter_spacing = {}", num(p.tq, 3)).unwrap();
        writeln!(out, "ratio = {}", num(ratio, 4)).unwrap();
        writeln!(out, "turn_degrees = {}", num(deg(ratio), 2)).unwrap();
        if !p.covered.is_empty() {
            let list: Vec<String> = p.covered.iter().map(|c| q(c)).collect();
            writeln!(out, "covered = [{}]", list.join(", ")).unwrap();
        }
        writeln!(
            log,
            "  {:<24} {:>8} {:>8}  ratio {}  turn {:>6}",
            p.name,
            num(p.front, 3),
            num(p.tq, 3),
            num(ratio, 4),
            num(deg(ratio), 2)
        )
        .unwrap();
    }
    (out, log)
}

// ---------------------------------------------------------------- reading the drawings

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Reads one pinned drawing from a folder, and refuses it when its digest is not the
/// recorded one.
fn read_pinned(folder: &Path, pinned: Pinned) -> Result<Doc> {
    if !folder.is_dir() {
        return Err(Error::new(format!(
            "The folder of hand-made drawings is not at {}. The drawings are private and are kept outside the repository; without them nothing can be read, and the committed numbers stand as recorded.",
            folder.display()
        )));
    }
    let path = folder.join(pinned.file);
    let bytes = std::fs::read(&path).map_err(|error| {
        Error::new(format!(
            "The hand-made drawing {} cannot be read: {error}. Nothing was read, and the committed numbers stand as recorded.",
            path.display()
        ))
    })?;
    let found = sha256_hex(&bytes);
    if found != pinned.sha256 {
        return Err(Error::new(format!(
            "{} has the SHA-256 digest {found}, not the recorded {}. It is not the drawing the record was taken from, so nothing was read.",
            path.display(),
            pinned.sha256
        )));
    }
    let text = String::from_utf8(bytes)
        .map_err(|_| Error::new(format!("{} is not text", path.display())))?;
    let drawing =
        drawing::read(&text).map_err(|error| Error::new(format!("{}: {error}", path.display())))?;
    // The arithmetic reads coordinates as written, in a frame every drawing shares.
    assert!(
        drawing.view_box.as_deref() == Some("0 0 1000 1000"),
        "unexpected viewBox"
    );
    Ok(Doc(drawing))
}

/// The records written from the drawings, and what to tell the person who ran the command.
#[derive(Debug, Clone)]
pub struct Recorded {
    /// `hand-made-views.toml`, `views-compared.toml` and `final-drawings.toml`.
    pub files: Vec<File>,
    /// The comparisons in short, for a person at a terminal.
    pub log: String,
}

/// Reads the four pinned drawings, the S2 underdrawings from one folder and the final
/// drawings from another, and writes the three records of them.
pub fn record(underdrawings: &Path, final_drawings: &Path) -> Result<Recorded> {
    let front = build_front(read_pinned(underdrawings, FRONT)?);
    let tq = build_three_quarter(read_pinned(underdrawings, THREE_QUARTER)?);
    let final_front = build_final_front(read_pinned(final_drawings, FINAL_FRONT)?);
    let final_tq = build_final_three_quarter(read_pinned(final_drawings, FINAL_THREE_QUARTER)?);

    // Left and right are the character's own: in every view her left is toward the viewer's right.
    for v in [&front, &tq, &final_front, &final_tq] {
        for lm in &v.lms {
            if lm.name.contains("_right") {
                let left = lm.name.replace("_right", "_left");
                if v.has(&left) {
                    assert!(
                        v.get(&left).x > lm.x,
                        "{} is not to the viewer's left of {left}",
                        lm.name
                    );
                }
            }
        }
    }

    let positions = write_positions(&front, &tq);
    let (compared, mut log) = write_compared(&front, &tq);
    let (final_record, final_log) = write_final(&final_front, &final_tq, &front, &tq);
    log.push_str(&final_log);
    let file = |name: &str, text: String| File {
        name: name.to_string(),
        text,
    };
    Ok(Recorded {
        files: vec![
            file("hand-made-views.toml", positions),
            file("views-compared.toml", compared),
            file("final-drawings.toml", final_record),
        ],
        log,
    })
}

/// Lists a drawing's named groups and shapes with their points: what a reading of a new
/// drawing is written from.
pub fn list(svg_text: &str) -> Result<String> {
    Ok(drawing::read(svg_text)?.describe())
}
