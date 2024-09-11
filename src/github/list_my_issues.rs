use clap::Parser;
use derive_builder::Builder;
use derive_new::new;
use octocrab::Octocrab;
use std::io::Write;

#[derive(new, Builder, Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ListMyIssues {
    /// GitHub personal access token
    #[arg(long, env = "GITHUB_TOKEN")]
    token: String,

    /// GitHub username
    #[arg(long)]
    username: String,
}

impl ListMyIssues {
    pub async fn run(self, stdout: &mut impl Write, _stderr: &mut impl Write) -> Result<(), Box<dyn std::error::Error>> {
        let Self { token, username } = self;

        let octocrab = Octocrab::builder()
            .personal_token(token)
            .build()?;

        let issues = octocrab.issues()
            .list()
            .state(octocrab::params::State::Open)
            .assignee(&username)
            .per_page(100)
            .send()
            .await?;

        if issues.items.is_empty() {
            writeln!(stdout, "No issues assigned to you.")?;
        } else {
            for issue in issues.items {
                writeln!(
                    stdout,
                    "Repository: {}\nIssue: #{} - {}\nURL: {}\n",
                    issue.repository.full_name,
                    issue.number,
                    issue.title,
                    issue.html_url
                )?;
            }
        }

        Ok(())
    }
}
