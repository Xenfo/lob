use std::{env, fs::File};

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    let dictionary = lib::create_dictionary(args[1].parse().expect("Invalid depth"));
    let file = File::create("library.json").unwrap();

    lib::write_dictionary(dictionary, file);

    println!("Done!");
}
