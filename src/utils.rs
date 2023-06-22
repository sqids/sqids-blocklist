use eyre::Result;
use std::collections::HashMap;

const DATA_EN: &str = include_str!("../data/en.json");
const DATA_DE: &str = include_str!("../data/de.json");

pub fn get_data() -> Result<Vec<String>> {
	let mut data: Vec<String> = serde_json::from_str(DATA_EN).unwrap();
	data.append(&mut serde_json::from_str(DATA_DE).unwrap());

	Ok(data)
}

pub fn get_leet_variations(word: &str) -> Vec<String> {
	let replacements =
		HashMap::from([('a', '4'), ('s', '5'), ('o', '0'), ('e', '3'), ('l', '1'), ('i', '1')]);

	let init = word.to_string();
	let mut all = vec![init.chars().collect::<Vec<_>>()];

	loop {
		let mut new_words = vec![];
		let count = all.len();

		for char_vec in all.clone().into_iter() {
			for (i, letter) in char_vec.iter().enumerate() {
				for (from, to) in replacements.iter() {
					if *letter == *from {
						let mut new_word = char_vec.clone();
						new_word[i] = *to;
						new_words.push(new_word);
					}
				}
			}
		}

		all.append(&mut new_words);

		all.sort_unstable();
		all.dedup();

		if count == all.len() {
			break;
		}
	}

	all.into_iter().map(|v| v.into_iter().collect()).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_leet_variations() {
		assert_eq!(get_leet_variations("abci"), vec!["4bc1", "4bci", "abc1", "abci"]);
	}
}
