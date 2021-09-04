mod init;
mod opt;

use init::InitError;
use opt::{Opt, Subcommand};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    InitError(#[from] InitError),
}

fn main() -> Result<(), Error> {
    let opt = Opt::parse();

    match &opt.subcommand {
        Subcommand::Init(args) => init::run(args)?,
        Subcommand::List(args) => unimplemented!(),
        Subcommand::Expand(args) => unimplemented!(),
        Subcommand::Exec(args) => unimplemented!(),
    };

    Ok(())
}
