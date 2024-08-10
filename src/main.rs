use std::io::{stderr, stdout};

use clap::Parser;

use fasterface::cli::Cli;
use fasterface::outcome::Outcome;

#[tokio::main]
async fn main() -> Outcome {
    Cli::parse().run(&mut stdout(), &mut stderr()).await
}
