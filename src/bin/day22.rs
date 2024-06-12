use std::fs;
use cached::proc_macro::cached;

enum RegionType {
	Rocky,
	Wet,
	Narrow,
}

fn main() {
	let file = fs::read_to_string("input/day22.txt").expect("Should have read file");
	
	let lines = file.lines().collect::<Vec<_>>();
	let depth = lines[0].replace("depth: ", "").parse::<u128>().unwrap();
	let ts = lines[1].replace("target: ", "");
	let targets = ts.split(",").collect::<Vec<_>>();
	let target = (targets[0].parse::<u128>().unwrap(), targets[1].parse::<u128>().unwrap());

	let mut part1 = 0;

	for y in 0..=target.1 {
		for x in 0..=target.0 {
			let erosion_level = geologic_index((x, y), depth, target);

			let typ = match erosion_level % 3 {
				0 => RegionType::Rocky,
				1 => RegionType::Wet,
				2 => RegionType::Narrow,
				_ => unreachable!("My math is off"),
			};

			let risk = match typ {
				RegionType::Rocky => 0,
				RegionType::Wet => 1,
				RegionType::Narrow => 2,
			};
			
			part1 += risk;

			match typ {
				RegionType::Narrow => print!("|"),
				RegionType::Rocky => print!("."),
				RegionType::Wet => print!("="),
			}
		}
		println!("");
	}

	println!("Day 22 part 1: {}", part1);
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