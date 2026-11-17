//! The command line: a thin layer over the library. It reads files, calls one library
//! function and writes files.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use maquette::head_model;
use maquette::reference_folder::Folder;
use maquette::views::Placement;
use maquette::{
    Error, Result, armature, differences, enlargement, hand_made_drawings, hand_made_views,
    hard_case, known_positions, measuring, overlay, proportions, read_back, readings,
    reference_check_sheet, vectorcraft,
};

const USAGE: &str = "\
maquette: armatures for anime-style characters, from a set of proportions.

Usage:
  maquette list-proportions
      Print the list of proportions: the unit, and each proportion's name, whether it is
      measured or a convention, and how it is taken in each view.

  maquette draw --proportions <file> --out <folder>
                [--unit <n>] [--eye-line-y <n>]
                [--front-axis-x <n>] [--three-quarter-axis-x <n>] [--side-axis-x <n>]
      Write the front view, the three-quarter view and the side view of a set of
      proportions: for each, an SVG of named guides and a landmark file. The side view is
      the three-quarter view's turn carried to 90 degrees, so it faces the viewer's right.
      The options place the head on the page: the length of one unit, the height of the eye
      line, and where the upright axis the head turns about stands in each view. The same
      input gives the same bytes.

  maquette proportions-from-landmarks --front <file> --three-quarter <file> [--out <file>]
      Take two landmark files back into a set of proportions.

  maquette proportions-from-hand-made-views --record <file> [--final-record <file>]
                                            [--out <file>]
      Work out a set of proportions from the record of a hand-made armature's views,
      such as data/saffi/hand-made-views.toml. With --final-record, a record of final
      drawings such as data/saffi/final-drawings.toml, each of its landmarks replaces the
      record's landmark of the same name in the same view.

  maquette differences-from-hand-made-views --front <file> --three-quarter <file>
                                            --record <file> [--final-record <file>]
                                            [--out <file>]
      Lay the landmark files of a generated front view and three-quarter view over the
      record of a hand-made armature's views, and list how far apart the two put each
      landmark, as a share of head height. Every difference above 1% is numbered. The file
      states how the views are laid over each other and what has no counterpart. With
      --final-record the record's landmarks are replaced as above, and each difference
      against a replaced one is marked. With --out the file is written there and the
      numbered list is printed. No hand-made side view exists, so a side view has nothing
      to be compared with.

  maquette overlay-hand-made-views --front <file> --three-quarter <file> --record <file>
                                   [--final-record <file>] --out <folder>
      For a review sheet: write, for each view, an SVG in the generated view's own units
      that marks every position the record gives that view, a dashed ring where the head
      covers it in the drawing, and every difference above 1% as a line from the generated
      position to the hand-made one with its number. The numbers and positions are the
      ones differences-from-hand-made-views gives for the same files. Placed at the same
      place and size as the view's SVG, the overlay lies over it exactly. Writes
      front.hand-made-overlay.svg and three-quarter.hand-made-overlay.svg.

  maquette list-drawing --svg <file>
      List what a hand-made drawing holds: every group, and every shape under its address
      (the id of its group, a slash, and its name), with its nodes numbered in path order
      and how far it reaches. This is what a reading of a new drawing is written from. A
      drawing is read as written: one that moves its coordinates with a transform, or
      draws a path with anything but the absolute M, L, C and Z, is refused.

  maquette record-hand-made-drawings --underdrawings <folder> --final-drawings <folder>
                                     --out <folder>
      Read Saffi's four hand-made drawings and write the records of them that data/saffi/
      holds: hand-made-views.toml and views-compared.toml from the two S2 underdrawings,
      and final-drawings.toml, the ears and lids of the two final drawings. The drawings
      are private; each is read from its folder under its recorded name, and only when its
      SHA-256 digest is the recorded one. Where a folder or a drawing is absent, or a
      digest differs, the command says so, writes nothing, and the committed numbers stand.

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
        "list-drawing" => {
            let found = options(rest, &["--svg"])?;
            let path = required(&found, "--svg")?;
            let listed = hand_made_drawings::list(&read(path)?)
                .map_err(|error| Error::new(format!("{path}: {error}")))?;
            print!("{listed}");
        }
        "record-hand-made-drawings" => record_hand_made_drawings(rest)?,
        "reading-instructions" => {
            options(rest, &[])?;
            print!("{}", readings::INSTRUCTIONS);
        }
        "start-reading" => start_reading(rest)?,
        "check-reading" => return check_reading(rest),
        "enlarge-image" => enlarge_image(rest)?,
        "skull-by-rule" => {
            let found = options(rest, &["--proportions"])?;
            let path = required(&found, "--proportions")?;
            let set = proportions::Set::read(&read(path)?)
                .map_err(|error| Error::new(format!("{path}: {error}")))?;
            print!("{}", measuring::skull_rule_on(&set, path)?);
        }
        "agreement-of-readings" => agreement_of_readings(rest)?,
        "turn-from-readings" => turn_from_readings(rest)?,
        "fit-of-views" => fit_of_views(rest)?,
        "proportions-from-readings" => proportions_from_readings(rest)?,
        "score-reading" => score_reading(rest)?,
        "reference-check-sheet" => reference_check_sheet(rest)?,
        "compare-proportions" => compare_proportions(rest)?,
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
            "--side-axis-x",
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
        ("--side-axis-x", &mut placement.side_axis_x),
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
        &[
            "--front",
            "--three-quarter",
            "--record",
            "--final-record",
            "--out",
        ],
    )?;
    let front = required(&found, "--front")?;
    let three_quarter = required(&found, "--three-quarter")?;
    let record = required(&found, "--record")?;
    let (front_text, three_quarter_text, record_text) =
        (read(front)?, read(three_quarter)?, read(record)?);
    let final_record = final_record(&found)?;
    let input = |path, text| differences::Input { path, text };
    let found_differences = differences::compare(
        input(front, &front_text),
        input(three_quarter, &three_quarter_text),
        input(record, &record_text),
        final_record
            .as_ref()
            .map(|(path, text)| input(path, text.as_str())),
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
        &[
            "--front",
            "--three-quarter",
            "--record",
            "--final-record",
            "--out",
        ],
    )?;
    let front = required(&found, "--front")?;
    let three_quarter = required(&found, "--three-quarter")?;
    let record = required(&found, "--record")?;
    let folder = Path::new(required(&found, "--out")?);
    let (front_text, three_quarter_text, record_text) =
        (read(front)?, read(three_quarter)?, read(record)?);
    let final_record = final_record(&found)?;
    let input = |path, text| differences::Input { path, text };
    let files = overlay::draw(
        input(front, &front_text),
        input(three_quarter, &three_quarter_text),
        input(record, &record_text),
        final_record
            .as_ref()
            .map(|(path, text)| input(path, text.as_str())),
    )?;
    std::fs::create_dir_all(folder)
        .map_err(|error| Error::new(format!("cannot make {}: {error}", folder.display())))?;
    for file in files {
        write(&folder.join(&file.name), &file.text)?;
    }
    Ok(())
}

fn record_hand_made_drawings(arguments: &[String]) -> Result<()> {
    let found = options(arguments, &["--underdrawings", "--final-drawings", "--out"])?;
    let recorded = hand_made_drawings::record(
        Path::new(required(&found, "--underdrawings")?),
        Path::new(required(&found, "--final-drawings")?),
    )?;
    let folder = Path::new(required(&found, "--out")?);
    std::fs::create_dir_all(folder)
        .map_err(|error| Error::new(format!("cannot make {}: {error}", folder.display())))?;
    for file in recorded.files {
        write(&folder.join(&file.name), &file.text)?;
    }
    print!("{}", recorded.log);
    Ok(())
}

fn proportions_from_hand_made_views(arguments: &[String]) -> Result<()> {
    let found = options(arguments, &["--record", "--final-record", "--out"])?;
    let record_path = required(&found, "--record")?;
    let final_record = final_record(&found)?;
    let hand_made = hand_made_views::read(
        &read(record_path)?,
        final_record
            .as_ref()
            .map(|(path, text)| hand_made_views::FinalRecord {
                path,
                text: text.as_str(),
            }),
    )?;
    deliver(&found, &hand_made.proportions_file(record_path)?)
}

/// The reference folder and the options of a measuring command. The folder comes first: with
/// none, a measuring command stops before it looks at anything else.
fn measuring<'a>(
    arguments: &'a [String],
    allowed: &[&str],
) -> Result<(Folder, BTreeMap<&'a str, &'a str>)> {
    let folder = Folder::configured()?;
    Ok((folder, options(arguments, allowed)?))
}

/// A reading of the reference folder, read. Its path is named in what goes wrong.
fn reading(folder: &Folder, path: &str) -> Result<readings::Record> {
    readings::Record::read(&folder.read_text(path)?)
        .map_err(|error| Error::new(format!("{path}: {error}")))
}

/// A view as a measuring command is given it: one reading, or the two readings of the
/// drawing with a comma between them.
fn drawing(folder: &Folder, named: &str) -> Result<readings::Drawing> {
    let records = named
        .split(',')
        .map(|path| reading(folder, path.trim()))
        .collect::<Result<Vec<_>>>()?;
    readings::Drawing::of(&records)
}

/// Writes what a measuring command worked out into the reference folder, at `--out`, and
/// prints its summary; or prints the whole of it.
fn deliver_measured(
    folder: &Folder,
    found: &BTreeMap<&str, &str>,
    file: &str,
    summary: &str,
) -> Result<()> {
    match found.get("--out") {
        Some(out) => {
            folder.write(out, file.as_bytes())?;
            print!("{summary}");
        }
        None => print!("{file}"),
    }
    Ok(())
}

fn agreement_of_readings(arguments: &[String]) -> Result<()> {
    let (folder, found) = measuring(arguments, &["--first", "--second", "--out"])?;
    let first = required(&found, "--first")?;
    let second = required(&found, "--second")?;
    let agreement = measuring::agreement(
        first,
        &reading(&folder, first)?,
        second,
        &reading(&folder, second)?,
    )?;
    deliver_measured(&folder, &found, &agreement.file(), &agreement.summary())
}

fn turn_from_readings(arguments: &[String]) -> Result<()> {
    let (folder, found) = measuring(arguments, &["--front", "--three-quarter", "--out"])?;
    let front = required(&found, "--front")?;
    let three_quarter = required(&found, "--three-quarter")?;
    let turn = measuring::turn(
        &measuring::Given {
            named: front,
            drawing: &drawing(&folder, front)?,
        },
        &measuring::Given {
            named: three_quarter,
            drawing: &drawing(&folder, three_quarter)?,
        },
    )?;
    deliver_measured(&folder, &found, &turn.file(), &turn.summary())
}

fn fit_of_views(arguments: &[String]) -> Result<()> {
    let (folder, found) = measuring(
        arguments,
        &["--front", "--three-quarter", "--side", "--out"],
    )?;
    let front = required(&found, "--front")?;
    let three_quarter = required(&found, "--three-quarter")?;
    let side = match found.get("--side") {
        Some(side) => Some((*side, drawing(&folder, side)?)),
        None => None,
    };
    let fit = measuring::fit(
        &measuring::Given {
            named: front,
            drawing: &drawing(&folder, front)?,
        },
        &measuring::Given {
            named: three_quarter,
            drawing: &drawing(&folder, three_quarter)?,
        },
        side.as_ref()
            .map(|(named, drawing)| measuring::Given { named, drawing })
            .as_ref(),
    )?;
    deliver_measured(&folder, &found, &fit.file(), &fit.summary())
}

fn proportions_from_readings(arguments: &[String]) -> Result<()> {
    let (folder, found) = measuring(arguments, &["--front", "--three-quarter", "--out"])?;
    let front = required(&found, "--front")?;
    let three_quarter = required(&found, "--three-quarter")?;
    let taken = measuring::proportions(
        &measuring::Given {
            named: front,
            drawing: &drawing(&folder, front)?,
        },
        &measuring::Given {
            named: three_quarter,
            drawing: &drawing(&folder, three_quarter)?,
        },
    )?;
    deliver_measured(&folder, &found, &taken.file()?, &taken.summary())
}

fn enlarge_image(arguments: &[String]) -> Result<()> {
    let (folder, found) = measuring(
        arguments,
        &[
            "--image", "--left", "--top", "--width", "--height", "--factor", "--grid", "--out",
        ],
    )?;
    let image = required(&found, "--image")?;
    let out = required(&found, "--out")?;
    let whole = |name: &str| -> Result<u32> {
        let value = required(&found, name)?;
        value.parse().map_err(|_| {
            Error::new(format!(
                "`{name}` needs a whole number of pixels, not below zero, and `{value}` is not one"
            ))
        })
    };
    let part = enlargement::Part {
        left: whole("--left")?,
        top: whole("--top")?,
        width: whole("--width")?,
        height: whole("--height")?,
        factor: whole("--factor")?,
        grid: match found.contains_key("--grid") {
            true => Some(whole("--grid")?),
            false => None,
        },
    };
    let enlarged = enlargement::enlarge(&folder.read(image)?, &part)
        .map_err(|error| Error::new(format!("{image}: {error}")))?;
    folder.write(out, &enlarged.png)?;
    println!(
        "Wrote `{out}`, {} by {} pixels: the part of `{image}` from {}, {} to {}, {}, enlarged {} times.",
        enlarged.width,
        enlarged.height,
        part.left,
        part.top,
        part.left + part.width,
        part.top + part.height,
        part.factor
    );
    if let Some(step) = part.grid {
        println!(
            "A grid line stands every {step} pixels of the picture and is labelled with its position in the picture: across the top, and down the left side."
        );
    }
    println!("{}", enlarged.way_back(&part));
    Ok(())
}

fn reference_check_sheet(arguments: &[String]) -> Result<()> {
    let (folder, found) = measuring(arguments, &["--sheet", "--out"])?;
    let sheet = required(&found, "--sheet")?;
    let out = required(&found, "--out")?;
    let description = reference_check_sheet::describe(&folder.read_text(sheet)?)
        .map_err(|error| Error::new(format!("{sheet}: {error}")))?;
    let mut pictures = BTreeMap::new();
    let mut characters = Vec::new();
    for (label, named) in &description.characters {
        let mut reads = Vec::new();
        for path in named {
            let record = reading(&folder, path)?;
            // A sheet shows the picture that was read, and no other.
            let bytes = folder.read(&record.picture.file)?;
            let digest = readings::sha256_hex(&bytes);
            if digest != record.picture.sha256 {
                return Err(Error::new(format!(
                    "{path}: the picture `{}` now has the digest {digest}, and the reading was started from {}: it is not the picture that was read",
                    record.picture.file, record.picture.sha256
                )));
            }
            pictures.insert(digest, bytes);
            reads.push(reference_check_sheet::Read {
                path: path.clone(),
                record,
            });
        }
        characters.push((label.clone(), reads));
    }
    let svg = reference_check_sheet::render(&description, &characters, &pictures)?;
    folder.write(out, svg.as_bytes())?;
    println!(
        "Wrote the reference check sheet `{out}` in the reference folder. It shows the drawings that were read: it stays there and is never committed."
    );
    Ok(())
}

fn score_reading(arguments: &[String]) -> Result<()> {
    let (folder, found) = measuring(arguments, &["--reading", "--known-positions", "--out"])?;
    let path = required(&found, "--reading")?;
    // The record of known positions is committed numbers, read from where it is named.
    let record = required(&found, "--known-positions")?;
    let score = known_positions::score(path, &reading(&folder, path)?, record, &read(record)?)?;
    deliver_measured(&folder, &found, &score.file(), &score.summary())
}

fn compare_proportions(arguments: &[String]) -> Result<()> {
    let found = options(arguments, &["--first", "--second"])?;
    let first = required(&found, "--first")?;
    let second = required(&found, "--second")?;
    print!(
        "{}",
        known_positions::compare_proportions(first, &read(first)?, second, &read(second)?)?
    );
    Ok(())
}

fn start_reading(arguments: &[String]) -> Result<()> {
    let (folder, found) = measuring(
        arguments,
        &["--image", "--view", "--faces", "--reader", "--out"],
    )?;
    let image = required(&found, "--image")?;
    let view = readings::view_named(required(&found, "--view")?)?;
    let faces = match found.get("--faces") {
        Some(&"right") => Some(readings::Faces::ViewersRight),
        Some(&"left") => Some(readings::Faces::ViewersLeft),
        Some(other) => {
            return Err(Error::new(format!(
                "`--faces` is `right` or `left`, the viewer's, and `{other}` is neither"
            )));
        }
        None => None,
    };
    let reader = required(&found, "--reader")?;
    let out = required(&found, "--out")?;
    let bytes = folder.read(image)?;
    let (width, height) =
        readings::picture_size(&bytes).map_err(|error| Error::new(format!("{image}: {error}")))?;
    let picture = readings::Picture {
        file: folder.name_of(&folder.path(image)?),
        sha256: readings::sha256_hex(&bytes),
        width: f64::from(width),
        height: f64::from(height),
    };
    let template = readings::template(&picture, view, faces, reader)?;
    if folder.path(out)?.exists() {
        return Err(Error::new(format!(
            "`{out}` is already there: a reading that has been started is not written over"
        )));
    }
    folder.write(out, template.as_bytes())?;
    println!(
        "Started a reading of `{}`, a {} view, in `{out}`: {} landmarks to place or to state as not seen.",
        picture.file,
        view.name(),
        readings::asked(view, faces).len()
    );
    Ok(())
}

fn check_reading(arguments: &[String]) -> Result<Ended> {
    let (folder, found) = measuring(arguments, &["--reading"])?;
    let path = required(&found, "--reading")?;
    let record = reading(&folder, path)?;
    let picture = folder.read(&record.picture.file).ok();
    let problems = record.problems(picture.as_deref());
    if problems.is_empty() {
        if record.set_aside {
            println!("The reading is complete: the drawing is set aside, and nothing is placed.");
        } else {
            println!(
                "The reading is complete: {} landmarks placed, {} stated as not seen.",
                record.placed.len(),
                record.not_seen.len()
            );
        }
        return Ok(Ended::Well);
    }
    println!("The reading is not complete:");
    for problem in &problems {
        println!("- {problem}");
    }
    Ok(Ended::CheckFailed)
}

/// The record of the final drawings, when `--final-record` names one: its path and its text.
fn final_record<'a>(found: &BTreeMap<&str, &'a str>) -> Result<Option<(&'a str, String)>> {
    match found.get("--final-record") {
        Some(path) => Ok(Some((path, read(path)?))),
        None => Ok(None),
    }
}
