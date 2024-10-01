use std::io::{stderr, stdout};

use clap::Parser;

use fasterface::Cli;
use fasterface::Outcome;

#[tokio::main]
async fn main() -> Outcome {
    Cli::parse().run(&mut stdout(), &mut stderr()).await
}
