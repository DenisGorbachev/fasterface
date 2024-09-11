use clap::Parser;
use derive_builder::Builder;
use derive_new::new;
use futures_util::pin_mut;
use octocrab::{models::Repository, params::State, Octocrab};
use std::io::Write;
use futures_util::StreamExt;

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

        let repos_stream = octocrab.current()
            .list_repos_for_authenticated_user()
            .per_page(100)
            .into_stream(&octocrab);
        pin_mut!(repos_stream);

        let mut found_issues = false;

        while let Some(repos_result) = repos_stream.next().await {
            let repos = repos_result?;
            for repo in repos {
                if self.list_issues_for_repo(&octocrab, &repo, &username, stdout).await? {
                    found_issues = true;
                }
            }
        }

        if !found_issues {
            writeln!(stdout, "No issues assigned to you in any of your repositories.")?;
        }

        Ok(())
    }

    async fn list_issues_for_repo(
        &self,
        octocrab: &Octocrab,
        repo: &Repository,
        username: &str,
        stdout: &mut impl Write,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let issues_stream = octocrab.issues(repo.owner.login.clone(), repo.name.clone())
            .list()
            .state(State::Open)
            .assignee(username)
            .per_page(100)
            .into_stream(&octocrab);
        pin_mut!(issues_stream);

        let mut found_issues = false;
        let mut first_issue = true;

        while let Some(issues_result) = issues_stream.next().await {
            let issues = issues_result?;
            if !issues.is_empty() {
                if first_issue {
                    writeln!(stdout, "Repository: {}", repo.full_name)?;
                    first_issue = false;
                }
                for issue in issues {
                    writeln!(
                        stdout,
                        "Issue: #{} - {}\nURL: {}\n",
                        issue.number,
                        issue.title,
                        issue.html_url
                    )?;
                }
                found_issues = true;
            }
        }

        Ok(found_issues)
    }
}
