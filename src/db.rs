use rusqlite::{Connection, Result as SqlResult};
use std::sync::Mutex;
use lazy_static::lazy_static;
use chrono::Utc;

use crate::models::{Issue, StargazerCount};

lazy_static! {
    pub static ref DB: Mutex<Connection> = {
        let conn = Connection::open("issues.db").expect("Failed to open database");
        Mutex::new(conn)
    };
}

pub fn init_db() -> SqlResult<()> {
    let conn = DB.lock().unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS issues (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_name TEXT NOT NULL,
            url TEXT NOT NULL UNIQUE,
            creator TEXT NOT NULL,
            created_at TEXT NOT NULL,
            title TEXT NOT NULL,
            labels TEXT NOT NULL,
            star_count INTEGER NOT NULL,
            inserted_at TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS stargazer_counts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_name TEXT NOT NULL UNIQUE,
            star_count INTEGER NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

pub fn get_stargazer_count(repo_name: &str) -> SqlResult<Option<StargazerCount>> {
    let conn = DB.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT repo_name, star_count, updated_at FROM stargazer_counts WHERE repo_name = ?1"
    )?;
    
    let result = stmt.query_row([repo_name], |row| {
        Ok(StargazerCount {
            repo_name: row.get(0)?,
            star_count: row.get(1)?,
            updated_at: row.get(2)?,
        })
    });

    match result {
        Ok(star_count) => Ok(Some(star_count)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn insert_or_update_stargazer_count(repo_name: &str, star_count: i64) -> SqlResult<()> {
    let conn = DB.lock().unwrap();
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO stargazer_counts (repo_name, star_count, updated_at) 
         VALUES (?1, ?2, ?3)
         ON CONFLICT(repo_name) DO UPDATE SET star_count = ?2, updated_at = ?3",
        [repo_name, &star_count.to_string(), &now],
    )?;

    Ok(())
}

pub fn insert_issue(issue: &Issue) -> SqlResult<()> {
    let conn = DB.lock().unwrap();
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT OR IGNORE INTO issues (repo_name, url, creator, created_at, title, labels, star_count, inserted_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![
            &issue.repo_name,
            &issue.url,
            &issue.creator,
            &issue.created_at,
            &issue.title,
            &issue.labels,
            issue.star_count,
            now,
        ],
    )?;

    Ok(())
}

pub fn get_paginated_issues(page: i64, per_page: i64, min_stars: i64) -> SqlResult<(Vec<Issue>, i64)> {
    let conn = DB.lock().unwrap();

    // Get total count with filter
    let total_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM issues WHERE star_count >= ?1",
        [min_stars],
        |row| row.get(0),
    )?;

    // Get paginated issues (latest first) with filter
    let offset = (page - 1) * per_page;
    let mut stmt = conn.prepare(
        "SELECT id, repo_name, url, creator, created_at, title, labels, star_count 
         FROM issues 
         WHERE star_count >= ?1
         ORDER BY created_at DESC 
         LIMIT ?2 OFFSET ?3"
    )?;

    let issues = stmt.query_map([min_stars, per_page, offset], |row| {
        Ok(Issue {
            id: row.get(0)?,
            repo_name: row.get(1)?,
            url: row.get(2)?,
            creator: row.get(3)?,
            created_at: row.get(4)?,
            title: row.get(5)?,
            labels: row.get(6)?,
            star_count: row.get(7)?,
        })
    })?
    .collect::<SqlResult<Vec<_>>>()?;

    Ok((issues, total_count))
}
