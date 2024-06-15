use std::{collections::{BinaryHeap, HashMap}, fs};
use cached::proc_macro::cached;

#[derive(Debug, Clone, Copy)]
enum RegionType {
	Rocky,
	Wet,
	Narrow,
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Hash, Debug)]
enum Gear {
	Torch,
	ClimbingGear,
	Neither,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct State {
	cost: u128,
	pos: (u128, u128),
	equipment: Gear,
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		other.cost.cmp(&self.cost)
			.then_with(|| self.pos.cmp(&other.pos))
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		match other.cost.partial_cmp(&self.cost) {
			Some(core::cmp::Ordering::Equal) => {}
			ord => return ord,
		}
		match other.pos.partial_cmp(&self.pos) {
			Some(core::cmp::Ordering::Equal) => {}
			ord => return ord,
		}
		other.equipment.partial_cmp(&self.equipment)
	}
}

impl Gear {
	fn possible(dest: RegionType) -> Vec<Gear> {
		match dest {
			RegionType::Rocky => vec![Gear::Torch, Gear::ClimbingGear],
			RegionType::Wet => vec![Gear::ClimbingGear, Gear::Neither],
			RegionType::Narrow => vec![Gear::Torch, Gear::Neither],
		}
	}
}

impl State {
	fn move_to(&self, pos:(u128, u128), curr: RegionType, dest: RegionType) -> Vec<State> {
		let dest_gear = Gear::possible(dest);
		let curr_gear = Gear::possible(curr);

		let mut vec = Vec::new();
		for x in dest_gear {
			if curr_gear.contains(&x) {
				vec.push(self.moved(pos, x));
			}
		}
		vec
	}
	
	fn moved(&self, pos:(u128, u128), equipment: Gear) -> State {
		State {
			cost: self.cost + if self.equipment == equipment { 1 } else { 8 },
			pos,
			equipment
		}
	}
}

fn main() {
	let file = fs::read_to_string("input/day22.txt").expect("Should have read file");
	
	let lines = file.lines().collect::<Vec<_>>();
	let depth = lines[0].replace("depth: ", "").parse::<u128>().unwrap();
	let ts = lines[1].replace("target: ", "");
	let targets = ts.split(",").collect::<Vec<_>>();
	let target = (targets[0].parse::<u128>().unwrap(), targets[1].parse::<u128>().unwrap());

	let mut part1 = 0;

	let mut map = HashMap::new();

	let map_size = target.0.max(target.1)*2;

	for y in 0..=map_size {
		for x in 0..=map_size {
			let erosion_level = geologic_index((x, y), depth, target);

			let typ = match erosion_level % 3 {
				0 => RegionType::Rocky,
				1 => RegionType::Wet,
				2 => RegionType::Narrow,
				_ => unreachable!("My math is off"),
			};

			map.insert((x, y), typ);
			
			if y <= target.1 && x <= target.0 {
				let risk = match typ {
					RegionType::Rocky => 0,
					RegionType::Wet => 1,
					RegionType::Narrow => 2,
				};

				part1 += risk;
			}
		}
	}

	println!("Day 22 part 1: {}", part1);

	let part2 = part2(&map, target);

	println!("Day 22 part 2: {}", part2);
}

fn part2(map: &HashMap<(u128, u128), RegionType>, target: (u128, u128)) -> u128 {

	let mut visited = HashMap::new();
	let mut states = BinaryHeap::new();

	let start = State {
		cost: 0,
		pos: (0, 0),
		equipment: Gear::Torch
	};

	states.push(start);

	while let Some(next) = states.pop() {
		let vkey = (next.pos, next.equipment);
		if visited.contains_key(&vkey) && visited[&vkey] <= next.cost {
			continue;
		}
		visited.insert(vkey, next.cost);

		if next.pos == target {
			if next.equipment == Gear::Torch {
				return next.cost;
			}
			states.push(State {
				cost: next.cost + 7,
				equipment: Gear::Torch,
				pos: next.pos
			});
		}

		let neighbours = neighbours(next.pos, map);

		for neigh in neighbours {
			for x in next.move_to(neigh, map[&next.pos], map[&neigh]) {
				states.push(x);
			}
		}
	}

	unreachable!("No path found");
}

fn neighbours(pos: (u128, u128), map: &HashMap<(u128, u128), RegionType>) -> Vec<(u128, u128)> {
	let mut vec = Vec::new();

	if pos.0 > 0 {
		vec.push((pos.0 - 1, pos.1));
	}

	if pos.1 > 0 {
		vec.push((pos.0, pos.1 - 1));
	}

	let xplus = (pos.0 + 1, pos.1);
	if map.contains_key(&xplus) {
		vec.push(xplus);
	}

	let yplus = (pos.0, pos.1 + 1);
	if map.contains_key(&yplus) {
		vec.push(yplus);
	} 

	vec
}

#[cached]
fn geologic_index((x, y): (u128, u128), depth: u128, target: (u128, u128)) -> u128 {
	let res = if x == 0 {
		y * 48271
	} else if y == 0 {
		x * 16807
	} else if  (x,y) == target {
		0
	} else {
		geologic_index((x-1, y), depth, target) * geologic_index((x, y-1), depth, target)
	};

	(res + depth) % 20183
}