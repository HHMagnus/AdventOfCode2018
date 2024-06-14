use std::{collections::{HashMap, HashSet, VecDeque}, fs};

#[derive(Debug, PartialEq, Clone, Copy)]
enum MobType {
    Goblin,
    Elf
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Mob {
    typ: MobType,
    position: (usize, usize),
    hit_points: i32,
    attack_power: i32,
}

impl Mob {
    pub fn new(c: char, position: (usize, usize)) -> Option<Mob> {
        let typ = match c {
            'G' => MobType::Goblin,
            'E' => MobType::Elf,
            _ => return None,
        };
        Some(Mob {
            typ,
            position,
            hit_points: 200,
            attack_power: 3
        })
    }

    pub fn dmg(&mut self, hit: i32) {
        self.hit_points -= hit;
    }

    pub fn is_dead(&self) -> bool {
        self.hit_points < 1
    }

	pub fn move_to(&mut self, position: (usize, usize)) {
		self.position = position;
	}
}

#[derive(Debug)]
struct Map {
    map: HashMap<(usize, usize), char>,
    mobs: Vec<Mob>,
}

impl Map {
    fn new(map: HashMap<(usize, usize), char>) -> Map {
        let mobs = map
            .iter()
            .filter_map(|(&(x, y), &c)| Mob::new(c, (x, y)))
            .collect::<Vec<_>>();
        Map {
            map,
            mobs,
        }
    }

    fn round(&mut self) -> bool {
		let mut mobs = self.mobs
			.clone()
			.into_iter()
			.collect::<Vec<_>>();
		mobs.sort_by_key(|x| (x.position.1, x.position.0));
		let mut mobs = mobs.into_iter().collect::<VecDeque<_>>();

		let mut new_mobs: Vec<Mob> = Vec::new();

		while let Some(mut mob) = mobs.pop_front() {
            if mob.is_dead() {
                continue;
            }

            let targets = mobs
				.iter()
				.chain(new_mobs.iter())
				.filter(|x| !x.is_dead() && x.typ != mob.typ)
				.filter(|x| x.position != mob.position)
				.collect::<Vec<_>>();

			if targets.is_empty() {
				new_mobs.push(mob);
				for mob in mobs {
					new_mobs.push(mob);
				}
				self.mobs = new_mobs.into_iter().filter(|x| !x.is_dead()).collect();
				return false;
			}

			let active_mobs = mobs.iter().chain(new_mobs.iter()).filter(|x| !x.is_dead()).collect::<Vec<_>>();

			let mut positions = targets.iter()
				.flat_map(|x| neighbours(x.position))
				.filter(|&x| !active_mobs.iter().any(|mob| mob.position == x) && self.in_map(x))
				.filter_map(|x| self.dist_to(&active_mobs, mob.position, x).map(|y| (x, y)))
				.collect::<Vec<_>>();
			positions.sort_by_key(|x| (x.1, x.0.1, x.0.0));

			if !positions.is_empty() && positions[0].1 > 1 {
				let mut new_spots = neighbours(mob.position)
					.into_iter()
					.filter(|&x| self.in_map(x) && !active_mobs.iter().any(|mob| mob.position == x))
					.filter_map(|x| self.dist_to(&active_mobs, x, positions[0].0).map(|y| (x, y)))
					.collect::<Vec<_>>();
				new_spots.sort_by_key(|x| (x.1, x.0.1, x.0.0));

				mob.move_to(new_spots[0].0);
			}

			let mut target = neighbours(mob.position)
				.into_iter()
				.filter_map(|neigh| mobs.iter().chain(new_mobs.iter()).find(|x| x.typ != mob.typ && x.position == neigh))
				.collect::<Vec<_>>();
			target.sort_by_key(|x| (x.hit_points, x.position.1, x.position.0));
			let target = target.first().map(|x| x.position);

			if let Some(target) = target {
				if let Some(target) = (&mut mobs).into_iter().chain((&mut new_mobs).into_iter()).find(|x| x.position == target) {
					target.dmg(mob.attack_power);
				}
			}

			new_mobs.push(mob);
        }

		self.mobs = new_mobs.into_iter().filter(|x| !x.is_dead()).collect();

		true
    }

	fn in_map(&self, position: (usize, usize)) -> bool {
		self.map.get(&position).unwrap() != &'#'
	}

	fn dist_to(&self, active_mobs: &Vec<&Mob>, start: (usize, usize), dest: (usize, usize)) -> Option<usize> {
		let mut queue = VecDeque::new();

		let mut history = HashSet::new();
		history.insert(start);

		queue.push_back((1, start));

		while let Some((l, pos)) = queue.pop_front() {
			if pos == dest {
				return Some(l);
			}

			for neigh in neighbours(pos) {
				if history.contains(&neigh) { continue }
				if self.in_map(neigh) && !active_mobs.iter().any(|mob| mob.position == pos) {
					queue.push_back((l + 1, neigh));
					history.insert(neigh);
				}
			}
		}

		None
	}

	fn print(&self) {
		for y in 0..=self.map.keys().map(|x| x.1).max().unwrap() {
			for x in 0..=self.map.keys().map(|x| x.0).max().unwrap() {
				if let Some(mob) = self.mobs.iter().find(|mob| mob.position == (x, y)) {
					if mob.typ == MobType::Elf {
						print!("E")
					} else {
						print!("G");
					}
				} else {
					if self.in_map((x, y)) {
						print!(".");
					} else {
						print!("#");
					}
				}
			}
			println!("");
		}
	}
}

fn neighbours(position: (usize, usize)) -> Vec<(usize, usize)> {
	vec![
		(position.0    , position.1 - 1),
		(position.0 - 1, position.1    ),
		(position.0 + 1, position.1    ),
		(position.0    , position.1 + 1),
	]
}

fn main() {
	let file = fs::read_to_string("input/day15.txt").expect("Should have read file");

    let maps = file
        .lines()
        .enumerate()
        .flat_map(|(y, cs)| cs
            .chars()
            .enumerate()
            .map(|(x, c)| ((x,y), c))
            .collect::<Vec<_>>())
        .collect::<HashMap<_, _>>();
    
    let mut map = Map::new(maps);

	let mut rounds = 0;

	while map.round() {
		rounds += 1;
	}

	map.print();

	let hit_points_left = map.mobs.iter().map(|x| x.hit_points).sum::<i32>();

	println!("Day 15 part 1 {}", hit_points_left*rounds);

}