//! A head invented for the tests of measuring, and made-up readings of it.
//!
//! `HEAD` is a few landmarks in space, in units, with numbers chosen so that the sums can be
//! done by hand. A view of it is the plain projection the list of proportions states: a point
//! shows at its offset times the cosine of the turn plus its depth times the sine, at its own
//! height. No reading made here was read from a drawing.

use std::collections::BTreeMap;
use std::path::Path;

/// A made-up head: a landmark's stem and point, whether it is a left and right pair, and its
/// offset from the centre line, its drop below the eye line and its depth, in units.
/// The eye line is at drop 0, midway between the opening's top and bottom, and the chin at 1.
pub const HEAD: &[(&str, &str, bool, f64, f64, f64)] = &[
    ("hair-top", "", false, 0.0, -1.4, -0.6),
    ("chin", "", false, 0.0, 1.0, 0.2),
    ("nose-mark-bottom", "", false, 0.0, 0.45, 0.35),
    ("mouth-line-centre", "", false, 0.0, 0.7, 0.3),
    ("eye", "outer-corner", true, 0.6, 0.0, -0.05),
    ("eye", "inner-corner", true, 0.2, 0.02, 0.05),
    ("eye", "opening-top", true, 0.4, -0.1, 0.0),
    ("eye", "opening-bottom", true, 0.4, 0.1, 0.0),
    ("brow-line", "inner-end", true, 0.18, -0.3, 0.06),
    ("brow-line", "outer-end", true, 0.62, -0.28, -0.06),
    ("nose-mark", "corner", true, 0.08, 0.4, 0.3),
    ("mouth-line", "end", true, 0.14, 0.7, 0.25),
    ("ear", "top", true, 0.5, -1.3, -0.7),
];

/// One unit of the made-up head, in pixels: eye line to chin. Head height is 2.4 units, so
/// 480 pixels, and 1% of it is 4.8 pixels.
pub const UNIT: f64 = 200.0;
pub const EYE_LINE_Y: f64 = 400.0;
pub const PICTURE_WIDTH: f64 = 1000.0;

/// The made-up head seen turned by `turn_degrees` toward the viewer's right, about an upright
/// axis that stands at `axis_x`: each landmark's name and its position in pixels.
pub fn seen(turn_degrees: f64, axis_x: f64) -> BTreeMap<String, [f64; 2]> {
    let (sin, cos) = turn_degrees.to_radians().sin_cos();
    let mut points = BTreeMap::new();
    for (stem, point, pair, offset, drop, depth) in HEAD {
        let y = EYE_LINE_Y + drop * UNIT;
        let x = |offset: f64| axis_x + (offset * cos + depth * sin) * UNIT;
        if *pair {
            points.insert(format!("{stem}-left-{point}"), [x(*offset), y]);
            points.insert(format!("{stem}-right-{point}"), [x(-offset), y]);
        } else {
            points.insert(stem.to_string(), [x(0.0), y]);
        }
    }
    points
}

/// The same picture flipped left to right: the head now faces the viewer's left, and the
/// character's left and right exchange places and names.
pub fn flipped(points: &BTreeMap<String, [f64; 2]>) -> BTreeMap<String, [f64; 2]> {
    points
        .iter()
        .map(|(name, at)| {
            let name = if name.contains("-left-") {
                name.replace("-left-", "-right-")
            } else {
                name.replace("-right-", "-left-")
            };
            (name, [PICTURE_WIDTH - at[0], at[1]])
        })
        .collect()
}

/// A made-up reading, written as a reader's file is.
pub struct MadeUp {
    pub reader: String,
    /// The picture's file in the reference folder.
    pub picture: String,
    /// The picture's digest. Where no picture stands behind the reading it is made up too.
    pub sha256: String,
    pub view: &'static str,
    pub faces: Option<&'static str>,
    pub stated_turn: f64,
    pub level: bool,
    pub set_aside: bool,
    pub points: BTreeMap<String, [f64; 2]>,
}

impl MadeUp {
    pub fn front(reader: &str, points: BTreeMap<String, [f64; 2]>) -> MadeUp {
        MadeUp {
            reader: reader.to_string(),
            picture: "made-up-front.png".to_string(),
            sha256: "made-up-front-digest".to_string(),
            view: "front",
            faces: None,
            stated_turn: 0.0,
            level: true,
            set_aside: false,
            points,
        }
    }

    pub fn turned(reader: &str, points: BTreeMap<String, [f64; 2]>) -> MadeUp {
        MadeUp {
            reader: reader.to_string(),
            picture: "made-up-three-quarter.png".to_string(),
            sha256: "made-up-three-quarter-digest".to_string(),
            view: "three-quarter",
            faces: Some("the viewer's right"),
            stated_turn: 30.0,
            level: true,
            set_aside: false,
            points,
        }
    }

    /// Writes the reading into a made-up reference folder.
    pub fn write(&self, folder: &Path, name: &str) {
        let mut out = String::from(
            "# A made-up reading for a test. No drawing was read: the positions are a made-up head's.\n",
        );
        out.push_str("instructions = \"made up\"\n\n[reader]\n");
        out.push_str(&format!("name = \"{}\"\naids = \"made up\"\n", self.reader));
        out.push_str(&format!(
            "\n[image]\nfile = \"{}\"\nsha256 = \"{}\"\nwidth = {PICTURE_WIDTH}\nheight = 1000\n",
            self.picture, self.sha256
        ));
        out.push_str(
            "\n[reference]\nwork = \"a made-up work\"\ndesigner = \"a made-up designer\"\nsource = \"a made-up source\"\nage_band = \"adult\"\nsex = \"woman\"\nspecies = \"human\"\nmanner = \"made up\"\n",
        );
        out.push_str(&format!(
            "\n[judgement]\nview = \"{}\"\nview_why = \"made up: the view\"\n",
            self.view
        ));
        if let Some(faces) = self.faces {
            out.push_str(&format!(
                "faces = \"{faces}\"\nturn_why = \"made up: the turn\"\n"
            ));
        }
        out.push_str(&format!(
            "turn_degrees = {:?}\nlevel = {}\nlevel_why = \"made up: the level\"\nset_aside = {}\nset_aside_why = \"made up: the head looks down\"\n\n[landmarks]\n",
            self.stated_turn, self.level, self.set_aside
        ));
        if !self.set_aside {
            for (name, at) in &self.points {
                out.push_str(&format!(
                    "{name} = {{ x = {:?}, y = {:?} }}\n",
                    at[0], at[1]
                ));
            }
        }
        let path = folder.join(name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("the folder can be made");
        }
        std::fs::write(path, out).expect("the made-up reading is written");
    }
}
