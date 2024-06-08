use std::fs;


fn main() {
	let file = fs::read_to_string("input/day14.txt").expect("Should have read file");
	let final_count = file.parse::<usize>().unwrap();

	let mut vecs = Vec::new();
	vecs.push(3);
	vecs.push(7);

	let mut elf1 = 0;
	let mut elf2 = 1;

	let mut last = 0;

	let mut part2 = None;
	let compare = final_count.to_string().chars().map(|x| x.to_digit(10).unwrap() as usize).collect::<Vec<_>>();

	while part2.is_none() {
		let sum = vecs[elf1] + vecs[elf2];
		let digits = sum.to_string().chars().map(|x| x.to_digit(10).unwrap() as usize).collect::<Vec<_>>();
		for digit in digits {
			vecs.push(digit);
		}

		elf1 = (elf1 + (vecs[elf1] + 1) % vecs.len()) % vecs.len();
		elf2 = (elf2 + (vecs[elf2] + 1) % vecs.len()) % vecs.len();

		while vecs.len() - last > compare.len() {
			let mut equal = true;
			for j in 0..compare.len() {
				if compare[j] != vecs[last+j] {
					equal = false;
				}
			}

			if equal {
				part2 = Some(last);
			}

			last += 1;
		}
	}

	print!("Day 14 part 1: ");
	for i in 0..10 {
		print!("{}", vecs[final_count +  i]);
	}
	println!("");

	println!("Day 14 part 2: {}", part2.unwrap());
}