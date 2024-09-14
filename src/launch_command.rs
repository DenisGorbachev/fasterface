use std::env::current_dir;
use std::io::Write;

use crate::app::App;
use crate::outcome::Outcome;
use crate::tui;
use clap::Parser;

#[derive(Parser, Clone, Debug)]
pub struct LaunchCommand {}

impl LaunchCommand {
    pub async fn run(self, _stdout: &mut impl Write, _stderr: &mut impl Write) -> Outcome {
        let terminal = tui::init()?;
        // let app = App::from(terminal);
        // let app = AppBuilder::default().terminal(terminal).build()?;
        let current_dir = current_dir()?;
        let app = App::create(current_dir);
        let result = app.run(terminal).await;
        tui::restore();
        result
    }
}
