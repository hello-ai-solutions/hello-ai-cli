use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct GitHubRepo {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub language: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubCommit {
    pub commit: CommitDetails,
}

#[derive(Debug, Deserialize)]
pub struct CommitDetails {
    pub message: String,
    pub author: CommitAuthor,
}

#[derive(Debug, Deserialize)]
pub struct CommitAuthor {
    pub name: String,
    pub date: String,
}

pub async fn analyze_github_repo(repo_url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let api_url = convert_to_api_url(repo_url)?;
    
    // Fetch repository info
    let repo_info: GitHubRepo = client
        .get(&api_url)
        .header("User-Agent", "Q-CLI-Analyzer")
        .send()
        .await?
        .json()
        .await?;
    
    // Fetch recent commits
    let commits_url = format!("{}/commits?per_page=10", api_url);
    let commits: Vec<GitHubCommit> = client
        .get(&commits_url)
        .header("User-Agent", "Q-CLI-Analyzer")
        .send()
        .await?
        .json()
        .await?;
    
    Ok(format_analysis(&repo_info, &commits))
}

fn convert_to_api_url(repo_url: &str) -> Result<String, Box<dyn Error>> {
    if repo_url.contains("github.com") {
        let parts: Vec<&str> = repo_url.split('/').collect();
        if parts.len() >= 2 {
            let owner = parts[parts.len() - 2];
            let repo = parts[parts.len() - 1];
            return Ok(format!("https://api.github.com/repos/{}/{}", owner, repo));
        }
    }
    Err("Invalid GitHub URL".into())
}

fn format_analysis(repo: &GitHubRepo, commits: &[GitHubCommit]) -> String {
    let commits_summary = commits.iter()
        .take(5)
        .map(|c| format!("- {} by {} ({})", 
            c.commit.message.lines().next().unwrap_or(""),
            c.commit.author.name,
            c.commit.author.date))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"
# GitHub Repository Analysis: {}

## 📊 Repository Overview
- **Name**: {}
- **Description**: {}
- **Language**: {}
- **Stars**: {} ⭐
- **Forks**: {} 🍴
- **Created**: {}
- **Last Updated**: {}

## 🔍 Key Achievements
- Active open-source project with {} stars
- Community engagement with {} forks
- Primary language: {}

## 📈 Recent Development Activity
{}

## 🌍 Community Impact
- **Visibility**: {} stars indicate strong community interest
- **Collaboration**: {} forks show active contribution
- **Maintenance**: Recent commits show active development

## 💡 Notable Commits (Last 5)
{}

## 🎯 Project Significance
This repository demonstrates active development and community engagement in the {} ecosystem.
"#,
        repo.full_name,
        repo.name,
        repo.description.as_deref().unwrap_or("No description available"),
        repo.language.as_deref().unwrap_or("Unknown"),
        repo.stargazers_count,
        repo.forks_count,
        repo.created_at,
        repo.updated_at,
        repo.stargazers_count,
        repo.forks_count,
        repo.language.as_deref().unwrap_or("Unknown"),
        if commits.is_empty() {
            "No recent commits found".to_string()
        } else {
            format!("{} recent commits found", commits.len())
        },
        repo.stargazers_count,
        repo.forks_count,
        commits_summary,
        repo.language.as_deref().unwrap_or("software development")
    )
}
