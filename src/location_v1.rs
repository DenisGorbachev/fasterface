use derive_more::From;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub enum LocationV1 {
    ThePathBuf(PathBuf),
}

use LocationV1::*;

impl Display for LocationV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ThePathBuf(path_buf) => f.write_str(&path_buf.display().to_string()),
        }
    }
}

impl LocationV1 {}
