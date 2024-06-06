use std::{collections::{HashMap, HashSet}, fs};

fn main() {
	let file = fs::read_to_string("input/day6.txt").expect("Should have read file");

	let lines = file.lines().map(|x| {
		let split = x.split(", ").collect::<Vec<_>>();
		(split[0].parse::<i32>().unwrap(), split[1].parse::<i32>().unwrap())
	}).collect::<Vec<_>>();

	let min_x = lines.iter().map(|x| x.0).min().unwrap();
	let max_x = lines.iter().map(|x| x.0).max().unwrap();

	let min_y = lines.iter().map(|x| x.1).min().unwrap();
	let max_y = lines.iter().map(|x| x.1).max().unwrap();

	let mut map = HashMap::new();

	for y in min_y..=max_y {
		for x in min_x..=max_x {
			let mins = lines.iter().enumerate().map(|x0| (x0.0, man(*x0.1, (x,y)))).collect::<Vec<_>>();
			let min = mins.iter().min_by_key(|(_, x)| x).unwrap();
			let cs = mins.iter().map(|(_, x)| x).filter(|&&x| x == min.1).count();
			if cs == 1 {
				*map.entry(min.0).or_insert(0) += 1;
			}
		}
	}

	let mut infinites = HashSet::new();

	for y in min_y..=max_y {
		for x in [min_x-100, max_x+100] {
			let min = lines.iter().enumerate().min_by_key(|x0| man(*x0.1, (x,y))).unwrap().0;
			infinites.insert(min);
		}
	}

	
	for y in [min_y-100, max_y+100] {
		for x in min_x..=max_x {
			let min = lines.iter().enumerate().min_by_key(|x0| man(*x0.1, (x,y))).unwrap().0;
			infinites.insert(min);
		}
	}

	let part1 = map.iter().filter(|&(i, _)| !infinites.contains(i)).map(|(_, &v)| v).max().unwrap();

	println!("Day 6 part 1: {}", part1);

	let mut part2 = 0;

	for y in min_y-10000..=max_y+10000 {
		for x in min_x-10000..=max_x+10000 {
			let lengths = lines.iter().map(|x0| man(*x0, (x,y))).sum::<i32>();
			if lengths < 10000 {
				part2 += 1;
			}
		}
	}

	println!("Day 6 part 2: {}", part2);
}

fn man(x: (i32, i32), y: (i32, i32)) -> i32 {
	(x.0 - y.0).abs() + (x.1 - y.1).abs()
}