mod lib;
mod args;


use args::{Lsr, SubCommand};
// use args::{Args, Ls};
use clap::Parser;

fn main() {
    let lsr = Lsr::parse();

    match lsr.subcmd {
        SubCommand::Ls(ls) => {
            lib::list_files(&ls.directory, &ls.file_type.as_deref().unwrap_or(""), &ls.order_by_size, false, ls.purge, ls.recursive);
        }
        SubCommand::Lsz(lsz) => {
            lib::list_files(&lsz.directory, &lsz.file_type.as_deref().unwrap_or(""), &lsz.order_by_size, true, lsz.purge, lsz.recursive);
        }
    }
}
