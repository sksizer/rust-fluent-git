use super::Author;

#[derive(Debug, Clone)]
pub struct TagInfo {
    pub name: String,
    pub sha: String,
    pub message: Option<String>,
    pub tagger: Option<Author>,
    pub annotated: bool,
}
