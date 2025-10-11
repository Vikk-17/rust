use serde::{Deserialize, Serialize};

// Macro that autmatically generates boilerplate code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub id: usize,
    pub title: String,
    pub code: String,
    pub language: String,
    pub tags: Vec<String>,
    pub description: String,
}

impl Snippet {
    pub fn new(
        id: usize,
        title: String,
        code: String,
        language: String,
        tags: Vec<String>,
        description: String,
    ) -> Self {
        Self {
            id,
            title,
            code,
            language,
            tags,
            description,
        }
    }

    pub fn matches_search(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        self.title.to_lowercase().contains(&query_lower)
            || self.description.to_lowercase().contains(&query_lower)
            || self.code.to_lowercase().contains(&query_lower)
            || self.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
    }
}
