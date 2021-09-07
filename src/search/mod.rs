use crate::config::{default_snippets_dir, Snippet, Snippets};
use crate::finder;
use crate::opt::SearchArgs;
use ansi_term::{Color, Style};
use std::collections::HashMap;
use std::process;

#[derive(Debug)]
struct SearchResult<'a> {
    command: &'a str,
}

pub fn run(args: &SearchArgs) {
    let snippets = Snippets::load_from_dir_or_exit(default_snippets_dir());

    if let Some(result) = search(args, &snippets) {
        println!("{}", result.command);
    } else {
        process::exit(1);
    }
}

fn search<'a>(args: &SearchArgs, snippets: &'a Snippets) -> Option<SearchResult<'a>> {
    let finder = finder::Fzf::new();
    let snippet = find_snippet(&finder, snippets, args.query.as_ref())
        .expect("failed to execute fuzzy finder");

    snippet.map(|s| SearchResult {
        command: &s.command,
    })
}

fn find_snippet<'a, S: AsRef<str>>(
    finder: &dyn finder::Finder,
    snippets: &'a Snippets,
    query: Option<S>,
) -> finder::Result<Option<&'a Snippet>> {
    let lines = snippets
        .snippets
        .iter()
        .map(|s| format_snippet(s, true))
        .collect::<Vec<_>>()
        .join("\n");

    let result = finder.run(&finder::FinderOpts {
        input: &lines,
        query: query.as_ref().map(|q| q.as_ref()),
    })?;

    if result.exit_status.success() {
        let snippet_map = &snippets
            .snippets
            .iter()
            .map(|s| (format_snippet(s, false), s))
            .collect::<HashMap<_, _>>();

        let output = result.output.as_ref().map(|o| o.trim_end());
        let snippet = output.and_then(|o| snippet_map.get(o).copied());

        Ok(snippet)
    } else {
        Ok(None)
    }
}

fn format_snippet(snippet: &Snippet, colored: bool) -> String {
    let (desc_style, cmd_style) = if colored {
        (Color::Blue.normal(), Style::default())
    } else {
        (Style::default(), Style::default())
    };

    let cmd = cmd_style.paint(&snippet.command);

    if let Some(description) = &snippet.description {
        let desc = desc_style.paint(format!("[{}]", description));
        format!("{} {}", desc, cmd)
    } else {
        format!("{}", cmd)
    }
}

#[test]
fn test_format_snippet() {
    let s1 = Snippet {
        description: None,
        command: "echo hello".to_string(),
    };

    let s2 = Snippet {
        description: Some("world".to_string()),
        command: "echo world".to_string(),
    };

    assert_eq!(format_snippet(&s1, false), "echo hello");
    assert_eq!(format_snippet(&s2, false), "[world] echo world");
}
