use std::fs;

type Point = (i32, i32, i32, i32);

fn main() {
	let file = fs::read_to_string("input/day25.txt").expect("Should have read file");

	let points = file.lines().map(|x| {
		let x = x.split(",").map(|y| y.parse::<i32>().unwrap()).collect::<Vec<_>>();
		(x[0], x[1], x[2], x[3])
	}).collect::<Vec<_>>();

	let mut constellations = 0;
	let mut in_constellations = Vec::new();

	while in_constellations.len() != points.len() {
		let mut constellation = Vec::new();
		let point = *points.iter().find(|&x| !in_constellations.contains(x)).unwrap();
		constellation.push(point);
		in_constellations.push(point);
		constellations += 1;

		let mut i = 0;

		while i < constellation.len() {
			let next = constellation[i];

			let news = points.iter().filter(|&x| !in_constellations.contains(x) && man_dist(next, *x) <= 3).collect::<Vec<_>>();
			for &n in news {
				constellation.push(n);
				in_constellations.push(n);
			}
			i += 1;
		}
	}

	println!("Day 25: {}", constellations);
}

fn man_dist(p1: Point, p2: Point) -> i32 {
	(p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs() + (p1.3 - p2.3).abs()
}