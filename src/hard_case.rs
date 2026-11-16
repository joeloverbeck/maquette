//! The hard case of the SVG contract, run on a generated three-quarter view.
//!
//! `docs/foundation/02-first-table.md` lists it: a three-quarter view whose positions are
//! not round numbers; every guide at once, in nested groups, with one name contained in
//! another; left and right pairs whose two members differ in shape and place; closed guides,
//! which must arrive unfilled; after import, one member of a pair hidden by its name, and
//! only it hidden; and two spoiled copies that the oracle must fail, one with a guide given
//! `visibility="hidden"` and one with a pair's names exchanged.
//!
//! The list was fixed before the generator existed, from what the contract's probes showed
//! to be fragile. This module only runs it. What it expects of a view comes from the view's
//! landmark file and from [`INVENTORY`], never from the SVG; the spoiled copies are made in
//! a temporary folder and removed.

use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::armature::{self, Drawn, DrawnGuide};
use crate::read_back::{self, Check, Outcome, Replies, TOLERANCE};
use crate::vectorcraft::Server;
use crate::{Error, Result};

/// Every guide an armature's head carries, as issue #3 and the contract list them: the
/// skull, the face outline, the centre line, the level lines for the eyes, the nose and the
/// mouth, each eye's opening, lid line and iris, the two brow lines, the nose mark, the mouth
/// line, the two ears, the neck, the hair volume and the shoulder line.
pub const INVENTORY: [&str; 22] = [
    "skull",
    "face-outline",
    "centre-line",
    "eye-level-line",
    "nose-level-line",
    "mouth-level-line",
    "eye-opening-left",
    "eye-opening-right",
    "lid-line-left",
    "lid-line-right",
    "iris-left",
    "iris-right",
    "brow-line-left",
    "brow-line-right",
    "nose-mark",
    "mouth-line",
    "ear-left",
    "ear-right",
    "neck-left",
    "neck-right",
    "hair-volume",
    "shoulder-line",
];

/// One line of the hard case: what is asked, what was found, and whether it held.
#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    pub asked: String,
    pub found: String,
    pub held: bool,
}

/// The hard case, run.
#[derive(Debug, Clone)]
pub struct HardCase {
    pub steps: Vec<Step>,
    /// The table, as a person reads it.
    pub text: String,
    /// What VectorCraft replied for the view as generated and for each spoiled copy,
    /// each under the file name a recording of it takes.
    pub replies: Vec<(&'static str, Replies)>,
}

impl HardCase {
    pub fn held(&self) -> bool {
        self.steps.iter().all(|step| step.held)
    }
}

/// A folder for the spoiled copies, removed when the run ends, however it ends.
struct Temporary(PathBuf);

impl Temporary {
    fn new() -> Result<Temporary> {
        let folder =
            std::env::temp_dir().join(format!("maquette-hard-case-{}", std::process::id()));
        if folder.exists() {
            std::fs::remove_dir_all(&folder).map_err(|error| {
                Error::new(format!("cannot clear {}: {error}", folder.display()))
            })?;
        }
        std::fs::create_dir_all(&folder)
            .map_err(|error| Error::new(format!("cannot make {}: {error}", folder.display())))?;
        let folder = folder
            .canonicalize()
            .map_err(|error| Error::new(format!("cannot find the temporary folder: {error}")))?;
        Ok(Temporary(folder))
    }

    /// Writes a copy of the SVG under its own name, in a folder of its own.
    fn copy(&self, kind: &str, name: &str, text: &str) -> Result<PathBuf> {
        let folder = self.0.join(kind);
        let path = folder.join(name);
        std::fs::create_dir_all(&folder)
            .and_then(|()| std::fs::write(&path, text))
            .map_err(|error| Error::new(format!("cannot write {}: {error}", path.display())))?;
        Ok(path)
    }
}

impl Drop for Temporary {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The left and right pairs of guides, in drawing order: each as its left and right member.
fn pairs(drawn: &Drawn) -> Vec<(&DrawnGuide, &DrawnGuide)> {
    let mut found = Vec::new();
    for guide in &drawn.guides {
        let Some(stem) = guide.name.strip_suffix("-right") else {
            continue;
        };
        let left = format!("{stem}-left");
        if let Some(left) = drawn.guides.iter().find(|other| other.name == left) {
            found.push((left, guide));
        }
    }
    found
}

/// How wide and how high a guide's anchors spread.
fn spread(guide: &DrawnGuide) -> [f64; 2] {
    let span = |index: usize| {
        let values = guide
            .passes_through
            .iter()
            .map(|(_, position)| position[index]);
        let (low, high) = values.fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), value| {
            (low.min(value), high.max(value))
        });
        high - low
    };
    [span(0), span(1)]
}

fn differ_in_place(left: &DrawnGuide, right: &DrawnGuide) -> bool {
    left.passes_through.len() != right.passes_through.len()
        || left
            .passes_through
            .iter()
            .zip(&right.passes_through)
            .any(|(first, second)| {
                (first.1[0] - second.1[0]).hypot(first.1[1] - second.1[1]) > TOLERANCE
            })
}

fn differ_in_shape(left: &DrawnGuide, right: &DrawnGuide) -> bool {
    let (left, right) = (spread(left), spread(right));
    (left[0] - right[0]).abs() > TOLERANCE || (left[1] - right[1]).abs() > TOLERANCE
}

/// Gives the element of that `id` the attribute `visibility="hidden"`.
fn hide_in_svg(svg: &str, name: &str) -> Result<String> {
    let id = format!("id=\"{name}\"");
    if svg.matches(&id).count() != 1 {
        return Err(Error::new(format!(
            "the SVG does not have exactly one `{id}`, so no spoiled copy can be made"
        )));
    }
    Ok(svg.replace(&id, &format!("{id} visibility=\"hidden\"")))
}

/// Exchanges the `id`s of two elements.
fn exchange_in_svg(svg: &str, first: &str, second: &str) -> Result<String> {
    let (first_id, second_id) = (format!("id=\"{first}\""), format!("id=\"{second}\""));
    if svg.matches(&first_id).count() != 1 || svg.matches(&second_id).count() != 1 {
        return Err(Error::new(format!(
            "the SVG does not have exactly one `{first_id}` and one `{second_id}`, so no spoiled copy can be made"
        )));
    }
    const HELD_ASIDE: &str = "id=\"\u{0}\"";
    Ok(svg
        .replace(&first_id, HELD_ASIDE)
        .replace(&second_id, &first_id)
        .replace(HELD_ASIDE, &second_id))
}

fn listed(names: &[String]) -> String {
    names
        .iter()
        .map(|name| format!("`{name}`"))
        .collect::<Vec<String>>()
        .join(", ")
}

fn failures_of(outcome: &Outcome, check: Check) -> Vec<String> {
    outcome
        .failures
        .iter()
        .filter(|failure| failure.check == check)
        .map(|failure| failure.message.clone())
        .collect()
}

/// Runs the hard case on a view: its landmark file and its SVG. Gives `None` when
/// VectorCraft is not installed.
pub fn run(landmark_path: &str, landmark_text: &str, svg: &Path) -> Result<Option<HardCase>> {
    let drawn = armature::read_drawn(landmark_text)
        .map_err(|error| Error::new(format!("{landmark_path}: {error}")))?;
    if drawn.turn_degrees == 0.0 {
        return Err(Error::new(format!(
            "{landmark_path} is the landmark file of a {} view that is not turned. The hard case is a three-quarter view.",
            drawn.view
        )));
    }
    let svg_text = std::fs::read_to_string(svg)
        .map_err(|error| Error::new(format!("cannot read {}: {error}", svg.display())))?;
    let svg_name = svg
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| Error::new(format!("{} is not a file", svg.display())))?;
    let folder = svg.parent().unwrap_or(Path::new("/"));

    let mut steps: Vec<Step> = Vec::new();
    let mut step = |asked: &str, held: bool, found: String| {
        steps.push(Step {
            asked: asked.to_string(),
            found,
            held,
        });
    };

    // What the landmark file says of the view, before VectorCraft is asked anything.
    let coordinates: Vec<f64> = drawn
        .guides
        .iter()
        .flat_map(|guide| {
            guide
                .passes_through
                .iter()
                .flat_map(|(_, position)| *position)
        })
        .collect();
    let not_whole = coordinates
        .iter()
        .filter(|value| value.fract() != 0.0)
        .count();
    step(
        "A three-quarter view whose positions are not round numbers",
        not_whole * 2 > coordinates.len(),
        format!(
            "turned {} degrees; {not_whole} of the {} anchor coordinates the landmark file gives are not whole numbers",
            drawn.turn_degrees,
            coordinates.len()
        ),
    );

    let missing: Vec<String> = INVENTORY
        .iter()
        .filter(|name| !drawn.guides.iter().any(|guide| guide.name == **name))
        .map(|name| name.to_string())
        .collect();
    let extra: Vec<String> = drawn
        .guides
        .iter()
        .filter(|guide| !INVENTORY.contains(&guide.name.as_str()))
        .map(|guide| guide.name.clone())
        .collect();
    step(
        "Every guide at once",
        missing.is_empty() && extra.is_empty(),
        if missing.is_empty() && extra.is_empty() {
            format!(
                "the landmark file gives the {} guides of the inventory",
                INVENTORY.len()
            )
        } else {
            format!(
                "missing from the landmark file: {}; not in the inventory: {}",
                listed(&missing),
                listed(&extra)
            )
        },
    );

    let grouped = drawn
        .guides
        .iter()
        .filter(|guide| guide.group.is_some())
        .count();
    step(
        "In nested groups",
        !drawn.groups.is_empty(),
        format!(
            "{} groups inside the layer `{}` hold {grouped} of the guides: {}",
            drawn.groups.len(),
            drawn.layer,
            listed(&drawn.groups)
        ),
    );

    let mut names: Vec<&str> = vec![drawn.layer.as_str()];
    names.extend(drawn.groups.iter().map(String::as_str));
    names.extend(drawn.guides.iter().map(|guide| guide.name.as_str()));
    let contained = names.iter().copied().find(|name| {
        names
            .iter()
            .any(|other| other != name && other.contains(name))
    });

    // The line reads as a description of the view: it has pairs that differ in both. Every
    // pair has to differ in place, or exchanged names could not be told from positions. A
    // pair of one shape in two places is named, and does not fail the line.
    let pairs = pairs(&drawn);
    let both: Vec<&(&DrawnGuide, &DrawnGuide)> = pairs
        .iter()
        .filter(|(left, right)| differ_in_place(left, right) && differ_in_shape(left, right))
        .collect();
    let name_of = |(left, right): &(&DrawnGuide, &DrawnGuide)| {
        format!("`{}` and `{}`", left.name, right.name)
    };
    let same_place: Vec<String> = pairs
        .iter()
        .filter(|(left, right)| !differ_in_place(left, right))
        .map(name_of)
        .collect();
    let same_shape: Vec<String> = pairs
        .iter()
        .filter(|(left, right)| differ_in_place(left, right) && !differ_in_shape(left, right))
        .map(name_of)
        .collect();
    step(
        "Left and right pairs whose two members differ in shape and place",
        !both.is_empty() && same_place.is_empty(),
        format!(
            "{} pairs. {} differ in both: their anchors stand apart and spread over a different width or height. In one place: {}. Of one shape, in two places: {}",
            pairs.len(),
            both.len(),
            if same_place.is_empty() {
                "none".to_string()
            } else {
                same_place.join("; ")
            },
            if same_shape.is_empty() {
                "none".to_string()
            } else {
                same_shape.join("; ")
            }
        ),
    );

    // Which guides the later steps hide, spoil and exchange: of the pairs that differ in
    // both, the first that sits inside a group and the first that does not.
    let (Some(inside), Some(outside)) = (
        both.iter()
            .find(|(left, _)| left.group.is_some())
            .or(both.first()),
        both.iter()
            .find(|(left, _)| left.group.is_none())
            .or(both.last()),
    ) else {
        return Err(Error::new(format!(
            "{landmark_path} gives no left and right pair of guides that differ in shape and place, so the hard case cannot be run on it"
        )));
    };

    // The view as generated.
    let Some(mut server) = Server::start(folder)? else {
        return Ok(None);
    };
    let version = server.version().to_string();
    let replies = Replies::read(&mut server, svg)?;
    let passing = read_back::check(landmark_path, landmark_text, &replies)?;
    step(
        "The oracle passes the view as generated",
        passing.passed(),
        if passing.passed() {
            format!(
                "no import warning; {} objects under the landmark file's names, each once; every anchor within {TOLERANCE} unit; every guide visible, stroked and unfilled",
                passing.objects.len()
            )
        } else {
            passing
                .failures
                .iter()
                .map(|failure| failure.message.clone())
                .collect::<Vec<String>>()
                .join("; ")
        },
    );

    let closed: Vec<&DrawnGuide> = drawn.guides.iter().filter(|guide| guide.closed).collect();
    let not_so: Vec<String> = closed
        .iter()
        .filter(|guide| {
            !passing.objects.iter().any(|object| {
                object.name.as_deref() == Some(guide.name.as_str())
                    && object.closed == Some(true)
                    && object.fill.as_deref() == Some("None")
            })
        })
        .map(|guide| guide.name.clone())
        .collect();
    step(
        "Closed guides arrive unfilled",
        !closed.is_empty() && not_so.is_empty(),
        if not_so.is_empty() {
            format!(
                "{} closed guides arrive closed and unfilled: {}",
                closed.len(),
                listed(
                    &closed
                        .iter()
                        .map(|guide| guide.name.clone())
                        .collect::<Vec<String>>()
                )
            )
        } else {
            format!("not closed and unfilled: {}", listed(&not_so))
        },
    );

    // One name contained in another: looking a name up finds more than the name.
    let find = |server: &mut Server, name: &str| -> Result<Vec<(i64, String)>> {
        let reply = server.command("document.find", json!({ "name": name }))?;
        reply
            .get("matches")
            .and_then(Value::as_array)
            .and_then(|matches| {
                matches
                    .iter()
                    .map(|found| {
                        Some((
                            found.get("id")?.as_i64()?,
                            found.get("name")?.as_str()?.to_string(),
                        ))
                    })
                    .collect::<Option<Vec<(i64, String)>>>()
            })
            .ok_or_else(|| {
                Error::new("VectorCraft's `document.find` gave no list `matches` of ids and names")
            })
    };
    match contained {
        Some(name) => {
            let matches = find(&mut server, name)?;
            let exact = matches.iter().filter(|(_, found)| found == name).count();
            let found_names: Vec<String> = matches.iter().map(|(_, name)| name.clone()).collect();
            step(
                "One name contained in another",
                matches.len() > 1 && exact == 1,
                format!(
                    "looking up `{name}` finds {} objects: {}; comparing the names exactly leaves {exact}",
                    matches.len(),
                    listed(&found_names)
                ),
            );
        }
        None => step(
            "One name contained in another",
            false,
            "no name of the landmark file is contained in another".to_string(),
        ),
    }

    // One member of a pair hidden by its name, and only it hidden.
    let to_hide = inside.0.name.as_str();
    let matches = find(&mut server, to_hide)?;
    let exact: Vec<i64> = matches
        .iter()
        .filter(|(_, found)| found == to_hide)
        .map(|(id, _)| *id)
        .collect();
    if let [id] = exact.as_slice() {
        server.command("object.setProps", json!({ "ids": [id], "visible": false }))?;
        let after = read_back::objects(&replies.again(&mut server)?)?;
        let hidden: Vec<String> = after
            .iter()
            .filter(|object| !object.visible || object.hidden_above.is_some())
            .map(|object| object.name.clone().unwrap_or_else(|| object.label.clone()))
            .collect();
        step(
            "After import, one member of a pair hidden by its name, and only it hidden",
            hidden == [to_hide.to_string()],
            format!(
                "`{to_hide}` found by its name and hidden; of {} objects, hidden or under something hidden: {}",
                after.len(),
                if hidden.is_empty() {
                    "none".to_string()
                } else {
                    listed(&hidden)
                }
            ),
        );
    } else {
        step(
            "After import, one member of a pair hidden by its name, and only it hidden",
            false,
            format!(
                "looking up `{to_hide}` and comparing the names exactly leaves {} objects, not one",
                exact.len()
            ),
        );
    }
    drop(server);

    // The two spoiled copies, each in a folder of its own, read by a server of its own.
    let temporary = Temporary::new()?;
    let read_spoiled = |kind: &str, text: &str| -> Result<Option<(Replies, Outcome)>> {
        let copy = temporary.copy(kind, svg_name, text)?;
        let Some(mut server) = Server::start(copy.parent().unwrap_or(Path::new("/")))? else {
            return Ok(None);
        };
        let replies = Replies::read(&mut server, &copy)?;
        let outcome = read_back::check(landmark_path, landmark_text, &replies)?;
        Ok(Some((replies, outcome)))
    };

    let to_spoil = outside.0.name.as_str();
    let Some((hidden_replies, hidden)) =
        read_spoiled("hidden", &hide_in_svg(&svg_text, to_spoil)?)?
    else {
        return Ok(None);
    };
    let not_read_back = hidden.failures.iter().any(|failure| {
        failure.check == Check::Names
            && failure.about.as_deref() == Some(to_spoil)
            && failure.message.contains("is not read back")
    });
    step(
        "Spoiled copy: a guide given visibility=\"hidden\". The oracle must fail it",
        !hidden.passed() && not_read_back,
        format!(
            "`{to_spoil}` spoiled; {} objects read back and {}; the oracle {}: {}",
            hidden.objects.len(),
            if hidden.check_passed(Check::Warning) {
                "no import warning"
            } else {
                "an import warning"
            },
            if hidden.passed() {
                "PASSED IT"
            } else {
                "failed it"
            },
            failures_of(&hidden, Check::Names).join("; ")
        ),
    );

    let (left, right) = (inside.0.name.as_str(), inside.1.name.as_str());
    let Some((exchanged_replies, exchanged)) =
        read_spoiled("exchanged", &exchange_in_svg(&svg_text, left, right)?)?
    else {
        return Ok(None);
    };
    let moved: Vec<&str> = exchanged
        .failures
        .iter()
        .filter(|failure| failure.check == Check::Positions)
        .filter_map(|failure| failure.about.as_deref())
        .collect();
    let only_positions = exchanged
        .failures
        .iter()
        .all(|failure| failure.check == Check::Positions);
    step(
        "Spoiled copy: a pair's names exchanged. Only positions show it, and the oracle must fail it",
        !exchanged.passed()
            && only_positions
            && moved.contains(&left)
            && moved.contains(&right)
            && moved.iter().all(|name| *name == left || *name == right),
        format!(
            "`{left}` and `{right}` exchanged; {} objects read back, names {}, {}; the oracle {}: {} anchors of the two are not where the landmark file puts them",
            exchanged.objects.len(),
            if exchanged.check_passed(Check::Names) {
                "the same set, each once"
            } else {
                "NOT the same set"
            },
            if exchanged.check_passed(Check::Warning) {
                "no import warning"
            } else {
                "an import warning"
            },
            if exchanged.passed() {
                "PASSED IT"
            } else {
                "failed it"
            },
            failures_of(&exchanged, Check::Positions).len()
        ),
    );
    drop(temporary);

    let mut text = format!(
        "The hard case of the SVG contract, on {svg_name} in VectorCraft {version} (headless MCP server), run now\n\
         Compared with {landmark_path}. Expected values come from the landmark file, never from the SVG.\n\
         The spoiled copies were made in a temporary folder and removed.\n\n"
    );
    for (index, step) in steps.iter().enumerate() {
        text.push_str(&format!(
            "{:>2}. {}\n    {}: {}\n",
            index + 1,
            step.asked,
            if step.held { "held" } else { "NOT HELD" },
            step.found
        ));
    }
    let not_held = steps.iter().filter(|step| !step.held).count();
    text.push_str(&if not_held == 0 {
        format!(
            "\nHELD: all {} lines of the hard case, in VectorCraft {version}.\n",
            steps.len()
        )
    } else {
        format!(
            "\nNOT HELD: {not_held} of the {} lines of the hard case, in VectorCraft {version}. The armature is neither put to Jon nor exported.\n",
            steps.len()
        )
    });

    Ok(Some(HardCase {
        steps,
        text,
        replies: vec![
            ("as-generated.json", replies),
            ("spoiled-hidden.json", hidden_replies),
            ("spoiled-exchanged.json", exchanged_replies),
        ],
    }))
}
