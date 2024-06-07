use std::{collections::{HashMap, HashSet}, fs};

fn main() {
	let file = fs::read_to_string("input/day8.txt").expect("Should have read file");
	let nums: Vec<u32> = file.split(" ").map(|x| x.parse().unwrap()).collect();

	let (_, part1) = read(&nums, 0);

	println!("Day 8 part 1: {}", part1);
	
}

fn read(nums: &Vec<u32>, i: usize) -> (usize, u32) {
	let mut quantity = nums[i];
	let meta_entries = nums[i+1];

	let mut entry_count = 0;

	let mut i = i+2;

	while quantity > 0 {
		quantity -= 1;
		let (ni, entries) = read(nums, i);
		i = ni;
		entry_count += entries;
	}

	for j in 0..meta_entries {
		entry_count += nums[i+j as usize];

	}

	i += meta_entries as usize;

	(i, entry_count)
}