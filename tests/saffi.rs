//! Saffi's set of proportions, through the command line: her record in, her proportions out,
//! and her proportions drawn.
//!
//! The expected numbers below are worked by hand from the positions that
//! `data/saffi/README.md` and `data/saffi/hand-made-views.toml` give, not taken from the program.
//! Nothing here says her armature looks right: that is Jon's look.

mod support;

use support::{assert_close, draw, maquette, maquette_ok, path_text, repository, text};

const RECORD: &str = "data/saffi/hand-made-views.toml";
const HER_PROPORTIONS: &str = "data/saffi/proportions.toml";

/// The pinned turn of her three-quarter view.
const TURN_DEGREES: f64 = 29.0;

/// Her three-quarter view's eye line: the openings run from y = 350 to 390 on her right eye
/// and from 350 to 390.108 on her left, so the eye line is at 370.027.
const EYE_LINE: f64 = (350.0 + 390.0 + 350.0 + 390.108) / 4.0;
/// Her chin is at y = 567 in both views.
const UNIT: f64 = 567.0 - EYE_LINE;

#[test]
fn her_set_of_proportions_is_what_the_command_writes_from_her_record() {
    // The committed file was not typed in by hand: it is the command's output, byte for byte.
    // This pins no armature. If the head model changes, the file is written again.
    let written = maquette_ok(&["proportions-from-hand-made-views", "--record", RECORD]);
    let committed = text(&repository().join(HER_PROPORTIONS));
    assert!(
        written == committed,
        "{HER_PROPORTIONS} is not what `maquette proportions-from-hand-made-views --record {RECORD}` writes. Write it again with `--out {HER_PROPORTIONS}`."
    );
}

#[test]
fn her_file_records_where_it_came_from() {
    let table: toml::Table = text(&repository().join(HER_PROPORTIONS))
        .parse()
        .expect("TOML");
    let source = table["source"].as_table().expect("[source]");
    assert_eq!(source["record"].as_str(), Some(RECORD));
    // The digests of the two drawings, as her record gives them.
    assert_eq!(
        source["front_sha256"].as_str(),
        Some("34ee7b92a8422031a20f7d2f195d6952d16e1099de3002f5fffcb6697a57f76f")
    );
    assert_eq!(
        source["three_quarter_sha256"].as_str(),
        Some("c50bb0529f254fe12ac0af7b3f6175ae175df14b61d24f16bfec54afcc2efb02")
    );
    assert_eq!(support::number(&source["turn_degrees"]), TURN_DEGREES);
    for rules in ["reading_rules", "disagreement_rules"] {
        assert!(
            source[rules].as_array().expect("a list of rules").len() >= 5,
            "{rules} are not recorded"
        );
    }
}

#[test]
fn her_unit_is_eye_line_to_chin_and_her_two_views_agree_on_it() {
    let table: toml::Table = text(&repository().join(HER_PROPORTIONS))
        .parse()
        .expect("TOML");
    let source = table["source"].as_table().expect("[source]");
    // Front view: both openings run from y = 350 to 390, and the chin is at 567.
    let front = source["front"].as_table().expect("[source.front]");
    assert_eq!(support::number(&front["eye_line_y"]), 370.0);
    assert_eq!(support::number(&front["unit"]), 197.0);
    assert_eq!(support::number(&front["turn_axis_x"]), 500.0);
    let turned = source["three_quarter"]
        .as_table()
        .expect("[source.three_quarter]");
    assert_close(
        support::number(&turned["eye_line_y"]),
        EYE_LINE,
        0.0005,
        "eye line",
    );
    assert_close(support::number(&turned["unit"]), UNIT, 0.0005, "unit");
    assert_eq!(support::number(&turned["turn_axis_x"]), 495.0);
}

#[test]
fn her_proportions_follow_from_her_record_by_the_stated_rules() {
    let hers = support::proportions(&repository().join(HER_PROPORTIONS));
    let (sin, cos) = TURN_DEGREES.to_radians().sin_cos();
    // Depth zero in her three-quarter view: the mean x of her four eye corners.
    let zero_x = (602.0 + 530.0 + 655.0 + 453.0) / 4.0;
    let expected = [
        // Heights come from the three-quarter view.
        ("hair_top_rise", (EYE_LINE - 85.6) / UNIT),
        ("nose_mark_bottom_drop", (458.0 - EYE_LINE) / UNIT),
        ("skull_radius", 191.5 / UNIT),
        // A pair's height is the mean of its two heights there: her outer corners are at 366 and 369.
        (
            "eye_outer_corner_drop",
            ((366.0 + 369.0) / 2.0 - EYE_LINE) / UNIT,
        ),
        // An offset is half the three-quarter spacing over the cosine of the turn:
        // outer corners 202 apart, iris centres 136, mouth ends 48.
        ("eye_outer_corner_offset", 202.0 / 2.0 / cos / UNIT),
        ("iris_centre_offset", 136.0 / 2.0 / cos / UNIT),
        ("mouth_end_offset", 48.0 / 2.0 / cos / UNIT),
        // Depth from a midpoint: the inner corners' is x = 566 and the outer corners' 554.
        ("eye_wrap_depth", (566.0 - 554.0) / sin / UNIT),
        // The skull's centre is at x = 495, the nose mark's bottom at 591, the chin at 576.
        ("skull_centre_depth", (495.0 - zero_x) / sin / UNIT),
        ("nose_mark_bottom_depth", (591.0 - zero_x) / sin / UNIT),
        ("chin_depth", (576.0 - zero_x) / sin / UNIT),
        // The neck and the shoulder line take their widths from the front view, whose unit is 197:
        // the shoulder points are 744 apart there and the neck 180 wide.
        ("shoulder_half_width", 744.0 / 2.0 / 197.0),
        ("neck_half_width", 180.0 / 2.0 / 197.0),
        // The head covers the far ear's base in the three-quarter view, so the offset is the
        // front view's (340 apart) and the height the near ear's (y = 176).
        ("ear_base_outer_offset", 340.0 / 2.0 / 197.0),
        ("ear_base_outer_rise", (EYE_LINE - 176.0) / UNIT),
        // The face's half-width at the nose level is the front view's: the outline crosses at
        // x = 373.196 and 626.804. Its cheek point's depth is from the far side's crossing at
        // x = 665.486, and its jaw-line point's from the near side's at x = 394.
        (
            "face_half_width_at_nose_level",
            (626.804 - 373.196) / 2.0 / 197.0,
        ),
        (
            "face_depth_at_nose_level",
            ((665.486 - zero_x) / UNIT - (626.804 - 373.196) / 2.0 / 197.0 * cos) / sin,
        ),
        (
            "jaw_line_depth_at_nose_level",
            ((394.0 - zero_x) / UNIT + (626.804 - 373.196) / 2.0 / 197.0 * cos) / sin,
        ),
    ];
    for (name, value) in expected {
        // Her file carries five decimals.
        assert_close(hers[name], value, 6e-6, name);
    }
}

#[test]
fn her_proportions_draw_and_come_back() {
    let folder = draw("saffi-drawn", HER_PROPORTIONS, &[]);
    let taken_back = folder.join("taken-back.toml");
    maquette_ok(&[
        "proportions-from-landmarks",
        "--front",
        path_text(&folder.join("front.landmarks.toml")),
        "--three-quarter",
        path_text(&folder.join("three-quarter.landmarks.toml")),
        "--out",
        path_text(&taken_back),
    ]);
    let hers = support::proportions(&repository().join(HER_PROPORTIONS));
    let came_back = support::proportions(&taken_back);
    assert_eq!(hers.len(), came_back.len());
    for (name, value) in &hers {
        assert_close(came_back[name], *value, 1e-4, name);
    }
}

#[test]
fn a_record_that_is_not_there_is_reported_in_plain_words() {
    let output = maquette(&[
        "proportions-from-hand-made-views",
        "--record",
        "data/saffi/no-such-record.toml",
    ]);
    assert!(!output.status.success());
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(
        message.contains("cannot read data/saffi/no-such-record.toml"),
        "{message}"
    );
}
