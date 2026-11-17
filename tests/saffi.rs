//! Saffi's set of proportions, through the command line: her record in, her proportions out,
//! and her proportions drawn.
//!
//! The expected numbers below are worked by hand from the positions that
//! `data/saffi/README.md`, `data/saffi/hand-made-views.toml` and
//! `data/saffi/final-drawings.toml` give, not taken from the program.
//! Nothing here says her armature looks right: that is Jon's look. One test pins the
//! bytes of the armature he accepted, and says only that it is still what the program draws.

mod support;

use support::{assert_close, draw, maquette, maquette_ok, path_text, repository, text};

const RECORD: &str = "data/saffi/hand-made-views.toml";
/// Her final drawings' ears and lids, which replace her record's (Jon, 2026-11-16).
const FINAL_RECORD: &str = "data/saffi/final-drawings.toml";
const HER_PROPORTIONS: &str = "data/saffi/proportions.toml";
/// The armature Jon accepted at look 3 (2026-11-17): the six files `maquette draw` wrote from
/// her set of proportions at the default placement.
const HER_ACCEPTED_ARMATURE: &str = "armatures/saffi";
/// The share of the chin's full amount at which her set carries the named correction
/// `lower_face_set_back`. It was candidate A of look 3, and Jon accepted her armature with
/// it. No rule gives the share: his look did.
const SET_BACK_SHARE: &str = "0.5";

/// The pinned turn of her three-quarter view.
const TURN_DEGREES: f64 = 29.0;

/// Her three-quarter view's eye line, from her final rig's eye openings: they run from
/// y = 353.785 to 390 on her right eye and from 355.176 to 390.163 on her left, so the eye
/// line is at 372.281. Her S2 underdrawing's openings started at 350, and its eye line was at
/// 370.027.
const EYE_LINE: f64 = (353.785 + 390.0 + 355.176 + 390.163) / 4.0;
/// Her chin is at y = 567 in both views.
const UNIT: f64 = 567.0 - EYE_LINE;
/// Her front view's eye line, from her model sheet's front: both openings run from 353.785
/// to 390.
const FRONT_EYE_LINE: f64 = (353.785 + 390.0) / 2.0;
const FRONT_UNIT: f64 = 567.0 - FRONT_EYE_LINE;

#[test]
fn her_set_of_proportions_is_what_the_command_writes_from_her_records() {
    // The committed file was not typed in by hand: it is the command's output, byte for byte.
    // This pins no armature. If the head model changes, the file is written again.
    let written = maquette_ok(&[
        "proportions-from-hand-made-views",
        "--record",
        RECORD,
        "--final-record",
        FINAL_RECORD,
        "--lower-face-set-back",
        SET_BACK_SHARE,
    ]);
    let committed = text(&repository().join(HER_PROPORTIONS));
    assert!(
        written == committed,
        "{HER_PROPORTIONS} is not what `maquette proportions-from-hand-made-views --record {RECORD} --final-record {FINAL_RECORD} --lower-face-set-back {SET_BACK_SHARE}` writes. Write it again with `--out {HER_PROPORTIONS}`."
    );
}

#[test]
fn her_side_view_draws_the_chin_and_the_mouth_where_her_correction_puts_them() {
    // Her set carries the named correction at the share her file records. Nothing here says
    // the amount is right: no drawing of hers has a side view, and that is Jon's look.
    let folder = draw("saffi-side-set-back", HER_PROPORTIONS, &[]);
    let side = support::LandmarkFile::read(&folder.join("side.landmarks.toml"));
    let share: f64 = SET_BACK_SHARE.parse().expect("a number");
    // At the default placement a unit is 200. Her chin_depth is 0.166046 and her iris_depth
    // -0.042595: the chin is drawn that share of the way back from the one to the other.
    let chin = side.point("chin");
    assert_close(chin.depth.expect("a depth"), 33.209, 0.0011, "chin depth");
    assert_close(
        chin.drawn_at_depth.expect("where the chin is drawn"),
        200.0 * (0.166046 - share * (0.166046 + 0.042595)),
        0.0015,
        "where the chin is drawn",
    );
    // The mouth line's centre is drawn on the straight line from the nose mark's bottom to
    // the chin as drawn: the three stand in one line, to the rounding of a position.
    let (nose, mouth) = (
        side.point("nose-mark-bottom"),
        side.point("mouth-line-centre"),
    );
    assert_eq!(nose.drawn_at_depth, None, "the nose mark is not moved");
    let on_the_line = nose.x + (mouth.y - nose.y) / (chin.y - nose.y) * (chin.x - nose.x);
    assert_close(mouth.x, on_the_line, 0.002, "the mouth on the line");
    // Without the correction the mouth stood in front of that line, at its depth.
    let at_its_depth = mouth.x + (mouth.depth.expect("a depth") - mouth.drawn_at_depth.unwrap());
    assert!(at_its_depth > on_the_line + 5.0);
}

#[test]
fn without_her_final_drawings_her_record_gives_her_s2_proportions() {
    // The final record is what moves her ears and her lids: without it the command works her
    // proportions out from the S2 underdrawings alone, whose eye openings start at y = 350.
    let folder = support::scratch("saffi-s2-only");
    let out = folder.join("s2.toml");
    maquette_ok(&[
        "proportions-from-hand-made-views",
        "--record",
        RECORD,
        "--out",
        path_text(&out),
    ]);
    let table: toml::Table = text(&out).parse().expect("TOML");
    let source = table["source"].as_table().expect("[source]");
    assert!(source.get("final_drawings").is_none());
    // No share was given, so the set carries no correction and every view of it is flat.
    assert!(table.get("corrections").is_none());
    assert!(source.get("lower_face_set_back_share").is_none());
    let front = source["front"].as_table().expect("[source.front]");
    assert_eq!(support::number(&front["eye_line_y"]), 370.0);
    assert_eq!(support::number(&front["unit"]), 197.0);
    // The outer base of her S2 ears is 340 apart in the front view and at y = 176 on the near
    // ear of the three-quarter view, whose eye line is at (350 + 390 + 350 + 390.108) / 4.
    let s2 = support::proportions(&out);
    let s2_eye_line = (350.0 + 390.0 + 350.0 + 390.108) / 4.0;
    assert_close(
        s2["ear_base_outer_offset"],
        340.0 / 2.0 / 197.0,
        6e-7,
        "S2 ear offset",
    );
    assert_close(
        s2["ear_base_outer_rise"],
        (s2_eye_line - 176.0) / (567.0 - s2_eye_line),
        6e-7,
        "S2 ear rise",
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
    // And the record and the digests of the two final drawings her ears and lids come from.
    assert_eq!(source["final_drawings"].as_str(), Some(FINAL_RECORD));
    assert_eq!(
        source["final_front_sha256"].as_str(),
        Some("15d6fe02177427db7b0ae956411b97295f250d3328554f598134bb8e91c8136e")
    );
    assert_eq!(
        source["final_three_quarter_sha256"].as_str(),
        Some("23251a2e5c196738514f324cec1c25c171fb0785112044d45168541d060d304a")
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
    // Front view: both openings run from y = 353.785 to 390, and the chin is at 567.
    let front = source["front"].as_table().expect("[source.front]");
    assert_close(
        support::number(&front["eye_line_y"]),
        FRONT_EYE_LINE,
        0.00051,
        "front eye line",
    );
    assert_close(
        support::number(&front["unit"]),
        FRONT_UNIT,
        0.00051,
        "front unit",
    );
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
    // Depth zero in her three-quarter view: the mean x of her four eye corners, which are her
    // final rig's.
    let zero_x = (602.0 + 530.3 + 656.0 + 453.0) / 4.0;
    let expected = [
        // Heights come from the three-quarter view.
        ("hair_top_rise", (EYE_LINE - 85.6) / UNIT),
        ("nose_mark_bottom_drop", (458.0 - EYE_LINE) / UNIT),
        ("skull_radius", 191.5 / UNIT),
        // A pair's height is the mean of its two heights there: her outer corners are at 367
        // and 368.5.
        (
            "eye_outer_corner_drop",
            ((367.0 + 368.5) / 2.0 - EYE_LINE) / UNIT,
        ),
        // The top of each opening is where the hooded lid leaves it: at 353.785 and 355.176,
        // with the bottoms at 390 and 390.163.
        (
            "eye_opening_height",
            ((390.0 + 390.163) / 2.0 - (353.785 + 355.176) / 2.0) / UNIT,
        ),
        // An offset is half the three-quarter spacing over the cosine of the turn:
        // outer corners 203 apart, iris centres 136, mouth ends 48.
        ("eye_outer_corner_offset", 203.0 / 2.0 / cos / UNIT),
        ("iris_centre_offset", 136.0 / 2.0 / cos / UNIT),
        ("mouth_end_offset", 48.0 / 2.0 / cos / UNIT),
        // Her irises stay her S2 underdrawing's: their centres are at y = 370.377.
        ("iris_centre_drop", (370.377 - EYE_LINE) / UNIT),
        // Depth from a midpoint: the inner corners' is x = 566.15 and the outer corners' 554.5.
        ("eye_wrap_depth", (566.15 - 554.5) / sin / UNIT),
        // The skull's centre is at x = 495, the nose mark's bottom at 591, the chin at 576.
        ("skull_centre_depth", (495.0 - zero_x) / sin / UNIT),
        ("nose_mark_bottom_depth", (591.0 - zero_x) / sin / UNIT),
        ("chin_depth", (576.0 - zero_x) / sin / UNIT),
        // The neck and the shoulder line take their widths from the front view and its unit:
        // the shoulder points are 744 apart there and the neck 180 wide.
        ("shoulder_half_width", 744.0 / 2.0 / FRONT_UNIT),
        ("neck_half_width", 180.0 / 2.0 / FRONT_UNIT),
        // The head covers the far ear's base in the three-quarter view, so the offset is the
        // front view's (667.004 - 332.996 apart in her model sheet's front) and the height
        // the near ear's in her final rig (y = 191.834).
        (
            "ear_base_outer_offset",
            (667.004 - 332.996) / 2.0 / FRONT_UNIT,
        ),
        ("ear_base_outer_rise", (EYE_LINE - 191.834) / UNIT),
        // Its depth is from the near ear's outer base at x = 340.203 and that offset.
        (
            "ear_base_outer_depth",
            ((340.203 - zero_x) / UNIT + (667.004 - 332.996) / 2.0 / FRONT_UNIT * cos) / sin,
        ),
        // The face's half-width at the nose level is the front view's: the outline crosses at
        // x = 373.196 and 626.804. Its cheek point's depth is from the far side's crossing at
        // x = 665.486, and its jaw-line point's from the near side's at x = 394.
        (
            "face_half_width_at_nose_level",
            (626.804 - 373.196) / 2.0 / FRONT_UNIT,
        ),
        (
            "face_depth_at_nose_level",
            ((665.486 - zero_x) / UNIT - (626.804 - 373.196) / 2.0 / FRONT_UNIT * cos) / sin,
        ),
        (
            "jaw_line_depth_at_nose_level",
            ((394.0 - zero_x) / UNIT + (626.804 - 373.196) / 2.0 / FRONT_UNIT * cos) / sin,
        ),
    ];
    for (name, value) in expected {
        // Her file carries six decimals.
        assert_close(hers[name], value, 6e-7, name);
    }
}

/// Her set's named correction as its file gives it: the amounts of `lower_face_set_back`.
fn set_back(path: &std::path::Path) -> (f64, f64) {
    let table: toml::Table = text(path).parse().expect("TOML");
    let amounts = table["corrections"]["lower_face_set_back"]
        .as_table()
        .expect("[corrections.lower_face_set_back]");
    assert_eq!(amounts.len(), 2, "the correction has two amounts");
    (
        support::number(&amounts["chin"]),
        support::number(&amounts["mouth"]),
    )
}

#[test]
fn her_lower_face_set_back_follows_from_her_record_by_the_stated_rule() {
    // Worked by hand in her three-quarter drawing's own units, where a point of the centre
    // line stands sideways at its depth times the sine of the turn: the chin is at x = 576,
    // y = 567; her two iris centres, which stay her S2 underdrawing's, at x = 624.304 and
    // 488.304, so midway at 556.304; the nose mark's bottom at x = 591, y = 458; and the mouth
    // line's centre at x = 590, y = 500.
    let sin = TURN_DEGREES.to_radians().sin();
    let depth = |sideways: f64| sideways / sin / UNIT;
    for (name, share) in [("full", 1.0), ("half", 0.5), ("none", 0.0)] {
        let folder = support::scratch(&format!("saffi-set-back-{name}"));
        let out = folder.join("proportions.toml");
        maquette_ok(&[
            "proportions-from-hand-made-views",
            "--record",
            RECORD,
            "--final-record",
            FINAL_RECORD,
            "--lower-face-set-back",
            &share.to_string(),
            "--out",
            path_text(&out),
        ]);
        let (chin, mouth) = set_back(&out);
        // The chin: the share of what draws it straight below the iris's centre.
        let chin_back = share * (576.0 - 556.304);
        assert_close(chin, depth(chin_back), 1.2e-6, &format!("{name}: chin"));
        // The mouth: onto the straight line from the nose mark's bottom to the chin as drawn.
        let chin_drawn = 576.0 - chin_back;
        let line_at_mouth = 591.0 + (500.0 - 458.0) / (567.0 - 458.0) * (chin_drawn - 591.0);
        assert_close(
            mouth,
            depth(590.0 - line_at_mouth),
            1.2e-6,
            &format!("{name}: mouth"),
        );
        // The correction changes none of her proportions.
        assert_eq!(
            support::proportions(&out),
            support::proportions(&repository().join(HER_PROPORTIONS)),
            "{name}"
        );
        // The file says what share it was written at.
        let table: toml::Table = text(&out).parse().expect("TOML");
        assert_eq!(
            support::number(&table["source"]["lower_face_set_back_share"]),
            share
        );
    }
}

#[test]
fn a_share_that_is_not_between_nothing_and_the_whole_is_refused() {
    for share in ["1.5", "-0.5", "half"] {
        let output = maquette(&[
            "proportions-from-hand-made-views",
            "--record",
            RECORD,
            "--final-record",
            FINAL_RECORD,
            "--lower-face-set-back",
            share,
        ]);
        assert!(!output.status.success(), "a share of {share} was taken");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(
            message.contains("--lower-face-set-back") && message.contains("from 0 to 1"),
            "{message}"
        );
    }
}

#[test]
fn her_accepted_armature_comes_back_as_her_set_with_its_correction() {
    // The landmark files Jon accepted, taken back: her proportions from the front and the
    // three-quarter view, and her correction from the side view, which alone states it.
    let accepted = repository().join(HER_ACCEPTED_ARMATURE);
    let folder = support::scratch("saffi-taken-back");
    let taken_back = folder.join("taken-back.toml");
    maquette_ok(&[
        "proportions-from-landmarks",
        "--front",
        path_text(&accepted.join("front.landmarks.toml")),
        "--three-quarter",
        path_text(&accepted.join("three-quarter.landmarks.toml")),
        "--side",
        path_text(&accepted.join("side.landmarks.toml")),
        "--out",
        path_text(&taken_back),
    ]);
    let hers = support::proportions(&repository().join(HER_PROPORTIONS));
    let came_back = support::proportions(&taken_back);
    assert_eq!(hers.len(), came_back.len());
    // Positions are written to a thousandth of an SVG unit, which is what a proportion can
    // be taken back to: 0.0001 of her unit of 200.
    for (name, value) in &hers {
        assert_close(came_back[name], *value, 1e-4, name);
    }
    let (her_chin, her_mouth) = set_back(&repository().join(HER_PROPORTIONS));
    let (chin, mouth) = set_back(&taken_back);
    assert_close(chin, her_chin, 1e-4, "the chin's amount");
    assert_close(mouth, her_mouth, 1e-4, "the mouth's amount");

    // Drawn again, the set taken back is the side view Jon accepted and not the one he did
    // not: every landmark stands within 0.05 SVG unit of its accepted place, because each
    // proportion came back to 0.0001 of a unit of 200 and a sideways position is the sum of
    // two. The chin of look 2's side view stood 20.9 units further forward.
    let again = draw("saffi-taken-back-drawn", path_text(&taken_back), &[]);
    let side =
        |folder: &std::path::Path| support::LandmarkFile::read(&folder.join("side.landmarks.toml"));
    let (accepted_side, drawn_again) = (side(&accepted), side(&again));
    assert_eq!(accepted_side.landmarks.len(), drawn_again.landmarks.len());
    for (name, point) in &accepted_side.landmarks {
        let again = drawn_again.landmarks[name];
        assert_close(again.x, point.x, 0.05, &format!("the x of {name}"));
        assert_close(again.y, point.y, 0.05, &format!("the y of {name}"));
        assert_eq!(
            again.drawn_at_depth.is_some(),
            point.drawn_at_depth.is_some(),
            "whether the correction moves {name}"
        );
    }
}

#[test]
fn her_proportions_draw_the_armature_jon_accepted_byte_for_byte() {
    // The one reference for an armature's bytes. Jon accepted these six files at look 3
    // (2026-11-17), and the committed ones have the digests the record of his acceptance
    // gives (`armatures/saffi/README.md`). This says the program still draws that armature,
    // and nothing about whether it looks right.
    let accepted = repository().join(HER_ACCEPTED_ARMATURE);
    let drawn = draw("saffi-accepted-armature", HER_PROPORTIONS, &[]);
    let files_in = |folder: &std::path::Path| -> std::collections::BTreeSet<String> {
        std::fs::read_dir(folder)
            .unwrap_or_else(|error| panic!("{}: {error}", folder.display()))
            .map(|entry| entry.expect("a folder entry").file_name())
            .map(|name| name.into_string().expect("the file name is text"))
            .filter(|name| name != "README.md")
            .collect()
    };
    assert_eq!(
        files_in(&drawn),
        files_in(&accepted),
        "`maquette draw --proportions {HER_PROPORTIONS}` writes other files than {HER_ACCEPTED_ARMATURE}/ holds"
    );
    for file in files_in(&accepted) {
        let (was_accepted, is_drawn) = (
            std::fs::read(accepted.join(&file)).expect("the accepted file can be read"),
            std::fs::read(drawn.join(&file)).expect("the drawn file can be read"),
        );
        assert!(
            was_accepted == is_drawn,
            "`maquette draw --proportions {HER_PROPORTIONS}` no longer writes {file} as Jon accepted it. Compare {} with {HER_ACCEPTED_ARMATURE}/{file}. If the change is meant, her armature has changed and needs his look again: do not write the accepted file again to make this pass.",
            drawn.join(&file).display()
        );
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
