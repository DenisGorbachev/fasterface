use clap::Parser;
use derive_builder::Builder;
use derive_new::new;
use futures::Stream;
use futures_util::{StreamExt, TryStreamExt};
use octocrab::models::issues::Issue;
use octocrab::params::issues::Filter;
use octocrab::params::State;
use octocrab::Octocrab;
use std::error::Error;
use std::io::Write;
use tokio::pin;

#[derive(new, Builder, Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct GetMyIssues {
    /// GitHub personal access token
    #[arg(long, env = "GITHUB_TOKEN")]
    token: String,

    /// GitHub username
    #[arg(long)]
    username: String,
}

impl GetMyIssues {
    pub async fn run(self, stdout: &mut impl Write, _stderr: &mut impl Write) -> Result<(), Box<dyn Error>> {
        let Self {
            token,
            username,
        } = self;

        let octocrab = Octocrab::builder().personal_token(token).build()?;

        let repos_stream = octocrab
            .current()
            .list_repos_for_authenticated_user()
            .per_page(100)
            .send()
            .await?
            .into_stream(&octocrab);
        pin!(repos_stream);

        let mut found_issues = false;

        while let Some(repos_result) = repos_stream.next().await {
            let repo = repos_result?;
            let author = repo
                .owner
                .expect("the list_repos_for_authenticated_user call should return the `owner` field");
            if list_open_issues_on_repo_for_assignee(&octocrab, &author.login, &repo.name, username.clone(), stdout).await? {
                found_issues = true;
            }
        }

        if !found_issues {
            writeln!(stdout, "No issues assigned to you in any of your repositories.")?;
        }

        Ok(())
    }
}

async fn list_open_issues_on_repo_for_assignee(octocrab: &Octocrab, owner: impl Into<String>, repo: impl Into<String>, username: impl Into<Filter<String>>, stdout: &mut impl Write) -> Result<bool, Box<dyn Error>> {
    let issues_stream = get_issues_stream(octocrab, owner, repo, username).await?;
    pin!(issues_stream);
    while let Some(issue) = issues_stream.try_next().await? {
        writeln!(stdout, "Issue: #{} - {}\nURL: {}\n", issue.number, issue.title, issue.html_url)?;
    }
    Ok(true)
}

#[expect(clippy::needless_lifetimes)]
async fn get_issues_stream<'o>(octocrab: &'o Octocrab, owner: impl Into<String>, repo: impl Into<String>, username: impl Into<Filter<String>>) -> Result<impl Stream<Item = Result<Issue, octocrab::Error>> + 'o, Box<dyn Error>> {
    let issues = octocrab.issues(owner, repo);
    let username = username.into();
    let username_str = into_str_filter(&username);
    let page = issues
        .list()
        .state(State::Open)
        .assignee(username_str)
        .per_page(100)
        .send()
        .await?;
    let stream = page.into_stream(octocrab);
    Ok(stream)
}

pub fn into_str_filter(filter: &Filter<String>) -> Filter<&str> {
    match filter {
        Filter::Matches(string) => Filter::Matches(string.as_str()),
        Filter::Any => Filter::Any,
        Filter::None => Filter::None,
        _ => Filter::None,
    }
}
