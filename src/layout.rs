use ratatui::layout::Constraint::*;
use ratatui::layout::Direction::*;
use ratatui::layout::{Layout, Rect};
use ratatui::Frame;

#[expect(dead_code)]
pub struct AppLayout {
    location: Rect,
    command: Rect,
    rest: Rect,
}

impl AppLayout {
    pub fn from_frame(frame: &Frame) -> Self {
        Self::from_frame_area(frame.area())
    }

    pub fn from_frame_area(area: Rect) -> Self {
        let rects = Layout::default()
            .direction(Vertical)
            .constraints([
                Min(3),  // location
                Min(3),  // command
                Fill(1), // rest
            ])
            .split(area);
        Self {
            location: rects[0],
            command: rects[1],
            rest: rects[2],
        }
    }
}
