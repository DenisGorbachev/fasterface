use ratatui::layout::Rect;

pub trait RectExt {
    fn within(self, x: u16, y: u16, width: u16, height: u16) -> Self;
}

impl RectExt for Rect {
    fn within(self, x: u16, y: u16, width: u16, height: u16) -> Self {
        self.intersection(Self::new(x, y, width, height))
    }
}
