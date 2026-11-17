//! The reference folder: the one folder, outside the repository, that holds reference images,
//! the readings of them and the reference check sheets.
//!
//! This module hides where that folder is configured and what "inside it" means. The
//! repository is public and the drawings are other people's, so nothing measuring reads or
//! writes may land in the repository by accident: every path a measuring command is given
//! for an image, a reading, a sheet or a result is taken inside this folder, and a path that
//! leaves it is refused. When the folder is not configured or does not exist, a measuring
//! command stops with [`Folder::configured`]'s message, and everything else still works.

use std::path::{Component, Path, PathBuf};

use crate::{Error, Result};

/// The environment variable that names the reference folder. It is the configuration: it is
/// set on the machine that has the folder and written in no committed file.
pub const VARIABLE: &str = "MAQUETTE_REFERENCES";

/// What every message about a missing folder ends with: where it is configured, and what
/// still works without it.
const HOW: &str = "Measuring keeps reference images, readings and reference check sheets in one folder outside the repository, and nowhere else. Name that folder in the environment variable MAQUETTE_REFERENCES; nothing committed names it. README.md, \"The reference folder\", says what it holds. Drawing an armature and its checks need no reference folder.";

/// The reference folder of this machine.
#[derive(Debug, Clone)]
pub struct Folder {
    root: PathBuf,
}

impl Folder {
    /// The folder the environment names, or the plain message that says what is missing and
    /// where it is configured.
    pub fn configured() -> Result<Folder> {
        Folder::named_by(std::env::var_os(VARIABLE).as_deref())
    }

    fn named_by(value: Option<&std::ffi::OsStr>) -> Result<Folder> {
        let Some(value) = value.filter(|value| !value.is_empty()) else {
            return Err(Error::new(format!(
                "the reference folder is not configured, so nothing can be measured. {HOW}"
            )));
        };
        let named = PathBuf::from(value);
        if !named.is_dir() {
            return Err(Error::new(format!(
                "the reference folder `{}`, which the environment variable {VARIABLE} names, does not exist, so nothing can be measured. {HOW}",
                named.display()
            )));
        }
        let root = named.canonicalize().map_err(|error| {
            Error::new(format!(
                "cannot open the reference folder `{}`: {error}",
                named.display()
            ))
        })?;
        Ok(Folder { root })
    }

    /// Where the folder is.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The whole path of a file named to a measuring command. A relative path is taken from
    /// the reference folder, and a path that leaves the folder is refused.
    pub fn path(&self, given: &str) -> Result<PathBuf> {
        let outside = || {
            Error::new(format!(
                "`{given}` is outside the reference folder `{}`. A reference image, a reading, a reference check sheet and anything worked out from one reading are kept in that folder and nowhere else; a path that is not absolute is taken from it.",
                self.root.display()
            ))
        };
        let path = Path::new(given);
        if given.is_empty()
            || path
                .components()
                .any(|part| matches!(part, Component::ParentDir))
        {
            return Err(outside());
        }
        if path.is_absolute() {
            // A folder reached through a link is the same folder.
            let inside = path.starts_with(&self.root)
                || path
                    .parent()
                    .and_then(|parent| parent.canonicalize().ok())
                    .is_some_and(|parent| parent.starts_with(&self.root));
            if !inside {
                return Err(outside());
            }
            return Ok(path.to_path_buf());
        }
        Ok(self.root.join(path))
    }

    /// A path of the folder as a reading or a sheet names it: from the folder, so that
    /// nothing written says where the folder is on this machine.
    pub fn name_of(&self, path: &Path) -> String {
        let within = path
            .strip_prefix(&self.root)
            .ok()
            .map(Path::to_path_buf)
            .or_else(|| {
                let parent = path.parent()?.canonicalize().ok()?;
                let within = parent.strip_prefix(&self.root).ok()?;
                Some(within.join(path.file_name()?))
            });
        match within {
            Some(within) => within.display().to_string(),
            None => path.display().to_string(),
        }
    }

    pub fn read(&self, given: &str) -> Result<Vec<u8>> {
        let path = self.path(given)?;
        std::fs::read(&path).map_err(|error| {
            Error::new(format!(
                "cannot read `{given}` in the reference folder ({}): {error}",
                path.display()
            ))
        })
    }

    pub fn read_text(&self, given: &str) -> Result<String> {
        String::from_utf8(self.read(given)?).map_err(|_| {
            Error::new(format!(
                "`{given}` in the reference folder is not a text file"
            ))
        })
    }

    /// Writes a file into the folder, making the folders above it, and returns its whole path.
    pub fn write(&self, given: &str, bytes: &[u8]) -> Result<PathBuf> {
        let path = self.path(given)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| {
                Error::new(format!("cannot make `{}`: {error}", parent.display()))
            })?;
        }
        std::fs::write(&path, bytes)
            .map_err(|error| Error::new(format!("cannot write `{}`: {error}", path.display())))?;
        Ok(path)
    }
}
