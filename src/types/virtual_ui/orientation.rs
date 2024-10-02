use strum::Display;
#[allow(dead_code)]
pub use Orientation::*;

#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

impl Orientation {}
