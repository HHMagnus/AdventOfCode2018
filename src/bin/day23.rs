#![recursion_limit = "256"]
use std::fs;

use aoc_parse::{parser, prelude::*};

fn main() {
	let file = fs::read_to_string("input/day23.txt").expect("Should have read file");
	
	let parser = parser!(lines("pos=<" i32 "," i32 "," i32 ">, r=" i32));

	let parsed: Vec<_> = parser.parse(file.as_str()).unwrap();

	let idx = parsed.iter().map(|x| x.3).enumerate().max_by_key(|&(_, x)| x).unwrap().0;

	let max = parsed[idx];

	let part1 = parsed.iter().filter(|&&x| in_range(max, x)).count();

	println!("Day 23 part 1: {}", part1);

	let mut part2map = parsed.into_iter().flat_map(|x| {
		let d = x.0.abs() + x.1.abs() + x.2.abs();
		vec![((d - x.3).max(0), 1), (d + x.3 + 1, - 1)]
	}).collect::<Vec<_>>();
	part2map.sort();

	let mut part2 = 0;
	let mut part2count = 0;
	part2map.into_iter().fold(0, |acc, (res, diff)| {
		let nacc = acc + diff;
		if nacc > part2count {
			part2count = nacc;
			part2 = res;
		}
		nacc
	});

	println!("Day 23 part 2: {}", part2);
}

fn in_range(nanobot1: (i32, i32, i32, i32), nanobot2: (i32, i32, i32, i32)) -> bool {
	let dist = (nanobot1.0 - nanobot2.0).abs() + (nanobot1.1 - nanobot2.1).abs() + (nanobot1.2 - nanobot2.2).abs();

	dist <= nanobot1.3
}