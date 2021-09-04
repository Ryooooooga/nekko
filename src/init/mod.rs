use crate::opt::InitArgs;
use shell_escape::escape;
use std::borrow::Cow;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InitError {}

static INIT_SCRIPT: &str = include_str!("nekko.zsh");
static EXPAND_FUNCTION: &str = "__nekko::expand";

pub fn run(args: &InitArgs) -> Result<(), InitError> {
    print!("{}", INIT_SCRIPT);

    if let Some(bindkey) = &args.bindkey {
        println!("bindkey {} {}", escape(Cow::from(bindkey)), EXPAND_FUNCTION);
    }

    Ok(())
}
