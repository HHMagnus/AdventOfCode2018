use std::{collections::HashMap, fs};

#[derive(Debug)]
enum MobType {
    Goblin,
    Elf
}

#[derive(Debug)]
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

    fn round() {
        
    }
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
    
    let map = Map::new(maps);

}

fn bfs(curr: usize, map: &HashMap<(usize, usize), char>, enemies: &Vec<Mob>) -> (usize, usize) {
    (0,0)
}