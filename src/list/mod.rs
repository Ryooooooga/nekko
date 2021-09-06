use crate::config::{default_snippets_dir, Snippets};
use crate::opt::ListArgs;
use ansi_term::{Color, Style};
use std::io;

pub fn run(args: &ListArgs) {
    let snippets = Snippets::load_from_dir_or_exit(default_snippets_dir());

    print_snippets(args, &snippets, &mut io::stdout(), true).unwrap();
}

fn print_snippets<W: io::Write>(
    _args: &ListArgs,
    snippets: &Snippets,
    out: &mut W,
    colored: bool,
) -> io::Result<()> {
    for snippet in &snippets.snippets {
        writeln!(out, "--------")?;

        let description_style = if colored {
            Color::Blue.normal()
        } else {
            Style::default()
        };

        let command_style = Style::default();

        if let Some(description) = &snippet.description {
            writeln!(
                out,
                "{}: {}",
                description_style.paint("description"),
                description
            )?;
        }

        writeln!(
            out,
            "{}: {}",
            command_style.paint("    command"),
            snippet.command
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let args = ListArgs {};

        let snippets = Snippets::load_from_str(
            r#"
            snippets:
            - description: Description A
              command: echo Command A

            - command: echo Command B
            "#,
        )
        .unwrap();

        let mut out = Vec::new();
        print_snippets(&args, &snippets, &mut io::BufWriter::new(&mut out), false).unwrap();

        let actual = std::str::from_utf8(&out).unwrap();

        assert_eq!(
            actual,
            r#"--------
description: Description A
    command: echo Command A
--------
    command: echo Command B
"#
        );
    }
}
