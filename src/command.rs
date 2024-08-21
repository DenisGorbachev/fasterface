use std::io::Write;

use clap::Parser;

use crate::launch_command::LaunchCommand;
use crate::outcome::Outcome;
use crate::search_command::SearchCommand;
use Command::*;

#[derive(Parser, Clone, Debug)]
pub enum Command {
    Launch(LaunchCommand),
    Search(SearchCommand),
}

impl Command {
    pub async fn run(self, stdout: &mut impl Write, stderr: &mut impl Write) -> Outcome {
        match self {
            Search(command) => command.run(stdout, stderr).await,
            Launch(command) => command.run(stdout, stderr).await,
        }
    }
}
