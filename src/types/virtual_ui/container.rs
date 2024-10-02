use crate::{Orientation, VirtualWidget};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Debug)]
pub struct Container {
    children: Vec<Box<dyn VirtualWidget>>,
    orientation: Orientation,
}

impl Container {}

impl VirtualWidget for Container {
    fn draw(&self) {
        todo!()
    }
}
