use std::{env, fs::File, io::BufWriter};

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    let library = lib::create_library(args[1].parse().expect("Invalid depth"));
    let file = File::create("library.json").unwrap();

    serde_json::to_writer(BufWriter::new(file), &library).unwrap();

    println!("Done!");
}
