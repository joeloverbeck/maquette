//! The read-back: the oracle of the SVG contract in `docs/foundation/02-first-table.md`.
//!
//! An armature's SVG is opened in VectorCraft, what VectorCraft holds is read back, and that
//! is compared with the view's landmark file. It passes when the import reports no warning;
//! the names read back are the landmark file's names, each once; every anchor of every guide
//! is within 0.001 unit of the landmark file's position, a circle or an ellipse through its
//! four outer points; and every guide is visible, stroked and unfilled.
//!
//! Expected values come from the landmark file, never from the SVG. This is a mechanical
//! validator: it shows that VectorCraft keeps what the landmark file says, not that the
//! landmark file is right.
//!
//! [`Replies`] is what the server said, read live or from a recording; [`check`] is the
//! comparison, which never speaks to the server. So the comparison is tested on recordings
//! where VectorCraft is not installed.

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::{Value, json};

use crate::armature::{self, Drawn, DrawnGuide};
use crate::number::plain;
use crate::vectorcraft::Server;
use crate::{Error, Result};

/// How far an anchor read back may stand from the landmark file's position, in SVG units.
pub const TOLERANCE: f64 = 0.001;

const RECORDING: &str = "A recording of what VectorCraft's headless MCP server replied to `open_file`, `inspect_document` and `document.node` for one SVG, written by maquette. It is not a live run: it shows what that version did with that file when it was recorded.";
const CUT: &str = "The folder of the opened file is cut from every path, so the recording does not say where on whose machine it was made.";

/// What VectorCraft said of one SVG.
#[derive(Debug, Clone, PartialEq)]
pub struct Replies {
    /// The version the server gave of itself.
    pub vectorcraft_version: String,
    /// The SVG's file name.
    pub svg: String,
    /// False when these were read from a recording.
    pub live: bool,
    pub open_file: Value,
    pub inspect_document: Value,
    /// One reply for each layer.
    pub document_node: Vec<Value>,
}

fn missing(reply: &str, what: &str) -> Error {
    Error::new(format!(
        "VectorCraft's reply to `{reply}` has no {what}. This version answers in another shape than the one the SVG contract was observed on, so the read-back cannot be made; the contract's probes have to be rerun."
    ))
}

impl Replies {
    /// Opens an SVG in a running server and reads back what the server holds.
    pub fn read(server: &mut Server, svg: &Path) -> Result<Replies> {
        let folder = svg.parent().unwrap_or(Path::new(""));
        let name = svg
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| Error::new(format!("{} is not a file", svg.display())))?;
        let open_file = server.call("open_file", json!({ "path": svg }))?;
        let replies = Replies {
            vectorcraft_version: server.version().to_string(),
            svg: name.to_string(),
            live: true,
            open_file,
            inspect_document: Value::Null,
            document_node: Vec::new(),
        };
        Ok(replies.again(server)?.without_folder(folder))
    }

    /// Reads the open document's tree and anchors once more, after something was changed in
    /// it. The reply to `open_file` is kept.
    pub fn again(&self, server: &mut Server) -> Result<Replies> {
        let inspect_document = server.call("inspect_document", json!({}))?;
        let layers = inspect_document
            .get("layers")
            .and_then(Value::as_array)
            .ok_or_else(|| missing("inspect_document", "list `layers`"))?;
        let mut document_node = Vec::new();
        for layer in layers {
            let id = layer
                .get("id")
                .and_then(Value::as_i64)
                .ok_or_else(|| missing("inspect_document", "`id` for a layer"))?;
            document_node.push(server.command("document.node", json!({ "id": id }))?);
        }
        Ok(Replies {
            inspect_document,
            document_node,
            ..self.clone()
        })
    }

    /// Cuts the opened file's folder out of every text of the replies.
    fn without_folder(mut self, folder: &Path) -> Replies {
        let Some(folder) = folder.to_str().filter(|folder| !folder.is_empty()) else {
            return self;
        };
        let with_slash = format!("{folder}/");
        fn cut(value: &mut Value, with_slash: &str, folder: &str) {
            match value {
                Value::String(text) if text.contains(folder) => {
                    *text = text.replace(with_slash, "").replace(folder, "");
                }
                Value::Array(items) => {
                    for item in items {
                        cut(item, with_slash, folder);
                    }
                }
                Value::Object(entries) => {
                    for (_, entry) in entries.iter_mut() {
                        cut(entry, with_slash, folder);
                    }
                }
                _ => {}
            }
        }
        cut(&mut self.open_file, &with_slash, folder);
        cut(&mut self.inspect_document, &with_slash, folder);
        for reply in &mut self.document_node {
            cut(reply, &with_slash, folder);
        }
        self
    }

    /// The replies as a recording: JSON, one reply to a line.
    pub fn recording(&self) -> String {
        let nodes: Vec<String> = self.document_node.iter().map(Value::to_string).collect();
        format!(
            "{{\n\"recording\": {},\n\"paths\": {},\n\"vectorcraft_version\": {},\n\"svg\": {},\n\"open_file\": {},\n\"inspect_document\": {},\n\"document_node\": [\n{}\n]\n}}\n",
            Value::from(RECORDING),
            Value::from(CUT),
            Value::from(self.vectorcraft_version.as_str()),
            Value::from(self.svg.as_str()),
            self.open_file,
            self.inspect_document,
            nodes.join(",\n"),
        )
    }

    /// Reads a recording written by [`Replies::recording`].
    pub fn from_recording(text: &str) -> Result<Replies> {
        let value: Value = serde_json::from_str(text)
            .map_err(|error| Error::new(format!("the recording is not valid JSON: {error}")))?;
        let lacks = |key: &str| Error::new(format!("the recording has no `{key}`"));
        if value.get("recording").and_then(Value::as_str).is_none() {
            return Err(Error::new(
                "the file does not say it is a recording: it has no `recording`",
            ));
        }
        let word = |key: &str| -> Result<String> {
            value
                .get(key)
                .and_then(Value::as_str)
                .map(str::to_string)
                .ok_or_else(|| lacks(key))
        };
        Ok(Replies {
            vectorcraft_version: word("vectorcraft_version")?,
            svg: word("svg")?,
            live: false,
            open_file: value
                .get("open_file")
                .cloned()
                .ok_or_else(|| lacks("open_file"))?,
            inspect_document: value
                .get("inspect_document")
                .cloned()
                .ok_or_else(|| lacks("inspect_document"))?,
            document_node: value
                .get("document_node")
                .and_then(Value::as_array)
                .cloned()
                .ok_or_else(|| lacks("document_node"))?,
        })
    }
}

/// The four things the oracle asks, and what stands in the way of asking.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Check {
    /// The import reports no warning.
    Warning,
    /// The names read back are the landmark file's, each once.
    Names,
    /// Every anchor of every guide is where the landmark file says.
    Positions,
    /// Every guide is visible, stroked and unfilled.
    Paint,
}

impl Check {
    fn word(self) -> &'static str {
        match self {
            Check::Warning => "import warning",
            Check::Names => "names",
            Check::Positions => "positions",
            Check::Paint => "paint",
        }
    }
}

/// One way the read-back does not agree with the landmark file.
#[derive(Debug, Clone, PartialEq)]
pub struct Failure {
    pub check: Check,
    /// The guide, group or layer it is about, by the landmark file's name, when it has one.
    pub about: Option<String>,
    pub message: String,
}

/// One object VectorCraft holds, joined from its two replies.
#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub id: i64,
    /// `Layer`, `Group`, `Path` or whatever else VectorCraft calls it.
    pub kind: String,
    /// The name, or `None` for an object VectorCraft holds without one.
    pub name: Option<String>,
    /// What `inspect_document` shows in place of a name.
    pub label: String,
    /// Its own switch. A visible object inside a hidden group is not shown.
    pub visible: bool,
    /// The nearest hidden group or layer above it.
    pub hidden_above: Option<String>,
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<f64>,
    pub closed: Option<bool>,
    /// How deep in the tree it sits: 0 for a layer.
    pub depth: usize,
    /// Each subpath's anchors, and whether it is closed. A guide has one.
    pub subpaths: Vec<(Vec<[f64; 2]>, bool)>,
}

impl Object {
    fn shown_name(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| format!("an unnamed {} ({})", self.kind, self.label))
    }
}

/// What the comparison found, and the read-back as a person reads it.
#[derive(Debug, Clone)]
pub struct Outcome {
    pub failures: Vec<Failure>,
    /// Every object read back, in drawing order.
    pub objects: Vec<Object>,
    pub text: String,
}

impl Outcome {
    pub fn passed(&self) -> bool {
        self.failures.is_empty()
    }

    /// Whether one of the four checks found nothing wrong.
    pub fn check_passed(&self, check: Check) -> bool {
        !self.failures.iter().any(|failure| failure.check == check)
    }
}

struct Node {
    name: Option<String>,
    subpaths: Vec<(Vec<[f64; 2]>, bool)>,
}

fn pair(value: Option<&Value>) -> Option<[f64; 2]> {
    let list = value?.as_array()?;
    Some([list.first()?.as_f64()?, list.get(1)?.as_f64()?])
}

/// Gathers every object of a `document.node` reply by its id: its name and its anchors.
fn gather_nodes(node: &Value, into: &mut BTreeMap<i64, Node>) -> Result<()> {
    let id = node
        .get("id")
        .and_then(Value::as_i64)
        .ok_or_else(|| missing("document.node", "`id` for an object"))?;
    let kind = node
        .get("kind")
        .ok_or_else(|| missing("document.node", "`kind` for an object"))?;
    let mut subpaths = Vec::new();
    if let Some(path) = kind.get("path") {
        let listed = path
            .get("subpaths")
            .and_then(Value::as_array)
            .ok_or_else(|| missing("document.node", "list `subpaths` for a path"))?;
        for subpath in listed {
            let anchors = subpath
                .get("anchors")
                .and_then(Value::as_array)
                .ok_or_else(|| missing("document.node", "list `anchors` for a subpath"))?
                .iter()
                .map(|anchor| pair(anchor.get("p")))
                .collect::<Option<Vec<[f64; 2]>>>()
                .ok_or_else(|| missing("document.node", "position `p` for an anchor"))?;
            let closed = subpath.get("closed").and_then(Value::as_bool) == Some(true);
            subpaths.push((anchors, closed));
        }
    }
    into.insert(
        id,
        Node {
            name: node.get("name").and_then(Value::as_str).map(str::to_string),
            subpaths,
        },
    );
    if let Some(children) = kind.get("children").and_then(Value::as_array) {
        for child in children {
            gather_nodes(child, into)?;
        }
    }
    Ok(())
}

/// Walks `inspect_document`'s tree in drawing order, joining each object with its node.
fn gather_objects(
    entry: &Value,
    above: Option<&Object>,
    nodes: &mut BTreeMap<i64, Node>,
    into: &mut Vec<Object>,
) -> Result<()> {
    let id = entry
        .get("id")
        .and_then(Value::as_i64)
        .ok_or_else(|| missing("inspect_document", "`id` for an object"))?;
    let word = |key: &str| entry.get(key).and_then(Value::as_str).map(str::to_string);
    let node = nodes.remove(&id).ok_or_else(|| {
        Error::new(format!(
            "VectorCraft's `inspect_document` shows an object {id} that `document.node` does not give"
        ))
    })?;
    let object = Object {
        id,
        kind: word("kind").ok_or_else(|| missing("inspect_document", "`kind` for an object"))?,
        name: node.name,
        label: word("name").unwrap_or_default(),
        visible: entry
            .get("visible")
            .and_then(Value::as_bool)
            .ok_or_else(|| missing("inspect_document", "`visible` for an object"))?,
        hidden_above: above.and_then(|above| {
            if above.visible {
                above.hidden_above.clone()
            } else {
                Some(above.shown_name())
            }
        }),
        fill: word("fill"),
        stroke: word("stroke"),
        stroke_width: entry.get("strokeWidth").and_then(Value::as_f64),
        closed: entry.get("closed").and_then(Value::as_bool),
        depth: above.map_or(0, |above| above.depth + 1),
        subpaths: node.subpaths,
    };
    into.push(object.clone());
    if let Some(children) = entry.get("children").and_then(Value::as_array) {
        // The tree lists the topmost first; drawing order is the other way round.
        for child in children.iter().rev() {
            gather_objects(child, Some(&object), nodes, into)?;
        }
    }
    Ok(())
}

/// Every object VectorCraft holds, in drawing order.
pub fn objects(replies: &Replies) -> Result<Vec<Object>> {
    let mut nodes = BTreeMap::new();
    for reply in &replies.document_node {
        gather_nodes(reply, &mut nodes)?;
    }
    let layers = replies
        .inspect_document
        .get("layers")
        .and_then(Value::as_array)
        .ok_or_else(|| missing("inspect_document", "list `layers`"))?;
    let mut found = Vec::new();
    for layer in layers.iter().rev() {
        gather_objects(layer, None, &mut nodes, &mut found)?;
    }
    Ok(found)
}

fn distance(first: [f64; 2], second: [f64; 2]) -> f64 {
    (first[0] - second[0]).hypot(first[1] - second[1])
}

fn at(position: [f64; 2]) -> String {
    format!("{}, {}", plain(position[0], 6), plain(position[1], 6))
}

/// What the positions of one guide came to.
struct Compared {
    anchors: usize,
    largest: f64,
    largest_at: String,
}

fn compare_positions(
    guide: &DrawnGuide,
    object: &Object,
    failures: &mut Vec<Failure>,
) -> Option<Compared> {
    let mut fail = |message: String| {
        failures.push(Failure {
            check: Check::Positions,
            about: Some(guide.name.clone()),
            message,
        });
    };
    let [(read, _)] = object.subpaths.as_slice() else {
        fail(format!(
            "`{}` is read back as {} pieces, not as one path whose anchors can be read",
            guide.name,
            object.subpaths.len()
        ));
        return None;
    };
    if read.len() != guide.anchors.len() {
        fail(format!(
            "`{}` is read back with {} anchors, and the landmark file gives it {}",
            guide.name,
            read.len(),
            guide.anchors.len()
        ));
        return None;
    }
    let mut compared = Compared {
        anchors: read.len(),
        largest: 0.0,
        largest_at: String::new(),
    };
    for ((name, expected), read) in guide.anchors.iter().zip(read) {
        let off = distance(*expected, *read);
        if off > compared.largest || compared.largest_at.is_empty() {
            compared.largest = off;
            compared.largest_at = name.clone();
        }
        if off > TOLERANCE {
            fail(format!(
                "`{}`: its anchor `{name}` is read back at {}, which is {} from the landmark file's {}",
                guide.name,
                at(*read),
                plain(off, 6),
                at(*expected)
            ));
        }
    }
    // A circle or an ellipse has no anchor at its centre: the centre and the radii the
    // landmark file gives are checked through the four outer points, read back as right,
    // bottom, left and top.
    if let (Some(round), [right, bottom, left, top]) = (guide.round, read.as_slice()) {
        let checks = [
            ("centre x, from the top anchor", round.centre[0], top[0]),
            (
                "centre x, from the bottom anchor",
                round.centre[0],
                bottom[0],
            ),
            ("centre y, from the right anchor", round.centre[1], right[1]),
            ("centre y, from the left anchor", round.centre[1], left[1]),
            (
                "sideways radius, half of right to left",
                round.rx,
                (right[0] - left[0]) / 2.0,
            ),
            (
                "upright radius, half of top to bottom",
                round.ry,
                (bottom[1] - top[1]) / 2.0,
            ),
        ];
        for (what, expected, read) in checks {
            let off = (expected - read).abs();
            if off > TOLERANCE {
                fail(format!(
                    "`{}`: its {what}, is read back as {}, which is {} from the landmark file's {}",
                    guide.name,
                    plain(read, 6),
                    plain(off, 6),
                    plain(expected, 6)
                ));
            }
        }
    }
    Some(compared)
}

fn compare_paint(guide: &DrawnGuide, object: &Object, failures: &mut Vec<Failure>) {
    let mut fail = |message: String| {
        failures.push(Failure {
            check: Check::Paint,
            about: Some(guide.name.clone()),
            message,
        });
    };
    if !object.visible {
        fail(format!("`{}` is hidden", guide.name));
    }
    if let Some(above) = &object.hidden_above {
        fail(format!(
            "`{}` is not shown, because `{above}` above it is hidden",
            guide.name
        ));
    }
    let stroked = object
        .stroke
        .as_deref()
        .is_some_and(|stroke| stroke != "None")
        && object.stroke_width.is_some_and(|width| width > 0.0);
    if !stroked {
        fail(format!(
            "`{}` has no stroke: stroke {}, width {}",
            guide.name,
            object.stroke.as_deref().unwrap_or("not given"),
            object
                .stroke_width
                .map_or("not given".to_string(), |width| plain(width, 3))
        ));
    }
    if object.fill.as_deref() != Some("None") {
        fail(format!(
            "`{}` is filled: fill {}",
            guide.name,
            object.fill.as_deref().unwrap_or("not given")
        ));
    }
}

fn verdict(failures: &[Failure], check: Check) -> &'static str {
    if failures.iter().any(|failure| failure.check == check) {
        "FAIL"
    } else {
        "pass"
    }
}

/// Compares what VectorCraft said with a landmark file.
///
/// `landmark_path` only names the landmark file in the report.
pub fn check(landmark_path: &str, landmark_text: &str, replies: &Replies) -> Result<Outcome> {
    let drawn = armature::read_drawn(landmark_text)
        .map_err(|error| Error::new(format!("{landmark_path}: {error}")))?;
    let objects = objects(replies)?;
    let mut failures: Vec<Failure> = Vec::new();

    // 1. The import reports no warning.
    let warnings = replies
        .open_file
        .get("warnings")
        .and_then(Value::as_array)
        .ok_or_else(|| missing("open_file", "list `warnings`"))?;
    for warning in warnings {
        failures.push(Failure {
            check: Check::Warning,
            about: None,
            message: format!("the import warns: {warning}"),
        });
    }

    // 2. The names read back are the landmark file's, each once: the layer as a layer, a
    // group as a group and a guide as a path. Where in the tree a name sits is shown in the
    // read-back and is not one of the oracle's conditions.
    let mut expected: Vec<(&str, &str)> = vec![(drawn.layer.as_str(), "Layer")];
    for group in &drawn.groups {
        expected.push((group, "Group"));
    }
    for guide in &drawn.guides {
        expected.push((&guide.name, "Path"));
    }
    let mut names = |about: Option<&str>, message: String| {
        failures.push(Failure {
            check: Check::Names,
            about: about.map(str::to_string),
            message,
        });
    };
    let mut found: BTreeMap<&str, &Object> = BTreeMap::new();
    for (name, kind) in &expected {
        let read: Vec<&Object> = objects
            .iter()
            .filter(|object| object.name.as_deref() == Some(name))
            .collect();
        match read.as_slice() {
            [] => names(
                Some(name),
                format!("`{name}` is in the landmark file and is not read back"),
            ),
            [only] => {
                if only.kind != *kind {
                    names(
                        Some(name),
                        format!(
                            "`{name}` is read back as a {}, and the landmark file gives a {kind}",
                            only.kind
                        ),
                    );
                } else {
                    found.insert(name, only);
                }
            }
            several => names(
                Some(name),
                format!(
                    "`{name}` is read back {} times, and a name stands once",
                    several.len()
                ),
            ),
        }
    }
    for object in &objects {
        match &object.name {
            None => names(
                None,
                format!(
                    "an object without a name is read back: a {} that VectorCraft shows as {}",
                    object.kind, object.label
                ),
            ),
            Some(name) if !expected.iter().any(|(expected, _)| expected == name) => names(
                None,
                format!(
                    "`{name}` is read back, a {}, and is not in the landmark file",
                    object.kind
                ),
            ),
            Some(_) => {}
        }
    }

    // 3. and 4. Positions and paint, for every guide that was read back under its name.
    let mut compared: BTreeMap<&str, Compared> = BTreeMap::new();
    for guide in &drawn.guides {
        let Some(object) = found.get(guide.name.as_str()) else {
            continue;
        };
        if let Some(result) = compare_positions(guide, object, &mut failures) {
            compared.insert(&guide.name, result);
        }
        compare_paint(guide, object, &mut failures);
    }

    let text = report(
        landmark_path,
        &drawn,
        replies,
        &objects,
        &compared,
        &failures,
    );
    Ok(Outcome {
        failures,
        objects,
        text,
    })
}

fn report(
    landmark_path: &str,
    drawn: &Drawn,
    replies: &Replies,
    objects: &[Object],
    compared: &BTreeMap<&str, Compared>,
    failures: &[Failure],
) -> String {
    let mut out = String::new();
    let how = if replies.live {
        "run now"
    } else {
        "from a recording, not a live run"
    };
    out.push_str(&format!(
        "Read-back of {} in VectorCraft {} (headless MCP server), {how}\n",
        replies.svg, replies.vectorcraft_version
    ));
    out.push_str(&format!(
        "Compared with {landmark_path}, the landmark file of the {} view. Expected values come from it, never from the SVG.\n\n",
        drawn.view
    ));

    let warnings = failures
        .iter()
        .filter(|failure| failure.check == Check::Warning)
        .count();
    out.push_str(&format!(
        "1. Import warning: {}. {}\n",
        if warnings == 0 {
            "none".to_string()
        } else {
            warnings.to_string()
        },
        verdict(failures, Check::Warning)
    ));
    out.push_str(&format!(
        "2. Names: {} objects read back. The landmark file gives {} names: 1 layer, {} groups and {} guides, each to be read back once. {}\n",
        objects.len(),
        1 + drawn.groups.len() + drawn.guides.len(),
        drawn.groups.len(),
        drawn.guides.len(),
        verdict(failures, Check::Names)
    ));
    let anchors: usize = compared.values().map(|result| result.anchors).sum();
    let largest = compared
        .iter()
        .max_by(|first, second| first.1.largest.total_cmp(&second.1.largest));
    out.push_str(&format!(
        "3. Positions: {anchors} anchors of {} guides compared, each to be within {} unit{}. {}\n",
        compared.len(),
        plain(TOLERANCE, 3),
        largest.map_or(String::new(), |(guide, result)| format!(
            "; the largest difference is {}, at `{}` of `{guide}`",
            plain(result.largest, 6),
            result.largest_at
        )),
        verdict(failures, Check::Positions)
    ));
    let closed = drawn.guides.iter().filter(|guide| guide.closed).count();
    out.push_str(&format!(
        "4. Paint: every guide to be visible, stroked and unfilled, the {closed} closed ones included. {}\n",
        verdict(failures, Check::Paint)
    ));

    let rounds = drawn
        .guides
        .iter()
        .filter(|guide| guide.round.is_some())
        .count();
    out.push_str(&format!(
        "\nWhat the positions cover: every anchor of every guide, and the centre and radii of the {rounds} circles and ellipses through their four outer points. The handles between anchors are not in the landmark file and are not compared.\n"
    ));
    if !drawn.not_drawn.is_empty() {
        let names: Vec<String> = drawn
            .not_drawn
            .iter()
            .map(|name| format!("`{name}`"))
            .collect();
        out.push_str(&format!(
            "Not compared: {} positions of the landmark file are neither an anchor nor a centre of any guide, so the SVG does not carry them and VectorCraft holds nothing to set against them: {}.\n",
            names.len(),
            names.join(", ")
        ));
    }

    out.push_str(
        "\nWhat was read back, in drawing order. Each anchor is x,y as VectorCraft gave it, to six decimals.\n",
    );
    for object in objects {
        let indent = "  ".repeat(object.depth);
        let mut line = format!(
            "{indent}{} [{}",
            object.name.as_deref().unwrap_or(&object.label),
            object.kind
        );
        if object.name.is_none() {
            line.push_str(", unnamed");
        }
        if !object.subpaths.is_empty() {
            let closed = object.subpaths.iter().all(|(_, closed)| *closed);
            line.push_str(if closed { ", closed" } else { ", open" });
        }
        line.push_str(if object.visible {
            "] visible"
        } else {
            "] HIDDEN"
        });
        if object.kind == "Path" {
            line.push_str(&format!(
                " stroke={} width={} fill={}",
                object.stroke.as_deref().unwrap_or("?"),
                object
                    .stroke_width
                    .map_or("?".to_string(), |width| plain(width, 3)),
                object.fill.as_deref().unwrap_or("?")
            ));
        }
        if let Some(result) = object.name.as_deref().and_then(|name| compared.get(name)) {
            line.push_str(&format!(
                "; {} anchors, largest difference {}",
                result.anchors,
                plain(result.largest, 6)
            ));
        }
        out.push_str(&line);
        out.push('\n');
        for (anchors, _) in &object.subpaths {
            let listed: Vec<String> = anchors
                .iter()
                .map(|anchor| format!("{},{}", plain(anchor[0], 6), plain(anchor[1], 6)))
                .collect();
            out.push_str(&format!("{indent}    {}\n", listed.join(" | ")));
        }
    }

    out.push('\n');
    if failures.is_empty() {
        out.push_str(&format!(
            "PASSED: VectorCraft {} keeps every name, position and paint the landmark file gives. That does not say the landmark file is right.\n",
            replies.vectorcraft_version
        ));
    } else {
        out.push_str(&format!(
            "FAILED: {} in VectorCraft {}. The armature is neither put to Jon nor exported; nothing is renamed, loosened or passed with a note.\n",
            if failures.len() == 1 {
                "1 failure".to_string()
            } else {
                format!("{} failures", failures.len())
            },
            replies.vectorcraft_version
        ));
        for failure in failures {
            out.push_str(&format!(
                "  - {}: {}\n",
                failure.check.word(),
                failure.message
            ));
        }
    }
    out
}

/// The SVG a landmark file names, which stands beside it.
pub fn svg_beside(landmark_path: &Path, landmark_text: &str) -> Result<std::path::PathBuf> {
    let drawn = armature::read_drawn(landmark_text)
        .map_err(|error| Error::new(format!("{}: {error}", landmark_path.display())))?;
    Ok(landmark_path
        .parent()
        .unwrap_or(Path::new(""))
        .join(drawn.svg))
}
