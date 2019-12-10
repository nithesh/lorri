//! # lorri
//! lorri is a wrapper over Nix to abstract project-specific build
//! configuration and patterns in to a declarative configuration.

#![warn(missing_docs)]

#[macro_use]
extern crate structopt;

extern crate regex;
#[macro_use]
extern crate lazy_static;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate notify;
extern crate tempfile;
extern crate vec1;

extern crate proptest;

pub mod bash;
pub mod build_loop;
pub mod builder;
pub mod cas;
pub mod changelog;
pub mod cli;
pub mod constants;
pub mod daemon;
pub mod locate_file;
pub mod logging;
pub mod nix;
pub mod ops;
pub mod osstrlines;
pub mod pathreduction;
pub mod project;
pub mod socket;
pub mod thread;
pub mod watch;

use std::path::{Path, PathBuf};

// OUT_DIR and build_rev.rs are generated by cargo, see ../build.rs
include!(concat!(env!("OUT_DIR"), "/build_rev.rs"));

/// A .nix file.
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct NixFile(PathBuf);

impl NixFile {
    /// Underlying `&OsStr`.
    pub fn as_os_str(&self) -> &std::ffi::OsStr {
        self.0.as_os_str()
    }
}

/// Proxy through the `Display` class for `PathBuf`.
impl std::fmt::Display for NixFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.display().fmt(f)
    }
}

impl From<&std::ffi::OsStr> for NixFile {
    fn from(s: &std::ffi::OsStr) -> NixFile {
        NixFile(PathBuf::from(s.to_owned()))
    }
}

impl From<PathBuf> for NixFile {
    fn from(p: PathBuf) -> NixFile {
        NixFile(p)
    }
}

impl slog::Value for NixFile {
    fn serialize(
        &self,
        _record: &slog::Record,
        key: slog::Key,
        serializer: &mut dyn slog::Serializer,
    ) -> slog::Result {
        serializer.emit_str(key, &self.as_os_str().to_string_lossy())
    }
}

/// A .drv file (generated by `nix-instantiate`).
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct DrvFile(PathBuf);

impl DrvFile {
    /// Underlying `Path`.
    pub fn as_path(&self) -> &Path {
        self.0.as_ref()
    }
}

impl From<PathBuf> for DrvFile {
    fn from(p: PathBuf) -> DrvFile {
        DrvFile(p)
    }
}
