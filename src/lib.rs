//! Maquette generates armatures for anime-style characters: the construction guide for a
//! head, as SVG with named landmarks, from a set of proportions.
//!
//! Each module hides one decision:
//!
//! - [`proportions`]: the list of named proportions, and the reading and checking of a set.
//! - [`head_model`]: how proportions and landmarks relate, in both directions.
//! - [`views`]: how the landmarks are shown from the front, from the three-quarter turn and
//!   from the side.
//! - [`guides`]: how the named guides of a view are built from its landmarks.
//! - [`armature`]: the SVG and the landmark file of a view, as text.
//! - [`drawing`]: how a hand-made drawing's SVG is read as groups and shapes with their
//!   points, and how a position is taken from a curve.
//! - [`hand_made_drawings`]: which node of which shape is which landmark in the hand-made
//!   drawings maquette relies on, and the records written from them.
//! - [`hand_made_views`]: how the record of a hand-made armature is read as landmarks, and
//!   the set of proportions worked out from it written.
//! - [`differences`]: how a generated armature is laid over a hand-made one, and how far
//!   apart the two put each landmark.
//! - [`overlay`]: how a hand-made view and its numbered differences are drawn over a
//!   generated view, for a review sheet.
//! - [`read_back`]: the oracle of the SVG contract: what VectorCraft holds of a view's SVG,
//!   compared with its landmark file.
//! - [`hard_case`]: the hard case the contract lists, run on a three-quarter view, with the
//!   two spoiled copies the oracle must fail.
//! - [`vectorcraft`]: how VectorCraft's server is started and spoken to.
//! - [`reference_folder`]: where the folder of reference images, readings and reference
//!   check sheets is configured, and what is inside it.
//! - [`readings`]: the record of a reading: which landmarks a view asks a reader for, what
//!   makes a reading complete, and how its positions are read by the head model.
//! - [`measuring`]: what readings say when they are set against each other: the agreement
//!   between two readings of a drawing, the turn a three-quarter drawing shows, the fit of a
//!   character's views to one head, and the proportions the readings give.
//! - [`known_positions`]: how a reading is scored against a drawing whose true positions
//!   are on record.
//! - [`enlargement`]: how a part of a picture is enlarged for a reader, and how a position
//!   in the enlargement is taken back to the picture.
//! - [`reference_check_sheet`]: how readings and the pictures they were read from are laid
//!   out on a reference check sheet.
//! - [`jons_check`]: the record of Jon's check of a reference check sheet: what he accepts,
//!   marks as off and overrules, and which readings are measured after it.
//! - [`table`]: the proportion table: what it is built from, when a proportion has a valid
//!   range, what a set of proportions is told when it is set against one, and how a whole
//!   set is made from one for a head to be drawn.
//! - [`sources`]: where the numbers of a drawn head come from, marked on its views for a
//!   review sheet.
//! - [`number`] and [`toml_text`]: how numbers and text are written, so that every file
//!   agrees on them.

pub mod armature;
pub mod differences;
pub mod drawing;
pub mod enlargement;
pub mod guides;
pub mod hand_made_drawings;
pub mod hand_made_views;
pub mod hard_case;
pub mod head_model;
pub mod jons_check;
pub mod known_positions;
pub mod measuring;
pub mod number;
pub mod overlay;
pub mod proportions;
pub mod read_back;
pub mod readings;
pub mod reference_check_sheet;
pub mod reference_folder;
pub mod sources;
pub mod table;
pub mod toml_text;
pub mod vectorcraft;
pub mod views;

/// What went wrong, in words a person can act on.
#[derive(Debug, Clone, PartialEq)]
pub struct Error(pub String);

impl Error {
    pub fn new(message: impl Into<String>) -> Error {
        Error(message.into())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
