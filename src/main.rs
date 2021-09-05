mod config;
mod init;
mod list;
mod opt;

use opt::{Opt, Subcommand};

fn main() {
    let opt = Opt::parse();

    match &opt.subcommand {
        Subcommand::Init(args) => init::run(args),
        Subcommand::List(args) => list::run(args),
        Subcommand::Expand(_args) => unimplemented!(),
        Subcommand::Exec(_args) => unimplemented!(),
    };
}
