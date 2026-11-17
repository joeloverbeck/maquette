//! The reference check sheet: each character's views side by side, with the points the
//! readers placed on them, what they set aside and why, and the decision asked of Jon.
//!
//! This module hides how a sheet is laid out and drawn. The layout follows the mock-up in
//! `design/first-table-sheets/reference-check-sheet.figcraft`: its frame width, margins,
//! cards, colours and text sizes. The sheet is one SVG that holds its drawings, each embedded
//! whole, so that it stays readable after the working copy of a reference is gone and names
//! no file of the reference folder. It shows other people's drawings, so it is written into
//! the reference folder and never committed.
//!
//! Where the mock-up and a real sheet part ways, and why:
//!
//! - A drawing is shown cropped to what was read, so that a head fills its card and a dot
//!   can be told from its neighbour.
//! - A reading places about sixty points, not eleven, so a dot is smaller than the mock-up's.
//! - Where two readings put a point more than 1% of head height apart, both places are
//!   drawn and numbered, and listed under the drawing with the readers' reasons.
//! - The fit stands on its own line under the heading and in the ink colour: a colour would
//!   be a verdict, and no rule sets one.
//! - The sheet says what its numbers are measured against, and whether a look is asked.

use std::collections::BTreeMap;

use base64::Engine;

use crate::measuring::{self, Agreement, Given};
use crate::number::{plain, plain3};
use crate::readings::{self, Drawing, Record};
use crate::views::View;
use crate::{Error, Result};

const WIDTH: f64 = 3200.0;
const MARGIN: f64 = 80.0;
const CARD: f64 = 1000.0;
const CARD_GAP: f64 = 20.0;
const PADDING: f64 = 32.0;

const PAPER: &str = "#F4F2EC";
const INK: &str = "#1F2430";
const SECONDARY: &str = "#6B7080";
const BORDER: &str = "#D8D4C8";
const SET_ASIDE: &str = "#D9622B";
const SET_ASIDE_TEXT: &str = "#B4511F";
const SET_ASIDE_FILL: &str = "#FFF7F2";
const POINT: &str = "#2F6FDD";
const FONT: &str = "Inter, 'Helvetica Neue', Arial, sans-serif";
/// The heading is semi-bold and large, and runs wider than [`fitted`] reckons: the share of
/// its room it may fill, so that it stops short of the boxes beside it.
const SEMI_BOLD: f64 = 0.94;

/// What a sheet asks by default: the decision of the mock-up.
const DECISION: &str = "Your decision for each character: is every head level and in the view stated, and do the dots sit on the right features?";
const MEASURED_AGAINST: &str = "Every percentage is a share of head height, from the hair's top to the chin, as the readings of that drawing place them. A fit is a share of the front view's.";
const LEGEND: &str = "Blue dot: a point both readings place, midway between them. Orange, numbered: a point the two readings put more than 1% apart, at both places. Ring: a point only one reading places.";

/// A sheet as its description names it: what it says, and each character's readings.
#[derive(Debug, Clone)]
pub struct Description {
    pub title: String,
    pub note: String,
    /// Which look this is, or that none is asked.
    pub look: String,
    pub decision: String,
    pub characters: Vec<(String, Vec<String>)>,
}

/// Reads a sheet description: a TOML file with a `note`, a `look`, and a `[[character]]`
/// for each row, with its `label` and the `readings` of its drawings.
pub fn describe(text: &str) -> Result<Description> {
    let table: toml::Table = text
        .parse()
        .map_err(|error| Error::new(format!("the sheet description is not valid TOML: {error}")))?;
    let said = |key: &str| -> Option<String> {
        table
            .get(key)
            .and_then(toml::Value::as_str)
            .map(|text| text.trim().to_string())
            .filter(|text| !text.is_empty())
    };
    let look = said("look").ok_or_else(|| {
        Error::new(
            "the sheet description has no `look`: which of Jon's looks this sheet is, such as \"Look 3 of 8\", or that no look is asked",
        )
    })?;
    let mut characters = Vec::new();
    for entry in table
        .get("character")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
    {
        let label = entry
            .get("label")
            .and_then(toml::Value::as_str)
            .ok_or_else(|| Error::new("a [[character]] of the sheet description has no `label`"))?;
        let named: Vec<String> = entry
            .get("readings")
            .and_then(toml::Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|path| path.as_str().map(str::to_string))
            .collect();
        if named.is_empty() {
            return Err(Error::new(format!(
                "the character `{label}` of the sheet description names no `readings`"
            )));
        }
        characters.push((label.to_string(), named));
    }
    if characters.is_empty() {
        return Err(Error::new(
            "the sheet description has no [[character]]: each is a row of the sheet, with a `label` and the `readings` of its drawings",
        ));
    }
    Ok(Description {
        title: said("title").unwrap_or_else(|| "Reference check sheet".to_string()),
        note: said("note").unwrap_or_default(),
        look,
        decision: said("decision").unwrap_or_else(|| DECISION.to_string()),
        characters,
    })
}

/// A reading of the sheet, with the path it was named by.
#[derive(Debug, Clone)]
pub struct Read {
    pub path: String,
    pub record: Record,
}

/// One drawing of a character: its one or two readings.
struct Shown<'a> {
    view: View,
    readings: Vec<&'a Read>,
}

impl Shown<'_> {
    fn first(&self) -> &Record {
        &self.readings[0].record
    }

    fn records(&self) -> Vec<Record> {
        self.readings
            .iter()
            .map(|read| read.record.clone())
            .collect()
    }

    /// Why the drawing is not measured, when it is not.
    fn not_measured(&self) -> Option<String> {
        if let [first, second] = self.readings.as_slice()
            && let Err(error) = readings::same_drawing(&first.record, &second.record)
        {
            return Some(error.to_string());
        }
        let aside = self
            .readings
            .iter()
            .filter(|read| read.record.set_aside || read.record.level != Some(true))
            .count();
        match (aside, self.readings.len()) {
            (0, _) => None,
            (1, 1) => Some("Set aside by the reader: not measured".to_string()),
            (aside, all) if aside == all => {
                Some("Set aside by the readers: not measured".to_string())
            }
            _ => Some("Set aside by one reader: not measured".to_string()),
        }
    }

    fn named(&self) -> String {
        let paths: Vec<&str> = self
            .readings
            .iter()
            .map(|read| read.path.as_str())
            .collect();
        paths.join(",")
    }
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// A line of text with its top at `top`, as the mock-up places text.
fn line(x: f64, top: f64, size: f64, weight: u32, fill: &str, text: &str) -> String {
    format!(
        "<text x=\"{}\" y=\"{}\" font-size=\"{}\" font-weight=\"{weight}\" fill=\"{fill}\">{}</text>\n",
        plain3(x),
        plain3(top + size * 0.9),
        plain3(size),
        escape(text)
    )
}

/// Text cut to the room it has on one line, with a mark that it was cut. No font is
/// measured: a character is taken as 0.46 of the text's size across, which is near what the
/// sheet's typeface gives at these sizes.
fn fitted(text: &str, size: f64, room: f64) -> String {
    let most = (room / (size * 0.46)) as usize;
    if text.chars().count() <= most {
        return text.to_string();
    }
    let kept: String = text.chars().take(most.saturating_sub(1)).collect();
    format!("{}…", kept.trim_end())
}

/// Text broken into lines of at most `width` characters, at spaces.
fn wrapped(text: &str, width: usize) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for word in text.split_whitespace() {
        match lines.last_mut() {
            Some(last) if last.chars().count() + 1 + word.chars().count() <= width => {
                last.push(' ');
                last.push_str(word);
            }
            _ => lines.push(word.to_string()),
        }
    }
    lines
}

fn view_title(view: View) -> &'static str {
    match view {
        View::Front => "Front",
        View::ThreeQuarter => "Three-quarter",
        View::Side => "Side",
    }
}

fn degrees(turn: f64) -> String {
    format!("{}°", plain(turn, 1))
}

/// The part of a picture that holds what was read, with room around it: left, top, width
/// and height in pixels of the picture.
fn crop(records: &[&Record]) -> [f64; 4] {
    let picture = &records[0].picture;
    let points: Vec<[f64; 2]> = records
        .iter()
        .flat_map(|record| record.placed.values().copied())
        .collect();
    if points.is_empty() {
        return [0.0, 0.0, picture.width, picture.height];
    }
    let extent = |axis: usize| {
        let low = points.iter().map(|at| at[axis]).fold(f64::MAX, f64::min);
        let high = points.iter().map(|at| at[axis]).fold(f64::MIN, f64::max);
        (low, high)
    };
    let (left, right) = extent(0);
    let (top, bottom) = extent(1);
    let room = 0.06 * (right - left).max(bottom - top).max(1.0);
    let left = (left - room).max(0.0);
    let top = (top - room).max(0.0);
    let right = (right + room).min(picture.width);
    let bottom = (bottom + room).min(picture.height);
    [left, top, (right - left).max(1.0), (bottom - top).max(1.0)]
}

/// The notes that stand under a card: the readers' reasons, and the points to look at.
struct Card {
    svg: String,
    notes: Vec<(String, &'static str)>,
}

/// Lays the sheet out and draws it. `pictures` holds each drawing's file by its digest.
pub fn render(
    description: &Description,
    characters: &[(String, Vec<Read>)],
    pictures: &BTreeMap<String, Vec<u8>>,
) -> Result<String> {
    let mut body = String::new();
    let mut drawings = 0;
    let mut top = 250.0;
    for (label, reads) in characters {
        // The readings of one picture are one drawing, in the order they are named.
        let mut shown: Vec<Shown> = Vec::new();
        for read in reads {
            let digest = &read.record.picture.sha256;
            match shown
                .iter_mut()
                .find(|shown| &shown.first().picture.sha256 == digest)
            {
                Some(shown) => shown.readings.push(read),
                None => shown.push(Shown {
                    view: read.record.view,
                    readings: vec![read],
                }),
            }
        }
        for one in &shown {
            if one.readings.len() > 2 {
                return Err(Error::new(format!(
                    "`{}` is read {} times for the character `{label}`: a drawing is read twice",
                    one.first().picture.file,
                    one.readings.len()
                )));
            }
            if let Some(other) = shown.iter().find(|other| {
                other.view == one.view && other.first().picture.sha256 != one.first().picture.sha256
            }) {
                return Err(Error::new(format!(
                    "two drawings of the character `{label}` are read as {} views: `{}` and `{}`. A sheet shows one drawing of each view",
                    one.view.name(),
                    one.first().picture.file,
                    other.first().picture.file
                )));
            }
        }
        drawings += shown.len();

        // The fit, from the views that are measured.
        let usable = |view: View| -> Option<(String, Drawing)> {
            let shown = shown.iter().find(|shown| shown.view == view)?;
            if shown.not_measured().is_some() {
                return None;
            }
            Some((shown.named(), Drawing::of(&shown.records()).ok()?))
        };
        let (front, turned, side) = (
            usable(View::Front),
            usable(View::ThreeQuarter),
            usable(View::Side),
        );
        fn given(view: &Option<(String, Drawing)>) -> Option<Given<'_>> {
            view.as_ref()
                .map(|(named, drawing)| Given { named, drawing })
        }
        let fit = match (given(&front), given(&turned)) {
            (Some(front), Some(turned)) => {
                Some(measuring::fit(&front, &turned, given(&side).as_ref()))
            }
            _ => None,
        };
        let usable_views = [&front, &turned, &side]
            .iter()
            .filter(|view| view.is_some())
            .count();
        let (fit_text, fit_colour) = match &fit {
            Some(Ok(fit)) => (
                format!(
                    "The views fit one head to within {}% of head height",
                    plain(fit.percent_of_head_height, 2)
                ),
                INK,
            ),
            Some(Err(error)) => (
                format!("The fit could not be worked out: {error}"),
                SET_ASIDE_TEXT,
            ),
            None if usable_views == 1 => (
                "One usable view, so nothing to fit".to_string(),
                SET_ASIDE_TEXT,
            ),
            None if usable_views == 0 => ("No usable view".to_string(), SET_ASIDE_TEXT),
            None => (
                "No front view and three-quarter view to fit".to_string(),
                SET_ASIDE_TEXT,
            ),
        };
        let turn = fit
            .as_ref()
            .and_then(|fit| fit.as_ref().ok())
            .map(|fit| fit.turn.degrees);

        // The heading: what the character is, from its readings.
        let about = &shown[0].first().reference;
        let field = |name: &str| about.get(name).cloned().unwrap_or_default();
        let heading = format!(
            "{label} · {} {}, {} · {}, {}",
            field("age_band"),
            field("sex"),
            field("species"),
            field("work"),
            field("designer")
        );
        body.push_str(&line(
            MARGIN,
            top,
            36.0,
            600,
            INK,
            &fitted(&heading, 36.0, 2260.0 * SEMI_BOLD),
        ));
        body.push_str(&line(
            MARGIN,
            top + 52.0,
            28.0,
            400,
            fit_colour,
            &fitted(&fit_text, 28.0, 3040.0),
        ));
        for (x, text) in [
            (2400.0, "Accept"),
            (2700.0, "Reject"),
            (2880.0, "A dot is off"),
        ] {
            body.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"32\" height=\"32\" rx=\"6\" fill=\"#FFFFFF\" stroke=\"{INK}\" stroke-width=\"2\"/>\n",
                plain3(x),
                plain3(top + 4.0)
            ));
            body.push_str(&line(x + 44.0, top + 6.0, 26.0, 400, INK, text));
        }

        // The three cards, in the order front, three-quarter, side.
        let cards_top = top + 110.0;
        let mut tallest_notes = 0;
        for (index, view) in View::ALL.into_iter().enumerate() {
            let x = MARGIN + index as f64 * (CARD + CARD_GAP);
            let card = match shown.iter().find(|shown| shown.view == view) {
                Some(shown) => card(shown, turn, pictures)?,
                None => missing(view),
            };
            body.push_str(&format!(
                "<g transform=\"translate({} {})\">\n{}</g>\n",
                plain3(x),
                plain3(cards_top),
                card.svg
            ));
            let mut y = cards_top + CARD + 24.0;
            for (text, colour) in &card.notes {
                body.push_str(&line(x + PADDING, y, 22.0, 400, colour, text));
                y += 30.0;
            }
            tallest_notes = tallest_notes.max(card.notes.len());
        }
        top = cards_top + CARD + 24.0 + tallest_notes as f64 * 30.0 + 70.0;
    }

    // The decision asked.
    let footer = top;
    body.push_str(&format!(
        "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"110\" rx=\"16\" fill=\"#FFFFFF\" stroke=\"{BORDER}\" stroke-width=\"2\"/>\n",
        plain3(MARGIN),
        plain3(footer),
        plain3(WIDTH - 2.0 * MARGIN)
    ));
    body.push_str(&line(
        MARGIN + 36.0,
        footer + 18.0,
        28.0,
        400,
        INK,
        &fitted(&description.decision, 28.0, 2960.0),
    ));
    body.push_str(&line(
        MARGIN + 36.0,
        footer + 62.0,
        24.0,
        400,
        SECONDARY,
        &format!(
            "A drawing you reject is not measured. {}. Say your answer for each character in words: the boxes are not read back.",
            description.look
        ),
    ));
    let height = footer + 110.0 + 50.0;

    let mut out = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\" font-family=\"{FONT}\">\n",
        plain3(WIDTH),
        plain3(height),
        plain3(WIDTH),
        plain3(height)
    );
    out.push_str(&format!(
        "<rect width=\"{}\" height=\"{}\" fill=\"{PAPER}\"/>\n",
        plain3(WIDTH),
        plain3(height)
    ));
    out.push_str(&line(MARGIN, 44.0, 64.0, 700, INK, &description.title));
    out.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" font-size=\"34\" font-weight=\"600\" fill=\"{INK}\" text-anchor=\"end\">{}</text>\n",
        plain3(WIDTH - MARGIN),
        plain3(56.0 + 34.0 * 0.9),
        escape(&format!(
            "{} · {}, {}",
            description.look,
            count(characters.len(), "character"),
            count(drawings, "drawing")
        ))
    ));
    out.push_str(&line(
        MARGIN,
        128.0,
        30.0,
        400,
        SECONDARY,
        &fitted(&description.note, 30.0, 3040.0),
    ));
    out.push_str(&line(MARGIN, 170.0, 24.0, 400, SECONDARY, MEASURED_AGAINST));
    out.push_str(&line(MARGIN, 202.0, 24.0, 400, SECONDARY, LEGEND));
    out.push_str(&body);
    out.push_str("</svg>\n");
    Ok(out)
}

fn count(number: usize, noun: &str) -> String {
    if number == 1 {
        format!("1 {noun}")
    } else {
        format!("{number} {noun}s")
    }
}

/// The card of a view that no reading is of.
fn missing(view: View) -> Card {
    let mut svg = format!(
        "<rect width=\"{CARD}\" height=\"{CARD}\" rx=\"16\" fill=\"{PAPER}\" stroke=\"{BORDER}\" stroke-width=\"2\" stroke-dasharray=\"14 12\"/>\n"
    );
    svg.push_str(&line(PADDING, 22.0, 30.0, 600, INK, view_title(view)));
    svg.push_str(&line(
        210.0,
        480.0,
        28.0,
        400,
        SECONDARY,
        &format!("No {} view of this character was read", view.name()),
    ));
    Card {
        svg,
        notes: Vec::new(),
    }
}

/// The card of one drawing: the drawing with its points, the judgement, and the notes.
fn card(shown: &Shown, turn: Option<f64>, pictures: &BTreeMap<String, Vec<u8>>) -> Result<Card> {
    let first = shown.first();
    let picture = pictures.get(&first.picture.sha256).ok_or_else(|| {
        Error::new(format!(
            "the picture `{}` was not given to the sheet",
            first.picture.file
        ))
    })?;
    let not_measured = shown.not_measured();
    let records: Vec<&Record> = shown.readings.iter().map(|read| &read.record).collect();

    let mut svg = match &not_measured {
        None => format!(
            "<rect width=\"{CARD}\" height=\"{CARD}\" rx=\"16\" fill=\"#FFFFFF\" stroke=\"{BORDER}\" stroke-width=\"2\"/>\n"
        ),
        Some(_) => format!(
            "<rect width=\"{CARD}\" height=\"{CARD}\" rx=\"16\" fill=\"{SET_ASIDE_FILL}\" stroke=\"{SET_ASIDE}\" stroke-width=\"4\"/>\n"
        ),
    };
    svg.push_str(&line(PADDING, 22.0, 30.0, 600, INK, view_title(shown.view)));

    // The drawing, cropped to what was read, with the points in its own pixels.
    let [left, top, width, height] = crop(&records);
    let (frame_width, frame_height) = (CARD - 2.0 * PADDING, 700.0);
    let scale = (frame_width / width).min(frame_height / height);
    // A length on the sheet, in pixels of the picture.
    let on_sheet = |length: f64| plain3(length / scale);
    svg.push_str(&format!(
        "<svg x=\"{}\" y=\"70\" width=\"{}\" height=\"{}\" viewBox=\"{} {} {} {}\" preserveAspectRatio=\"xMidYMin meet\">\n",
        plain3(PADDING),
        plain3(frame_width),
        plain3(frame_height),
        plain3(left),
        plain3(top),
        plain3(width),
        plain3(height)
    ));
    svg.push_str(&format!(
        "<image width=\"{}\" height=\"{}\" href=\"data:image/png;base64,{}\"/>\n",
        plain3(first.picture.width),
        plain3(first.picture.height),
        base64::engine::general_purpose::STANDARD.encode(picture)
    ));

    let mut notes: Vec<(String, &'static str)> = Vec::new();
    let mut agreement_line = String::new();
    if not_measured.is_none() {
        let dot = |at: [f64; 2], class: &str, fill: &str, stroke: &str| {
            format!(
                "<circle class=\"{class}\" cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"{}\"/>\n",
                plain3(at[0]),
                plain3(at[1]),
                on_sheet(5.0),
                on_sheet(1.5)
            )
        };
        match shown.readings.as_slice() {
            [only] => {
                for at in only.record.placed.values() {
                    svg.push_str(&dot(*at, "by-one", "#FFFFFF", POINT));
                }
                agreement_line =
                    "One reading only: nothing to set it against. Read it a second time"
                        .to_string();
            }
            [first_read, second_read] => {
                let agreement: Agreement = measuring::agreement(
                    &first_read.path,
                    &first_read.record,
                    &second_read.path,
                    &second_read.record,
                )?;
                for compared in &agreement.at_or_under {
                    let midway = [
                        (compared.first[0] + compared.second[0]) / 2.0,
                        (compared.first[1] + compared.second[1]) / 2.0,
                    ];
                    svg.push_str(&dot(midway, "agreed", POINT, "#FFFFFF"));
                }
                for name in agreement
                    .placed_by_first_only
                    .iter()
                    .map(|name| first_read.record.placed[name])
                    .chain(
                        agreement
                            .placed_by_second_only
                            .iter()
                            .map(|name| second_read.record.placed[name]),
                    )
                {
                    svg.push_str(&dot(name, "by-one", "#FFFFFF", POINT));
                }
                for compared in &agreement.above {
                    svg.push_str(&format!(
                        "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{SET_ASIDE}\" stroke-width=\"{}\"/>\n",
                        plain3(compared.first[0]),
                        plain3(compared.first[1]),
                        plain3(compared.second[0]),
                        plain3(compared.second[1]),
                        on_sheet(2.0)
                    ));
                    svg.push_str(&dot(compared.first, "apart", SET_ASIDE, "#FFFFFF"));
                    svg.push_str(&dot(compared.second, "apart", SET_ASIDE, "#FFFFFF"));
                    svg.push_str(&format!(
                        "<text x=\"{}\" y=\"{}\" font-size=\"{}\" font-weight=\"700\" fill=\"{SET_ASIDE_TEXT}\" stroke=\"#FFFFFF\" stroke-width=\"{}\" paint-order=\"stroke\">{}</text>\n",
                        plain3(compared.first[0].max(compared.second[0]) + 9.0 / scale),
                        plain3(compared.first[1].min(compared.second[1]) - 6.0 / scale),
                        on_sheet(22.0),
                        on_sheet(4.0),
                        compared.number.unwrap_or_default()
                    ));
                }
                agreement_line = match agreement.largest_percent() {
                    Some(largest) => format!(
                        "Two readings agree to {}% of head height at worst; {}",
                        plain(largest, 2),
                        match agreement.above.len() {
                            0 => "no point is over 1% apart".to_string(),
                            1 => "1 point is over 1% apart".to_string(),
                            count => format!("{count} points are over 1% apart"),
                        }
                    ),
                    None => "The two readings place no point in common".to_string(),
                };
                for compared in &agreement.above {
                    notes.push((
                        format!(
                            "{}  {}  {}%",
                            compared.number.unwrap_or_default(),
                            compared.landmark,
                            plain(compared.percent_of_head_height, 2)
                        ),
                        SET_ASIDE_TEXT,
                    ));
                }
                let by_one: Vec<&String> = agreement
                    .placed_by_first_only
                    .iter()
                    .chain(&agreement.placed_by_second_only)
                    .collect();
                if !by_one.is_empty() {
                    let names: Vec<&str> = by_one.iter().map(|name| name.as_str()).collect();
                    for text in wrapped(
                        &format!("Placed by one reader only: {}", names.join(", ")),
                        82,
                    ) {
                        notes.push((text, SECONDARY));
                    }
                }
                for (what, first, second) in &agreement.judged_differently {
                    notes.push((
                        format!(
                            "Judged differently: {what} is {first} for {} and {second} for {}",
                            first_read.record.reader, second_read.record.reader
                        ),
                        SET_ASIDE_TEXT,
                    ));
                }
            }
            _ => unreachable!("a drawing has one reading or two"),
        }
    }
    svg.push_str("</svg>\n");

    // The judgement, the agreement and the source.
    let level = "level";
    match &not_measured {
        None => {
            let judgement = match shown.view {
                View::Front => format!("Front · {level} · straight on"),
                View::ThreeQuarter => {
                    let faces = first
                        .faces
                        .map(|faces| faces.words())
                        .unwrap_or("the viewer's right");
                    match turn {
                        Some(turn) => format!(
                            "Three-quarter · turned {} toward {faces} · {level}",
                            degrees(turn)
                        ),
                        None => format!(
                            "Three-quarter · turned about {} toward {faces}, by the reader's eye · {level}",
                            degrees(first.turn_degrees.unwrap_or_default())
                        ),
                    }
                }
                View::Side => format!(
                    "Side · facing {} · {level}",
                    first
                        .faces
                        .map(|faces| faces.words())
                        .unwrap_or("the viewer's right")
                ),
            };
            svg.push_str(&line(
                PADDING,
                792.0,
                30.0,
                600,
                INK,
                &fitted(&judgement, 30.0, 936.0),
            ));
            svg.push_str(&line(
                PADDING,
                840.0,
                24.0,
                400,
                SECONDARY,
                &fitted(&agreement_line, 24.0, 936.0),
            ));
        }
        Some(why) => {
            // The mock-up's pill, with its label inside it.
            svg.push_str(&format!(
                "<rect x=\"{PADDING}\" y=\"784\" width=\"936\" height=\"52\" rx=\"26\" fill=\"{SET_ASIDE}\"/>\n"
            ));
            let pill = wrapped(why, 74).into_iter().next().unwrap_or_default();
            svg.push_str(&line(58.0, 796.0, 26.0, 600, "#FFFFFF", &pill));
            svg.push_str(&line(
                PADDING,
                848.0,
                24.0,
                400,
                SECONDARY,
                "You can overrule this and have it measured.",
            ));
        }
    }
    svg.push_str(&line(
        PADDING,
        880.0,
        24.0,
        400,
        SECONDARY,
        &fitted(
            &format!(
                "Source: {}",
                first.reference.get("source").cloned().unwrap_or_default()
            ),
            24.0,
            936.0,
        ),
    ));
    let readers: Vec<&str> = records
        .iter()
        .map(|record| record.reader.as_str())
        .collect();
    svg.push_str(&line(
        PADDING,
        918.0,
        24.0,
        400,
        SECONDARY,
        &fitted(&format!("Read by {}", readers.join(" and ")), 24.0, 936.0),
    ));

    // Under the card: each reader's reasons, which are what Jon checks.
    let mut reasons: Vec<(String, &'static str)> = Vec::new();
    for record in &records {
        let mut said = vec![format!("view: {}", record.view_why)];
        if record.view != View::Front {
            said.push(format!(
                "turn: {} (its own estimate: {})",
                record.turn_why,
                degrees(record.turn_degrees.unwrap_or_default())
            ));
        }
        said.push(format!("level: {}", record.level_why));
        if record.set_aside || record.level != Some(true) {
            said.push(format!("set aside: {}", record.set_aside_why));
        }
        for text in wrapped(&format!("{} — {}", record.reader, said.join("; ")), 82) {
            reasons.push((text, SECONDARY));
        }
    }
    reasons.extend(notes);
    Ok(Card {
        svg,
        notes: reasons,
    })
}
