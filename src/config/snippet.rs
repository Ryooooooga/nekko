use serde::Deserialize;
use serde_yaml;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SnippetError {
    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    DeseriazationError(#[from] serde_yaml::Error),
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

    #[allow(unused)]
    pub fn load_from_str<S: AsRef<str>>(content: S) -> Result<Self, SnippetError> {
        let snippets = serde_yaml::from_str(content.as_ref())?;

        Ok(snippets)
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, SnippetError> {
        let file = fs::File::open(path)?;
        let snippets = serde_yaml::from_reader(file)?;

        Ok(snippets)
    }

    pub fn load_from_dir<P: AsRef<Path>>(dir_path: P) -> Result<Self, SnippetError> {
        let mut snippets = Snippets::new();
        snippets.load_from_dir_internal(dir_path)?;
        Ok(snippets)
    }

    fn load_from_dir_internal<P: AsRef<Path>>(&mut self, dir_path: P) -> Result<(), SnippetError> {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            let ext = path.extension();

            let has_yaml_ext = ext
                .map(|ext| ext == OsStr::new("yaml") || ext == OsStr::new("yml"))
                .unwrap_or(false);

            if entry.file_type()?.is_dir() {
                // Load recursively.
                self.load_from_dir_internal(path)?;
            } else if has_yaml_ext {
                // Load YAML and merge into self.
                let mut snippets = Self::load_from_file(path)?;
                self.merge(&mut snippets);
            }
        }
        Ok(())
    }

    pub fn load_from_dir_or_exit<P: AsRef<Path>>(dir_path: P) -> Self {
        Self::load_from_dir(&dir_path).unwrap_or_else(|err| {
            let path = dir_path.as_ref().to_string_lossy();
            let error_message = format!("failed to load snippets from `{}': {}", path, err);

            eprintln!("{}", error_message);
            std::process::exit(1);
        })
    }

    fn merge(&mut self, other: &mut Self) {
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
            snippets:
            - description: Description A
              command: echo Command A

            - command: echo Command B
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

        assert_eq!(snippets.snippets.len(), 6);
    }

    #[test]
    fn test_merge() {
        let mut snippets_1 = Snippets::load_from_str(
            r#"
            snippets:
            - description: Description A
              command: echo Command A

            - command: echo Command B
        "#,
        )
        .unwrap();

        let mut snippets_2 = Snippets::load_from_str(
            r#"
            snippets:
            - description: Description C
              command: echo Command C
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
