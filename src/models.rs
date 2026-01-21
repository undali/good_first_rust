use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: i64,
    pub repo_name: String,
    pub url: String,
    pub creator: String,
    pub created_at: String,
    pub title: String,
    pub labels: String,
    pub star_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubIssue {
    pub url: String,
    pub title: String,
    pub user: GitHubUser,
    pub created_at: String,
    pub labels: Vec<GitHubLabel>,
    pub repository_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubUser {
    pub login: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubLabel {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubSearchResponse {
    pub items: Vec<GitHubIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryInfo {
    pub stargazers_count: i64,
}

#[derive(Debug, Clone)]
pub struct StargazerCount {
    #[allow(dead_code)]
    pub repo_name: String,
    pub star_count: i64,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse {
    pub issues: Vec<Issue>,
    pub total_count: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
