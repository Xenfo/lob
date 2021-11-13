use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    index: u32,
    message: String,
}

pub fn create_library(depth: u32, uppercase: bool, digits: bool, symbols: bool) -> Vec<Entry> {
    let characters = (b'a'..=b'z')
        .chain(uppercase.then(|| b'A'..=b'Z').into_iter().flatten())
        .chain(digits.then(|| b'0'..=b'9').into_iter().flatten())
        .chain(uppercase.then(|| [b' ', b'-']).into_iter().flatten());

    let characters = String::from_utf8(characters.collect()).unwrap();
    let mut library = vec![];
    for char in characters.chars() {
        library.push(Entry {
            index: 0,
            message: char.to_string(),
        });

        if depth > 1 {
            let new_depth = depth - 1;
            let mut new_library = create_library(new_depth, uppercase, digits, symbols);
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
