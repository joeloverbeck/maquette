//! Reads a hand-made drawing: the named groups and shapes of its SVG, with their points, and
//! the curve arithmetic that takes a position from a shape.
//!
//! A drawing is read as written. Its coordinates are the file's own, so a file that moves
//! them with a transform is refused, and so is a path command other than the absolute `M`,
//! `L`, `C` and `Z`: a position taken from a curve this module does not follow would be a
//! guess. Nothing here looks at a rendered picture.
//!
//! [`Drawing::describe`] lists what a drawing holds, for a person or an agent who has to say
//! which node of which shape is which landmark. [`crate::hand_made_drawings`] is the one place
//! that says so for the drawings maquette relies on.

use crate::number::plain;
use crate::{Error, Result};

/// A point of a drawing, in the SVG's own units: x toward the viewer's right, y downward.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct P {
    pub x: f64,
    pub y: f64,
}

/// A direction a curve's extreme is looked for along.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    pub fn letter(self) -> char {
        match self {
            Axis::X => 'x',
            Axis::Y => 'y',
        }
    }

    pub fn of(self, point: P) -> f64 {
        match self {
            Axis::X => point.x,
            Axis::Y => point.y,
        }
    }
}

/// One stretch of a path, between two of its nodes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Seg {
    Line(P, P),
    Cubic(P, P, P, P),
}

impl Seg {
    /// The point at a parameter from 0 to 1.
    pub fn at(&self, t: f64) -> P {
        match *self {
            Seg::Line(a, b) => P {
                x: a.x + (b.x - a.x) * t,
                y: a.y + (b.y - a.y) * t,
            },
            Seg::Cubic(p0, p1, p2, p3) => {
                let u = 1.0 - t;
                let (b0, b1, b2, b3) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
                P {
                    x: b0 * p0.x + b1 * p1.x + b2 * p2.x + b3 * p3.x,
                    y: b0 * p0.y + b1 * p1.y + b2 * p2.y + b3 * p3.y,
                }
            }
        }
    }

    /// The parameters strictly inside 0 to 1 where the curve neither rises nor falls along
    /// one axis: where its derivative there is zero.
    pub fn stationary(&self, axis: Axis) -> Vec<f64> {
        let Seg::Cubic(p0, p1, p2, p3) = *self else {
            return Vec::new();
        };
        let c = [axis.of(p0), axis.of(p1), axis.of(p2), axis.of(p3)];
        let (d0, d1, d2) = (c[1] - c[0], c[2] - c[1], c[3] - c[2]);
        // B'(t)/3 = (d0 - 2 d1 + d2) t^2 + 2 (d1 - d0) t + d0
        let (a, b, cc) = (d0 - 2.0 * d1 + d2, 2.0 * (d1 - d0), d0);
        let mut roots = Vec::new();
        if a.abs() < 1e-12 {
            if b.abs() > 1e-12 {
                roots.push(-cc / b);
            }
        } else {
            let disc = b * b - 4.0 * a * cc;
            if disc >= 0.0 {
                let s = disc.sqrt();
                roots.push((-b - s) / (2.0 * a));
                roots.push((-b + s) / (2.0 * a));
            }
        }
        roots.retain(|t| *t > 1e-9 && *t < 1.0 - 1e-9);
        roots.sort_by(f64::total_cmp);
        roots
    }

    /// The parameters from 0 to 1 where `f` of the point changes sign or is zero.
    pub fn roots(&self, f: &dyn Fn(P) -> f64) -> Vec<f64> {
        const N: usize = 4096;
        let mut out: Vec<f64> = Vec::new();
        let mut push = |t: f64| {
            if !out.iter().any(|o| (o - t).abs() < 1e-7) {
                out.push(t);
            }
        };
        let mut prev_t = 0.0;
        let mut prev_v = f(self.at(0.0));
        if prev_v == 0.0 {
            push(0.0);
        }
        for i in 1..=N {
            let t = i as f64 / N as f64;
            let v = f(self.at(t));
            if v == 0.0 {
                push(t);
            } else if prev_v != 0.0 && (prev_v < 0.0) != (v < 0.0) {
                let (mut lo, mut hi, mut vlo) = (prev_t, t, prev_v);
                for _ in 0..80 {
                    let mid = 0.5 * (lo + hi);
                    let vm = f(self.at(mid));
                    if (vm < 0.0) == (vlo < 0.0) {
                        lo = mid;
                        vlo = vm;
                    } else {
                        hi = mid;
                    }
                }
                push(0.5 * (lo + hi));
            }
            prev_t = t;
            prev_v = v;
        }
        out
    }
}

/// One subpath: its nodes in path order, and the stretch from each to the next.
#[derive(Debug, Clone, PartialEq)]
pub struct Sub {
    pub nodes: Vec<P>,
    /// `segs[k]` runs from `nodes[k]` to `nodes[k + 1]`.
    pub segs: Vec<Seg>,
    pub closed: bool,
}

impl Sub {
    /// Segment `k`. For a closed subpath, `k == segs.len()` is the closing edge.
    ///
    /// # Panics
    ///
    /// If the subpath has no such segment, which is a fault in the program.
    pub fn seg(&self, k: usize) -> Seg {
        if k < self.segs.len() {
            self.segs[k]
        } else if k == self.segs.len() && self.closed {
            Seg::Line(self.nodes[self.nodes.len() - 1], self.nodes[0])
        } else {
            panic!("the subpath has no segment {k}")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    /// A `path`, a `line`, a `polyline` or a `polygon`: nodes joined by stretches.
    Path(Vec<Sub>),
    Circle {
        c: P,
        r: f64,
    },
    Ellipse {
        c: P,
        rx: f64,
        ry: f64,
    },
    Rect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
}

/// One shape of a drawing.
#[derive(Debug, Clone, PartialEq)]
pub struct Elem {
    /// The id of the enclosing group, a slash, and the shape's name: its `data-name`, or its
    /// `id` where it has none.
    pub addr: String,
    /// The SVG element it is written as.
    pub element: String,
    pub shape: Shape,
    pub filled: bool,
    /// The file gives it an opacity of 0, so it is drawn and cannot be seen.
    pub hidden: bool,
    /// Paint order: a later shape covers an earlier one.
    pub order: usize,
    /// How many groups enclose it.
    depth: usize,
}

/// A group or a shape, in the order the file gives them.
#[derive(Debug, Clone, PartialEq)]
enum Entry {
    Group { name: String, depth: usize },
    Shape(usize),
}

/// A drawing, read.
#[derive(Debug, Clone, PartialEq)]
pub struct Drawing {
    /// The `viewBox` as the file writes it, when it has one.
    pub view_box: Option<String>,
    pub elems: Vec<Elem>,
    entries: Vec<Entry>,
}

fn number(node: roxmltree::Node, name: &str) -> Result<f64> {
    let label = node.attribute("id").unwrap_or("without an id");
    let text = node.attribute(name).ok_or_else(|| {
        Error::new(format!(
            "the <{}> {label} has no `{name}`",
            node.tag_name().name()
        ))
    })?;
    text.trim().parse().map_err(|_| {
        Error::new(format!(
            "the <{}> {label} has `{name}=\"{text}\"`, which is not a number",
            node.tag_name().name()
        ))
    })
}

fn straight(nodes: Vec<P>, closed: bool) -> Sub {
    let segs = nodes
        .windows(2)
        .map(|pair| Seg::Line(pair[0], pair[1]))
        .collect();
    Sub {
        nodes,
        segs,
        closed,
    }
}

fn parse_points(list: &str) -> Result<Vec<P>> {
    let numbers: Vec<f64> = list
        .split(|character: char| character.is_whitespace() || character == ',')
        .filter(|piece| !piece.is_empty())
        .map(|piece| {
            piece
                .parse()
                .map_err(|_| Error::new(format!("`{piece}` in a list of points is not a number")))
        })
        .collect::<Result<_>>()?;
    if !numbers.len().is_multiple_of(2) || numbers.is_empty() {
        return Err(Error::new(
            "a list of points does not hold pairs of numbers",
        ));
    }
    Ok(numbers
        .chunks(2)
        .map(|pair| P {
            x: pair[0],
            y: pair[1],
        })
        .collect())
}

fn parse_path(d: &str) -> Result<Vec<Sub>> {
    enum Tok {
        Cmd(char),
        Num(f64),
    }
    let mut toks = Vec::new();
    let mut cur = String::new();
    let flush = |cur: &mut String, toks: &mut Vec<Tok>| -> Result<()> {
        if !cur.is_empty() {
            let value = cur
                .parse()
                .map_err(|_| Error::new(format!("`{cur}` in a path is not a number")))?;
            toks.push(Tok::Num(value));
            cur.clear();
        }
        Ok(())
    };
    for ch in d.chars() {
        if ch.is_ascii_alphabetic() {
            flush(&mut cur, &mut toks)?;
            toks.push(Tok::Cmd(ch));
        } else if ch.is_whitespace() || ch == ',' {
            flush(&mut cur, &mut toks)?;
        } else if ch == '-' {
            flush(&mut cur, &mut toks)?;
            cur.push(ch);
        } else {
            cur.push(ch);
        }
    }
    flush(&mut cur, &mut toks)?;

    let mut subs: Vec<Sub> = Vec::new();
    let mut i = 0;
    let mut cmd = ' ';
    let take = |i: &mut usize, toks: &[Tok]| -> Result<P> {
        match (toks.get(*i), toks.get(*i + 1)) {
            (Some(Tok::Num(x)), Some(Tok::Num(y))) => {
                *i += 2;
                Ok(P { x: *x, y: *y })
            }
            _ => Err(Error::new("a path command lacks a pair of coordinates")),
        }
    };
    let no_start = || Error::new("a path draws before its first `M`");
    while i < toks.len() {
        if let Tok::Cmd(c) = toks[i] {
            if !matches!(c, 'M' | 'L' | 'C' | 'Z') {
                return Err(Error::new(format!(
                    "the path command `{c}` is not read: only the absolute `M`, `L`, `C` and `Z` are, so that every position is one the file writes"
                )));
            }
            cmd = c;
            i += 1;
            if c == 'Z' {
                subs.last_mut().ok_or_else(no_start)?.closed = true;
            }
            continue;
        }
        match cmd {
            'M' => {
                let p = take(&mut i, &toks)?;
                subs.push(Sub {
                    nodes: vec![p],
                    segs: Vec::new(),
                    closed: false,
                });
                cmd = 'L';
            }
            'L' => {
                let p = take(&mut i, &toks)?;
                let sub = subs.last_mut().ok_or_else(no_start)?;
                let a = sub.nodes[sub.nodes.len() - 1];
                sub.segs.push(Seg::Line(a, p));
                sub.nodes.push(p);
            }
            'C' => {
                let p1 = take(&mut i, &toks)?;
                let p2 = take(&mut i, &toks)?;
                let p3 = take(&mut i, &toks)?;
                let sub = subs.last_mut().ok_or_else(no_start)?;
                let a = sub.nodes[sub.nodes.len() - 1];
                sub.segs.push(Seg::Cubic(a, p1, p2, p3));
                sub.nodes.push(p3);
            }
            other => {
                return Err(Error::new(format!(
                    "a path has numbers after its command `{other}`"
                )));
            }
        }
    }
    Ok(subs)
}

/// Reads the text of an SVG drawing.
pub fn read(svg: &str) -> Result<Drawing> {
    let document = roxmltree::Document::parse(svg).map_err(|error| {
        Error::new(format!(
            "the drawing is not an SVG that can be read: {error}"
        ))
    })?;
    let root = document.root_element();
    if root.tag_name().name() != "svg" {
        return Err(Error::new("the drawing's outermost element is not <svg>"));
    }
    let mut drawing = Drawing {
        view_box: root.attribute("viewBox").map(str::to_string),
        elems: Vec::new(),
        entries: Vec::new(),
    };
    let mut groups: Vec<String> = Vec::new();
    walk(root, &mut groups, &mut drawing)?;
    Ok(drawing)
}

fn walk(parent: roxmltree::Node, groups: &mut Vec<String>, drawing: &mut Drawing) -> Result<()> {
    for node in parent.children().filter(roxmltree::Node::is_element) {
        let tag = node.tag_name().name();
        if node.has_attribute("transform") {
            return Err(Error::new(format!(
                "the <{tag}> {} carries a transform, so its coordinates are not the ones the file writes. A drawing is read as written, and this one would need mapping first.",
                node.attribute("id").unwrap_or("without an id")
            )));
        }
        let shape = match tag {
            "title" | "desc" | "defs" | "metadata" => continue,
            "g" => {
                let name = node.attribute("id").unwrap_or("").to_string();
                drawing.entries.push(Entry::Group {
                    name: name.clone(),
                    depth: groups.len(),
                });
                groups.push(name);
                walk(node, groups, drawing)?;
                groups.pop();
                continue;
            }
            "path" => {
                let d = node.attribute("d").ok_or_else(|| {
                    Error::new(format!(
                        "the <path> {} has no `d`",
                        node.attribute("id").unwrap_or("without an id")
                    ))
                })?;
                Shape::Path(parse_path(d).map_err(|error| {
                    Error::new(format!(
                        "the <path> {}: {error}",
                        node.attribute("id").unwrap_or("without an id")
                    ))
                })?)
            }
            "line" => {
                let a = P {
                    x: number(node, "x1")?,
                    y: number(node, "y1")?,
                };
                let b = P {
                    x: number(node, "x2")?,
                    y: number(node, "y2")?,
                };
                Shape::Path(vec![straight(vec![a, b], false)])
            }
            "polyline" | "polygon" => {
                let points = parse_points(node.attribute("points").unwrap_or(""))?;
                Shape::Path(vec![straight(points, tag == "polygon")])
            }
            "circle" => Shape::Circle {
                c: P {
                    x: number(node, "cx")?,
                    y: number(node, "cy")?,
                },
                r: number(node, "r")?,
            },
            "ellipse" => Shape::Ellipse {
                c: P {
                    x: number(node, "cx")?,
                    y: number(node, "cy")?,
                },
                rx: number(node, "rx")?,
                ry: number(node, "ry")?,
            },
            "rect" => Shape::Rect {
                x: number(node, "x")?,
                y: number(node, "y")?,
                width: number(node, "width")?,
                height: number(node, "height")?,
            },
            other => {
                return Err(Error::new(format!(
                    "the element <{other}> is not read: a drawing is groups of path, line, polyline, polygon, circle, ellipse and rect"
                )));
            }
        };
        let label = node
            .attribute("data-name")
            .or_else(|| node.attribute("id"))
            .unwrap_or("");
        let index = drawing.elems.len();
        drawing.elems.push(Elem {
            addr: format!("{}/{label}", groups.last().map_or("", String::as_str)),
            element: tag.to_string(),
            shape,
            filled: tag != "line" && node.attribute("fill") != Some("none"),
            hidden: node.attribute("opacity") == Some("0"),
            order: index,
            depth: groups.len(),
        });
        drawing.entries.push(Entry::Shape(index));
    }
    Ok(())
}

/// The least and the greatest value a shape's outline reaches along an axis: the best of its
/// nodes and of the points where a curved stretch is stationary.
fn extent(subs: &[Sub], axis: Axis) -> (f64, f64) {
    let mut values: Vec<f64> = Vec::new();
    for sub in subs {
        values.extend(sub.nodes.iter().map(|node| axis.of(*node)));
        for seg in &sub.segs {
            values.extend(seg.stationary(axis).into_iter().map(|t| axis.of(seg.at(t))));
        }
    }
    values
        .into_iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), value| {
            (low.min(value), high.max(value))
        })
}

impl Drawing {
    /// The shapes under an address: one, unless the drawing names two things alike.
    pub fn find(&self, addr: &str) -> Vec<&Elem> {
        self.elems.iter().filter(|elem| elem.addr == addr).collect()
    }

    /// The drawing as a list a person reads: every group, and every shape with its address,
    /// its nodes numbered in path order and how far it reaches.
    pub fn describe(&self) -> String {
        let number = |value: f64| plain(value, 3);
        let point = |p: P| format!("{}, {}", number(p.x), number(p.y));
        let mut out = String::new();
        out.push_str(&format!(
            "viewBox: {}\n",
            self.view_box.as_deref().unwrap_or("none")
        ));
        out.push_str(
            "Each shape is listed under its address: the id of its group, a slash, and its name.\n",
        );
        out.push_str(
            "Nodes are numbered from 0 in path order; segment k runs from node k to node k + 1.\n\n",
        );
        for entry in &self.entries {
            match entry {
                Entry::Group { name, depth } => {
                    out.push_str(&format!("{}group {name}\n", "  ".repeat(*depth)));
                }
                Entry::Shape(index) => {
                    let elem = &self.elems[*index];
                    let indent = "  ".repeat(elem.depth);
                    let mut notes = vec![if elem.filled { "filled" } else { "not filled" }];
                    if elem.hidden {
                        notes.push("opacity 0");
                    }
                    out.push_str(&format!(
                        "{indent}{} {}  ({})\n",
                        elem.element,
                        elem.addr,
                        notes.join(", ")
                    ));
                    match &elem.shape {
                        Shape::Circle { c, r } => out.push_str(&format!(
                            "{indent}    centre {}, radius {}\n",
                            point(*c),
                            number(*r)
                        )),
                        Shape::Ellipse { c, rx, ry } => out.push_str(&format!(
                            "{indent}    centre {}, radii {} and {}\n",
                            point(*c),
                            number(*rx),
                            number(*ry)
                        )),
                        Shape::Rect {
                            x,
                            y,
                            width,
                            height,
                        } => out.push_str(&format!(
                            "{indent}    from {}, {} wide and {} high\n",
                            point(P { x: *x, y: *y }),
                            number(*width),
                            number(*height)
                        )),
                        Shape::Path(subs) => {
                            for (s, sub) in subs.iter().enumerate() {
                                if subs.len() > 1 {
                                    out.push_str(&format!("{indent}    subpath {s}\n"));
                                }
                                for (k, node) in sub.nodes.iter().enumerate() {
                                    let reached = match k
                                        .checked_sub(1)
                                        .map(|before| sub.segs[before])
                                    {
                                        None => "",
                                        Some(Seg::Line(..)) => "  (straight from the node before)",
                                        Some(Seg::Cubic(..)) => "  (curved from the node before)",
                                    };
                                    out.push_str(&format!(
                                        "{indent}    node {k}: {}{reached}\n",
                                        point(*node)
                                    ));
                                }
                                if sub.closed {
                                    out.push_str(&format!("{indent}    closed\n"));
                                }
                            }
                            let (left, right) = extent(subs, Axis::X);
                            let (top, bottom) = extent(subs, Axis::Y);
                            out.push_str(&format!(
                                "{indent}    reaches x {} to {}, y {} to {}\n",
                                number(left),
                                number(right),
                                number(top),
                                number(bottom)
                            ));
                        }
                    }
                }
            }
        }
        out
    }
}

/// Whether a point lies inside a shape's fill and more than 0.02 unit from its edge.
pub fn covers(shape: &Shape, p: P) -> bool {
    const EDGE: f64 = 0.02;
    match shape {
        Shape::Rect { .. } => false,
        Shape::Circle { c, r } => ((p.x - c.x).powi(2) + (p.y - c.y).powi(2)).sqrt() < r - EDGE,
        Shape::Ellipse { c, rx, ry } => {
            let (rx, ry) = (rx - EDGE, ry - EDGE);
            rx > 0.0 && ry > 0.0 && ((p.x - c.x) / rx).powi(2) + ((p.y - c.y) / ry).powi(2) < 1.0
        }
        Shape::Path(subs) => {
            let mut winding = 0i32;
            let mut nearest = f64::MAX;
            for sub in subs {
                let mut poly: Vec<P> = vec![sub.nodes[0]];
                for seg in &sub.segs {
                    for j in 1..=256 {
                        poly.push(seg.at(j as f64 / 256.0));
                    }
                }
                for j in 0..poly.len() {
                    let (a, b) = (poly[j], poly[(j + 1) % poly.len()]);
                    let (dx, dy) = (b.x - a.x, b.y - a.y);
                    let len2 = dx * dx + dy * dy;
                    let t = if len2 == 0.0 {
                        0.0
                    } else {
                        (((p.x - a.x) * dx + (p.y - a.y) * dy) / len2).clamp(0.0, 1.0)
                    };
                    let d = ((p.x - a.x - t * dx).powi(2) + (p.y - a.y - t * dy).powi(2)).sqrt();
                    nearest = nearest.min(d);
                    let cross = dx * (p.y - a.y) - dy * (p.x - a.x);
                    if a.y <= p.y {
                        if b.y > p.y && cross > 0.0 {
                            winding += 1;
                        }
                    } else if b.y <= p.y && cross < 0.0 {
                        winding -= 1;
                    }
                }
            }
            winding != 0 && nearest > EDGE
        }
    }
}
