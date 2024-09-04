use std::io::Write;

use clap::Parser;

use crate::command::Command;
use crate::outcome::Outcome;

#[derive(Parser, Clone, Debug)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub async fn run(self, stdout: &mut impl Write, stderr: &mut impl Write) -> Outcome {
        self.command.run(stdout, stderr).await
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
