use std::fmt::Debug;

pub trait VirtualWidget: Debug {
    /// Dummy function; required for this trait to be a base trait of a trait object
    fn draw(&self);
}
