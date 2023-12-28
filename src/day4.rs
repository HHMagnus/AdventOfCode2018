use std::{fs, collections::HashMap};

use aoc_parse::{parser, prelude::*};

#[derive(Clone, Copy)]
enum I {
	Started(u32, u32, u32, u32, u32, u32),
	Asleep(u32, u32, u32, u32, u32),
	Awake(u32, u32, u32, u32, u32),
}

pub fn day4() {
	let file = fs::read_to_string("input/day4.txt").expect("Should have read file");

	let parser = parser!(lines({
		"[" y:u32 "-" m:u32 "-" d:u32 " " h:u32 ":" mi:u32 "] Guard #" g:u32 " begins shift" => I::Started(y, m, d, h, mi, g),
		"[" y:u32 "-" m:u32 "-" d:u32 " " h:u32 ":" mi:u32 "] falls asleep" => I::Asleep(y, m, d, h, mi),
		"[" y:u32 "-" m:u32 "-" d:u32 " " h:u32 ":" mi:u32 "] wakes up" => I::Awake(y, m, d, h, mi),
	}));

	let mut parsed: Vec<I> = parser.parse(file.as_str()).unwrap();

	parsed.sort_by_key(|&x| {
		match x {
			I::Asleep(x1, x2, x3, x4, x5) => (x1, x2, x3, x4, x5),
			I::Awake(x1, x2, x3, x4, x5) => (x1, x2, x3, x4, x5),
			I::Started(x1, x2, x3, x4, x5, _) => (x1, x2, x3, x4, x5),
		}
	});
	
	let mut guard = 0;

	let mut map = HashMap::new();

	let mut last = 0;

	for s in &parsed {
		match s {
			&I::Started(_, _, _, _, _, g) => guard = g,
			&I::Asleep(_, _, _, _, x5) => last = x5,
			&I::Awake(_, _, _, _, x5) => {
				for i in last..x5 {
					let entry = map.entry((guard, i)).or_insert(0);
					*entry += 1;
				}
			}
		}
	}

	let guards: Vec<u32> = parsed.iter().filter_map(|x| {
		match x {
			I::Started(_, _, _, _, _, g) => Some(g.clone()),
			_ => None
		}
	}).collect();

	let asleep: HashMap<u32, i32> = guards.into_iter().map(|guard| {
		let mut sum = 0;
		for i in 0..60 {
			sum += map.get(&(guard, i)).unwrap_or(&0);
		}
		(guard, sum)
	}).collect();

	let part1guard = asleep.iter().max_by_key(|x| x.1).unwrap().0.clone();

	let part1minute = (0..60).max_by_key(|&time| map.get(&(part1guard, time)).unwrap_or(&0)).unwrap();

	let part1 = part1guard * part1minute;

	println!("Day 4 part 1: {}", part1);

	let part2choice = map.iter().max_by_key(|x| x.1).unwrap();

	let part2 = part2choice.0.0 * part2choice.0.1;

	println!("Day 5 part 2: {}", part2);
}