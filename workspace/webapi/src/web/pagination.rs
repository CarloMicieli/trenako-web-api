use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaginateQuery {
    start: Option<usize>,
    limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResults<T: serde::Serialize> {
    links: Links,
    limit: usize,
    results: Vec<T>,
}

impl<T: Serialize> PaginatedResults<T> {
    pub fn new(results: Vec<T>) -> Self {
        PaginatedResults {
            links: Links::one_page("http://localhost:5000/api/"),
            limit: 10,
            results,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Links {
    self_link: String,
    prev_link: Option<String>,
    next_link: Option<String>,
}

impl Links {
    pub fn one_page(self_link: &str) -> Self {
        Links {
            self_link: self_link.to_owned(),
            prev_link: None,
            next_link: None,
        }
    }
}
