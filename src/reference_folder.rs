//! The reference folder: the one folder, outside every repository, that holds reference
//! images, the readings of them and the reference check sheets.
//!
//! This module hides where that folder is configured, what "outside every repository" means
//! and what "inside the folder" means. The repository is public and the drawings are other
//! people's, so nothing measuring reads or writes may land in a repository by accident: a
//! folder inside a Git repository is refused, every path a measuring command is given for an
//! image, a reading, a sheet or a result is taken inside the folder, and a path that leaves
//! it, by its own steps or through a link, is refused. When the folder is not configured or
//! does not exist, a measuring command stops with [`Folder::configured`]'s message, and
//! everything else still works.

use std::path::{Component, Path, PathBuf};

use crate::{Error, Result};

/// The environment variable that names the reference folder. It is the configuration: it is
/// set on the machine that has the folder and written in no committed file.
pub const VARIABLE: &str = "MAQUETTE_REFERENCES";

/// What every message about a missing folder ends with: where it is configured, and what
/// still works without it.
const HOW: &str = "Measuring keeps reference images, readings and reference check sheets in one folder outside every repository, and nowhere else. Name that folder in the environment variable MAQUETTE_REFERENCES; nothing committed names it. README.md, \"The reference folder\", says what it holds. Drawing an armature and its checks need no reference folder.";

/// Where a path leads once every link in it is followed. The part of it that exists is
/// followed, and what is named beyond that part, which a command is about to write, is kept
/// as named. None when a link in it leads nowhere: writing there would make a file wherever
/// the link points.
fn followed(path: &Path) -> Option<PathBuf> {
    let mut existing = path;
    let mut beyond = Vec::new();
    // A link that leads nowhere exists as a link, and is not followed past.
    while existing.symlink_metadata().is_err() {
        beyond.push(existing.file_name()?);
        existing = existing.parent()?;
    }
    let mut whole = existing.canonicalize().ok()?;
    whole.extend(beyond.into_iter().rev());
    Some(whole)
}

/// The Git repository a folder is in, if it is in one: the nearest folder, itself or above
/// it, whose `.git` is a repository as Git itself tells one. In a checkout that is a folder
/// with a `HEAD`; in a worktree and in a submodule it is a file that says where the
/// repository is. An empty folder named `.git` is no repository, and Git looks past it too.
fn repository_holding(folder: &Path) -> Option<&Path> {
    folder.ancestors().find(|above| {
        let git = above.join(".git");
        git.join("HEAD").is_file()
            || std::fs::read(&git).is_ok_and(|said| said.starts_with(b"gitdir:"))
    })
}

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
        if let Some(repository) = repository_holding(&root) {
            return Err(Error::new(format!(
                "the reference folder `{}`, which the environment variable {VARIABLE} names, is inside the Git repository `{}`, so nothing can be measured there. References are kept outside every repository, so that no image, reading or sheet can be committed: name a folder that is in no repository. {HOW}",
                named.display(),
                repository.display()
            )));
        }
        Ok(Folder { root })
    }

    /// Where the folder is.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The whole path of a file named to a measuring command, with every link in it
    /// followed. A relative path is taken from the reference folder, and a path that leaves
    /// the folder is refused: by its own steps, or through a link inside the folder that
    /// leads out of it. A folder reached through a link from outside is the same folder.
    pub fn path(&self, given: &str) -> Result<PathBuf> {
        let outside = || {
            Error::new(format!(
                "`{given}` is outside the reference folder `{}`. A reference image, a reading, a reference check sheet and anything worked out from one reading are kept in that folder and nowhere else; a path that is not absolute is taken from it, and a link that leads out of the folder is not followed.",
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
        // Joining an absolute path gives that path.
        followed(&self.root.join(path))
            .filter(|whole| whole.starts_with(&self.root))
            .ok_or_else(outside)
    }

    /// A path [`Folder::path`] gave, as a reading or a sheet names it: from the folder, so
    /// that nothing written says where the folder is on this machine.
    pub fn name_of(&self, path: &Path) -> String {
        path.strip_prefix(&self.root)
            .unwrap_or(path)
            .display()
            .to_string()
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
