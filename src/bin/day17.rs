use std::fs;

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

fn main() {
	let file = fs::read_to_string("input/day17.txt").expect("Should have read file");
	
	let ranges = file.lines().map(|x| {
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

	println!("{:?}", ranges);
}