use crate::config::{default_snippets_dir, Snippets};
use crate::opt::ListArgs;
use std::io;

pub fn run(args: &ListArgs) {
    let snippets = Snippets::load_from_dir_or_exit(default_snippets_dir());

    print_snippets(args, &snippets, &mut io::stdout()).unwrap();
}

fn print_snippets<W: io::Write>(
    _args: &ListArgs,
    snippets: &Snippets,
    out: &mut W,
) -> io::Result<()> {
    for snippet in &snippets.snippets {
        writeln!(out, "--------")?;

        if let Some(description) = &snippet.description {
            writeln!(out, "description:\n{}\n", description)?;
        }

        writeln!(out, "command:\n{}", snippet.command)?;
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
        print_snippets(&args, &snippets, &mut io::BufWriter::new(&mut out)).unwrap();

        assert_eq!(
            std::str::from_utf8(&out).unwrap(),
            r#"--------
description:
Description A

command:
echo Command A
--------
command:
echo Command B
"#
        );
    }
}
