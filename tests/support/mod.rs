//! Shared by the command-line tests: run the program, and read the files it writes.
//! Every test goes through the command line, files in and files out.
#![allow(dead_code)]

pub mod made_up_head;

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
    maquette_with_path(arguments, None)
}

/// Runs the program from the repository's root, with `PATH` set to `path` when one is given:
/// where the program looks for `vectorcraft-cli`.
pub fn maquette_with_path(arguments: &[&str], path: Option<&std::ffi::OsStr>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_maquette"));
    command.args(arguments).current_dir(repository());
    if let Some(path) = path {
        command.env("PATH", path);
    }
    command.output().expect("the program starts")
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

/// An empty made-up reference folder for one test. A reference folder is outside every
/// repository, and `target/` is inside this one, so it is made in the system's temporary
/// folder, under a name that is this checkout's own: two checkouts tested at once do not
/// share it.
pub fn references(name: &str) -> PathBuf {
    use std::hash::{Hash, Hasher};
    let mut checkout = std::collections::hash_map::DefaultHasher::new();
    env!("CARGO_MANIFEST_DIR").hash(&mut checkout);
    let folder = std::env::temp_dir()
        .join(format!(
            "maquette-made-up-references-{:016x}",
            checkout.finish()
        ))
        .join(name);
    if folder.exists() {
        std::fs::remove_dir_all(&folder).expect("the old made-up reference folder can be removed");
    }
    std::fs::create_dir_all(&folder).expect("the made-up reference folder can be made");
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
    /// Only a landmark that a named correction moves in this view says where it is drawn.
    pub drawn_at_depth: Option<f64>,
}

pub struct Guide {
    pub element: String,
    pub passes_through: Vec<String>,
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
                        drawn_at_depth: entry.get("drawn_at_depth").map(number),
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
                    passes_through: entry["passes_through"]
                        .as_array()
                        .expect("passes_through")
                        .iter()
                        .map(|name| name.as_str().expect("a name").to_string())
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

/// Runs the program with the reference folder configured as `references`, or with no
/// reference folder configured at all: the environment variable is set or removed for this
/// one run, so a test never depends on the machine it runs on.
pub fn maquette_in(references: Option<&Path>, arguments: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_maquette"));
    command.args(arguments).current_dir(repository());
    match references {
        Some(folder) => command.env("MAQUETTE_REFERENCES", folder),
        None => command.env_remove("MAQUETTE_REFERENCES"),
    };
    command.output().expect("the program starts")
}

/// Runs a measuring command in a reference folder and insists that it succeeds, returning
/// what it printed.
pub fn measure_ok(references: &Path, arguments: &[&str]) -> String {
    let output = maquette_in(Some(references), arguments);
    assert!(
        output.status.success(),
        "`maquette {}` failed:\n{}",
        arguments.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("the output is text")
}

/// What a command that failed wrote to its error output.
pub fn failure(output: &Output) -> String {
    assert!(
        !output.status.success(),
        "the command succeeded:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// A made-up picture for a made-up reference folder: plain grey with one darker block, so
/// that a crop can be told from the whole. It shows no drawing.
pub fn made_up_image(path: &Path, width: u32, height: u32) {
    let picture = image::RgbImage::from_fn(width, height, |x, y| {
        if (40..60).contains(&x) && (20..30).contains(&y) {
            image::Rgb([40, 40, 40])
        } else {
            image::Rgb([200, 200, 200])
        }
    });
    picture.save(path).expect("the made-up picture is written");
}

/// A reading template, as `maquette start-reading` writes it, filled in with made-up answers:
/// every text given, the head called level, a turn of `turn_degrees` where one is asked, and
/// every landmark placed where `place` puts it, or left for `[not_seen]` when it gives none.
/// Nothing in it was read from a drawing.
pub fn filled_in(
    template: &str,
    turn_degrees: f64,
    place: &dyn Fn(&str) -> Option<[f64; 2]>,
) -> String {
    let mut out = String::new();
    let mut not_seen = Vec::new();
    for line in template.lines() {
        if let Some(name) = line
            .strip_prefix("# ")
            .and_then(|rest| rest.strip_suffix(" = { x = , y = }"))
        {
            match place(name) {
                Some([x, y]) => out.push_str(&format!("{name} = {{ x = {x:?}, y = {y:?} }}\n")),
                None => not_seen.push(name.to_string()),
            }
        } else if line == "# level = " {
            out.push_str("level = true\n");
        } else if line == "# turn_degrees = " {
            out.push_str(&format!("turn_degrees = {turn_degrees:?}\n"));
        } else if let Some(key) = line.strip_suffix(" = \"\"") {
            if key == "set_aside_why" {
                out.push_str(line);
            } else {
                out.push_str(&format!("{key} = \"made up for a test\""));
            }
            out.push('\n');
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    for name in not_seen {
        out.push_str(&format!("{name} = \"made up: left out for a test\"\n"));
    }
    out
}
