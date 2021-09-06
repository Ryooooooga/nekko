use crate::cmd::find_snippet;
use crate::config::{default_snippets_dir, Snippets};
use crate::opt::ExpandArgs;
use std::process;

#[derive(Debug)]
struct ExpandResult<'a> {
    command: &'a str,
}

pub fn run(args: &ExpandArgs) {
    let snippets = Snippets::load_from_dir_or_exit(default_snippets_dir());

    if let Some(result) = expand(args, &snippets) {
        println!("{}", result.command);
    } else {
        process::exit(1);
    }
}

fn expand<'a>(args: &ExpandArgs, snippets: &'a Snippets) -> Option<ExpandResult<'a>> {
    let command =
        find_snippet(snippets, args.query.as_ref()).expect("failed to execute fuzzy finder");

    command.map(|command| ExpandResult { command })
}
