use std::collections::HashMap;

fn main() {
	let grid_serial_number = 0; // insert input

	let mut map = HashMap::new();

	for x in 1..=300 {
		for y in 1..=300 {
			map.insert((x, y), power(x, y, grid_serial_number));
		}
	}
	let mut max = -1;
	let mut coord = (-1, -1);
	for y in 1..298 {
		for x in 1..298 {
			let mut p = 0;
			for i in 0..3 {
				for j in 0..3 {
					p += map[&(x+i,y+j)];
				}
			}
			if p > max {
				max = p;
				coord = (x, y);
			}
		}
	}

	println!("Day 11 part 1: {},{}", coord.0, coord.1);

	let mut sums = HashMap::new();
	for i in 0..=300 {
		sums.insert((0, i), 0);
		sums.insert((i, 0), 0);
	}

	for x in 1..=300 {
		for y in 1..=300 {
			let sum = map[&(x,y)] + sums[&(x, y-1)] + sums[&(x-1, y)] - sums[&(x-1,y-1)];
			sums.insert((x,y), sum);
		}
	}
	
	let mut max = -1;
	let mut coord = (-1, -1, -1);
	for size in 1..300 {
		for y in 1..=300-size {
			for x in 1..=300-size {
				let ex = x+size;
				let ey = y+size;
				let p = sums[&(ex,ey)] + sums[&(x,y)] - sums[&(ex, y)] - sums[&(x, ey)];
				if p > max {
					max = p;
					coord = (x+1, y+1, size);
				}
			}
		}
	}

	println!("Day 11 part 2: {},{},{}", coord.0, coord.1, coord.2);
}

fn power(x: i32, y: i32, grid_serial_number: i32) -> i32 {
	let rack_id = x + 10;
	let power_level = rack_id * y;
	let power_level = power_level + grid_serial_number;
	let power_level = power_level * rack_id;
	let power_level = (power_level % 1000) / 100;
	let power_level = power_level - 5;
	power_level
}