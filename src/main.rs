//! The command line: a thin layer over the library. It reads files, calls one library
//! function and writes files.

use std::collections::BTreeMap;
use std::path::Path;
use std::process::ExitCode;

use maquette::head_model::{self, Reading};
use maquette::views::Placement;
use maquette::{Error, Result, armature, hand_made_views, proportions};

const USAGE: &str = "\
maquette: armatures for anime-style characters, from a set of proportions.

Usage:
  maquette list-proportions
      Print the list of proportions: the unit, and each proportion's name, whether it is
      measured or a convention, and how it is taken in each view.

  maquette draw --proportions <file> --out <folder>
                [--unit <n>] [--eye-line-y <n>] [--front-axis-x <n>] [--three-quarter-axis-x <n>]
      Write the front view and the three-quarter view of a set of proportions: for each,
      an SVG of named guides and a landmark file. The options place the head on the page:
      the length of one unit, the height of the eye line, and where the upright axis the
      head turns about stands in each view. The same input gives the same bytes.

  maquette proportions-from-landmarks --front <file> --three-quarter <file> [--out <file>]
      Take two landmark files back into a set of proportions.

  maquette proportions-from-hand-made-views --record <file> [--out <file>]
      Work out a set of proportions from the record of a hand-made armature's views,
      such as data/saffi/hand-made-views.toml.
";

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match run(&arguments) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("maquette: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(arguments: &[String]) -> Result<()> {
    let Some((command, rest)) = arguments.split_first() else {
        print!("{USAGE}");
        return Ok(());
    };
    match command.as_str() {
        "help" | "--help" | "-h" => {
            print!("{USAGE}");
            Ok(())
        }
        "list-proportions" => {
            options(rest, &[])?;
            print!("{}", proportions::list()?.describe());
            Ok(())
        }
        "draw" => draw(rest),
        "proportions-from-landmarks" => proportions_from_landmarks(rest),
        "proportions-from-hand-made-views" => proportions_from_hand_made_views(rest),
        other => Err(Error::new(format!(
            "there is no command `{other}`. Run `maquette help` for the commands."
        ))),
    }
}

/// Reads `--name value` pairs, refusing a name the command does not take.
fn options<'a>(arguments: &'a [String], allowed: &[&str]) -> Result<BTreeMap<&'a str, &'a str>> {
    let mut found = BTreeMap::new();
    let mut rest = arguments.iter();
    while let Some(name) = rest.next() {
        if !allowed.contains(&name.as_str()) {
            return Err(Error::new(format!(
                "`{name}` is not an option of this command. Run `maquette help` for the options."
            )));
        }
        let value = rest
            .next()
            .ok_or_else(|| Error::new(format!("`{name}` needs a value")))?;
        if found.insert(name.as_str(), value.as_str()).is_some() {
            return Err(Error::new(format!("`{name}` is given twice")));
        }
    }
    Ok(found)
}

fn required<'a>(found: &BTreeMap<&str, &'a str>, name: &str) -> Result<&'a str> {
    found.get(name).copied().ok_or_else(|| {
        Error::new(format!(
            "`{name}` is missing. Run `maquette help` for the options."
        ))
    })
}

fn read(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .map_err(|error| Error::new(format!("cannot read {path}: {error}")))
}

fn write(path: &Path, text: &str) -> Result<()> {
    std::fs::write(path, text)
        .map_err(|error| Error::new(format!("cannot write {}: {error}", path.display())))
}

/// Writes to the file named by `--out`, or prints.
fn deliver(found: &BTreeMap<&str, &str>, text: &str) -> Result<()> {
    match found.get("--out") {
        Some(path) => write(Path::new(path), text),
        None => {
            print!("{text}");
            Ok(())
        }
    }
}

fn draw(arguments: &[String]) -> Result<()> {
    let found = options(
        arguments,
        &[
            "--proportions",
            "--out",
            "--unit",
            "--eye-line-y",
            "--front-axis-x",
            "--three-quarter-axis-x",
        ],
    )?;
    let set = proportions::Set::read(&read(required(&found, "--proportions")?)?)?;
    let mut placement = Placement::default();
    for (name, field) in [
        ("--unit", &mut placement.unit),
        ("--eye-line-y", &mut placement.eye_line_y),
        ("--front-axis-x", &mut placement.front_axis_x),
        (
            "--three-quarter-axis-x",
            &mut placement.three_quarter_axis_x,
        ),
    ] {
        if let Some(value) = found.get(name) {
            *field = value.parse().map_err(|_| {
                Error::new(format!("`{name}` needs a number, and `{value}` is not one"))
            })?;
        }
    }
    let files = armature::draw(&set, &placement)?;
    let folder = Path::new(required(&found, "--out")?);
    std::fs::create_dir_all(folder)
        .map_err(|error| Error::new(format!("cannot make {}: {error}", folder.display())))?;
    for file in files {
        write(&folder.join(&file.name), &file.text)?;
    }
    Ok(())
}

fn proportions_from_landmarks(arguments: &[String]) -> Result<()> {
    let found = options(arguments, &["--front", "--three-quarter", "--out"])?;
    let front = armature::read_landmark_file(&read(required(&found, "--front")?)?)?;
    let three_quarter = armature::read_landmark_file(&read(required(&found, "--three-quarter")?)?)?;
    let set = head_model::proportions_from_views(&front, &three_quarter)?;
    let opening = "\
# A set of proportions taken back from two landmark files, a front view and a three-quarter view,
# by `maquette proportions-from-landmarks`. `maquette list-proportions` defines each proportion.
";
    deliver(&found, &set.write(opening)?)
}

fn text_value(text: &str) -> String {
    format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
}

fn text_list(lines: &[String]) -> String {
    let mut out = String::from("[\n");
    for line in lines {
        out.push_str(&format!("  {},\n", text_value(line)));
    }
    out.push(']');
    out
}

/// Where a hand-made view puts the unit, the eye line and the turn axis, in its own units.
fn frame_lines(reading: &Reading, view: &str, table: &str) -> Result<String> {
    let frame = reading.frame(view)?;
    let axis = reading
        .points
        .get("skull-centre")
        .ok_or_else(|| Error::new(format!("the record's {view} view has no skull centre")))?;
    Ok(format!(
        "[source.{table}]\nunit = {}\neye_line_y = {}\nturn_axis_x = {}\n",
        maquette::number::toml(frame.unit, 3),
        maquette::number::toml(frame.eye_line_y, 3),
        maquette::number::toml(axis[0], 3),
    ))
}

fn proportions_from_hand_made_views(arguments: &[String]) -> Result<()> {
    let found = options(arguments, &["--record", "--out"])?;
    let record_path = required(&found, "--record")?;
    let hand_made = hand_made_views::read(&read(record_path)?)?;
    let set = head_model::proportions_from_views(&hand_made.front, &hand_made.three_quarter)?;

    let mut opening = String::from(
        "\
# A set of proportions worked out from the record of a hand-made armature's views,
# by `maquette proportions-from-hand-made-views`. It was not typed in by hand: the tests check
# that this file is what the command writes from the record.
#
# These are the proportions of a comparison case. They never enter a proportion table.
#
# `maquette list-proportions` defines each proportion. `maquette draw` reads this file as it
# reads any set of proportions.

",
    );
    opening.push_str("[source]\n");
    opening.push_str(&format!("record = {}\n", text_value(record_path)));
    opening.push_str(&format!("what = {}\n", text_value(&hand_made.what)));
    opening.push_str(&format!(
        "front_sha256 = {}\n",
        text_value(&hand_made.front_sha256)
    ));
    opening.push_str(&format!(
        "three_quarter_sha256 = {}\n",
        text_value(&hand_made.three_quarter_sha256)
    ));
    opening.push_str(&format!(
        "turn_degrees = {}\n",
        maquette::number::toml(hand_made.three_quarter.turn_degrees, 3)
    ));
    opening.push_str(
        "taken_by = \"The head model's inverse direction: landmark positions read from the two views, back into proportions.\"\n",
    );
    let reading_rules: Vec<String> = hand_made_views::RULES
        .iter()
        .map(|rule| rule.to_string())
        .collect();
    opening.push_str("# How the record's positions were read as the landmarks the list of proportions asks for.\n");
    opening.push_str(&format!("reading_rules = {}\n", text_list(&reading_rules)));
    opening
        .push_str("# What was done where the two views disagree, or one does not show a point.\n");
    opening.push_str(&format!(
        "disagreement_rules = {}\n",
        text_list(proportions::list()?.disagreement_rules())
    ));
    opening.push_str("\n# Where each hand-made view puts the unit, the eye line and the upright axis the head turns about,\n");
    opening.push_str("# in the record's own units. `maquette draw` with these lays a generated view over the hand-made one.\n");
    opening.push_str(&frame_lines(&hand_made.front, "front", "front")?);
    opening.push('\n');
    opening.push_str(&frame_lines(
        &hand_made.three_quarter,
        "three-quarter",
        "three_quarter",
    )?);

    deliver(&found, &set.write(&opening)?)
}
