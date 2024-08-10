use std::io::Write;

use clap::Parser;

use crate::command::Command;
use crate::outcome::Outcome;

#[derive(Parser, Clone, Debug)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Cli {
    pub async fn run(self, stdout: &mut impl Write, stderr: &mut impl Write) -> Outcome {
        match self.command {
            None => todo!(),
            Some(command) => command.run(stdout, stderr).await,
        }
    }
}
