use derive_more::From;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

/// It's OK to add more variants to this enum. If the enum tag has a `usize` type, it can have 2^64 variants.
#[derive(From, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub enum LocationV1 {
    #[default]
    None,
    ThePathBuf(PathBuf),
}

use LocationV1::*;

impl Display for LocationV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            None => f.write_str("<none>"),
            ThePathBuf(path_buf) => f.write_str(&path_buf.display().to_string()),
        }
    }
}

impl LocationV1 {}
