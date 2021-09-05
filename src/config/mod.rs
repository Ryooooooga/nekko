mod snippet;

use std::env::var_os;
use std::path::PathBuf;

static NEKKO_SNIPPETS_HOME: &str = "NEKKO_SNIPPETS_HOME";
static XDG_CONFIG_HOME: &str = "XDG_CONFIG_HOME";

pub fn default_snippets_dir() -> PathBuf {
    if let Some(dir) = var_os(NEKKO_SNIPPETS_HOME) {
        return PathBuf::from(dir);
    }

    let mut dir = match var_os(XDG_CONFIG_HOME) {
        Some(dir) => PathBuf::from(dir),
        None => {
            let mut dir = dirs::home_dir().unwrap();
            dir.push(".config");
            dir
        }
    };

    dir.push("nekko/snippets");
    dir
}
