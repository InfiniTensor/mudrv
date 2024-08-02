#![doc = include_str!("../README.md")]
#![deny(warnings, unsafe_code, missing_docs)]

use std::{env::var_os, path::PathBuf};

/// Returns the path to the Neuware home directory, if it is set.
#[inline]
pub fn find_musa_home() -> Option<PathBuf> {
    var_os("MUSA_INSTALL_PATH").map(PathBuf::from)
}