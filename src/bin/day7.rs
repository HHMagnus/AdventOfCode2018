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

	let part2depents = depent.clone();

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

	let mut depent = part2depents;

	let mut workers = vec![(0, '-'); 5];

	let mut part2 = 0;

	while depent.len() != 0 {
		for i in 0..workers.len() {
			workers[i].0 -= 1;
			if workers[i].0 == 0 {
				depent.remove(&workers[i].1);

				let keys = depent.keys().map(|x| *x).clone().collect::<Vec<_>>();
				for d in keys {
					let new = depent.get(&d).unwrap().iter().filter(|&&x| x != workers[i].1).map(|&x| x).collect::<Vec<_>>();
					*depent.get_mut(&d).unwrap() = new;
				}
			}
		}

		part2 += 1;
		
		if workers.iter().filter(|&&x| x.0 <= 0).count() == 0 {
			continue;
		}

		while workers.iter().filter(|&&x| x.0 <= 0).count() > 0 {
			let mut dep0s = depent
				.iter()
				.filter(|(_, v)| v.len() == 0)
				.filter(|(c, _)| !workers.iter().any(|(_, c0)| c == &c0))
				.map(|(&k, _)| k)
				.collect::<Vec<_>>();
			dep0s.sort();
			if dep0s.is_empty() {
				break;
			}

			let curr = dep0s[0];

			let idx = workers.iter().enumerate().find(|(_, &x)| x.0 <= 0).unwrap().0;
			workers[idx] = (time(curr), curr);
		}
	}
	
	println!("Day 7 part 1: {}", part2-1);
}

fn time(c: char) -> i32 {
	60 + c as i32 - 'A' as i32 + 1
}