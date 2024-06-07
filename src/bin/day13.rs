use std::{collections::{HashMap, HashSet}, fs};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Track {
	Horizontal,
	Vertical,
	Intersection,
	TurnRight,
	TurnLeft,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Cart {
	Up,
	Down,
	Left,
	Right,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Last {
	Left,
	Straight,
	Right,
}

fn main() {
	let file = fs::read_to_string("input/day13.txt").expect("Should have read file");

	let mut map = HashMap::new();

	let mut carts = Vec::new();

	let mut part1 = None;
	let part2: (usize, usize);

	for (y, a) in file.lines().enumerate() {
		for (x, c) in a.chars().enumerate() {
			if let Some(cart) = match c {
				'^' => Some(Cart::Up),
				'>' => Some(Cart::Right),
				'<' => Some(Cart::Left),
				'v' => Some(Cart::Down),
				_ => None,
			} {
				carts.push((cart, (x, y), Last::Left));
			}

			if let Some(track) = match c {
				'^' => Some(Track::Vertical),
				'v' => Some(Track::Vertical),
				'|' => Some(Track::Vertical),
				'>' => Some(Track::Horizontal),
				'<' => Some(Track::Horizontal),
				'-' => Some(Track::Horizontal),
				'/' => Some(Track::TurnRight),
				'\\' => Some(Track::TurnLeft),
				'+' => Some(Track::Intersection),
				_ => None,
			} {
				map.insert((x, y), track);
			}
		}
	}

	loop {
		let mut ncarts: Vec<(Cart, (usize, usize), Last)> = Vec::new();
		let mut oldcarts = carts;
		while let Some((cart, (x, y), last)) = oldcarts.pop() {
			let ne = match cart {
				Cart::Up => (x, y - 1),
				Cart::Down => (x, y + 1),
				Cart::Left => (x - 1, y),
				Cart::Right => (x + 1, y),
			};
			let track = map[&ne];

			let (cart, xy, last) = match (cart, track) {
				(Cart::Up, Track::Intersection) => match last {
						Last::Left => (Cart::Left, ne, Last::Straight),
						Last::Straight => (Cart::Up, ne, Last::Right),
						Last::Right => (Cart::Right, ne, Last::Left),
					},
				(Cart::Up, Track::TurnRight) => (Cart::Right, ne, last),
				(Cart::Up, Track::TurnLeft) => (Cart::Left, ne, last),
				(Cart::Down, Track::Intersection) => match last {
						Last::Left => (Cart::Right, ne, Last::Straight),
						Last::Straight => (Cart::Down, ne, Last::Right),
						Last::Right => (Cart::Left, ne, Last::Left),
					},
				(Cart::Down, Track::TurnRight) => (Cart::Left, ne, last),
				(Cart::Down, Track::TurnLeft) => (Cart::Right, ne, last),
				(Cart::Left, Track::Intersection) => match last {
						Last::Left => (Cart::Down, ne, Last::Straight),
						Last::Straight => (Cart::Left, ne, Last::Right),
						Last::Right => (Cart::Up, ne, Last::Left),
					},
				(Cart::Left, Track::TurnRight) => (Cart::Down, ne, last),
				(Cart::Left, Track::TurnLeft) => (Cart::Up, ne, last),
				(Cart::Right, Track::Intersection) => match last {
						Last::Left => (Cart::Up, ne, Last::Straight),
						Last::Straight => (Cart::Right, ne, Last::Right),
						Last::Right => (Cart::Down, ne, Last::Left),
					},
				(Cart::Right, Track::TurnRight) => (Cart::Up, ne, last),
				(Cart::Right, Track::TurnLeft) => (Cart::Down, ne, last),
				(Cart::Up, Track::Horizontal) => unreachable!(),
				(Cart::Up, Track::Vertical) => (Cart::Up, ne, last),
				(Cart::Down, Track::Horizontal) => unreachable!(),
				(Cart::Down, Track::Vertical) => (Cart::Down, ne, last),
				(Cart::Left, Track::Horizontal) => (Cart::Left, ne, last),
				(Cart::Left, Track::Vertical) => unreachable!(),
				(Cart::Right, Track::Horizontal) => (Cart::Right, ne, last),
				(Cart::Right, Track::Vertical) => unreachable!(),
			};

			if let Some(&other) = oldcarts.iter().find(|&&x| x.1 == xy) {
				if part1.is_none() {
					part1 = Some(xy);
				}
				oldcarts = oldcarts.into_iter().filter(|&x| x != other).collect();
			}

			else if let Some(&other) = ncarts.iter().find(|&&x| x.1 == xy) {
				if part1.is_none() {
					part1 = Some(xy);
				}
				ncarts = ncarts.into_iter().filter(|&x| x != other).collect();
			}
			else {
				ncarts.push((cart, xy, last));
			}
		}
		carts = ncarts;

		if carts.len() == 1 {
			part2 = carts.iter().next().unwrap().1;
			break;
		}
	}

	let part1 = part1.unwrap();
	println!("Day 13 part 1: {},{}", part1.0, part1.1);
	println!("Day 13 part 2: {},{}", part2.0, part2.1);
}