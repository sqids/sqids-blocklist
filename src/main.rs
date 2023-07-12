use std::{fs::File, io::Write};

mod utils;

fn main() {
	let min_length: usize = 3;

	let dataset = utils::get_data().expect("Could not fetch data");
	dataset.iter().for_each(|(language, data)| {
		let mut data: Vec<String> = data
			.iter()
			.filter_map(|v| if v.len() >= min_length { Some(v.to_lowercase()) } else { None })
			.collect();

		data = utils::filter_to_common_bases(&data);

		let mut leet: Vec<String> = vec![];
		for word in data.iter() {
			leet.append(&mut utils::get_leet_variations(word));
		}
		data.append(&mut leet);

		data.sort_unstable();
		data.dedup();

		match File::create(format!("output/{}.json", language)) {
			Err(e) => eprintln!("{}", e),
			Ok(mut f) => {
				let _ = f.write_all(serde_json::to_string(&data).unwrap().as_bytes());
			}
		}

		if language == "blocklist" {
			println!("{}", serde_json::to_string(&data).unwrap());
		}
	});
}
