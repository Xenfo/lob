use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufWriter};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    index: u32,
    message: String,
}

pub fn create_dictionary(depth: u32) -> Vec<Entry> {
	let lowercase = String::from_utf8((b'a'..=b'z').collect()).unwrap();
	let mut dictionary = vec![];
	for char in lowercase.chars() {
			dictionary.push(Entry {
					index: 0,
					message: char.to_string(),
			});

			if depth > 1 {
					let new_depth = depth - 1;
					let mut new_dictionary = create_dictionary(new_depth);
					for entry in new_dictionary.iter_mut() {
							entry.message.push(char);
					}
					dictionary.append(&mut new_dictionary);
			}
	}

	let final_dictionary = dictionary
			.into_iter()
			.enumerate()
			.map(|(index, mut entry)| {
					entry.index = index as u32 + 1;
					entry
			})
			.collect();

	final_dictionary
}

pub fn write_dictionary(dictionary: Vec<Entry>, file: File) -> () {
	serde_json::to_writer(BufWriter::new(file), &dictionary).unwrap();
}
