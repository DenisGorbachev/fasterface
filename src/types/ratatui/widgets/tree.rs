use crate::Node;
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::WidgetRef;

#[derive(new, Getters, Dissolve, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {}

impl WidgetRef for Tree {
    fn render_ref(&self, _area: Rect, _buf: &mut Buffer) {
        todo!()
    }
}
