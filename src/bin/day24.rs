#![recursion_limit = "256"]
use std::fs;

use aoc_parse::{parser, prelude::*};

#[derive(Debug)]
enum DamageType {
	Bludgeoning,
	Slashing,
	Cold,
	Fire,
	Radiation,
}

#[derive(Debug)]
struct Units {
	total: i32,
	hit_points: i32,
	weakness: Vec<DamageType>,
	immunities: Vec<DamageType>,
	damage: i32,
	damage_type: DamageType,
	initiative: i32,
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

	println!("{:?}", immune);

	println!("{:?}", split.len());
}