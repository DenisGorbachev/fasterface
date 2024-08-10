use std::io::Write;

use clap::Parser;

use crate::outcome::Outcome;

#[derive(Parser, Clone, Debug)]
pub struct SearchCommand {
    #[arg()]
    query: String,
}

impl SearchCommand {
    pub async fn run(self, _stdout: &mut impl Write, _stderr: &mut impl Write) -> Outcome {
        todo!()
    }
}
