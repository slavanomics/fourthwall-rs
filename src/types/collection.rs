use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Page<T> {
    pub results: Vec<T>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Paging {
    pub pageNumber: i32,
    pub pageSize: i32,
    pub elementsSize: i32,
    pub elementsTotal: i32,
    pub totalPages: i32,
    pub hasNextPage: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: String,
}
