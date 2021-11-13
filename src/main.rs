use std::{fs::File, io::BufWriter};
use structopt::StructOpt;

mod lib;

#[derive(StructOpt)]
#[structopt(name = "lob", about = "Library of Babel")]
struct Cli {
    /// The max amount of letters in a word
    depth: u32,

    /// Use uppercase letters
    #[structopt(long, short = "u")]
    uppercase: bool,

    /// Use digits
    #[structopt(long, short = "d")]
    digits: bool,

    /// Use symbols
    #[structopt(long, short = "s")]
    symbols: bool,
}

fn main() {
    let args = Cli::from_args();

    let library = lib::create_library(args.depth, args.uppercase, args.digits, args.symbols);
    let file = File::create("library.json").unwrap();

    serde_json::to_writer(BufWriter::new(file), &library).unwrap();

    println!("Done!");
}
