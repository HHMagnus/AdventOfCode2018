#![recursion_limit = "256"]
use std::{collections::{HashMap, HashSet}, fs};

use aoc_parse::{parser, prelude::{lines, upper}, Parser};

fn main() {
	let file = fs::read_to_string("input/day7.txt").expect("Should have read file");

	let parser = parser!(lines("Step " upper " must be finished before step " upper " can begin."));

	let parsed: Vec<(char, char)> = parser.parse(file.as_str()).unwrap();

	let mut depent: HashMap<char, Vec<char>> = HashMap::new();

	for (dep, curr) in parsed {
		depent.entry(curr).or_insert(Vec::new()).push(dep);
		depent.entry(dep).or_insert(Vec::new());
	}

	let mut part1 = String::new();
	
	while depent.len() != 0 {
		let mut dep0s = depent.iter().filter(|(_, v)| v.len() == 0).map(|(&k, _)| k).collect::<Vec<_>>();
		dep0s.sort();
		let curr = dep0s[0];

		part1.push(curr);

		depent.remove(&curr);

		let keys = depent.keys().map(|x| *x).clone().collect::<Vec<_>>();
		for d in keys {
			let new = depent.get(&d).unwrap().iter().filter(|&&x| x != curr).map(|&x| x).collect::<Vec<_>>();
			*depent.get_mut(&d).unwrap() = new;
		}
	}

	println!("Day 7 part 1: {}", part1);
}