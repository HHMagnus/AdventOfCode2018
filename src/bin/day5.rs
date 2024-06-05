use std::{collections::HashSet, fs};

fn main() {
	let file = fs::read_to_string("input/day5.txt").expect("Should have read file");

	let part1 = day5(file.clone());
	println!("Day 5 part 1: {}", part1);

	let part2 = file
		.clone()
		.to_ascii_lowercase()
		.chars()
		.collect::<HashSet<_>>()
		.iter()
		.map(|c|
			day5(file.clone().replace(&c.to_string(), "").replace(&c.to_ascii_uppercase().to_string(), ""))
		).min()
		.unwrap();
	println!("Day 5 part 2: {}", part2);
}

fn day5(file: String) -> usize {
	let mut moved = file;
	let mut changed = true;

	while changed {
		changed = false;

		let mut new = Vec::new();

		let mut last = None;

		for x in moved.chars() {
			if let Some(l) = last {
				if (x.is_lowercase() && x.to_ascii_uppercase() == l) || (x.is_uppercase() && x.to_ascii_lowercase() == l) {
					changed = true;
					last = None;
				} else {
					new.push(l);
					last = Some(x);
				}
			} else {
				last = Some(x);
			}
		}
		if last.is_some() {
			new.push(last.expect("Last"));
		}

		moved = new.into_iter().collect();
	}

	moved.len()
}