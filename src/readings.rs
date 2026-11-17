//! The record of a reading: one reader's placement of the landmarks on one drawing, with its
//! judgement of the view, the turn and whether the head is level.
//!
//! This module hides what a reading holds and how it is written: which landmarks a view asks
//! for, under which names, what makes a reading complete, and how its positions become the
//! positions the head model reads. A reading is a TOML file in the reference folder. It is
//! never committed.
//!
//! A reader places what is drawn. What the list of proportions builds by rule is not asked
//! for: the skull, the middle of an eye opening, the centre of an iris, and the jaw line
//! above the nose level.

use std::collections::BTreeMap;

use sha2::{Digest, Sha256};

use crate::head_model;
use crate::toml_text::quoted;
use crate::views::View;
use crate::{Error, Result};

/// The reading instructions, as an agent is given them. The program carries them so that a
/// reading can say which revision it followed, and so that a reader needs nothing else.
pub const INSTRUCTIONS: &str = include_str!("../docs/agents/reading-instructions.md");

/// The revision the reading instructions state for themselves.
pub fn instructions_revision() -> Result<&'static str> {
    INSTRUCTIONS
        .lines()
        .find_map(|line| line.strip_prefix("Revision: "))
        .map(str::trim)
        .ok_or_else(|| Error::new("the reading instructions state no `Revision: `"))
}

/// Which way a turned head faces, from the viewer's side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Faces {
    ViewersRight,
    ViewersLeft,
}

impl Faces {
    pub fn words(self) -> &'static str {
        match self {
            Faces::ViewersRight => "the viewer's right",
            Faces::ViewersLeft => "the viewer's left",
        }
    }

    /// The character's own side that is toward the viewer.
    pub fn near(self) -> &'static str {
        match self {
            Faces::ViewersRight => "right",
            Faces::ViewersLeft => "left",
        }
    }

    fn read(words: &str) -> Option<Faces> {
        match words {
            "the viewer's right" | "right" => Some(Faces::ViewersRight),
            "the viewer's left" | "left" => Some(Faces::ViewersLeft),
            _ => None,
        }
    }
}

/// A view by the name a reading or a command gives it.
pub fn view_named(name: &str) -> Result<View> {
    View::ALL
        .into_iter()
        .find(|view| view.name() == name)
        .ok_or_else(|| {
            Error::new(format!(
                "`{name}` is no view: a view is front, three-quarter or side"
            ))
        })
}

/// One landmark a view asks a reader for.
#[derive(Debug, Clone, PartialEq)]
pub struct Asked {
    pub name: String,
    /// The kind of landmark: the feature it belongs to. Errors are reported by kind.
    pub kind: &'static str,
    /// What to place, in one line. The reading instructions say more.
    pub what: &'static str,
}

struct Ask {
    stem: &'static str,
    point: &'static str,
    kind: &'static str,
    what: &'static str,
}

const fn ask(
    stem: &'static str,
    point: &'static str,
    kind: &'static str,
    what: &'static str,
) -> Ask {
    Ask {
        stem,
        point,
        kind,
        what,
    }
}

/// The landmarks of the centre line, asked for in every view.
const ON_CENTRE_LINE: &[Ask] = &[
    ask("hair-top", "", "hair", "The highest point of the hair."),
    ask("chin", "", "chin", "The lowest point of the face outline."),
    ask(
        "nose-mark-bottom",
        "",
        "nose mark",
        "The lowest point of the drawn nose mark.",
    ),
    ask(
        "mouth-line-centre",
        "",
        "mouth line",
        "The middle of the mouth line, where the centre line crosses it.",
    ),
];

/// The landmarks of each side, in the order a reader works down the head.
const ON_EACH_SIDE: &[Ask] = &[
    ask(
        "eye",
        "outer-corner",
        "eye opening",
        "The point of the eye opening furthest from the centre line.",
    ),
    ask(
        "eye",
        "inner-corner",
        "eye opening",
        "The point of the eye opening nearest the centre line.",
    ),
    ask(
        "eye",
        "opening-top",
        "eye opening",
        "The highest point of the eye opening, under the upper lid.",
    ),
    ask(
        "eye",
        "opening-bottom",
        "eye opening",
        "The lowest point of the eye opening.",
    ),
    ask(
        "iris",
        "top",
        "iris",
        "The highest point of the iris as drawn, where the lid may already cut it.",
    ),
    ask(
        "iris",
        "bottom",
        "iris",
        "The lowest point of the iris as drawn.",
    ),
    ask(
        "iris",
        "outer-extent",
        "iris",
        "The point of the iris furthest from the centre line.",
    ),
    ask(
        "iris",
        "inner-extent",
        "iris",
        "The point of the iris nearest the centre line.",
    ),
    ask(
        "lid-line",
        "inner-end",
        "lid line",
        "The middle of the drawn upper lid line's thickness, at its end nearest the centre line.",
    ),
    ask(
        "lid-line",
        "crest",
        "lid line",
        "The middle of the drawn upper lid line's thickness, where the line is highest.",
    ),
    ask(
        "lid-line",
        "outer-end",
        "lid line",
        "The middle of the drawn upper lid line's thickness, at its end furthest from the centre line.",
    ),
    ask(
        "brow-line",
        "inner-end",
        "brow line",
        "The middle of the drawn brow's thickness, at its end nearest the centre line.",
    ),
    ask(
        "brow-line",
        "outer-end",
        "brow line",
        "The middle of the drawn brow's thickness, at its end furthest from the centre line.",
    ),
    ask(
        "nose-mark",
        "corner",
        "nose mark",
        "The point of the drawn nose mark furthest toward this side.",
    ),
    ask(
        "mouth-line",
        "end",
        "mouth line",
        "The end of the mouth line on this side.",
    ),
    ask(
        "ear",
        "base-outer",
        "ear",
        "Of the two points where the ear's outline meets the head, the outer one; on an ear at the side of the head, the upper one.",
    ),
    ask(
        "ear",
        "base-inner",
        "ear",
        "Of the two points where the ear's outline meets the head, the inner one; on an ear at the side of the head, the lower one.",
    ),
    ask("ear", "top", "ear", "The highest point of the ear."),
    ask(
        "ear",
        "outer-extent",
        "ear",
        "The point of the ear furthest from the centre line.",
    ),
    ask(
        "ear",
        "inner-extent",
        "ear",
        "The point of the ear nearest the centre line.",
    ),
    ask(
        "face-outline",
        "top",
        "face outline",
        "The temple: the top end of the face outline on this side, where the hair leaves it showing.",
    ),
    ask(
        "face-outline",
        "at-eye-level",
        "face outline",
        "Where the face outline on this side crosses the eye line.",
    ),
    ask(
        "face-outline",
        "at-nose-level",
        "face outline",
        "Where the face outline on this side crosses the nose level.",
    ),
    ask(
        "face-outline",
        "at-mouth-level",
        "face outline",
        "Where the face outline on this side crosses the mouth level.",
    ),
    ask(
        "face-outline",
        "jaw",
        "face outline",
        "The jaw point: between the mouth level and the chin, where the jaw turns into the chin.",
    ),
    ask(
        "hair",
        "extent",
        "hair",
        "The point of the hair furthest out on this side.",
    ),
    ask(
        "neck",
        "base",
        "neck",
        "Where the neck ends on this side: where its edge meets the shoulder.",
    ),
    ask(
        "shoulder",
        "point",
        "shoulder line",
        "The outer end of the shoulder on this side, where its top turns down into the arm.",
    ),
];

/// What a side view shows that has no left or right: the hair and the neck from front to back.
const IN_SIDE_VIEW: &[Ask] = &[
    ask(
        "hair-back-extent",
        "",
        "hair",
        "The point of the hair furthest toward the back of the head.",
    ),
    ask(
        "hair-front-extent",
        "",
        "hair",
        "The point of the hair furthest toward the front of the head.",
    ),
    ask(
        "neck-back-base",
        "",
        "neck",
        "Where the back of the neck ends: where its edge meets the shoulder.",
    ),
    ask(
        "neck-front-base",
        "",
        "neck",
        "Where the front of the neck ends: where its edge meets the chest.",
    ),
];

/// The points of the near side's outline that a turned view does not ask for: above the nose
/// level the list of proportions builds the jaw line by rule, and the jaw point belongs to
/// the front of the face, which a side view shows as the centre line.
fn asked_of_near_outline(view: View, point: &str) -> bool {
    match view {
        View::Front => true,
        View::ThreeQuarter => !matches!(point, "top" | "at-eye-level"),
        View::Side => matches!(point, "at-nose-level" | "at-mouth-level"),
    }
}

/// The landmarks a view asks a reader for, in the order of a reading. `faces` is which way a
/// turned head faces; a front view has none.
pub fn asked(view: View, faces: Option<Faces>) -> Vec<Asked> {
    let near = faces.unwrap_or(Faces::ViewersRight).near();
    let mut out: Vec<Asked> = ON_CENTRE_LINE
        .iter()
        .map(|ask| Asked {
            name: ask.stem.to_string(),
            kind: ask.kind,
            what: ask.what,
        })
        .collect();
    for side in ["right", "left"] {
        // A side view shows the near side only, and the hair and the neck from front to back.
        if view == View::Side && side != near {
            continue;
        }
        for ask in ON_EACH_SIDE {
            let is_near = view != View::Front && side == near;
            if ask.stem == "face-outline" && is_near && !asked_of_near_outline(view, ask.point) {
                continue;
            }
            if view == View::Side && matches!(ask.stem, "hair" | "neck") {
                continue;
            }
            out.push(Asked {
                name: format!("{}-{side}-{}", ask.stem, ask.point),
                kind: ask.kind,
                what: ask.what,
            });
        }
    }
    if view == View::Side {
        out.extend(IN_SIDE_VIEW.iter().map(|ask| Asked {
            name: ask.stem.to_string(),
            kind: ask.kind,
            what: ask.what,
        }));
    }
    out
}

/// The picture a reading was read from.
#[derive(Debug, Clone, PartialEq)]
pub struct Picture {
    /// Its path, from the reference folder.
    pub file: String,
    pub sha256: String,
    pub width: f64,
    pub height: f64,
}

/// The SHA-256 digest of a file's bytes, in lower-case hex.
pub fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// How wide and how high a picture is, in pixels.
pub fn picture_size(bytes: &[u8]) -> Result<(u32, u32)> {
    image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|error| Error::new(format!("the picture cannot be read: {error}")))?
        .into_dimensions()
        .map_err(|error| {
            Error::new(format!(
                "the picture cannot be read as a PNG, the one format maquette reads: {error}"
            ))
        })
}

/// What a reference is: said by whoever hands over the drawing, except the manner.
pub const REFERENCE_FIELDS: &[(&str, &str)] = &[
    ("work", "The work the drawing is from."),
    ("designer", "Its character designer."),
    (
        "source",
        "The official publication the picture was taken from.",
    ),
    (
        "age_band",
        "The character's age band, such as adult or child.",
    ),
    ("sex", "The character's sex."),
    ("species", "The character's species."),
    (
        "manner",
        "The manner it is drawn in, in a few plain words and never a name: this one is the reader's to say.",
    ),
];

/// A reading, as its file gives it.
#[derive(Debug, Clone)]
pub struct Record {
    /// The revision of the reading instructions it says it follows.
    pub instructions: String,
    pub reader: String,
    /// How the drawing was looked at: as it is, or which enlargements were made.
    pub aids: String,
    pub picture: Picture,
    pub reference: BTreeMap<String, String>,
    pub view: View,
    pub view_why: String,
    pub faces: Option<Faces>,
    /// The reader's own estimate of the turn. The turn a drawing shows is worked out from
    /// the positions; this is the judgement Jon checks.
    pub turn_degrees: Option<f64>,
    pub turn_why: String,
    pub level: Option<bool>,
    pub level_why: String,
    pub set_aside: bool,
    pub set_aside_why: String,
    /// Positions in pixels of the picture, under the names the view asks for.
    pub placed: BTreeMap<String, [f64; 2]>,
    /// Landmarks the reader could not place, each with the reason.
    pub not_seen: BTreeMap<String, String>,
    /// What the file gives that is not right in itself: a wrong type, a value outside its
    /// choices. Found while reading, reported by [`Record::problems`].
    faults: Vec<String>,
}

fn table<'a>(file: &'a toml::Table, key: &str) -> Result<&'a toml::Table> {
    file.get(key)
        .and_then(toml::Value::as_table)
        .ok_or_else(|| Error::new(format!("the reading has no [{key}] table")))
}

fn words(table: &toml::Table, key: &str) -> String {
    table
        .get(key)
        .and_then(toml::Value::as_str)
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn number(value: Option<&toml::Value>) -> Option<f64> {
    match value {
        Some(toml::Value::Float(number)) => Some(*number),
        Some(toml::Value::Integer(number)) => Some(*number as f64),
        _ => None,
    }
}

impl Record {
    /// Reads the text of a reading. A reading that is not finished is still read: what it
    /// lacks is [`Record::problems`]'s to say.
    pub fn read(text: &str) -> Result<Record> {
        let file: toml::Table = text
            .parse()
            .map_err(|error| Error::new(format!("the reading is not valid TOML: {error}")))?;
        let mut faults = Vec::new();

        let reader = table(&file, "reader")?;
        let image = table(&file, "image")?;
        let picture = Picture {
            file: words(image, "file"),
            sha256: words(image, "sha256"),
            width: number(image.get("width")).unwrap_or(0.0),
            height: number(image.get("height")).unwrap_or(0.0),
        };
        if picture.file.is_empty() || picture.sha256.is_empty() {
            return Err(Error::new(
                "the reading's [image] does not name the picture and its sha256 digest",
            ));
        }
        if !(picture.width > 0.0 && picture.height > 0.0) {
            return Err(Error::new(
                "the reading's [image] does not give the picture's width and height",
            ));
        }
        let reference = file.get("reference").and_then(toml::Value::as_table);
        let reference = REFERENCE_FIELDS
            .iter()
            .map(|(field, _)| {
                let said = reference
                    .map(|table| words(table, field))
                    .unwrap_or_default();
                (field.to_string(), said)
            })
            .collect();

        let judgement = table(&file, "judgement")?;
        let view = view_named(&words(judgement, "view"))
            .map_err(|error| Error::new(format!("the reading's [judgement]: {error}")))?;
        let faces = match judgement.get("faces").and_then(toml::Value::as_str) {
            Some(said) => {
                let faces = Faces::read(said.trim());
                if faces.is_none() {
                    faults.push(format!(
                        "`faces` is \"{said}\": a turned head faces \"the viewer's right\" or \"the viewer's left\""
                    ));
                }
                faces
            }
            None => None,
        };
        let level = match judgement.get("level") {
            Some(toml::Value::Boolean(level)) => Some(*level),
            Some(_) => {
                faults.push("`level` is neither true nor false".to_string());
                None
            }
            None => None,
        };
        let set_aside = match judgement.get("set_aside") {
            Some(toml::Value::Boolean(set_aside)) => *set_aside,
            Some(_) => {
                faults.push("`set_aside` is neither true nor false".to_string());
                false
            }
            None => false,
        };

        let mut placed = BTreeMap::new();
        if let Some(landmarks) = file.get("landmarks") {
            let landmarks = landmarks
                .as_table()
                .ok_or_else(|| Error::new("the reading's `landmarks` is not a table"))?;
            for (name, entry) in landmarks {
                let at = entry
                    .as_table()
                    .and_then(|entry| Some([number(entry.get("x"))?, number(entry.get("y"))?]));
                match at {
                    Some(at) if at[0].is_finite() && at[1].is_finite() => {
                        placed.insert(name.clone(), at);
                    }
                    _ => faults.push(format!(
                        "`{name}` is not a position: write it as {name} = {{ x = <number>, y = <number> }}"
                    )),
                }
            }
        }
        let mut not_seen = BTreeMap::new();
        if let Some(entries) = file.get("not_seen") {
            let entries = entries
                .as_table()
                .ok_or_else(|| Error::new("the reading's `not_seen` is not a table"))?;
            for (name, why) in entries {
                not_seen.insert(
                    name.clone(),
                    why.as_str().unwrap_or_default().trim().to_string(),
                );
            }
        }

        Ok(Record {
            instructions: words(&file, "instructions"),
            reader: words(reader, "name"),
            aids: words(reader, "aids"),
            picture,
            reference,
            view,
            view_why: words(judgement, "view_why"),
            faces,
            turn_degrees: number(judgement.get("turn_degrees")),
            turn_why: words(judgement, "turn_why"),
            level,
            level_why: words(judgement, "level_why"),
            set_aside,
            set_aside_why: words(judgement, "set_aside_why"),
            placed,
            not_seen,
            faults,
        })
    }

    /// The landmarks this reading's view asks for.
    pub fn asked(&self) -> Vec<Asked> {
        asked(self.view, self.faces)
    }

    /// What the reading lacks or gets wrong, in words its reader can act on. Empty when the
    /// reading is complete. `picture` is the picture's bytes as they are now, to check that
    /// it is still the picture that was read.
    pub fn problems(&self, picture: Option<&[u8]>) -> Vec<String> {
        let mut problems = self.faults.clone();
        match instructions_revision() {
            Ok(revision) if self.instructions == revision => {}
            Ok(revision) => problems.push(format!(
                "`instructions` is \"{}\", and the reading instructions are at revision \"{revision}\": a reading follows the instructions as they stand",
                self.instructions
            )),
            Err(error) => problems.push(error.to_string()),
        }
        if self.reader.is_empty() {
            problems.push(
                "[reader] has no `name`, which tells the two readings of a drawing apart"
                    .to_string(),
            );
        }
        if self.aids.is_empty() {
            problems.push("[reader] has no `aids`: say whether the drawing was read as it is, or which enlargements were made".to_string());
        }
        match picture {
            Some(bytes) => {
                let digest = sha256_hex(bytes);
                if digest != self.picture.sha256 {
                    problems.push(format!(
                        "the picture `{}` now has the digest {digest}, and the reading was started from {}: it is not the picture that was read",
                        self.picture.file, self.picture.sha256
                    ));
                }
            }
            None => problems.push(format!(
                "the picture `{}` is not in the reference folder, so its digest cannot be checked",
                self.picture.file
            )),
        }
        for (field, what) in REFERENCE_FIELDS {
            if self.reference.get(*field).is_none_or(String::is_empty) {
                problems.push(format!("[reference] has no `{field}`. {what}"));
            }
        }

        if self.view_why.is_empty() {
            problems.push(
                "[judgement] has no `view_why`: what in the drawing shows the view".to_string(),
            );
        }
        match self.view {
            View::Front => {
                if self.faces.is_some() {
                    problems.push("a front view faces the viewer: it has no `faces`".to_string());
                }
                if self.turn_degrees.is_some_and(|turn| turn != 0.0) {
                    problems.push("a front view is not turned: its `turn_degrees` is 0. A head that is turned at all is a three-quarter view, or is set aside".to_string());
                }
            }
            View::ThreeQuarter | View::Side => {
                if self.faces.is_none() {
                    problems.push("[judgement] has no `faces`: a turned head faces \"the viewer's right\" or \"the viewer's left\"".to_string());
                }
                match self.turn_degrees {
                    None => problems.push("[judgement] has no `turn_degrees`: your own estimate of how far the head is turned".to_string()),
                    Some(turn) if self.view == View::ThreeQuarter && !(turn > 0.0 && turn < 90.0) => {
                        problems.push(format!("`turn_degrees` is {turn}: a three-quarter view is turned by more than 0 and less than 90 degrees"));
                    }
                    Some(_) => {}
                }
                if self.turn_why.is_empty() {
                    problems.push("[judgement] has no `turn_why`: what in the drawing shows how far the head is turned".to_string());
                }
            }
        }
        match self.level {
            None => problems.push("[judgement] has no `level`: true when the head is neither tilted up or down nor leaning to a side, false otherwise".to_string()),
            Some(false) if !self.set_aside => problems.push("the head is not level and the drawing is not set aside: a tilted head is never measured as a level one. Set it aside; Jon can overrule that".to_string()),
            Some(_) => {}
        }
        if self.level_why.is_empty() {
            problems.push("[judgement] has no `level_why`: what in the drawing shows that the head is level, or how it is tilted".to_string());
        }
        if self.set_aside {
            if self.set_aside_why.is_empty() {
                problems.push(
                    "the drawing is set aside and `set_aside_why` does not say why".to_string(),
                );
            }
            // A drawing that is set aside is not measured: its landmarks are not asked for.
            return problems;
        }

        let asked = self.asked();
        let is_asked = |name: &str| asked.iter().any(|asked| asked.name == name);
        let missing: Vec<&str> = asked
            .iter()
            .map(|asked| asked.name.as_str())
            .filter(|name| !self.placed.contains_key(*name) && !self.not_seen.contains_key(*name))
            .collect();
        if !missing.is_empty() {
            problems.push(format!(
                "{} neither placed nor stated as not seen: {}",
                match missing.len() {
                    1 => "1 landmark is".to_string(),
                    count => format!("{count} landmarks are"),
                },
                missing.join(", ")
            ));
        }
        for name in self.placed.keys().chain(self.not_seen.keys()) {
            if !is_asked(name) {
                problems.push(format!(
                    "`{name}` is not a landmark a {} view asks for",
                    self.view.name()
                ));
            }
        }
        for (name, why) in &self.not_seen {
            if self.placed.contains_key(name) {
                problems.push(format!("`{name}` is both placed and stated as not seen"));
            } else if why.is_empty() {
                problems.push(format!("`{name}` is stated as not seen without a reason"));
            }
        }
        for (name, at) in &self.placed {
            let inside = (0.0..=self.picture.width).contains(&at[0])
                && (0.0..=self.picture.height).contains(&at[1]);
            if !inside {
                problems.push(format!(
                    "`{name}` is at {}, {}, outside the picture, which is {} by {}: positions are in pixels of the picture itself, not of an enlargement",
                    at[0], at[1], self.picture.width, self.picture.height
                ));
            }
        }
        // The character's left is toward the viewer's right, whichever way the head faces.
        for (name, at) in &self.placed {
            let Some(twin) = name
                .contains("-left-")
                .then(|| name.replace("-left-", "-right-"))
            else {
                continue;
            };
            if let Some(twin_at) = self.placed.get(&twin)
                && at[0] < twin_at[0]
            {
                problems.push(format!(
                    "`{name}` is toward the viewer's left of `{twin}`: left and right in a name are the character's own, so the two may be exchanged"
                ));
            }
        }
        problems
    }

    /// The reading's positions as the head model reads them, for a head that is level and
    /// not set aside.
    ///
    /// A head that faces the viewer's left is mirrored, so that every turned view faces the
    /// viewer's right and the character's right is the near side: the picture is flipped
    /// left to right and left and right exchange in every name. The head model is symmetric,
    /// so the proportions are the same.
    ///
    /// The names become the list's: in a front view a point of the face outline is both the
    /// cheek point and the jaw-line point of its level; in a turned view the near side's
    /// outline is the jaw line; and an iris's centre is midway between its side extents and
    /// midway between its top and bottom.
    pub fn positions(&self) -> Result<head_model::Reading> {
        self.measurable()?;
        positions_of(self.view, self.faces, self.picture.width, &self.placed)
    }

    /// Refuses a reading whose head cannot be measured as a level head in its view.
    pub fn measurable(&self) -> Result<()> {
        let which = format!(
            "the reading of `{}` by {}",
            self.picture.file,
            if self.reader.is_empty() {
                "an unnamed reader"
            } else {
                &self.reader
            }
        );
        if self.set_aside {
            return Err(Error::new(format!(
                "{which} sets the drawing aside ({}), so it is not measured",
                if self.set_aside_why.is_empty() {
                    "without saying why"
                } else {
                    &self.set_aside_why
                }
            )));
        }
        if self.level != Some(true) {
            return Err(Error::new(format!(
                "{which} does not say the head is level, so its numbers cannot be taken as a level head's. Measuring a tilted head as tilted is not built"
            )));
        }
        if self.view != View::Front && self.faces.is_none() {
            return Err(Error::new(format!(
                "{which} does not say which way the head faces"
            )));
        }
        Ok(())
    }
}

fn exchanged(name: &str) -> String {
    if name.contains("-left-") {
        name.replace("-left-", "-right-")
    } else {
        name.replace("-right-", "-left-")
    }
}

/// Positions under a reading's names, as the head model reads them. See [`Record::positions`].
pub fn positions_of(
    view: View,
    faces: Option<Faces>,
    picture_width: f64,
    placed: &BTreeMap<String, [f64; 2]>,
) -> Result<head_model::Reading> {
    // Every turned view is read as facing the viewer's right.
    let placed: BTreeMap<String, [f64; 2]> = if faces == Some(Faces::ViewersLeft) {
        placed
            .iter()
            .map(|(name, at)| (exchanged(name), [picture_width - at[0], at[1]]))
            .collect()
    } else {
        placed.clone()
    };
    let near = Faces::ViewersRight.near();
    let mut reading = head_model::Reading {
        turn_degrees: view.turn_degrees(),
        ..head_model::Reading::default()
    };
    for (name, at) in &placed {
        let outline = name.strip_prefix("face-outline-").and_then(|rest| {
            let (side, point) = rest.split_once('-')?;
            (point != "jaw").then_some((side, point))
        });
        match outline {
            Some((side, point)) => {
                let jaw_line = format!("jaw-line-{side}-{point}");
                if view == View::Front {
                    reading.points.insert(name.clone(), *at);
                    reading.points.insert(jaw_line, *at);
                } else if side == near {
                    reading.points.insert(jaw_line, *at);
                } else {
                    reading.points.insert(name.clone(), *at);
                }
            }
            None => {
                reading.points.insert(name.clone(), *at);
            }
        }
    }
    for side in ["left", "right"] {
        let at = |point: &str| placed.get(&format!("iris-{side}-{point}"));
        if let (Some(top), Some(bottom), Some(outer), Some(inner)) = (
            at("top"),
            at("bottom"),
            at("outer-extent"),
            at("inner-extent"),
        ) {
            reading.points.insert(
                format!("iris-{side}-centre"),
                [(outer[0] + inner[0]) / 2.0, (top[1] + bottom[1]) / 2.0],
            );
        }
    }
    Ok(reading)
}

/// The readings of one drawing taken together: one reading, or the two readings of it.
#[derive(Debug, Clone)]
pub struct Drawing {
    pub view: View,
    pub faces: Option<Faces>,
    pub picture: Picture,
    /// Who read it.
    pub readers: Vec<String>,
    /// Each reader's own estimate of the turn, where it gave one.
    pub stated_turns: Vec<f64>,
    /// The positions under the reading's names: one reading's own, or midway between the two
    /// readings' for each landmark both place.
    pub placed: BTreeMap<String, [f64; 2]>,
    /// The landmarks only one of two readings places. They are left out.
    pub placed_by_one: Vec<String>,
}

impl Drawing {
    /// Takes one reading or two of the same drawing together. Each must be of a level head
    /// that is not set aside, and two must be of one picture and agree on its view.
    pub fn of(records: &[Record]) -> Result<Drawing> {
        let (first, second) = match records {
            [first] => (first, None),
            [first, second] => (first, Some(second)),
            _ => {
                return Err(Error::new(
                    "a drawing is measured from one reading or from its two readings",
                ));
            }
        };
        first.measurable()?;
        let mut drawing = Drawing {
            view: first.view,
            faces: first.faces,
            picture: first.picture.clone(),
            readers: vec![first.reader.clone()],
            stated_turns: first.turn_degrees.into_iter().collect(),
            placed: first.placed.clone(),
            placed_by_one: Vec::new(),
        };
        if let Some(second) = second {
            second.measurable()?;
            same_drawing(first, second)?;
            drawing.readers.push(second.reader.clone());
            drawing.stated_turns.extend(second.turn_degrees);
            drawing.placed = BTreeMap::new();
            for (name, at) in &first.placed {
                match second.placed.get(name) {
                    Some(other) => {
                        drawing.placed.insert(
                            name.clone(),
                            [(at[0] + other[0]) / 2.0, (at[1] + other[1]) / 2.0],
                        );
                    }
                    None => drawing.placed_by_one.push(name.clone()),
                }
            }
            for name in second.placed.keys() {
                if !first.placed.contains_key(name) {
                    drawing.placed_by_one.push(name.clone());
                }
            }
            drawing.placed_by_one.sort();
        }
        Ok(drawing)
    }

    /// The positions as the head model reads them. See [`Record::positions`].
    pub fn positions(&self) -> Result<head_model::Reading> {
        positions_of(self.view, self.faces, self.picture.width, &self.placed)
    }

    /// Who read the drawing, in words.
    pub fn read_by(&self) -> String {
        match self.readers.as_slice() {
            [only] => format!("one reading, by {only}"),
            readers => format!(
                "the midpoints of two readings, by {}",
                readers.join(" and ")
            ),
        }
    }
}

/// Refuses two readings that are not of one drawing in one view.
pub fn same_drawing(first: &Record, second: &Record) -> Result<()> {
    if first.picture.sha256 != second.picture.sha256 {
        return Err(Error::new(format!(
            "the two readings are not of one picture: `{}` has the digest {} and `{}` has {}",
            first.picture.file, first.picture.sha256, second.picture.file, second.picture.sha256
        )));
    }
    if first.view != second.view || first.faces != second.faces {
        return Err(Error::new(format!(
            "the two readings of `{}` do not judge the view alike: {} calls it a {} view{} and {} a {} view{}. Jon decides which it is before it is measured",
            first.picture.file,
            first.reader,
            first.view.name(),
            first
                .faces
                .map(|faces| format!(" facing {}", faces.words()))
                .unwrap_or_default(),
            second.reader,
            second.view.name(),
            second
                .faces
                .map(|faces| format!(" facing {}", faces.words()))
                .unwrap_or_default(),
        )));
    }
    Ok(())
}

/// A reading to fill in: the picture's identity, the questions of the judgement, and a line
/// for every landmark the view asks for.
pub fn template(
    picture: &Picture,
    view: View,
    faces: Option<Faces>,
    reader: &str,
) -> Result<String> {
    if view == View::Front && faces.is_some() {
        return Err(Error::new(
            "a front view faces the viewer: `--faces` is for a three-quarter or a side view",
        ));
    }
    if view != View::Front && faces.is_none() {
        return Err(Error::new(format!(
            "a {} view needs `--faces right` or `--faces left`: which way the head faces, from the viewer's side",
            view.name()
        )));
    }
    let mut out = String::new();
    out.push_str(
        "\
# A reading: one reader's placement of the landmarks on one drawing, with its judgement of the
# view, the turn and whether the head is level.
#
# Started by `maquette start-reading`. Fill it in by the reading instructions, which
# `maquette reading-instructions` prints, then run `maquette check-reading` on it until it
# passes. A reading is private: it stays in the reference folder and is never committed.

# The revision of the reading instructions this reading follows.
",
    );
    out.push_str(&format!(
        "instructions = {}\n",
        quoted(instructions_revision()?)
    ));
    out.push_str("\n[reader]\n");
    out.push_str(&format!("name = {}\n", quoted(reader)));
    out.push_str(
        "# How the drawing was looked at: as it is, or which enlargements were made of it.\n",
    );
    out.push_str("aids = \"\"\n");

    out.push_str("\n# The picture that is read. Positions below are in its pixels.\n[image]\n");
    out.push_str(&format!("file = {}\n", quoted(&picture.file)));
    out.push_str(&format!("sha256 = {}\n", quoted(&picture.sha256)));
    out.push_str(&format!("width = {}\n", picture.width));
    out.push_str(&format!("height = {}\n", picture.height));

    out.push_str(
        "\n# What the drawing is. Whoever hands over the drawing says these.\n[reference]\n",
    );
    for (field, what) in REFERENCE_FIELDS {
        out.push_str(&format!("# {what}\n{field} = \"\"\n"));
    }

    out.push_str(
        "\n# What the reader judges before placing anything. Each needs its reason.\n[judgement]\n",
    );
    out.push_str(&format!("view = {}\n", quoted(view.name())));
    out.push_str("# What in the drawing shows that it is this view.\nview_why = \"\"\n");
    match faces {
        None => {
            out.push_str("# A front view is not turned.\nturn_degrees = 0.0\n");
        }
        Some(faces) => {
            out.push_str("# Which way the head faces, from the viewer's side.\n");
            out.push_str(&format!("faces = {}\n", quoted(faces.words())));
            if view == View::Side {
                out.push_str("# A side view is a full profile.\nturn_degrees = 90.0\n");
            } else {
                out.push_str("# Your own estimate of how far the head is turned from facing the viewer, in degrees.\n# turn_degrees = \n");
            }
            out.push_str("# What in the drawing shows how far it is turned.\nturn_why = \"\"\n");
        }
    }
    out.push_str("# Whether the head is level: true or false. Level is neither tilted up or down nor leaning to a side.\n# level = \n");
    out.push_str("# What in the drawing shows that it is level, or how it is tilted and by about how much.\nlevel_why = \"\"\n");
    out.push_str("# true when the drawing should not be measured: the head is not level, or it is not in this view.\n# Then say why, and place nothing. Jon can overrule it.\nset_aside = false\nset_aside_why = \"\"\n");

    out.push_str(
        "\
\n# The landmarks. A position is in pixels of the picture above, not of an enlargement: x from its
# left edge and y from its top edge, with decimals where you can tell. Left and right in a name
# are the character's own. For each landmark, remove the `# ` before its line and give its
# position, or name it under [not_seen] with the reason.
[landmarks]
",
    );
    for asked in asked(view, faces) {
        out.push_str(&format!(
            "# {}\n# {} = {{ x = , y = }}\n",
            asked.what, asked.name
        ));
    }
    out.push_str(
        "\n# Every landmark above that cannot be placed, with the reason: name = \"why\".\n[not_seen]\n",
    );
    Ok(out)
}
