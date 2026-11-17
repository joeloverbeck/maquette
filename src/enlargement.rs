//! Enlarges a part of a picture for a reader, so that a small head is read as accurately as
//! a large one.
//!
//! This module hides how the part is enlarged and how a position in the enlargement is taken
//! back to the picture. Each pixel of the picture becomes a block of whole pixels, so nothing
//! is smoothed and an edge stays where it was. A grid, when asked for, marks the picture's
//! own pixels and labels them with their positions in the picture, so that a reader reads a
//! position off the labels and does no arithmetic. A reading's positions are always in
//! pixels of the picture itself, whose digest the reading records.
//!
//! An agent is shown a picture resized when an edge is longer than [`LONGEST_EDGE`], and a
//! position read off a resized picture is of another picture. So a longer enlargement is
//! refused.

use image::{ImageFormat, Rgb, RgbImage};

use crate::{Error, Result};

/// The longest edge, in pixels, that an agent is shown as it is.
pub const LONGEST_EDGE: u32 = 1568;

/// The largest factor: beyond it a pixel is a block too coarse to help.
const LARGEST_FACTOR: u32 = 16;

const GRID: Rgb<u8> = Rgb([0, 150, 215]);
const MARGIN: Rgb<u8> = Rgb([255, 255, 255]);
const LABEL: Rgb<u8> = Rgb([20, 30, 60]);

/// The digits 0 to 9, three wide and five high, a row to a number: bit 2 is the left pixel.
const DIGITS: [[u8; 5]; 10] = [
    [0b111, 0b101, 0b101, 0b101, 0b111],
    [0b010, 0b110, 0b010, 0b010, 0b111],
    [0b111, 0b001, 0b111, 0b100, 0b111],
    [0b111, 0b001, 0b111, 0b001, 0b111],
    [0b101, 0b101, 0b111, 0b001, 0b001],
    [0b111, 0b100, 0b111, 0b001, 0b111],
    [0b111, 0b100, 0b111, 0b101, 0b111],
    [0b111, 0b001, 0b001, 0b001, 0b001],
    [0b111, 0b101, 0b111, 0b101, 0b111],
    [0b111, 0b101, 0b111, 0b001, 0b111],
];
/// How many pixels a pixel of a digit is drawn as.
const DIGIT_SCALE: u32 = 2;
const DIGIT_WIDTH: u32 = 3 * DIGIT_SCALE;
const DIGIT_HEIGHT: u32 = 5 * DIGIT_SCALE;
const DIGIT_GAP: u32 = DIGIT_SCALE;
/// The margins that hold the labels: room for four digits to the left, and one row above.
const MARGIN_LEFT: u32 = 4 * (DIGIT_WIDTH + DIGIT_GAP) + 6;
const MARGIN_TOP: u32 = DIGIT_HEIGHT + 6;

/// The part of a picture to enlarge, in pixels of the picture.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Part {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
    /// How many times as large: each pixel becomes a block this many pixels across.
    pub factor: u32,
    /// A grid line every this many pixels of the picture, labelled. None for no grid.
    pub grid: Option<u32>,
}

/// An enlargement, as a PNG.
#[derive(Debug, Clone)]
pub struct Enlarged {
    pub png: Vec<u8>,
    pub width: u32,
    pub height: u32,
    /// Where the part begins in the enlargement: past the margins that hold a grid's labels.
    pub margin_left: u32,
    pub margin_top: u32,
}

fn label_width(text: &str) -> u32 {
    text.len() as u32 * (DIGIT_WIDTH + DIGIT_GAP) - DIGIT_GAP
}

/// Writes a number with its top-left corner at x, y, as far as it fits.
fn write_number(picture: &mut RgbImage, number: u32, x: u32, y: u32) {
    for (place, digit) in number.to_string().bytes().enumerate() {
        let rows = DIGITS[usize::from(digit - b'0')];
        let left = x + place as u32 * (DIGIT_WIDTH + DIGIT_GAP);
        for (row, bits) in rows.iter().enumerate() {
            for column in 0..3u32 {
                if bits & (0b100 >> column) == 0 {
                    continue;
                }
                for dy in 0..DIGIT_SCALE {
                    for dx in 0..DIGIT_SCALE {
                        let px = left + column * DIGIT_SCALE + dx;
                        let py = y + row as u32 * DIGIT_SCALE + dy;
                        if px < picture.width() && py < picture.height() {
                            picture.put_pixel(px, py, LABEL);
                        }
                    }
                }
            }
        }
    }
}

/// Enlarges a part of a picture, given as the bytes of its file.
pub fn enlarge(picture: &[u8], part: &Part) -> Result<Enlarged> {
    let picture = image::load_from_memory(picture)
        .map_err(|error| {
            Error::new(format!(
                "the picture cannot be read as a PNG, the one format maquette reads: {error}"
            ))
        })?
        .to_rgb8();
    if part.width == 0 || part.height == 0 {
        return Err(Error::new(
            "the part to enlarge must be at least 1 pixel wide and 1 pixel high",
        ));
    }
    let fits = |start: u32, length: u32, whole: u32| {
        start.checked_add(length).is_some_and(|end| end <= whole)
    };
    if !fits(part.left, part.width, picture.width())
        || !fits(part.top, part.height, picture.height())
    {
        return Err(Error::new(format!(
            "the part from {}, {} that is {} by {} reaches outside the picture, which is {} by {}",
            part.left,
            part.top,
            part.width,
            part.height,
            picture.width(),
            picture.height()
        )));
    }
    if !(1..=LARGEST_FACTOR).contains(&part.factor) {
        return Err(Error::new(format!(
            "the factor must be a whole number from 1 to {LARGEST_FACTOR}, and it is {}",
            part.factor
        )));
    }
    if part.grid == Some(0) {
        return Err(Error::new(
            "a grid has a line every so many pixels of the picture: at least 1",
        ));
    }
    let (margin_left, margin_top) = match part.grid {
        Some(_) => (MARGIN_LEFT, MARGIN_TOP),
        None => (0, 0),
    };
    let width = margin_left + part.width * part.factor;
    let height = margin_top + part.height * part.factor;
    if width > LONGEST_EDGE || height > LONGEST_EDGE {
        return Err(Error::new(format!(
            "the enlargement would be {width} by {height} pixels. An agent is shown a picture with an edge above {LONGEST_EDGE} pixels resized, and a position read off it would be of another picture. Enlarge a smaller part, or by a smaller factor"
        )));
    }

    let mut out = RgbImage::from_pixel(width, height, MARGIN);
    for y in 0..part.height * part.factor {
        for x in 0..part.width * part.factor {
            let from = picture.get_pixel(part.left + x / part.factor, part.top + y / part.factor);
            out.put_pixel(margin_left + x, margin_top + y, *from);
        }
    }
    if let Some(step) = part.grid {
        // A line stands where a pixel of the picture begins whose position is a multiple of
        // the step, and is labelled with that position.
        let lines = |start: u32, length: u32| {
            (start..start + length).filter(move |position| position % step == 0)
        };
        // A label is written at a line only when it clears the label before it.
        let mut next_free = 0;
        for position in lines(part.left, part.width) {
            let x = margin_left + (position - part.left) * part.factor;
            for y in margin_top..height {
                out.put_pixel(x, y, GRID);
            }
            let text = position.to_string();
            let at = x.saturating_sub(label_width(&text) / 2);
            if at >= next_free && at + label_width(&text) <= width {
                write_number(&mut out, position, at, 3);
                next_free = at + label_width(&text) + 2 * DIGIT_WIDTH;
            }
        }
        let mut next_free = 0;
        for position in lines(part.top, part.height) {
            let y = margin_top + (position - part.top) * part.factor;
            for x in margin_left..width {
                out.put_pixel(x, y, GRID);
            }
            let at = y.saturating_sub(DIGIT_HEIGHT / 2);
            if at >= next_free && at + DIGIT_HEIGHT <= height {
                write_number(&mut out, position, 3, at);
                next_free = at + DIGIT_HEIGHT + DIGIT_HEIGHT / 2;
            }
        }
    }

    let mut png = Vec::new();
    out.write_to(&mut std::io::Cursor::new(&mut png), ImageFormat::Png)
        .map_err(|error| Error::new(format!("the enlargement cannot be written: {error}")))?;
    Ok(Enlarged {
        png,
        width,
        height,
        margin_left,
        margin_top,
    })
}

impl Enlarged {
    /// How a position in the enlargement is taken back to the picture, in words.
    pub fn way_back(&self, part: &Part) -> String {
        let along = |start: u32, letter: &str, margin: u32| {
            if margin == 0 {
                format!("{start} + {letter} / {}", part.factor)
            } else {
                format!("{start} + ({letter} - {margin}) / {}", part.factor)
            }
        };
        format!(
            "A point at x, y in the enlargement is at {}, {} in the picture. A reading's positions are in pixels of the picture.",
            along(part.left, "x", self.margin_left),
            along(part.top, "y", self.margin_top)
        )
    }
}
