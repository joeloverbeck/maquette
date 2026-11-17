//! Enlarging a part of a picture for a reader, through the command line.
//!
//! The picture is made up: plain grey, 200 by 100, with one darker block from 40, 20 to 60, 30.
//! It shows no drawing.

mod support;

use std::path::{Path, PathBuf};

use support::{failure, made_up_image, maquette_in, measure_ok, references};

const DARK: [u8; 3] = [40, 40, 40];
const GREY: [u8; 3] = [200, 200, 200];

fn folder(name: &str) -> PathBuf {
    let folder = references(name);
    made_up_image(&folder.join("picture.png"), 200, 100);
    folder
}

fn enlarge(folder: &Path, part: [&str; 4], factor: &str, more: &[&str], out: &str) -> String {
    let mut arguments = vec![
        "enlarge-image",
        "--image",
        "picture.png",
        "--left",
        part[0],
        "--top",
        part[1],
        "--width",
        part[2],
        "--height",
        part[3],
        "--factor",
        factor,
        "--out",
        out,
    ];
    arguments.extend_from_slice(more);
    measure_ok(folder, &arguments)
}

fn pixel(picture: &image::RgbImage, x: u32, y: u32) -> [u8; 3] {
    picture.get_pixel(x, y).0
}

#[test]
fn a_part_of_a_picture_is_enlarged_and_the_way_back_to_the_picture_is_stated() {
    let folder = folder("enlargement-plain");
    let printed = enlarge(
        &folder,
        ["30", "10", "40", "30"],
        "4",
        &[],
        "working/part.png",
    );
    let enlarged = image::open(folder.join("working/part.png"))
        .expect("the enlargement is a picture")
        .to_rgb8();
    // 40 by 30 pixels, four times: each pixel of the picture is a block of 4 by 4.
    assert_eq!(enlarged.dimensions(), (160, 120));
    // The dark block starts at 40, 20 in the picture: 10 pixels into the part each way.
    assert_eq!(pixel(&enlarged, 39, 39), GREY);
    assert_eq!(pixel(&enlarged, 40, 40), DARK);
    assert_eq!(pixel(&enlarged, 119, 79), DARK);
    assert_eq!(pixel(&enlarged, 120, 79), GREY);
    assert_eq!(pixel(&enlarged, 119, 80), GREY);

    assert!(printed.contains("160 by 120"), "{printed}");
    assert!(
        printed.contains("is at 30 + x / 4, 10 + y / 4 in the picture"),
        "{printed}"
    );
}

#[test]
fn a_part_of_a_jpeg_is_enlarged_as_a_png_with_its_pixels_in_place() {
    let folder = references("enlargement-jpeg");
    support::made_up_jpeg(&folder.join("picture.jpg"), 200, 100);
    measure_ok(
        &folder,
        &[
            "enlarge-image",
            "--image",
            "picture.jpg",
            "--left",
            "30",
            "--top",
            "10",
            "--width",
            "40",
            "--height",
            "30",
            "--factor",
            "4",
            "--out",
            "working/part.png",
        ],
    );
    let written = std::fs::read(folder.join("working/part.png")).expect("the enlargement");
    assert!(
        written.starts_with(b"\x89PNG\r\n\x1a\n"),
        "the enlargement is not a PNG"
    );
    let enlarged = image::load_from_memory(&written)
        .expect("the enlargement is a picture")
        .to_rgb8();
    assert_eq!(enlarged.dimensions(), (160, 120));
    // A JPEG keeps a pixel's value only nearly, and least at an edge: so the middle of the
    // dark block, which is at 50, 25 in the picture, and a place well outside it.
    let near = |found: [u8; 3], colour: [u8; 3]| {
        (0..3).all(|channel| found[channel].abs_diff(colour[channel]) <= 12)
    };
    let (dark, grey) = (pixel(&enlarged, 82, 62), pixel(&enlarged, 140, 100));
    assert!(near(dark, DARK), "{dark:?}");
    assert!(near(grey, GREY), "{grey:?}");
    // Each pixel of the picture is one block of 4 by 4.
    for (x, y) in [(80, 60), (83, 60), (80, 63), (83, 63)] {
        assert_eq!(pixel(&enlarged, x, y), dark, "{x}, {y}");
    }
}

#[test]
fn a_grid_marks_the_pictures_own_pixels_and_labels_them() {
    let folder = folder("enlargement-grid");
    let printed = enlarge(
        &folder,
        ["30", "10", "40", "30"],
        "8",
        &["--grid", "10"],
        "part.png",
    );
    let enlarged = image::open(folder.join("part.png"))
        .expect("the enlargement is a picture")
        .to_rgb8();
    // The labels stand in a margin to the left and above, which the command states.
    let margin = |axis: &str| -> u32 {
        let after = printed
            .split(&format!("{axis} - "))
            .nth(1)
            .unwrap_or_else(|| panic!("no margin for {axis}: {printed}"));
        after
            .split(')')
            .next()
            .expect("a number")
            .parse()
            .expect("a number")
    };
    let (left, top) = (margin("x"), margin("y"));
    assert!(left > 0 && top > 0);
    assert_eq!(enlarged.dimensions(), (left + 320, top + 240));
    assert!(
        printed.contains(&format!(
            "is at 30 + (x - {left}) / 8, 10 + (y - {top}) / 8 in the picture"
        )),
        "{printed}"
    );

    // A grid line stands where a multiple of 10 of the picture begins: x = 40 is 10 pixels
    // into the part, so 80 into the enlargement, and it runs the whole height.
    let line = pixel(&enlarged, left + 80, top + 3);
    assert_ne!(line, GREY);
    assert_ne!(line, DARK);
    assert_eq!(pixel(&enlarged, left + 80, top + 200), line);
    assert_eq!(pixel(&enlarged, left + 3, top + 80), line, "y = 20");
    // Between the lines the picture is as it was.
    assert_eq!(pixel(&enlarged, left + 84, top + 84), DARK);
    assert_eq!(pixel(&enlarged, left + 76, top + 76), GREY);
    // The margin holds the labels: something is written above the line at x = 40.
    let written = (0..top)
        .any(|y| (left + 70..left + 110).any(|x| pixel(&enlarged, x, y) != pixel(&enlarged, 0, 0)));
    assert!(written, "no label above the grid line");
    assert!(printed.contains("every 10 pixels"), "{printed}");
}

#[test]
fn an_enlargement_a_reader_would_be_shown_resized_is_refused() {
    let folder = folder("enlargement-refused");
    for (part, factor, why) in [
        // 200 pixels, ten times: 2000 across.
        (["0", "0", "200", "100"], "10", "1568"),
        (["150", "0", "100", "50"], "2", "outside the picture"),
        (["0", "0", "0", "50"], "2", "at least 1"),
        (["0", "0", "50", "50"], "0", "whole number"),
        (["0", "0", "50", "50"], "2.5", "whole number"),
    ] {
        let output = maquette_in(
            Some(&folder),
            &[
                "enlarge-image",
                "--image",
                "picture.png",
                "--left",
                part[0],
                "--top",
                part[1],
                "--width",
                part[2],
                "--height",
                part[3],
                "--factor",
                factor,
                "--out",
                "refused.png",
            ],
        );
        let message = failure(&output);
        assert!(message.contains(why), "{part:?} x{factor}: {message}");
    }
    assert!(!folder.join("refused.png").exists());
}
