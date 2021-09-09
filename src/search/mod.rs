use crate::config::{default_snippets_dir, Snippet, Snippets};
use crate::finder;
use crate::opt::SearchArgs;
use ansi_term::{Color, Style};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::process;

pub fn run(args: &SearchArgs) {
    let snippets = Snippets::load_from_dir_or_exit(default_snippets_dir());

    if let Some(command) = search(args, &snippets).expect("search failed") {
        println!("{}", command);
    } else {
        process::exit(1);
    }
}

fn search(args: &SearchArgs, snippets: &Snippets) -> finder::Result<Option<String>> {
    let finder = finder::Fzf::new();

    let snippet = match find_snippet(&finder, snippets, args.query.as_ref())? {
        Some(snippet) => snippet,
        None => return Ok(None),
    };

    let command = expand_placeholders(snippet)?;

    Ok(command)
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
        placeholders: None,
    };

    let s2 = Snippet {
        description: Some("world".to_string()),
        command: "echo world".to_string(),
        placeholders: None,
    };

    assert_eq!(format_snippet(&s1, false), "echo hello");
    assert_eq!(format_snippet(&s2, false), "[world] echo world");
}

fn expand_placeholders(snippet: &Snippet) -> finder::Result<Option<String>> {
    let _placeholders = find_placeholders(&snippet.command);

    Ok(Some(snippet.command.clone()))
}

static PLACEHOLDER_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new("<(?P<name>[0-9_A-Za-z]+)>").unwrap());

fn find_placeholders(command: &str) -> Vec<&str> {
    PLACEHOLDER_PATTERN
        .captures_iter(command)
        .map(|cap| cap.name("name").unwrap().as_str())
        .unique()
        .collect::<Vec<_>>()
}

#[test]
fn test_find_placeholders() {
    let scenarios = [
        ("echo hello", vec![]),
        ("echo <a> <b>", vec!["a", "b"]),
        ("echo <<foo>> <bar> <foo>", vec!["foo", "bar"]),
    ];

    for (command, expected) in scenarios {
        assert_eq!(find_placeholders(command), expected);
    }
}

fn replace_placeholders(command: &str, values: Vec<(&str, &str)>) -> String {
    let mut command = command.to_string();
    for (name, value) in values {
        command = command.replace(&format!("<{}>", name), value);
    }
    command
}

#[test]
fn test_replace_placeholders() {
    let command = "echo <<foo>> <bar> <foo>";
    let placeholders = vec![("foo", "hello"), ("bar", "world")];

    assert_eq!(
        replace_placeholders(command, placeholders),
        "echo <hello> world hello"
    );
}
