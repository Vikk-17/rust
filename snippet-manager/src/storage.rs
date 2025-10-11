use crate::snippet::Snippet;
use std::{
    fs,
    path::Path,
};

const STORAGE_FILE: &str = "snippet.json";

pub fn load_snippets() -> Vec<Snippet> {
    if !Path::new(STORAGE_FILE).exists() {
        Vec::new()
    }

    let data = fs::read_to_string(STORAGE_FILE).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}


pub fn save_snippets(snippets: &[Snippet]) -> Result<(), Box<dyn Error>> {
    let json = serde_json.to_string_pretty(snippets)?;
    fs::write(STORAGE_FILE, json)?;
    Ok(())
}

pub fn get_next_id(snippets: &[Snippet]) -> usize {
    snippets.iter().map(|s| s.id).max().unwrap_or(0) + 1
}
