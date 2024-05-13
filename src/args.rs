
use clap::{
    Args,
    Parser,
    Subcommand
};


#[derive(Parser, Debug)]
pub struct Lsr {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}



#[derive(Parser, Debug)]
pub enum SubCommand {

    // The `ls` subcommand
    #[clap(name = "ls")]
    Ls(Ls),
    #[clap(name = "lsz")]
    Lsz(Lsz),
}

#[derive(Parser, Debug)]
pub struct Ls {
    // the directory to list
    #[clap(short, long, default_value = ".")]
    pub directory: String,
    #[clap(short, long)]
    pub file_type: String,
    #[clap(short, long, default_value = "")]
    pub order_by_size: String,
}


#[derive(Parser, Debug)]
pub struct Lsz {
    #[clap(short, long, default_value = ".")]
    pub directory: String,
    // the file type to filter by
    #[clap(short, long)]
    pub file_type: Option<String>,
    #[clap(short, long, default_value = "")]
    pub order_by_size: String,
}


// Path: src/dir_explorer.rs
