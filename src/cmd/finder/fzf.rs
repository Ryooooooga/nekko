use super::{Finder, FinderOpts, FinderOutput, Result};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

pub struct Fzf {
    exec: fn(program: &str, args: &[&str], input: &str) -> Result<FinderOutput>,
}

impl Fzf {
    pub fn new() -> Fzf {
        Fzf { exec: exec_command }
    }
}

impl Finder for Fzf {
    fn run(&self, opts: &'_ FinderOpts<'_>) -> Result<FinderOutput> {
        let mut args = vec![];

        if let Some(query) = opts.query {
            args.push("--query");
            args.push(query);
        }

        (self.exec)("fzf", &args, opts.input)
    }
}

fn exec_command(program: &str, args: &[&str], input: &str) -> Result<FinderOutput> {
    let mut cmd = Command::new(program);

    cmd.args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());

    let mut process = cmd.spawn()?;
    process.stdin.take().unwrap().write(input.as_bytes())?;

    let exit_status = process.wait()?;

    let output = if let Some(mut stdout) = process.stdout {
        let mut output = String::new();
        stdout.read_to_string(&mut output)?;
        Some(output)
    } else {
        None
    };

    Ok(FinderOutput {
        exit_status,
        output,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fzf() {
        let fzf = Fzf {
            exec: |program, args, input| {
                assert_eq!(program, "fzf");
                assert_eq!(args, &["--query", "QUERY"]);
                assert_eq!(input, "INPUT\n");

                exec_command("true", &[], "")
            },
        };

        let opts = FinderOpts {
            input: "INPUT\n",
            query: Some("QUERY"),
        };

        fzf.run(&opts).expect("command should succeed");
    }
}
