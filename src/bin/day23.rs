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
}

fn in_range(nanobot1: (i32, i32, i32, i32), nanobot2: (i32, i32, i32, i32)) -> bool {
	let dist = (nanobot1.0 - nanobot2.0).abs() + (nanobot1.1 - nanobot2.1).abs() + (nanobot1.2 - nanobot2.2).abs();

	dist <= nanobot1.3
}