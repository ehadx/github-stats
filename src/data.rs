use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GithubInfo {
    pub data: Data,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub viewer: Viewer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewer {
    pub login: Box<str>,
    pub name: Option<Box<str>>,
    pub repositories: Repositories,
    pub repositories_contributed_to: Repositories,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repositories {
    pub nodes: Box<[Node]>,
    pub page_info: PageInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub end_cursor: Option<Box<str>>,
    pub has_next_page: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub fork_count: usize,
    pub languages: Languages,
    pub name_with_owner: Box<str>,
    pub stargazers: Stargazers,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Languages {
    pub edges: Box<[Edge]>,
}

#[derive(Debug, Deserialize)]
pub struct Edge {
    pub node: EdgeNode,
    pub size: u64,
}

#[derive(Debug, Deserialize)]
pub struct EdgeNode {
    pub color: Option<Box<str>>,
    pub name: Box<str>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stargazers {
    pub total_count: usize,
}

#[derive(Debug)]
pub struct LangStats {
    pub size: u64,
    pub prop: f64,
    pub occurences: usize,
    pub color: Option<Box<str>>,
}

impl LangStats {
    pub fn new(size: u64, color: Option<Box<str>>) -> Self {
        Self {
            size,
            prop: 0.0,
            color,
            occurences: 1,
        }
    }
}

#[derive(Debug)]
pub struct Stats {
    pub forks: usize,
    pub stars: usize,
    pub langs: HashMap<Box<str>, LangStats>,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            forks: 0,
            stars: 0,
            langs: HashMap::new(),
        }
    }
}
