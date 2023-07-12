use eyre::Result;
use std::{
	cmp::Ordering,
	collections::{HashMap, HashSet},
};

const DATA_DE: &str = include_str!("../data/de.json");
const DATA_EN: &str = include_str!("../data/en.json");
const DATA_ES: &str = include_str!("../data/es.json");
const DATA_FR: &str = include_str!("../data/fr.json");
const DATA_HI: &str = include_str!("../data/hi.json");
const DATA_IT: &str = include_str!("../data/it.json");
const DATA_PT: &str = include_str!("../data/pt.json");

pub fn get_data() -> Result<HashMap<String, Vec<String>>> {
	let mut ret: HashMap<String, Vec<String>> = HashMap::new();

	ret.insert("de".to_string(), serde_json::from_str(DATA_DE)?);
	ret.insert("en".to_string(), serde_json::from_str(DATA_EN)?);
	ret.insert("es".to_string(), serde_json::from_str(DATA_ES)?);
	ret.insert("fr".to_string(), serde_json::from_str(DATA_FR)?);
	ret.insert("hi".to_string(), serde_json::from_str(DATA_HI)?);
	ret.insert("it".to_string(), serde_json::from_str(DATA_IT)?);
	ret.insert("pt".to_string(), serde_json::from_str(DATA_PT)?);

	let mut all: Vec<String> = vec![];
	all.append(&mut serde_json::from_str(DATA_DE)?);
	all.append(&mut serde_json::from_str(DATA_EN)?);
	all.append(&mut serde_json::from_str(DATA_ES)?);
	all.append(&mut serde_json::from_str(DATA_FR)?);
	all.append(&mut serde_json::from_str(DATA_HI)?);
	all.append(&mut serde_json::from_str(DATA_IT)?);
	all.append(&mut serde_json::from_str(DATA_PT)?);
	ret.insert("blocklist".to_string(), all);

	Ok(ret)
}

pub fn filter_to_common_bases(words: &[String]) -> Vec<String> {
	let mut ret = words.to_vec();

	ret.sort_by(|a, b| if a.len() < b.len() || a < b { Ordering::Less } else { Ordering::Greater });

	let mut remove_words = HashSet::new();
	for (i, word) in ret.iter().enumerate() {
		for next_word in &ret[i..ret.len()] {
			if next_word.len() > word.len() && next_word.starts_with(word) {
				remove_words.insert(next_word);
			}
		}
	}

	ret.clone()
		.into_iter()
		.filter_map(|word| if !remove_words.contains(&word) { Some(word) } else { None })
		.collect()
}

pub fn get_leet_variations(word: &str) -> Vec<String> {
	let replacements = HashMap::from([('o', '0'), ('l', '1'), ('i', '1')]);

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
	fn test_filter_to_common_bases() {
		assert_eq!(
			filter_to_common_bases(&[
				"hello".to_string(),
				"hellothere".to_string(),
				"hi".to_string()
			]),
			vec!["hi", "hello"]
		);
	}

	#[test]
	fn test_get_leet_variations() {
		assert_eq!(
			get_leet_variations("hello"),
			vec!["he110", "he11o", "he1l0", "he1lo", "hel10", "hel1o", "hell0", "hello"]
		);
	}
}
