use std::{collections::{HashMap, HashSet}, fs};

fn main() {
	let file = fs::read_to_string("input/day8.txt").expect("Should have read file");
	let nums: Vec<u32> = file.split(" ").map(|x| x.parse().unwrap()).collect();

	let (_, part1, part2) = read(&nums, 0);

	println!("Day 8 part 1: {}", part1);
	println!("Day 8 part 2: {}", part2);
	
}

fn read(nums: &Vec<u32>, i: usize) -> (usize, u32, u32) {
	let mut quantity = nums[i];
	let meta_entries = nums[i+1];

	let mut entry_count = 0;

	let mut part2 = 0;

	let mut i = i+2;

	let mut children = Vec::new();

	while quantity > 0 {
		quantity -= 1;
		let (ni, entries, p2) = read(nums, i);
		i = ni;
		entry_count += entries;
		children.push(p2);
	}

	for j in 0..meta_entries {
		let num = nums[i+j as usize];
		entry_count += num;

		if children.is_empty() {
			part2 += num;
		} else {
			if children.len() >= num as usize {
				part2 += children[num as usize - 1];
			}
		}
	}

	i += meta_entries as usize;

	(i, entry_count, part2)
}