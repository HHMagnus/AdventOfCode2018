#![recursion_limit = "256"]
use std::fs;

use aoc_parse::{parser, prelude::{i32, lines}, Parser};

fn main() {
	let file = fs::read_to_string("input/day10.txt").expect("Should have read file");

	
	let parser = parser!(lines("position=<" " "* i32 ", " " "* i32 "> velocity=<" " "* i32 ", " " "* i32  ">"));

	let parsed = parser.parse(file.as_str()).unwrap().iter().map(|x| (x.1, x.3, x.5, x.7)).collect::<Vec<_>>();

	let mut curr = parsed;

	let mut distx = i32::MAX;
	let mut disty = i32::MAX;

	let mut i = 0;
	loop {
		let ncurr = curr.iter().map(|&(x, y, vx, vy)| (x + vx, y + vy, vx, vy)).collect::<Vec<_>>();

		let ys = ncurr.iter().map(|(_, y, _, _)| *y).collect::<Vec<_>>();
		let xs = ncurr.iter().map(|(x, _, _, _)| *x).collect::<Vec<_>>();

		let minx = *xs.iter().min().unwrap();
		let maxx = *xs.iter().max().unwrap();
		let miny = *ys.iter().min().unwrap();
		let maxy = *ys.iter().max().unwrap();

		let dist1 = maxx - minx;
		let dist2 = maxy - miny;

		if distx < dist1 && disty < dist2 {
			println!("Day 10 part 1:");
			for y in miny..=maxy {
				for x in minx..=maxx {
					if curr.iter().any(|&(x0, y0, _, _)| x0 == x && y0 == y) {
						print!("#");
					} else {
						print!(".");
					}
				}
				println!("");
			}
			break;
		}
		distx = dist1;
		disty = dist2;

		curr = ncurr;
		i += 1;
	}

	println!("Day 10 part 2: {}", i);
}