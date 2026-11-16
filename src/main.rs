//! The command line: a thin layer over the library. It reads files, calls one library
//! function and writes files.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use maquette::head_model;
use maquette::views::Placement;
use maquette::{
    Error, Result, armature, differences, hand_made_views, hard_case, overlay, proportions,
    read_back, vectorcraft,
};

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

  maquette differences-from-hand-made-views --front <file> --three-quarter <file>
                                            --record <file> [--out <file>]
      Lay the landmark files of a generated front view and three-quarter view over the
      record of a hand-made armature's views, and list how far apart the two put each
      landmark, as a share of head height. Every difference above 1% is numbered. The file
      states how the views are laid over each other and what has no counterpart. With
      --out the file is written there and the numbered list is printed.

  maquette overlay-hand-made-views --front <file> --three-quarter <file> --record <file>
                                   --out <folder>
      For a review sheet: write, for each view, an SVG in the generated view's own units
      that marks every position the record gives that view, a dashed ring where the head
      covers it in the drawing, and every difference above 1% as a line from the generated
      position to the hand-made one with its number. The numbers and positions are the
      ones differences-from-hand-made-views gives for the same files. Placed at the same
      place and size as the view's SVG, the overlay lies over it exactly. Writes
      front.hand-made-overlay.svg and three-quarter.hand-made-overlay.svg.

  maquette read-back --landmarks <file> [--svg <file>] [--record <file>]
  maquette read-back --landmarks <file> --replies <file>
      The oracle of the SVG contract, for one view. Open the view's SVG in VectorCraft, read
      back what VectorCraft holds, and compare it with the landmark file: no import warning;
      the landmark file's names, each once; every anchor of every guide within 0.001 unit;
      every guide visible, stroked and unfilled. Expected values come from the landmark
      file, never from the SVG. The SVG is the one the landmark file names, beside it,
      unless --svg gives another. The read-back is printed, with the VectorCraft version it
      ran against, which is whichever is installed.
      It starts `vectorcraft-cli mcp` itself, allowed to read the SVG's folder and to write
      nothing. --record also writes VectorCraft's replies to a file; --replies compares
      such a recording instead of starting VectorCraft, and says so.
      Exit status: 0 when it passes, 1 when it fails, 2 when vectorcraft-cli is not
      installed and nothing was checked. A server that stops, or does not answer a request
      within a minute, also ends it with 1: the message says which request, and passes on
      what the server wrote to its error output.

  maquette read-back-hard-case --landmarks <file> [--svg <file>] [--record <folder>]
      The hard case of the SVG contract, on a three-quarter view, in VectorCraft: every
      guide at once in nested groups with one name contained in another; left and right
      pairs that differ in shape and place; closed guides arriving unfilled; one member of a
      pair hidden by its name, and only it; and two spoiled copies the oracle must fail,
      one with a guide given visibility=\"hidden\" and one with a pair's names exchanged.
      The spoiled copies are made in a temporary folder and removed. Each line and what was
      found is printed. --record writes VectorCraft's replies for the view and for each
      spoiled copy into a folder. Exit status as for read-back.
";

/// How a command ended, when nothing went wrong in running it.
enum Ended {
    /// It did what it was asked, and a check it made passed.
    Well,
    /// A check it made failed.
    CheckFailed,
    /// It could not make its check, because VectorCraft is not installed.
    NotRun,
}

/// The exit status when a check could not be made: neither a pass nor a failure.
const NOT_RUN: u8 = 2;

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match run(&arguments) {
        Ok(Ended::Well) => ExitCode::SUCCESS,
        Ok(Ended::CheckFailed) => ExitCode::FAILURE,
        Ok(Ended::NotRun) => {
            eprintln!("maquette: {}", vectorcraft::NOT_INSTALLED);
            ExitCode::from(NOT_RUN)
        }
        Err(error) => {
            eprintln!("maquette: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(arguments: &[String]) -> Result<Ended> {
    let Some((command, rest)) = arguments.split_first() else {
        print!("{USAGE}");
        return Ok(Ended::Well);
    };
    match command.as_str() {
        "help" | "--help" | "-h" => print!("{USAGE}"),
        "list-proportions" => {
            options(rest, &[])?;
            print!("{}", proportions::list()?.describe());
        }
        "draw" => draw(rest)?,
        "proportions-from-landmarks" => proportions_from_landmarks(rest)?,
        "proportions-from-hand-made-views" => proportions_from_hand_made_views(rest)?,
        "differences-from-hand-made-views" => differences_from_hand_made_views(rest)?,
        "overlay-hand-made-views" => overlay_hand_made_views(rest)?,
        "read-back" => return read_back_command(rest),
        "read-back-hard-case" => return read_back_hard_case(rest),
        other => {
            return Err(Error::new(format!(
                "there is no command `{other}`. Run `maquette help` for the commands."
            )));
        }
    }
    Ok(Ended::Well)
}

/// A file's whole path, so that VectorCraft can be given its folder to read.
fn whole_path(path: &Path) -> Result<PathBuf> {
    path.canonicalize()
        .map_err(|error| Error::new(format!("cannot read {}: {error}", path.display())))
}

fn read_back_hard_case(arguments: &[String]) -> Result<Ended> {
    let found = options(arguments, &["--landmarks", "--svg", "--record"])?;
    let landmark_path = required(&found, "--landmarks")?;
    let landmark_text = read(landmark_path)?;
    let svg = match found.get("--svg") {
        Some(svg) => PathBuf::from(svg),
        None => read_back::svg_beside(Path::new(landmark_path), &landmark_text)?,
    };
    let svg = whole_path(&svg)?;
    let Some(ran) = hard_case::run(landmark_path, &landmark_text, &svg)? else {
        return Ok(Ended::NotRun);
    };
    if let Some(folder) = found.get("--record") {
        let folder = Path::new(folder);
        std::fs::create_dir_all(folder)
            .map_err(|error| Error::new(format!("cannot make {}: {error}", folder.display())))?;
        for (name, replies) in &ran.replies {
            write(&folder.join(name), &replies.recording())?;
        }
    }
    print!("{}", ran.text);
    Ok(if ran.held() {
        Ended::Well
    } else {
        Ended::CheckFailed
    })
}

fn read_back_command(arguments: &[String]) -> Result<Ended> {
    let found = options(
        arguments,
        &["--landmarks", "--svg", "--replies", "--record"],
    )?;
    let landmark_path = required(&found, "--landmarks")?;
    let landmark_text = read(landmark_path)?;
    let replies = match found.get("--replies") {
        Some(recording) => {
            for live_only in ["--svg", "--record"] {
                if found.contains_key(live_only) {
                    return Err(Error::new(format!(
                        "`{live_only}` is for a live run, and `--replies` reads a recording instead"
                    )));
                }
            }
            read_back::Replies::from_recording(&read(recording)?)
                .map_err(|error| Error::new(format!("{recording}: {error}")))?
        }
        None => {
            let svg = match found.get("--svg") {
                Some(svg) => PathBuf::from(svg),
                None => read_back::svg_beside(Path::new(landmark_path), &landmark_text)?,
            };
            let svg = whole_path(&svg)?;
            let folder = svg.parent().unwrap_or(Path::new("/"));
            let Some(mut server) = vectorcraft::Server::start(folder)? else {
                return Ok(Ended::NotRun);
            };
            read_back::Replies::read(&mut server, &svg)?
        }
    };
    if let Some(recording) = found.get("--record") {
        write(Path::new(recording), &replies.recording())?;
    }
    let outcome = read_back::check(landmark_path, &landmark_text, &replies)?;
    print!("{}", outcome.text);
    Ok(if outcome.passed() {
        Ended::Well
    } else {
        Ended::CheckFailed
    })
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

fn differences_from_hand_made_views(arguments: &[String]) -> Result<()> {
    let found = options(
        arguments,
        &["--front", "--three-quarter", "--record", "--out"],
    )?;
    let front = required(&found, "--front")?;
    let three_quarter = required(&found, "--three-quarter")?;
    let record = required(&found, "--record")?;
    let (front_text, three_quarter_text, record_text) =
        (read(front)?, read(three_quarter)?, read(record)?);
    let input = |path, text| differences::Input { path, text };
    let found_differences = differences::compare(
        input(front, &front_text),
        input(three_quarter, &three_quarter_text),
        input(record, &record_text),
    )?;
    deliver(&found, &found_differences.file())?;
    if found.contains_key("--out") {
        print!("{}", found_differences.summary());
    }
    Ok(())
}

fn overlay_hand_made_views(arguments: &[String]) -> Result<()> {
    let found = options(
        arguments,
        &["--front", "--three-quarter", "--record", "--out"],
    )?;
    let front = required(&found, "--front")?;
    let three_quarter = required(&found, "--three-quarter")?;
    let record = required(&found, "--record")?;
    let folder = Path::new(required(&found, "--out")?);
    let (front_text, three_quarter_text, record_text) =
        (read(front)?, read(three_quarter)?, read(record)?);
    let input = |path, text| differences::Input { path, text };
    let files = overlay::draw(
        input(front, &front_text),
        input(three_quarter, &three_quarter_text),
        input(record, &record_text),
    )?;
    std::fs::create_dir_all(folder)
        .map_err(|error| Error::new(format!("cannot make {}: {error}", folder.display())))?;
    for file in files {
        write(&folder.join(&file.name), &file.text)?;
    }
    Ok(())
}

fn proportions_from_hand_made_views(arguments: &[String]) -> Result<()> {
    let found = options(arguments, &["--record", "--out"])?;
    let record_path = required(&found, "--record")?;
    let hand_made = hand_made_views::read(&read(record_path)?)?;
    deliver(&found, &hand_made.proportions_file(record_path)?)
}
