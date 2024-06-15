use std::{collections::{HashSet, VecDeque}, fs, thread};

const STACK_SIZE: usize = 4 * 1024 * 1024;

#[derive(Debug, Clone)]
enum Path {
	Element(char),
	Sub(Vec<Path>),
	Fork(Vec<Path>)
}

fn main() {
    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(day20)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}

fn day20() {
	let file = fs::read_to_string("input/day20.txt").expect("Should have read file");

	let (_, paths) = parse(file.as_str(), 1);

	let doors = walk(paths);

	let (part1, part2) = longest_shortest_path(&doors);

	print_doors(doors);

	println!("Day 20 part 1: {}", part1);
	println!("Day 20 part 2: {}", part2);
}

fn print_doors(doors: HashSet<(i32, i32)>) {
	for y in doors.iter().map(|x| x.1).min().unwrap()..=doors.iter().map(|x| x.1).max().unwrap() {
			for x in doors.iter().map(|x| x.0).min().unwrap()..=doors.iter().map(|x| x.0).max().unwrap() {
				if y == 0 && x == 0 {
					print!("X");
				} else if y % 2 == 0 && x % 2 == 0 {
					print!(".");
				} else {
					if doors.contains(&(x, y)) {
						if y % 2 == 0 {
							print!("|");
						} else {
							print!("-");
						}
					} else {
						print!("#");
					}
				}
			}
			println!("")
		}
}

fn longest_shortest_path(doors: &HashSet<(i32, i32)>) -> (usize, usize) {
	let mut queue = VecDeque::new();
	queue.push_back((0, (0, 0)));
	
	let mut visited = HashSet::new();

	let mut max_cost = 0;
	let mut above1000 = 0;

	while let Some((cost, next)) = queue.pop_front() {
		if max_cost < cost {
			max_cost = cost;
		}

		if cost >= 1000 {
			above1000+=1;
		}

		let steps = vec![
			((next.0 + 1, next.1), (next.0 + 2, next.1)),
			((next.0 - 1, next.1), (next.0 - 2, next.1)),
			((next.0, next.1 + 1), (next.0, next.1 + 2)),
			((next.0, next.1 - 1), (next.0, next.1 - 2)),
		];

		for (step, forward) in steps {
			if doors.contains(&step) && !visited.contains(&forward){
				queue.push_back((cost+1, forward));
				visited.insert(forward);
			}
		}
	}

	(max_cost, above1000)
}

fn parse(txt: &str, mut i: usize) -> (usize, Path) {
	let mut forks = Vec::new();
	let mut paths = Vec::new();
	loop {
		match txt.chars().nth(i).unwrap() {
			'$' => {
				if !forks.is_empty() {
					forks.push(paths);
					return (i, Path::Fork(forks.into_iter().map(|x| Path::Sub(x)).collect()));
				}
				return (i+1, Path::Sub(paths))
			},
			'(' => {
				let (ni, path) = parse(txt, i+1);
				paths.push(path);
				i = ni;
			},
			')' => {
				if !forks.is_empty() {
					forks.push(paths);
					return (i+1, Path::Fork(forks.into_iter().map(|x| Path::Sub(x)).collect()));
				}
				return (i+1, Path::Sub(paths));
			},
			'|' => {
				forks.push(paths);
				paths = Vec::new();
				i += 1;
			},
			'N' => {
				paths.push(Path::Element('N'));
				i += 1;
			},
			'S' => {
				paths.push(Path::Element('S'));
				i += 1;
			},
			'E' => {
				paths.push(Path::Element('E'));
				i += 1;
			},
			'W' => {
				paths.push(Path::Element('W'));
				i += 1;
			},
			x => unreachable!("Unknown {}", x),
		}
	}
}

fn walk(paths: Path) -> HashSet<(i32, i32)> {
	let mut doors = HashSet::new();
	let mut set = HashSet::new();
	set.insert((0, 0));
	sub_walk(paths, &mut doors, set);
	doors
}

fn sub_walk(path: Path, doors: &mut HashSet<(i32, i32)>, positions: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
	match path {
		Path::Element(c) => match c {
			'N' => {
				return positions.into_iter().map(|pos| {
					doors.insert((pos.0, pos.1 - 1));
					return (pos.0, pos.1 - 2);
				}).collect();
			},
			'S' => {
				return positions.into_iter().map(|pos| {
					doors.insert((pos.0, pos.1 + 1));
					return (pos.0, pos.1 + 2);
				}).collect();
			},
			'E' => {
				return positions.into_iter().map(|pos| {
					doors.insert((pos.0 + 1, pos.1));
					return (pos.0 + 2, pos.1);
				}).collect();
			},
			'W' => {
				return positions.into_iter().map(|pos| {
					doors.insert((pos.0 - 1, pos.1));
					return (pos.0 - 2, pos.1);
				}).collect();
			},
			x => unreachable!("Unknown {}", x),
		},
		Path::Sub(paths) => {
			return positions.into_iter().flat_map(|pos| paths.clone().into_iter().fold(vec![pos].into_iter().collect(), |acc, x| sub_walk(x, doors, acc))).collect();
		},
		Path::Fork(forks) => {
			return positions.into_iter().flat_map(|pos| {
				forks.clone().into_iter().flat_map(|fork| {
					let mut set = HashSet::new();
					set.insert(pos);
					sub_walk(fork, doors, set)
				}).collect::<Vec<_>>()
			}).collect();
		},
	}
}