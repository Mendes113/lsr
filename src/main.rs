mod dir_explorer;
mod args;

use std::env;

use args::{Ls, Lsr, SubCommand};
// use args::{Args, Ls};
use clap::Parser;

fn main() {
    let lsr = Lsr::parse();

    match lsr.subcmd {
        SubCommand::Ls(ls) => {
            dir_explorer::list_files(&ls.directory, &ls.file_type, &ls.order_by_size, false, ls.purge);
        }
        SubCommand::Lsz(lsz) => {
            dir_explorer::list_files(&lsz.directory, &lsz.file_type.as_deref().unwrap_or(""), &lsz.order_by_size, true, lsz.purge);
        }
    }
}
