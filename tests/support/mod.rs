//! Shared by the command-line tests: run the program, and read the files it writes.
//! Every test goes through the command line, files in and files out.
#![allow(dead_code)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A made-up set of proportions. It says so in its header and is no character's.
pub const MADE_UP: &str = "tests/fixtures/made-up-proportions.toml";

pub fn repository() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Runs the program from the repository's root.
pub fn maquette(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_maquette"))
        .args(arguments)
        .current_dir(repository())
        .output()
        .expect("the program starts")
}

/// Runs the program and insists that it succeeds, returning what it printed.
pub fn maquette_ok(arguments: &[&str]) -> String {
    let output = maquette(arguments);
    assert!(
        output.status.success(),
        "`maquette {}` failed:\n{}",
        arguments.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("the output is text")
}

/// An empty folder for one test's files.
pub fn scratch(name: &str) -> PathBuf {
    let folder = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    if folder.exists() {
        std::fs::remove_dir_all(&folder).expect("the old scratch folder can be removed");
    }
    std::fs::create_dir_all(&folder).expect("the scratch folder can be made");
    folder
}

pub fn text(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

pub fn path_text(path: &Path) -> &str {
    path.to_str().expect("the path is text")
}

/// Draws a set of proportions into a fresh scratch folder and returns the folder.
pub fn draw(name: &str, proportions: &str, more: &[&str]) -> PathBuf {
    let folder = scratch(name);
    let mut arguments = vec![
        "draw",
        "--proportions",
        proportions,
        "--out",
        path_text(&folder),
    ];
    arguments.extend_from_slice(more);
    maquette_ok(&arguments);
    folder
}

pub fn number(value: &toml::Value) -> f64 {
    match value {
        toml::Value::Float(number) => *number,
        toml::Value::Integer(number) => *number as f64,
        other => panic!("expected a number, found {other:?}"),
    }
}

/// A set of proportions as its file gives it.
pub fn proportions(path: &Path) -> BTreeMap<String, f64> {
    let table: toml::Table = text(path).parse().expect("the set of proportions is TOML");
    table["proportions"]
        .as_table()
        .expect("a [proportions] table")
        .iter()
        .map(|(name, value)| (name.clone(), number(value)))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    /// Only a landmark, a point in space, has a depth.
    pub depth: Option<f64>,
}

pub struct Guide {
    pub element: String,
    pub anchors: Vec<String>,
    pub table: toml::Table,
}

/// A landmark file as written beside a view's SVG.
pub struct LandmarkFile {
    pub table: toml::Table,
    pub landmarks: BTreeMap<String, Point>,
    pub view_points: BTreeMap<String, Point>,
    pub guides: BTreeMap<String, Guide>,
}

impl LandmarkFile {
    pub fn read(path: &Path) -> LandmarkFile {
        let table: toml::Table = text(path).parse().expect("the landmark file is TOML");
        let points = |key: &str| -> BTreeMap<String, Point> {
            table[key]
                .as_table()
                .unwrap_or_else(|| panic!("a [{key}] table"))
                .iter()
                .map(|(name, value)| {
                    let entry = value.as_table().expect("a point is a table");
                    let point = Point {
                        x: number(&entry["x"]),
                        y: number(&entry["y"]),
                        depth: entry.get("depth").map(number),
                    };
                    (name.clone(), point)
                })
                .collect()
        };
        let landmarks = points("landmarks");
        let view_points = points("view_points");
        let guides = table["guides"]
            .as_table()
            .expect("a [guides] table")
            .iter()
            .map(|(name, value)| {
                let entry = value.as_table().expect("a guide is a table").clone();
                let guide = Guide {
                    element: entry["element"].as_str().expect("an element").to_string(),
                    anchors: entry["anchors"]
                        .as_array()
                        .expect("anchors")
                        .iter()
                        .map(|anchor| anchor.as_str().expect("an anchor's name").to_string())
                        .collect(),
                    table: entry,
                };
                (name.clone(), guide)
            })
            .collect();
        LandmarkFile {
            table,
            landmarks,
            view_points,
            guides,
        }
    }

    pub fn frame(&self, key: &str) -> f64 {
        number(&self.table["frame"][key])
    }

    /// A landmark or a view point, by name.
    pub fn point(&self, name: &str) -> Point {
        self.landmarks
            .get(name)
            .or_else(|| self.view_points.get(name))
            .copied()
            .unwrap_or_else(|| panic!("no point named {name}"))
    }
}

pub fn assert_close(actual: f64, expected: f64, tolerance: f64, what: &str) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{what}: expected {expected}, found {actual} (tolerance {tolerance})"
    );
}
