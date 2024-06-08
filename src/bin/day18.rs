use std::{collections::HashMap, fs};

type Map = HashMap<(usize, usize), char>;

fn main() {
	let file = fs::read_to_string("input/day18.txt").expect("Should have read file");
	
	let map = file.lines().enumerate().flat_map(|(y, str)| str.chars().enumerate().map(|(x, c)| ((x, y), c)).collect::<Vec<_>>()).collect::<HashMap<_, _>>();

	let part1map = simulate(map.clone(), 10);

	let wood = part1map.iter().filter(|&((_, _), &c)| c == '|').count();
	let yard = part1map.iter().filter(|&((_, _), &c)| c == '#').count();
	let part1 = wood * yard;
	println!("Day 18 part 1: {}", part1);

	let part2map = simulate(map, 1000000000);

	let wood = part2map.iter().filter(|&((_, _), &c)| c == '|').count();
	let yard = part2map.iter().filter(|&((_, _), &c)| c == '#').count();
	let part2 = wood * yard;
	println!("Day 18 part 2: {}", part2);
}

fn simulate(map: Map, rounds: usize) -> Map {
	let mut map = map;
	let maxx = *map.iter().map(|((x, _), _)| x).max().unwrap() as i32;
	let maxy = *map.iter().map(|((_, y), _)| y).max().unwrap() as i32;

	let mut i = 0;

	let mut states = HashMap::new();

	let mut last1 = 0;
	let mut last2 = 0;

	while i < rounds {
		i += 1;
		map = map.clone().into_iter().map(|((x, y), c)| {
			let (x, y) = (x as i32, y as i32);
			let adjacent_tiles = [
				(x-1, y-1),
				(x-1, y),
				(x-1, y+1),
				(x, y-1),
				(x, y+1),
				(x+1, y-1),
				(x+1, y),
				(x+1, y+1),
			].iter().filter(|&&(x, y)| x >= 0 && y >= 0 && x <= maxx && y <= maxy)
				.map(|&(x, y)| map[&(x as usize, y as usize)])
				.collect::<Vec<_>>();
			let yards = adjacent_tiles.iter().filter(|&&x| x == '#').count();
			let trees = adjacent_tiles.iter().filter(|&&x| x == '|').count();
			let c = match c {
				'#' => if yards >= 1 && trees >= 1 { '#' } else { '.' },
				'|' => if yards >= 3 { '#' } else { '|' },
				'.' => if trees >= 3 { '|' } else { '.' },
				_ => unreachable!("Unknown char"),
			};
			((x as usize, y as usize), c)
		}).collect();

		if i < 1000 {
			continue;
		}

		let wood = map.iter().filter(|&((_, _), &c)| c == '|').count();
		let yard = map.iter().filter(|&((_, _), &c)| c == '#').count();
		
		let curr = wood * yard;

		let state_map = (last1, last2, curr);

		last1 = last2;
		last2 = curr;

		if states.contains_key(&state_map) {
			let last = states[&state_map];
			let diff = i - last;

			let remaining = rounds - i;
			let adding = remaining / diff;
			i += adding * diff;
		}

		states.insert(state_map, i);
	}

	map
}