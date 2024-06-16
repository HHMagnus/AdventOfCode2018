#![recursion_limit = "256"]
use std::{collections::HashMap, fs};

use aoc_parse::{parser, prelude::*};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum DamageType {
	Bludgeoning,
	Slashing,
	Cold,
	Fire,
	Radiation,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Units {
	total: i32,
	hit_points: i32,
	weakness: Vec<DamageType>,
	immunities: Vec<DamageType>,
	damage: i32,
	damage_type: DamageType,
	initiative: i32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Map {
	immune_system: Vec<Units>,
	infection_group: Vec<Units>,
}

impl Units {
	fn new(tuple: (i32, i32, Vec<String>, i32, Vec<char>, i32)) -> Units {
		let total = tuple.0;
		let hit_points = tuple.1;
		let (weakness, immunities) = parse_weekness_immunities(tuple.2);
		let damage = tuple.3;
		let damage_type_string = tuple.4.into_iter().collect::<String>();
		let damage_type = parse_type(&damage_type_string);
		let initiative = tuple.5;
		Units {
			total,
			hit_points,
			weakness,
			immunities,
			damage,
			damage_type,
			initiative
		}
	}

	fn effective_power(&self) -> i32 {
		self.total * self.damage
	}

	fn damage_taken(&self, units: i32, dmg: i32, damage_type: DamageType) -> i32 {
		let total = units * dmg;
		if self.immunities.contains(&damage_type) {
			return 0;
		}
		if self.weakness.contains(&damage_type) {
			return total * 2;
		}
		total
	}

	fn is_dead(&self) -> bool {
		self.total <= 0
	}

	fn attacked(&mut self, other: &Units) {
		if other.is_dead() {
			return;
		}
		let dmg_taken = self.damage_taken(other.total, other.damage, other.damage_type);
		let killing = dmg_taken / self.hit_points;
		self.total -= killing;
	}
	
	fn boost(&mut self, boost: i32) {
		self.damage += boost;
	}
}

impl Map {
	fn new(immune_system: Vec<Units>, infection_group: Vec<Units>) -> Map {
		Map {
			immune_system,
			infection_group
		}
	}

	fn play(&mut self) {
		let mut units = self.total_units();
		while !self.done() {
			self.round();
			// Break any infinite loops
			let nunits = self.total_units();
			if units == nunits {
				return;
			}
			units = nunits;
		}
	}

	fn round(&mut self) {
		// Targeting
		let mut immune_targets = HashMap::new();
		let mut infection_targets = HashMap::new();

		loop {
			let next_immune = self.immune_system.iter().enumerate().filter(|(i, _)| !immune_targets.contains_key(i)).max_by_key(|x| (x.1.effective_power(), x.1.initiative));
			let next_infection = self.infection_group.iter().enumerate().filter(|(i, _)| !infection_targets.contains_key(i)).max_by_key(|x| (x.1.effective_power(), x.1.initiative));

			if next_immune.is_none() && next_infection.is_none() {
				break;
			}

			if next_immune.is_some() && (next_infection.is_none() || next_infection.unwrap().1.effective_power() < next_immune.unwrap().1.effective_power()
				|| (next_infection.unwrap().1.effective_power() == next_immune.unwrap().1.effective_power() && next_infection.unwrap().1.initiative < next_immune.unwrap().1.initiative)) {
				let next = next_immune.unwrap();
				let mut infections = self.infection_group
					.iter()
					.enumerate()
					.filter(|(i, _)| !immune_targets.values().any(|v| v == i))
					.map(|(i, x)| (x.damage_taken(next.1.total, next.1.damage, next.1.damage_type), x.effective_power(), x.initiative, i))
					.collect::<Vec<_>>();
				infections.sort();
				infections.reverse();

				if !infections.is_empty() && infections[0].0 != 0 {
					immune_targets.insert(next.0, infections[0].3);
				} else {
					immune_targets.insert(next.0, usize::MAX);
				}
			} else {
				let next = next_infection.unwrap();
				let mut immunities = self.immune_system
					.iter()
					.enumerate()
					.filter(|(i, _)| !infection_targets.values().any(|v| v == i))
					.map(|(i, x)| (x.damage_taken(next.1.total, next.1.damage, next.1.damage_type), x.effective_power(), x.initiative, i))
					.collect::<Vec<_>>();
				immunities.sort();
				immunities.reverse();

				if !immunities.is_empty() && immunities[0].0 != 0  {
					infection_targets.insert(next.0, immunities[0].3);
				} else {
					infection_targets.insert(next.0, usize::MAX);
				}
			}
		}

		// Attacking
		let mut turns = self.infection_group.iter().enumerate().map(|(i, x)| (x.initiative, true, i)).chain(self.immune_system.iter().enumerate().map(|(i, x)| (x.initiative, false, i))).collect::<Vec<_>>();
		turns.sort();

		while let Some((_, t, i)) = turns.pop() {
			if t == true {
				// Infection
				let infection_group = &self.infection_group[i];
				if infection_targets[&i] == usize::MAX {
					continue;
				}
				let immunity_group = self.immune_system.get_mut(infection_targets[&i]).unwrap();
				immunity_group.attacked(infection_group);
			} else {
				// Immune
				let immune = &self.immune_system[i];
				if immune_targets[&i] == usize::MAX {
					continue;
				}
				let infection_group = self.infection_group.get_mut(immune_targets[&i]).unwrap();
				infection_group.attacked(immune);
			}
		}
		
		self.immune_system = self.immune_system.clone().into_iter().filter(|x| !x.is_dead()).collect();
		self.infection_group = self.infection_group.clone().into_iter().filter(|x| !x.is_dead()).collect();
	}

	fn done(&self) -> bool {
		self.immune_system.is_empty() || self.infection_group.is_empty()
	}

	fn total_units(&self) -> i32 {
		self.immune_system.iter().map(|x| x.total).sum::<i32>()
		+ self.infection_group.iter().map(|x| x.total).sum::<i32>()
	}

	fn boost(&mut self, boost: i32) {
		for x in &mut self.immune_system {
			x.boost(boost);
		}
	}

	fn immune_system_won(&self) -> bool{
		!self.immune_system.is_empty() && self.infection_group.is_empty()
	}
}

fn parse_type(str: &str) -> DamageType {
	match str.trim() {
		"bludgeoning" => DamageType::Bludgeoning,
		"slashing" => DamageType::Slashing,
		"cold" => DamageType::Cold,
		"fire" => DamageType::Fire,
		"radiation" => DamageType::Radiation,
		x => unreachable!("Unknown parse type: '{}'", x),
	}
}

fn parse_weekness_immunities(strs: Vec<String>) -> (Vec<DamageType>, Vec<DamageType>) {
	if strs.is_empty() {
		return (Vec::new(), Vec::new())
	}
	let st = strs[0].replace("(", "").replace(")", "");

	let split = st.split("; ").collect::<Vec<_>>();

	let mut immunities = vec![];
	let mut weakness = vec![];

	for x in split {
		let damage_types = x.replace("immune to ", "").replace("weak to ", "").split(", ").map(parse_type).collect();
		if x.starts_with("immune to ") {
			immunities = damage_types;
		} else if x.starts_with("weak to ") {
			weakness = damage_types;
		}
	}

	(weakness, immunities)
}

fn main() {
	let file = fs::read_to_string("input/day24.txt").expect("Should have read file");
	let parser = parser!(line(i32 " units each with " i32 " hit points " string("(" string("weak to " lower+ string(", " lower+)*)? string("immune to " lower+ string(", " lower+)*)? "; "? string("weak to " lower+ string(", " lower+)*)? string("immune to " lower+ string(", " lower+)*)? ") ")* "with an attack that does " i32 " " lower+ " damage at initiative " i32));

	let split = file.split("\n\n").collect::<Vec<_>>();

	let immune = split[0].lines().collect::<Vec<_>>()[1..].into_iter().map(|x| parser.parse(&x).unwrap()).map(Units::new).collect::<Vec<_>>();
	let infection = split[1].lines().collect::<Vec<_>>()[1..].into_iter().map(|x| parser.parse(&x).unwrap()).map(Units::new).collect::<Vec<_>>();

	let map = Map::new(immune, infection);
	
	let mut part1map = map.clone();
	part1map.play();
	let part1 = part1map.total_units();
	println!("Day 24 part 1: {}", part1);

	let mut i = 1;

	loop {
		let mut part2map = map.clone();
		part2map.boost(i);
		part2map.play();
		
		if part2map.immune_system_won() {
			let part2 = part2map.total_units();
			println!("Day 24 part 2: {}", part2);
			break;
		}
		i += 1;
	}
}