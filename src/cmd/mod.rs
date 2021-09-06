use crate::config::Snippets;
use ansi_term::Color;
use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::process::{Command, Stdio};
use std::string::FromUtf8Error;
use thiserror::Error;

static FUZZY_FINDER_CMD: &str = "fzf";

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),
}

pub fn find_snippet<S: AsRef<str>>(
    snippets: &'_ Snippets,
    query: Option<S>,
) -> Result<Option<&'_ str>, Error> {
    let mut cmd = Command::new(FUZZY_FINDER_CMD);

    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());

    if let Some(query) = query {
        cmd.args(&["--query", query.as_ref()]);
    }

    let mut process = cmd.spawn()?;
    let mut line_map = HashMap::new();
    write_snippets(&mut process.stdin.take().unwrap(), snippets, &mut line_map)?;

    let exit_status = process.wait()?;

    let mut stdout = String::new();
    process.stdout.unwrap().read_to_string(&mut stdout)?;

    if exit_status.success() {
        let line = stdout.trim_end();
        let command = line_map.get(line).copied();

        Ok(command)
    } else {
        Ok(None)
    }
}

fn write_snippets<'a, W: io::Write>(
    w: &mut W,
    snippets: &'a Snippets,
    line_map: &mut HashMap<String, &'a str>,
) -> io::Result<()> {
    for snippet in &snippets.snippets {
        let description = snippet.description.as_ref().map(|d| format!("[{}]", d));
        let command = &snippet.command;

        let description_style = Color::Blue.normal();

        match description {
            Some(description) => {
                writeln!(w, "{} {}", description_style.paint(&description), command)?;

                let line = format!("{} {}", description, command);
                line_map.insert(line.trim_end().to_string(), command);
            }
            None => {
                writeln!(w, "{}", command)?;

                line_map.insert(command.trim_end().to_string(), command);
            }
        }
    }

    Ok(())
}
