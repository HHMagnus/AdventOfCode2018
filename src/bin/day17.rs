use std::{collections::HashSet, fs};

#[derive(Debug)]
struct Range {
	axis: Axis,
	primary: i32,
	range: (i32, i32),
}

#[derive(Debug)]
enum Axis {
	Y,
	X,
}

impl Range {
	fn blocks(&self, (x, y): (i32, i32)) -> bool {
		match self.axis {
			Axis::X => self.primary == x && self.range.0 <= y && self.range.1 >= y,
			Axis::Y => self.primary == y && self.range.0 <= x && self.range.1 >= x,
		}
	}

	fn ymax(&self) -> i32 {
		match self.axis {
			Axis::Y => self.primary,
			Axis::X => self.range.0.max(self.range.1)
		}
	}

	fn ymin(&self) -> i32 {
		match self.axis {
			Axis::Y => self.primary,
			Axis::X => self.range.0.min(self.range.1)
		}
	}

	fn xmax(&self) -> i32 {
		match self.axis {
			Axis::X => self.primary,
			Axis::Y => self.range.0.max(self.range.1)
		}
	}

	fn xmin(&self) -> i32 {
		match self.axis {
			Axis::X => self.primary,
			Axis::Y => self.range.0.min(self.range.1)
		}
	}
}

fn main() {
	let file = fs::read_to_string("input/day17.txt").expect("Should have read file");
	
	let mut ranges = file.lines().map(|x| {
		let split = x.split(", ").collect::<Vec<_>>();
		let first = split[0].split("=").collect::<Vec<_>>();
		let axis = match first[0] {
			"x" => Axis::X,
			"y" => Axis::Y,
			_ => unreachable!("Unknown axis"),
		};
		let f = first[1].parse::<i32>().unwrap();
		let sec = split[1].split("..").collect::<Vec<_>>();
		let s1 = sec[0].replace("x=", "").replace("y=", "").parse::<i32>().unwrap();
		let s2 = sec[1].parse::<i32>().unwrap();
		Range {
			axis,
			primary: f,
			range: (s1, s2)
		}
	}).collect::<Vec<_>>();
	
	let mut set = HashSet::new();
	let _ = fall((500, 0), &mut ranges, &mut set);

	let ymin = ranges.iter().map(|r| r.ymin()).min().unwrap();
	let ymax = ranges.iter().map(|r| r.ymax()).max().unwrap();
	/*let xmin = ranges.iter().map(|r| r.xmin()).min().unwrap();
	let xmax = ranges.iter().map(|r| r.xmax()).max().unwrap();
	
	for y in ymin..=ymax {
		for x in xmin..=xmax {
			if set.contains(&(x, y)) {
				print!("~");
			} else if ranges.iter().any(|r| r.blocks((x, y))) {
				print!("#");
			} else {
				print!(".");
			}
		}
		println!("");
	}*/

	let part1 = set.iter().filter(|&&(_,y)| y >= ymin && y <= ymax).count();

	println!("Day 17 part 1: {}", part1);
}

fn fall(origin: (i32, i32), ranges: &mut Vec<Range>, water: &mut HashSet<(i32, i32)>) -> bool {
	let mut curr = origin;

	loop {
		let below = (curr.0, curr.1+1);
		if ranges.iter().map(|r| r.ymax()).max().unwrap() < curr.1 {
			return false;
		}

		if !water.contains(&curr) && !ranges.iter().any(|x| x.blocks(below)) {
			water.insert(curr);
			curr = below;
			continue;
		}

		loop {
			let right_ends = right(curr, ranges, water);
			let left_ends = left(curr, ranges, water);
			if curr == origin {
				return right_ends && left_ends;
			} else if right_ends && left_ends {
				curr = (curr.0, curr.1 - 1);
			} else {
				return false;
			}
		}
	}
}

fn right(origin: (i32, i32), ranges: &mut Vec<Range>, water: &mut HashSet<(i32, i32)>) -> bool {
	side(origin, ranges, 1, water)
}


fn left(origin: (i32, i32), ranges: &mut Vec<Range>, water: &mut HashSet<(i32, i32)>) -> bool {
	side(origin, ranges, -1, water)
}

fn side(origin: (i32, i32), ranges: &mut Vec<Range>, change: i32, water: &mut HashSet<(i32, i32)>) -> bool {
	let mut curr = origin;
	loop {
		let next = (curr.0 + change, curr.1);
		water.insert(curr);
		if ranges.iter().any(|r| r.blocks(next)) {
			return true;
		}
		curr = next;

		let below = (curr.0, curr.1 + 1);
		if !ranges.iter().any(|r| r.blocks(below)) && !water.contains(&below) {
			let last = (curr.0 - change, curr.1 + 1);
			if water.contains(&last) {
				return false;
			}
			let ended = fall(curr, ranges, water);
			return ended;
		}
	}
}
// 62884 not right
// 869568 too high
// 1217073 too high
// 1307680 too high
// 1631466