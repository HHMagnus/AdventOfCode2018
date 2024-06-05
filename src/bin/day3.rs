use std::{fs, collections::HashMap};

#[derive(Debug)]
struct Claim {
	id: String,
	position: (u32, u32),
	size: (u32, u32)
}

fn main() {
	let file = fs::read_to_string("input/day3.txt").expect("Should have read file");

	let claims: Vec<Claim> = file.lines().map(|x| {
		let s1: Vec<_> = x.split(" @ ").collect();
		let id = s1[0].replace("#", "");
		
		let s2: Vec<_> = s1[1].split(": ").collect();

		let ps: Vec<_> = s2[0].split(",").collect();
		let position = (ps[0].parse().unwrap(), ps[1].parse().unwrap());

		let wh: Vec<_> = s2[1].split("x").collect();
		let size = (wh[0].parse().unwrap(), wh[1].parse().unwrap());

		Claim {
			id: id,
			position: position,
			size: size
		}
	}).collect();

	let mut marked = HashMap::new();

	for claim in &claims {
		for x in claim.position.0..claim.position.0+claim.size.0 {
			for y in claim.position.1..claim.position.1+claim.size.1 {
				let entry = marked.entry((x, y)).or_insert(0);
				*entry += 1;
			}
		}
	}

	let part1 = marked.values().filter(|&&x| x > 1).count();

	println!("Day 3 part 1: {}", part1);

	let part2claim = claims.iter().find(|&claim| {
		for x in claim.position.0..claim.position.0+claim.size.0 {
			for y in claim.position.1..claim.position.1+claim.size.1 {
				if marked.get(&(x, y)).unwrap() != &1 {
					return false;
				}
			}
		}
		return true;
	}).unwrap();

	println!("Day 3 part 2: {}", part2claim.id);
}