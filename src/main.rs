mod config;
mod finder;
mod init;
mod list;
mod opt;
mod search;

use opt::{Opt, Subcommand};

fn main() {
    let opt = Opt::parse();

    match &opt.subcommand {
        Subcommand::Init(args) => init::run(args),
        Subcommand::List(args) => list::run(args),
        Subcommand::Search(args) => search::run(args),
    };
}
