use std::process;

mod utils;

fn main() {
	let min_length: usize = 3;

	let mut data;
	match utils::get_data() {
		Ok(d) => {
			data = d;
		}
		Err(e) => {
			println!("Could not fetch data: {e}");
			process::exit(1);
		}
	}

	data = data
		.into_iter()
		.filter_map(|v| if v.len() >= min_length { Some(v.to_lowercase()) } else { None })
		.collect();

	let mut leet = vec![];
	for word in data.iter() {
		leet.append(&mut utils::get_leet_variations(word));
	}
	data.append(&mut leet);

	data.sort_unstable();
	data.dedup();

	println!("{}", serde_json::to_string(&data).unwrap());
}
