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

        let repos = octocrab.current()
            .list_repos_for_authenticated_user()
            .per_page(100)
            .send()
            .await?;

        let mut found_issues = false;

        for repo in repos.items {
            let issues = octocrab.issues(repo.owner.login, repo.name)
                .list()
                .state(octocrab::params::State::Open)
                .assignee(&username)
                .per_page(100)
                .send()
                .await?;

            if !issues.items.is_empty() {
                found_issues = true;
                writeln!(stdout, "Repository: {}", repo.full_name)?;
                for issue in issues.items {
                    writeln!(
                        stdout,
                        "Issue: #{} - {}\nURL: {}\n",
                        issue.number,
                        issue.title,
                        issue.html_url
                    )?;
                }
            }
        }

        if !found_issues {
            writeln!(stdout, "No issues assigned to you in any of your repositories.")?;
        }

        Ok(())
    }
}
