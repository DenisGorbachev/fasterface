use std::io;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
};

/// A type alias for the terminal type used in this example.
pub type Terminal = ratatui::Terminal<CrosstermBackend<io::Stdout>>;

pub fn init() -> io::Result<Terminal> {
    set_panic_hook();
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stdout());
    Terminal::new(backend)
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore();
        hook(info);
    }));
}

/// Restores the terminal to its original state.
pub fn restore() {
    if let Err(err) = disable_raw_mode() {
        eprintln!("error disabling raw mode: {err}");
    }
    if let Err(err) = execute!(io::stdout(), LeaveAlternateScreen) {
        eprintln!("error leaving alternate screen: {err}");
    }
}
