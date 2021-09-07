use crate::opt::InitArgs;
use shell_escape::escape;
use std::borrow::Cow;

static INIT_SCRIPT: &str = include_str!("nekko.zsh");
static SEARCH_FUNCTION: &str = "__nekko::search";

pub fn run(args: &InitArgs) {
    print!("{}", INIT_SCRIPT);

    if let Some(bindkey) = &args.bindkey {
        println!("bindkey {} {}", escape(Cow::from(bindkey)), SEARCH_FUNCTION);
    }
}
