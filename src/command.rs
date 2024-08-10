use std::io::Write;

use clap::Parser;

use Command::*;

use crate::outcome::Outcome;
use crate::search_command::SearchCommand;

#[derive(Parser, Clone, Debug)]
pub enum Command {
    Search(SearchCommand),
}

impl Command {
    pub async fn run(self, stdout: &mut impl Write, stderr: &mut impl Write) -> Outcome {
        match self {
            Search(command) => command.run(stdout, stderr).await,
        }
    }
}
