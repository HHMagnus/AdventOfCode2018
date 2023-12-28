use std::fs;
use std::collections::HashSet;

pub fn day1() {
	let file = fs::read_to_string("input/day1.txt").expect("Should have read file");
    let day1:i32 = file.lines().map(|f| f.parse::<i32>().expect("Not an intger")).sum();
	println!("Day 1 part 1: {}", day1);
	
	let cycle = file.lines().map(|f| f.parse::<i32>().expect("Not an int")).collect::<Vec<i32>>();
	let day2:i32 = day1part2(cycle);
	println!("Day 2 part 2: {}", day2);
}

fn day1part2(lines: Vec<i32>) -> i32 {
	let mut seen = HashSet::new();
	let mut frequency = 0;
	loop {
		for line in lines.clone().into_iter() {
			frequency += line;
			if seen.contains(&frequency) {
				return frequency;
			}
			seen.insert(frequency);
		}
	}
}
