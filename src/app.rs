use crate::outcome::Outcome;
use crate::tui::Terminal;
use crossterm::event::{Event, EventStream, KeyCode};
use futures::prelude::*;
use ratatui::Frame;
use std::io;

#[derive(Default, Clone, Debug)]
pub struct App {}

// impl From<Terminal> for App {
//     fn from(terminal: Terminal) -> Self {
//         Self {
//             terminal,
//             should_quit: false,
//         }
//     }
// }

impl App {
    pub async fn run(self, mut terminal: Terminal) -> Outcome {
        terminal.try_draw(|frame| self.render_frame(frame))?;
        let mut events = EventStream::new();
        while let Some(result) = events.next().await {
            match result {
                Ok(event) => {
                    if event == Event::Key(KeyCode::Esc.into()) {
                        break;
                    }
                }
                Err(e) => return Err(e.into()),
            }
            // Some(Ok(event)) => {
            // }
            // Some(Err(e)) => eprintln!("Error: {:?}\r", e),
        }

        // while !self.should_quit {
        //     // if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        //     //     if let Event::Key(key) = event::read().context("event read failed")? {
        //     //         return Ok(KeyCode::Char('q') == key.code);
        //     //     }
        //     // }
        // }
        Ok(())
    }

    // pub fn render(&mut self) -> io::Result<CompletedFrame> {
    //     self.terminal.try_draw(|frame| self.render_frame(frame))
    // }

    pub fn render_frame(&self, _frame: &mut Frame) -> io::Result<()> {
        todo!()
    }
}
