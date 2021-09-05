use serde::Deserialize;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;
use thiserror::Error;
use toml;

#[derive(Debug, Error)]
pub enum SnippetError {
    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    DeseriazationError(#[from] toml::de::Error),
}

#[derive(Debug, Deserialize)]
pub struct Snippets {
    pub snippets: Vec<Snippet>,
}

#[derive(Debug, Deserialize)]
pub struct Snippet {
    pub description: Option<String>,
    pub command: String,
}

impl Snippets {
    pub fn new() -> Self {
        Self { snippets: vec![] }
    }

    pub fn load_from_str<S: AsRef<str>>(content: S) -> Result<Self, SnippetError> {
        let snippets = toml::from_str(content.as_ref())?;

        Ok(snippets)
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, SnippetError> {
        let content = fs::read_to_string(path)?;

        Self::load_from_str(&content)
    }

    pub fn load_from_dir<P: AsRef<Path>>(dir_path: P) -> Result<Self, SnippetError> {
        let mut snippets = Self::new();

        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension() == Some(OsStr::new("toml")) && !entry.file_type()?.is_dir() {
                let mut s = Self::load_from_file(path)?;

                snippets.merge(&mut s);
            }
        }

        Ok(snippets)
    }

    pub fn merge(&mut self, other: &mut Self) {
        self.snippets.append(&mut other.snippets);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_from_str() {
        let snippets = Snippets::load_from_str(
            r#"
            [[snippets]]
            description = "Description A"
            command = "echo Command A"

            [[snippets]]
            command = "echo Command B"
            "#,
        )
        .unwrap();

        assert_eq!(snippets.snippets.len(), 2);

        assert_eq!(
            snippets.snippets[0].description,
            Some(String::from("Description A"))
        );
        assert_eq!(&snippets.snippets[0].command, "echo Command A");

        assert_eq!(snippets.snippets[1].description, None);
        assert_eq!(&snippets.snippets[1].command, "echo Command B");
    }

    #[test]
    fn test_load_from_dir() {
        use std::env;
        use std::path::PathBuf;

        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("test/snippets");

        let snippets = Snippets::load_from_dir(dir).unwrap();

        assert_eq!(snippets.snippets.len(), 5);
    }

    #[test]
    fn test_merge() {
        let mut snippets_1 = Snippets::load_from_str(
            r#"
            [[snippets]]
            description = "Description A"
            command = "echo Command A"

            [[snippets]]
            command = "echo Command B"
        "#,
        )
        .unwrap();

        let mut snippets_2 = Snippets::load_from_str(
            r#"
            [[snippets]]
            description = "Description C"
            command = "echo Command C"
        "#,
        )
        .unwrap();

        snippets_1.merge(&mut snippets_2);

        assert_eq!(snippets_1.snippets.len(), 3);

        assert_eq!(
            snippets_1.snippets[0].description,
            Some(String::from("Description A"))
        );
        assert_eq!(&snippets_1.snippets[0].command, "echo Command A");

        assert_eq!(snippets_1.snippets[1].description, None);
        assert_eq!(&snippets_1.snippets[1].command, "echo Command B");

        assert_eq!(
            snippets_1.snippets[2].description,
            Some(String::from("Description C"))
        );
        assert_eq!(&snippets_1.snippets[2].command, "echo Command C");
    }
}
