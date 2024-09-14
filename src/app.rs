use crate::location_v1::LocationV1;
use crate::outcome::Outcome;
use crate::tui::Terminal;
use crate::Tree;
use crossterm::event::{Event, EventStream, KeyCode, KeyModifiers};
use derive_new::new;
use futures::prelude::*;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Padding, Paragraph};
use ratatui::Frame;
use std::io;
use tui_textarea::{Input, Key, TextArea};

#[allow(dead_code)]
#[derive(new, Clone, Debug)]
pub struct App {
    location: LocationV1,
    prompt: TextArea<'static>,
    tree: Tree,
}

// impl From<Terminal> for App {
//     fn from(terminal: Terminal) -> Self {
//         Self {
//             terminal,
//             should_quit: false,
//         }
//     }
// }

impl App {
    pub fn create(location: impl Into<LocationV1>) -> Self {
        let mut prompt = TextArea::default();
        prompt.set_cursor_line_style(Style::default());
        prompt.set_placeholder_text("Search apps");
        Self {
            location: location.into(),
            prompt,
            tree: Tree::default(),
        }
    }

    pub async fn run(mut self, mut terminal: Terminal) -> Outcome {
        terminal.try_draw(|frame| self.render_frame(frame))?;
        let mut events = EventStream::new();
        while let Some(result) = events.next().await {
            match result {
                Ok(event) => {
                    if let Event::Key(key) = event {
                        if key.code == KeyCode::Esc || key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                            break;
                        }
                    }
                    match event.into() {
                        // ignore newlines
                        Input {
                            key: Key::Enter,
                            ..
                        }
                        | Input {
                            key: Key::Char('m'),
                            ctrl: true,
                            alt: false,
                            ..
                        } => continue,
                        input => {
                            self.prompt.input(input);
                            terminal.try_draw(|frame| self.render_frame(frame))?;
                        }
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

    pub fn render_frame(&self, frame: &mut Frame) -> io::Result<()> {
        let top_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(1))
            .title(" Welcome to Fasterface! ");
        let top_block_area = frame.area();
        let top_block_inner_area = top_block.inner(top_block_area);
        let top_block_inner_layout = Layout::default()
            .constraints([
                // Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Percentage(100),
            ])
            .split(top_block_inner_area);
        // let top_paragraph_area = top_block_inner_layout[0];
        let search_area = top_block_inner_layout[0];
        let separator_area = top_block_inner_layout[1];
        let _content_area = top_block_inner_layout[2];
        let search_area_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(2), Constraint::Percentage(100)])
            .split(search_area);
        let search_prefix_area = search_area_layout[0];
        let search_input_area = search_area_layout[1];
        // let top_paragraph = Paragraph::new("Open an app to get started.");
        let search_prefix = Paragraph::new(" ");
        let separator = Line::from("─".repeat(separator_area.width as usize));

        // let location = Paragraph::new(format!("@ {}", &self.location));
        // let prompt_prefix = Span::styled("> ", Style::default().bold());
        // // Span::styled("Search commands", Style::default().dim())
        // let area = frame.area();
        // let location_area = area.within(0, 0, area.width, 1);
        // let prompt_prefix_area = area.within(0, 1, 2, 1);
        // let prompt_area = area.within(2, 1, area.width, 1);
        // frame.render_widget(location, location_area);
        // frame.render_widget(prompt_prefix, prompt_prefix_area);
        frame.render_widget(&top_block, top_block_area);
        // frame.render_widget(&top_paragraph, top_paragraph_area);
        frame.render_widget(search_prefix, search_prefix_area);
        frame.render_widget(separator, separator_area);
        frame.render_widget(&self.prompt, search_input_area);
        Ok(())
    }
}
