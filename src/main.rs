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
    hard_case, jons_check, known_positions, measuring, overlay, proportions, read_back, readings,
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
      A set that carries a named correction is drawn with it, and the landmark file of the
      view it changes says so; a set that carries none is drawn as a flat projection.
      The options place the head on the page: the length of one unit, the height of the eye
      line, and where the upright axis the head turns about stands in each view. The same
      input gives the same bytes.

  maquette proportions-from-landmarks --front <file> --three-quarter <file> [--side <file>]
                                      [--out <file>]
      Take the landmark files of an armature back into a set of proportions: the proportions
      from the front view and the three-quarter view, and with --side any named correction
      from the side view, whose landmark file is the one that states it. Without --side a
      correction is not taken back, and the set written says so.

  maquette proportions-from-hand-made-views --record <file> [--final-record <file>]
                                            [--lower-face-set-back <share>] [--out <file>]
      Work out a set of proportions from the record of a hand-made armature's views,
      such as data/saffi/hand-made-views.toml. With --final-record, a record of final
      drawings such as data/saffi/final-drawings.toml, each of its landmarks replaces the
      record's landmark of the same name in the same view. With --lower-face-set-back, the
      set also carries the named correction lower_face_set_back, which draws the mouth and
      the chin further back in the side view only: its amounts are worked out from the
      set's own depths, at that share, from 0 to 1, of what draws the chin straight below
      the iris's centre. `maquette list-proportions` states the rule of each amount.

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
      drawing is read as written: one that moves its coordinates with a transform, draws
      a path with anything but the absolute M, L, C and Z, or draws on after Z without a
      new M, is refused.

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

  maquette skull-by-rule --proportions <file>
      Run the rule that builds the skull where nothing is drawn on a set of proportions,
      and set what it builds against the set's own skull, part by part: the centre's height,
      its depth, and the radius. It tests the rule on a head whose skull is drawn.

  maquette compare-proportions --first <file> --second <file>
      Set two files of proportions against each other, proportion by proportion: each
      difference in units and as a share of head height, with the heights, the widths, the
      depths and the conventions summed up apart. A file that lacks some proportions is
      compared in the ones it has.

  maquette reading-instructions
      Print the reading instructions: what a reader judges and places on a drawing.

Measuring. The commands below read and write in the reference folder, the one folder
outside every repository that holds reference images, readings and reference check sheets.
The environment variable MAQUETTE_REFERENCES names it; a folder inside a Git repository is
refused. Every path given to them is taken from that folder, and one that leaves it, by its
own steps or through a link, is refused; with no folder they stop and say so.
Where a view is asked for, <readings> is one reading, or the two readings of the drawing
with a comma between them: a landmark then stands midway between the two.
A reading is measured only when it passes what check-reading checks: complete, and its
picture still the one that was read. The two readings of a drawing are two files, by two
readers under two labels, and follow one revision of the reading instructions. A reading
under an earlier revision is measured, and what is written says which it followed.
A picture is a PNG or a JPEG. It is read as it was published and never converted: a reading
records the digest of the file it was read from.

  maquette start-reading --image <picture> --view front|three-quarter|side
                         [--faces right|left] --reader <label> --out <reading>
      Start a reading of a picture: a file with the picture's SHA-256 digest and size, the
      questions of the judgement, and a line for every landmark the view asks for.
      --faces is for a turned view: which way the head faces, from the viewer's side.

  maquette check-reading --reading <reading>
      Say what a reading lacks or gets wrong: an unanswered judgement, a landmark neither
      placed nor stated as not seen, a position outside the picture, a pair whose names
      may be exchanged, a picture that has changed. Exit status 0 when it is complete, 1
      when it is not.

  maquette enlarge-image --image <picture> --left <x> --top <y> --width <w> --height <h>
                         --factor <n> [--grid <step>] --out <picture>
      Enlarge a part of a picture a whole number of times, at most 16, each pixel as a
      block. --grid draws a line every so many pixels of the picture and labels it with its
      position in the picture. An enlargement with an edge above 1568 pixels is refused: an
      agent is shown it resized. The enlargement is written as a PNG.

  maquette agreement-of-readings --first <reading> --second <reading> [--out <file>]
      Set two readings of one drawing against each other: for every landmark both place,
      how far apart they put it as a share of head height, with every disagreement above 1%
      numbered, the landmarks only one places, and what the readers judge differently.

  maquette turn-from-readings --front <readings> --three-quarter <readings>
                              [--check <check>] [--out <file>]
      Work out the turn a three-quarter drawing shows, from how far the spacing of its left
      and right pairs narrows against the front view of the same character. Each pair's own
      turn and misfit is listed, and the readers' own estimates beside it. It fails, and
      writes nothing, where no turn gives what the readings show: a pair with no spacing in
      the front view, or pairs wider apart in the three-quarter view than in the front view.

  maquette fit-of-views --front <readings> [--three-quarter <readings>] [--side <readings>]
                        [--check <check>] [--out <file>]
      Say, as one figure, how far a character's views are from being drawings of one level,
      symmetric head: the largest misfit in level, in spacing against the turn, and from the
      front view's centre line, as a share of head height. The front view is set against a
      three-quarter view, a side view or both. A side view is compared in height, and with
      no three-quarter view there is no turn and no spacing to compare.

  maquette proportions-from-readings --front <readings>
                                     [--three-quarter <readings> | --side <readings>]
                                     [--check <check>] [--out <file>]
      Take a character's readings back into proportions. A front view alone gives heights
      and widths and no depth. With a three-quarter view, depth is taken at the turn the
      readings show. With a side view, depth is taken as the side view shows it: each view
      is measured in its own unit, eye line to chin, so the two may be drawn at two sizes,
      and a side view may face either way. A side view is asked for no temple, no cheek
      point and no jaw point, so it gives those no depth. The skull is built by rule from
      the ears and the temples, where the readings give them. The proportions the readings
      do not give are listed with the reason; when all are given, the file is a set of
      proportions that `maquette draw` reads.

  maquette score-reading --reading <reading> --known-positions <record> [--out <file>]
      Score a reading against a drawing whose true positions are on record, such as
      data/saffi/hand-made-views.toml: the reader's error as a share of head height, by
      kind of landmark, with what cannot be seen in the picture kept apart. The record is
      read from where it is named. The reading is scored as it stands: what it leaves out
      is listed, and its picture is not checked.

  maquette known-positions-as-reading --known-positions <record> --view front|three-quarter
                                      --image <picture> --out <reading>
      Write a record's own positions as a reading of the picture: what a reader who placed
      every visible point exactly would write. It is not a reading. The measuring commands
      run on it show what they make of exact positions.

  maquette reference-check-sheet --sheet <description> --out <svg>
      Render a reference check sheet from a description: a TOML file with a `note`, a `look`
      (which of Jon's looks it is, or that none is asked) and, for each row, a
      [[character]] with a `label` and the `readings` of its drawings. Each character's
      views stand side by side with the points read, what was set aside and why, and the
      fit. Where two readings of a drawing cannot be compared, its card says so and the
      sheet is still rendered. The sheet is one SVG that holds its drawings, and stays in
      the reference folder. Where one picture holds two heads of a character, each drawing
      is named by itself: a [[character.drawing]] with its own `readings` and, in words,
      which `picture` and head it is. A character's `about` says which character of which
      work it is. Each card says which picture it is and how high its head is in pixels,
      and the header says what the numbers are set against.

  maquette start-jons-check --sheet <description> --out <check>
      Start the record of Jon's check of a sheet: a TOML file with every character of the
      sheet, each of its drawings, and each reading with the digest of its file, to be
      filled in with what Jon said. A `verdict` for each character, accepted or rejected;
      for a drawing, a `ruling` where he overrules its readers (measure, or set aside) and
      the landmarks he marks as `off`; for a character, the proportions he marks as off.

  maquette jons-check --check <check>
      Say what a record of Jon's check does and what it lacks: each character accepted or
      rejected, each drawing measured or not and from which readings, what his marks leave
      out, and the number of accepted works for each view, with a view under five reported
      as too thin. Exit status 0 when the record is complete, 1 when it is not.

turn-from-readings, fit-of-views and proportions-from-readings take `--check <check>`, the
record of Jon's check. With it only the readings of a character he accepted are measured,
each drawing as his record rules, with what he marked as off left out; a reading that has
changed since the record was started is refused. Without it the readings are measured as
their readers judged them, and what is written says that Jon has not checked them.
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
        "known-positions-as-reading" => known_positions_as_reading(rest)?,
        "reference-check-sheet" => reference_check_sheet(rest)?,
        "start-jons-check" => start_jons_check(rest)?,
        "jons-check" => return jons_check_command(rest),
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
    let found = options(
        arguments,
        &["--front", "--three-quarter", "--side", "--out"],
    )?;
    let front = armature::read_landmark_file(&read(required(&found, "--front")?)?)?;
    let three_quarter = armature::read_landmark_file(&read(required(&found, "--three-quarter")?)?)?;
    let mut set = head_model::proportions_from_views(&front, &three_quarter)?;
    let opening = match found.get("--side") {
        Some(side) => {
            let in_side = |error: Error| Error::new(format!("{side}: {error}"));
            let carried = armature::read_side_view_corrections(&read(side)?).map_err(in_side)?;
            for (name, amounts) in carried {
                set = set.with_correction(&name, amounts).map_err(in_side)?;
            }
            "\
# A set of proportions taken back from the landmark files of an armature's three views,
# by `maquette proportions-from-landmarks`: the proportions from the front view and the
# three-quarter view, and any correction from the side view, whose landmark file is the one
# that states it. `maquette list-proportions` defines each proportion and each correction.
"
        }
        None => {
            "\
# A set of proportions taken back from two landmark files, a front view and a three-quarter view,
# by `maquette proportions-from-landmarks`. `maquette list-proportions` defines each proportion.
#
# No side view was given, so a correction was not taken back: the side view's landmark file is
# the one that states it. If the armature carries a correction, this is not the whole set it
# was drawn from. Give the side view's landmark file with `--side` to take it back as well.
"
        }
    };
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
    const SET_BACK: &str = "--lower-face-set-back";
    let found = options(
        arguments,
        &["--record", "--final-record", SET_BACK, "--out"],
    )?;
    let record_path = required(&found, "--record")?;
    let final_record = final_record(&found)?;
    let lower_face_set_back = match found.get(SET_BACK) {
        // The head model says what a share is; a value it refuses is named here by its option.
        Some(value) => match value.parse::<f64>().map(head_model::set_back_share) {
            Ok(Ok(share)) => Some(share),
            _ => {
                return Err(Error::new(format!(
                    "`{SET_BACK}` needs a share from 0 to 1, and `{value}` is not one"
                )));
            }
        },
        None => None,
    };
    let hand_made = hand_made_views::read(
        record_path,
        &read(record_path)?,
        final_record
            .as_ref()
            .map(|(path, text)| hand_made_views::FinalRecord {
                path,
                text: text.as_str(),
            }),
    )?;
    deliver(
        &found,
        &hand_made.proportions_file(record_path, lower_face_set_back)?,
    )
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

/// A reading that is to be measured, with the bytes of its picture: read as [`reading`]
/// reads it, and held to what `maquette check-reading` checks, against the picture as it is
/// now. A reading that does not pass is not measured.
fn checked_reading(folder: &Folder, path: &str) -> Result<(readings::Record, Option<Vec<u8>>)> {
    let record = reading(folder, path)?;
    let picture = folder.read(&record.picture.file).ok();
    record
        .checked(picture.as_deref())
        .map_err(|error| Error::new(format!("{path}: {error}")))?;
    Ok((record, picture))
}

/// Refuses one file named twice where the readings of a drawing are named, under one
/// spelling of its path or two: a drawing's two readings are two readers' files.
fn separate_files(folder: &Folder, paths: &[&str]) -> Result<()> {
    let mut whole = Vec::new();
    for path in paths {
        let resolved = folder.path(path)?;
        if whole.contains(&resolved) {
            return Err(Error::new(format!(
                "`{path}` is named twice: a drawing is read twice, by two readers, and one reading named twice is not its two readings"
            )));
        }
        whole.push(resolved);
    }
    Ok(())
}

/// A view as a measuring command is given it: one reading, or the two readings of the
/// drawing with a comma between them.
fn drawing(folder: &Folder, named: &str) -> Result<readings::Drawing> {
    let paths: Vec<&str> = named.split(',').map(str::trim).collect();
    separate_files(folder, &paths)?;
    let records = paths
        .iter()
        .map(|path| Ok(checked_reading(folder, path)?.0))
        .collect::<Result<Vec<_>>>()?;
    readings::Drawing::of(&records)
}

/// The views a measuring command was given, read: each with the path or paths it was named
/// by, and what Jon's check says of them when `--check` names its record.
struct ViewsGiven<'a> {
    drawings: BTreeMap<&'static str, (&'a str, readings::Drawing)>,
    /// What Jon's check says of the readings. None when no record was given.
    checked: Option<String>,
    /// The proportions Jon marked as off for the character.
    proportions_off: Vec<String>,
}

impl ViewsGiven<'_> {
    /// A view the command may be given, as measuring takes it.
    fn get(&self, option: &str) -> Option<measuring::Given<'_>> {
        self.drawings
            .get(option)
            .map(|(named, drawing)| measuring::Given { named, drawing })
    }

    /// A view the command must be given.
    fn need(&self, option: &str) -> Result<measuring::Given<'_>> {
        self.get(option).ok_or_else(|| {
            Error::new(format!(
                "`{option}` is missing. Run `maquette help` for the options."
            ))
        })
    }

    /// Says in what was worked out what Jon's check says of its readings.
    fn mark(&self, read: &mut measuring::ReadFrom) {
        if let Some(checked) = &self.checked {
            read.checked = checked.clone();
        }
    }
}

/// Reads the views a measuring command was given. With `--check`, only the readings of a
/// character Jon accepted are measured, each drawing as his record rules, with the
/// landmarks he marked as off left out.
fn views_given<'a>(
    folder: &Folder,
    found: &BTreeMap<&str, &'a str>,
    options: &[&'static str],
) -> Result<ViewsGiven<'a>> {
    let check = match found.get("--check") {
        Some(path) => Some((*path, check_record(folder, path)?)),
        None => None,
    };
    let mut drawings = BTreeMap::new();
    let mut character: Option<&jons_check::Checked> = None;
    let mut notes = Vec::new();
    for option in options {
        let Some(named) = found.get(option).copied() else {
            continue;
        };
        let Some((check_path, check)) = &check else {
            drawings.insert(*option, (named, drawing(folder, named)?));
            continue;
        };
        let view = option.trim_start_matches("--");
        let paths: Vec<&str> = named.split(',').map(str::trim).collect();
        separate_files(folder, &paths)?;
        let whole = |path: &str| folder.path(path).ok();
        let first = whole(paths[0]);
        let (of, checked) = check
            .drawing_of(&|recorded| first.is_some() && whole(recorded) == first)
            .ok_or_else(|| {
                Error::new(format!(
                    "`{}` is not a reading of the record of Jon's check `{check_path}`: with that record, only the readings he checked are measured",
                    paths[0]
                ))
            })?;
        match character {
            Some(other) if other.label != of.label => {
                return Err(Error::new(format!(
                    "the views named are of two characters of the record of Jon's check, `{}` and `{}`: a character's own views are measured together",
                    other.label, of.label
                )));
            }
            _ => character = Some(of),
        }
        match of.verdict {
            Some(jons_check::Verdict::Accepted) => {}
            Some(jons_check::Verdict::Rejected) => {
                return Err(Error::new(format!(
                    "Jon rejected the character `{}`{}, so its readings are not measured",
                    of.label,
                    if of.why.is_empty() {
                        String::new()
                    } else {
                        format!(" (\"{}\")", of.why)
                    }
                )));
            }
            None => {
                return Err(Error::new(format!(
                    "the character `{}` has no verdict in the record of Jon's check `{check_path}`: it is not yet checked, so its readings are not measured",
                    of.label
                )));
            }
        }
        let mut records = Vec::new();
        for path in &paths {
            let recorded = checked
                .readings
                .iter()
                .find(|recorded| whole(&recorded.path) == whole(path))
                .ok_or_else(|| {
                    Error::new(format!(
                        "`{path}` is not a reading of the same drawing as `{}` in the record of Jon's check `{check_path}`",
                        paths[0]
                    ))
                })?;
            let (sha256, _) = reading_now(folder, path)?;
            if sha256 != recorded.sha256 {
                return Err(Error::new(format!(
                    "`{path}` has changed since the record of Jon's check `{check_path}` was started: it is not the reading he checked, so it is not measured"
                )));
            }
            records.push(checked_reading(folder, path)?.0);
        }
        match checked.outcome(&records) {
            jons_check::Outcome::Measured { records, said, .. } => {
                notes.push(format!("the {view} view is {said}"));
                drawings.insert(*option, (named, readings::Drawing::of(&records)?));
            }
            jons_check::Outcome::NotMeasured(why) => {
                return Err(Error::new(format!(
                    "the {view} view of `{}` is not measured: {why}",
                    of.label
                )));
            }
        }
    }
    let (checked, proportions_off) = match (&check, character) {
        (Some((check_path, _)), Some(of)) => (
            Some(format!(
                "By the record of Jon's check `{check_path}`, the character `{}` is accepted: {}.",
                of.label,
                notes.join("; ")
            )),
            of.proportions_off.clone(),
        ),
        _ => (None, Vec::new()),
    };
    Ok(ViewsGiven {
        drawings,
        checked,
        proportions_off,
    })
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
    separate_files(&folder, &[first, second])?;
    let agreement = measuring::agreement(
        first,
        &checked_reading(&folder, first)?.0,
        second,
        &checked_reading(&folder, second)?.0,
    )?;
    deliver_measured(&folder, &found, &agreement.file(), &agreement.summary())
}

fn turn_from_readings(arguments: &[String]) -> Result<()> {
    let (folder, found) = measuring(
        arguments,
        &["--front", "--three-quarter", "--check", "--out"],
    )?;
    let views = views_given(&folder, &found, &["--front", "--three-quarter"])?;
    let mut turn = measuring::turn(&views.need("--front")?, &views.need("--three-quarter")?)?;
    views.mark(&mut turn.read);
    deliver_measured(&folder, &found, &turn.file(), &turn.summary())
}

fn fit_of_views(arguments: &[String]) -> Result<()> {
    let (folder, found) = measuring(
        arguments,
        &["--front", "--three-quarter", "--side", "--check", "--out"],
    )?;
    let views = views_given(&folder, &found, &["--front", "--three-quarter", "--side"])?;
    let mut fit = measuring::fit(
        &views.need("--front")?,
        views.get("--three-quarter").as_ref(),
        views.get("--side").as_ref(),
    )?;
    views.mark(&mut fit.read);
    deliver_measured(&folder, &found, &fit.file(), &fit.summary())
}

fn proportions_from_readings(arguments: &[String]) -> Result<()> {
    let (folder, found) = measuring(
        arguments,
        &["--front", "--three-quarter", "--side", "--check", "--out"],
    )?;
    let views = views_given(&folder, &found, &["--front", "--three-quarter", "--side"])?;
    let mut taken = measuring::proportions(
        &views.need("--front")?,
        views.get("--three-quarter").as_ref(),
        views.get("--side").as_ref(),
    )?;
    views.mark(&mut taken.read);
    taken.leave_out(&views.proportions_off);
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
    let (description, characters, pictures) = sheet_readings(&folder, sheet)?;
    let svg = reference_check_sheet::render(&description, &characters, &pictures)?;
    folder.write(out, svg.as_bytes())?;
    println!(
        "Wrote the reference check sheet `{out}` in the reference folder. It shows the drawings that were read: it stays there and is never committed."
    );
    Ok(())
}

/// What a sheet description names, read: for each of its characters the readings of each
/// of its drawings, and every picture by its digest.
type SheetReadings = (
    reference_check_sheet::Description,
    Vec<Vec<Vec<reference_check_sheet::Read>>>,
    BTreeMap<String, Vec<u8>>,
);

fn sheet_readings(folder: &Folder, sheet: &str) -> Result<SheetReadings> {
    let description = reference_check_sheet::describe(&folder.read_text(sheet)?)
        .map_err(|error| Error::new(format!("{sheet}: {error}")))?;
    let mut pictures = BTreeMap::new();
    let mut characters = Vec::new();
    for character in &description.characters {
        let label = &character.label;
        let of_character = |error: Error| Error::new(format!("the character `{label}`: {error}"));
        let paths: Vec<&str> = character
            .drawings
            .iter()
            .flat_map(|drawing| drawing.readings.iter().map(String::as_str))
            .collect();
        separate_files(folder, &paths).map_err(of_character)?;
        let mut drawings = Vec::new();
        for drawing in &character.drawings {
            let mut reads = Vec::new();
            for path in &drawing.readings {
                // A sheet shows the picture that was read, and no other: the check says so.
                let (record, picture) = checked_reading(folder, path).map_err(of_character)?;
                if let Some(bytes) = picture {
                    pictures.insert(record.picture.sha256.clone(), bytes);
                }
                reads.push(reference_check_sheet::Read {
                    path: path.clone(),
                    record,
                });
            }
            drawings.push(reads);
        }
        characters.push(drawings);
    }
    Ok((description, characters, pictures))
}

/// The SHA-256 digest of a reading's file as it is now, with the reading.
fn reading_now(folder: &Folder, path: &str) -> Result<(String, readings::Record)> {
    let bytes = folder.read(path)?;
    let text = String::from_utf8(bytes.clone()).map_err(|_| {
        Error::new(format!(
            "`{path}` in the reference folder is not a text file"
        ))
    })?;
    let record =
        readings::Record::read(&text).map_err(|error| Error::new(format!("{path}: {error}")))?;
    Ok((readings::sha256_hex(&bytes), record))
}

fn start_jons_check(arguments: &[String]) -> Result<()> {
    let (folder, found) = measuring(arguments, &["--sheet", "--out"])?;
    let sheet = required(&found, "--sheet")?;
    let out = required(&found, "--out")?;
    let (description, characters, _) = sheet_readings(&folder, sheet)?;
    let mut digests = BTreeMap::new();
    for read in characters.iter().flatten().flatten() {
        digests.insert(read.path.clone(), reading_now(&folder, &read.path)?.0);
    }
    let mut to_check = Vec::new();
    for (character, reads) in description.characters.iter().zip(&characters) {
        let drawings = reference_check_sheet::on_sheet(character, reads)?
            .into_iter()
            .map(|drawing| jons_check::ToCheck {
                card: drawing.card,
                picture: drawing.picture,
                readings: drawing
                    .readings
                    .iter()
                    .map(|read| {
                        (
                            read.path.as_str(),
                            digests[&read.path].clone(),
                            &read.record,
                        )
                    })
                    .collect(),
            })
            .collect();
        to_check.push((character.label.as_str(), drawings));
    }
    if folder.path(out)?.exists() {
        return Err(Error::new(format!(
            "`{out}` is already there: a record of Jon's check that has been started is not written over"
        )));
    }
    folder.write(out, jons_check::template(sheet, &to_check).as_bytes())?;
    println!(
        "Started the record of Jon's check in `{out}`: {} to give a verdict on. Write in it what Jon said, and nothing else; `maquette jons-check --check {out}` then says what it does. It stays in the reference folder and is never committed.",
        match to_check.len() {
            1 => "1 character".to_string(),
            count => format!("{count} characters"),
        }
    );
    Ok(())
}

/// A record of Jon's check of the reference folder, read. Its path is named in what goes
/// wrong.
fn check_record(folder: &Folder, path: &str) -> Result<jons_check::Check> {
    jons_check::Check::read(&folder.read_text(path)?)
        .map_err(|error| Error::new(format!("{path}: {error}")))
}

fn jons_check_command(arguments: &[String]) -> Result<Ended> {
    let (folder, found) = measuring(arguments, &["--check"])?;
    let path = required(&found, "--check")?;
    let check = check_record(&folder, path)?;
    let said = check.said(path, &|reading| reading_now(&folder, reading));
    print!("{}", said.text);
    Ok(if said.complete {
        Ended::Well
    } else {
        Ended::CheckFailed
    })
}

fn score_reading(arguments: &[String]) -> Result<()> {
    let (folder, found) = measuring(arguments, &["--reading", "--known-positions", "--out"])?;
    let path = required(&found, "--reading")?;
    // The record of known positions is committed numbers, read from where it is named.
    let record = required(&found, "--known-positions")?;
    let score = known_positions::score(path, &reading(&folder, path)?, record, &read(record)?)?;
    deliver_measured(&folder, &found, &score.file(), &score.summary())
}

fn known_positions_as_reading(arguments: &[String]) -> Result<()> {
    let (folder, found) = measuring(
        arguments,
        &["--known-positions", "--view", "--image", "--out"],
    )?;
    let record = required(&found, "--known-positions")?;
    let view = readings::view_named(required(&found, "--view")?)?;
    let image = required(&found, "--image")?;
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
    let text = known_positions::as_reading(record, &read(record)?, view, &picture)?;
    folder.write(out, text.as_bytes())?;
    println!(
        "Wrote the record's positions for the {} view as a reading in `{out}`. It is not a reading: nobody read it from a picture.",
        view.name()
    );
    Ok(())
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
