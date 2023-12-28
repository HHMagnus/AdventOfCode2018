use std::fs;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

pub fn day2() {
	day2part1();
	day2part2();
}

fn day2part1() {
	let file = fs::read_to_string("input/day2.txt").expect("Should have read file");
    let mut two = 0;
	let mut three = 0;
	for line in file.lines() {
		let mut tw: bool = false;
		let mut tt = false;
		for c in ASCII_LOWER {
			let count = line.chars().filter(|f| f == &c).count();
			if count == 2 {
				tw = true;
			}
			if count == 3 {
				tt = true;
			}
		}
		if tw {
			two += 1;
		}
		if tt {
			three += 1;
		}
	}
	let day1 = two * three;
	println!("Day 2 part 1: {}", day1);
	
}

fn day2part2() {
	let file = fs::read_to_string("input/day2.txt").expect("Should have read file");

	let lines: Vec<_> = file.lines().collect();

	for i in 0..lines.len()-1 {
		for j in i+1..lines.len() {
			let line1 = lines[i];
			let line2 = lines[j];

			let mut diffs = 0;
			
			let mut it1 = line1.chars();
			let mut it2 = line2.chars();

			while let Some(c1) = it1.next() {
				let c2 = it2.next().unwrap();

				if c1 != c2 {
					diffs += 1;
				}
			}

			if diffs == 1 {
				print!("Day 2 part 2: ");
				let mut it1 = line1.chars();
				let mut it2 = line2.chars();

				while let Some(c1) = it1.next() {
					let c2 = it2.next().unwrap();

					if c1 == c2 {
						print!("{}", c1);
					}
				}
				println!("");
			}
		}
	}
}