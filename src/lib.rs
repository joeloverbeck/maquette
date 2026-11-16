//! Maquette generates armatures for anime-style characters: the construction guide for a
//! head, as SVG with named landmarks, from a set of proportions.
//!
//! Each module hides one decision:
//!
//! - [`proportions`]: the list of named proportions, and the reading and checking of a set.
//! - [`head_model`]: how proportions and landmarks relate, in both directions.
//! - [`views`]: how the landmarks are shown from the front and from the three-quarter turn.
//! - [`guides`]: how the named guides of a view are built from its landmarks.
//! - [`armature`]: the SVG and the landmark file of a view, as text.
//! - [`hand_made_views`]: how the record of a hand-made armature is read as landmarks.
//! - [`differences`]: how a generated armature is laid over a hand-made one, and how far
//!   apart the two put each landmark.
//! - [`overlay`]: how a hand-made view and its numbered differences are drawn over a
//!   generated view, for a review sheet.
//! - [`read_back`]: the oracle of the SVG contract: what VectorCraft holds of a view's SVG,
//!   compared with its landmark file.
//! - [`hard_case`]: the hard case the contract lists, run on a three-quarter view, with the
//!   two spoiled copies the oracle must fail.
//! - [`vectorcraft`]: how VectorCraft's server is started and spoken to.

pub mod armature;
pub mod differences;
pub mod guides;
pub mod hand_made_views;
pub mod hard_case;
pub mod head_model;
pub mod number;
pub mod overlay;
pub mod proportions;
pub mod read_back;
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
