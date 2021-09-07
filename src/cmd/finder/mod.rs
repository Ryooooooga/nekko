pub mod fzf;

use std::io;
use std::process::ExitStatus;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct FinderOpts<'a> {
    pub input: &'a str,
    pub query: Option<&'a str>,
}

#[derive(Debug)]
pub struct FinderOutput {
    pub exit_status: ExitStatus,
    pub output: Option<String>,
}

pub trait Finder {
    fn run(&self, opts: &'_ FinderOpts<'_>) -> Result<FinderOutput>;
}
