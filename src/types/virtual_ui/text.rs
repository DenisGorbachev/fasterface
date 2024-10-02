use crate::VirtualWidget;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Text {
    #[new(into)]
    inner: String,
}

impl VirtualWidget for Text {
    fn draw(&self) {
        todo!()
    }
}
