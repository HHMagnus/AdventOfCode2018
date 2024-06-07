use std::{collections::{HashMap, HashSet, VecDeque}, fs};

fn main() {
	let file = fs::read_to_string("input/day12.txt").expect("Should have read file");
	let split = file.split("\n\n").collect::<Vec<_>>();
	let initial_state = split[0].replace("initial state: ", "").chars().enumerate().filter_map(|(i, x)| if x == '#' { Some(i as i64) } else { None }).collect::<HashSet<_>>();


	let map = split[1].lines().map(|line| {
		let split = line.split(" => ").collect::<Vec<_>>();
		let c = split[0].chars().map(|c| c == '#').collect::<Vec<_>>();
		((c[0], c[1], c[2], c[3], c[4]), split[1] == "#")
	}).collect::<HashMap<_, _>>();

	let mut state = initial_state.clone();

	for _ in 0..20 {
		let mut nextgen = HashSet::new();
		for x in state.iter().min().unwrap()-2..state.iter().max().unwrap()+2 {
			let vec = (state.contains(&(x-2)), state.contains(&(x-1)), state.contains(&x), state.contains(&(x+1)), state.contains(&(x+2)));
			if map.contains_key(&vec) && map[&vec] {
				nextgen.insert(x);
			}
		}
		state = nextgen;
	}

	let part1: i64 = state.iter().sum();

	println!("Day 12 part 1: {}", part1);

	let mut state = initial_state;

	let mut i = 0 as i64;

	let mut changes = VecDeque::new();

	let mut part2 = 0;

	while i < 50000000000 {
		let mut nextgen = HashSet::new();
		for x in state.iter().min().unwrap()-2..state.iter().max().unwrap()+2 {
			let vec = (state.contains(&(x-2)), state.contains(&(x-1)), state.contains(&x), state.contains(&(x+1)), state.contains(&(x+2)));
			if map.contains_key(&vec) && map[&vec] {
				nextgen.insert(x);
			}
		}
		let curc = nextgen.iter().sum::<i64>() - state.iter().sum::<i64>();
		if changes.len() == 10 && changes.iter().all(|&c| c == curc) {
			println!("{}", curc);
			part2 = state.iter().sum::<i64>() + (50000000000 - i) * curc;
			break;
		}
		changes.push_back(curc);
		if changes.len() > 10 {
			changes.pop_front();
		}
		state = nextgen;
		i += 1;
	}

	println!("Day 12 part 2: {}", part2);
}