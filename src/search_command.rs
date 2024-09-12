use crate::found::Found;
use crate::i18n::{FunI18n, FUN_I18N_ALL_EN};
use crate::outcome::Outcome;
use clap::Parser;
use isolang::Language;
use not_found_error::Require;
use standard_traits::Contains;
use std::io::Write;

#[derive(Parser, Clone, Debug)]
pub struct SearchCommand {
    #[arg()]
    query: String,
}

impl SearchCommand {
    pub async fn run(self, stdout: &mut impl Write, _stderr: &mut impl Write) -> Outcome {
        let Self {
            query,
        } = self;
        let funs = FUN_I18N_ALL_EN.iter().filter(|fun| fun.contains(&query));
        for fun in funs {
            let title = fun.title();
            writeln!(stdout, "{title}")?;
        }
        Ok(())
    }
}

/// A naive implementation of the function search.
///
/// Ideally, the implementation should support fuzzy matching and ranking
pub fn find_func_i18n<'a>(funs: &'a [FunI18n], query: &'a str) -> Option<&'a FunI18n> {
    funs.iter().find(|fun| fun.contains(query))
}

pub fn get_language(code: &str) -> Found<Language> {
    Language::from_639_1(code).require()
}
