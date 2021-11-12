use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
	index: u32,
	message: String,
}

pub fn create_library(depth: u32) -> Vec<Entry> {
	let characters = String::from_utf8(
		(b'a'..=b'z')
			.chain(b'A'..=b'Z')
			.chain(b'0'..=b'9')
			.chain(vec![b' ', b'-'])
			.collect(),
	)
	.unwrap();
	let mut library = vec![];
	for char in characters.chars() {
		library.push(Entry {
			index: 0,
			message: char.to_string(),
		});

		if depth > 1 {
			let new_depth = depth - 1;
			let mut new_library = create_library(new_depth);
			for entry in new_library.iter_mut() {
				entry.message.push(char);
			}
			library.append(&mut new_library);
		}
	}

	let final_library = library
		.into_iter()
		.enumerate()
		.map(|(index, mut entry)| {
			entry.index = index as u32 + 1;
			entry
		})
		.collect();

	final_library
}
