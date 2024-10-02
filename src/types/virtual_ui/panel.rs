use crate::Container;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Debug)]
pub struct Panel {
    #[new(into)]
    name: String,
    container: Container,
}

impl Panel {}
