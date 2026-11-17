//! How text and a position are written into a TOML file, so that every file the program
//! writes reads back to the same text.

/// A TOML basic string holding `text`: quoted, with the quote, the backslash and every
/// control character TOML does not allow there escaped. A tab may stand as it is.
pub fn quoted(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\t' => out.push('\t'),
            control if control <= '\u{1f}' || control == '\u{7f}' => {
                out.push_str(&format!("\\u{:04X}", u32::from(control)));
            }
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

/// A position as a TOML inline table, to a thousandth: `{ x = 412.5, y = 96.0 }`.
pub fn position(at: [f64; 2]) -> String {
    format!(
        "{{ x = {}, y = {} }}",
        crate::number::toml(at[0], 3),
        crate::number::toml(at[1], 3)
    )
}

/// A TOML array of texts, on one line.
pub fn quoted_list<T: AsRef<str>>(texts: &[T]) -> String {
    let quoted: Vec<String> = texts.iter().map(|text| quoted(text.as_ref())).collect();
    format!("[{}]", quoted.join(", "))
}

/// A TOML array of texts, one to a line, for texts too long to stand side by side.
pub fn quoted_lines<T: AsRef<str>>(texts: &[T]) -> String {
    let mut out = String::from("[\n");
    for text in texts {
        out.push_str(&format!("  {},\n", quoted(text.as_ref())));
    }
    out.push(']');
    out
}
