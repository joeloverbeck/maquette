//! How numbers are rounded and written, so that every file agrees on them, and the one
//! median every figure of a middle value is taken by.

/// Positions are kept to a thousandth of an SVG unit: VectorCraft reads numbers at single
/// precision, and the SVG contract compares positions within 0.001.
pub fn round3(value: f64) -> f64 {
    let rounded = (value * 1000.0).round() / 1000.0;
    // No "-0" in a file.
    if rounded == 0.0 { 0.0 } else { rounded }
}

/// The median of some numbers: the middle one in order, or midway between the two middle
/// ones. Not a number when there are none.
pub fn median(values: &[f64]) -> f64 {
    let mut ordered = values.to_vec();
    ordered.sort_by(f64::total_cmp);
    let count = ordered.len();
    if count == 0 {
        f64::NAN
    } else if count % 2 == 1 {
        ordered[count / 2]
    } else {
        (ordered[count / 2 - 1] + ordered[count / 2]) / 2.0
    }
}

/// A number for an SVG attribute: at most `places` decimals, with no trailing zeros.
pub fn plain(value: f64, places: usize) -> String {
    let mut text = format!("{value:.places$}");
    if text.contains('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
    }
    if text == "-0" { "0".to_string() } else { text }
}

/// A position or a length for an SVG: [`plain`] to a thousandth, the precision every
/// position is kept to.
pub fn plain3(value: f64) -> String {
    plain(value, 3)
}

/// A number for a TOML file: as [`plain`], but always with a decimal point, so that every
/// value of a table is a float.
pub fn toml(value: f64, places: usize) -> String {
    let text = plain(value, places);
    if text.contains('.') {
        text
    } else {
        format!("{text}.0")
    }
}
