use structopt::{clap, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = clap::crate_name!(),
    version = clap::crate_version!(),
    author = clap::crate_authors!(),
    about = clap::crate_description!(),
    version_short = "v",
    setting(clap::AppSettings::ColoredHelp),
)]
pub struct Opt {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

impl Opt {
    pub fn parse() -> Self {
        Self::from_args()
    }
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    #[structopt(about = "Initialize the plugin")]
    Init(InitArgs),

    #[structopt(about = "List snippets")]
    List(ListArgs),

    #[structopt(about = "Search snippets")]
    Search(SearchArgs),
}

#[derive(Debug, StructOpt)]
pub struct InitArgs {
    #[structopt(help = "Key binding", long)]
    pub bindkey: Option<String>,
}

#[derive(Debug, StructOpt)]
pub struct ListArgs {}

#[derive(Debug, StructOpt)]
pub struct SearchArgs {
    #[structopt(help = "Initial value for query", long, short = "q")]
    pub query: Option<String>,
}
