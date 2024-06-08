use std::{collections::HashMap, fs};

type Inst = (u32, u32, u32, u32);
type State = [u32; 4];


fn main() {
	let file = fs::read_to_string("input/day16.txt").expect("Should have read file");
	
	let inputsets = file.split("\n\n\n").collect::<Vec<_>>();

	let part1input = inputsets[0].split("\n\n").map(|x| {
		let x = x.lines().collect::<Vec<_>>();
		let x1 = x[0].replace("Before: [", "").replace("]", "").split(", ").map(|y| y.parse::<u32>().unwrap()).collect::<Vec<_>>();
		let x2 = x[1].split(" ").map(|y| y.parse::<u32>().unwrap()).collect::<Vec<_>>();
		let x3 = x[2].replace("After:  [", "").replace("]", "").split(", ").map(|y| y.parse::<u32>().unwrap()).collect::<Vec<_>>();
		([x1[0], x1[1], x1[2], x1[3]], (x2[0], x2[1], x2[2], x2[3]), [x3[0], x3[1], x3[2], x3[3]])
	}).collect::<Vec<_>>();

	let part1 = part1input.into_iter().map(|(before, op, after)| part1(before, after, op)).filter(|&x| x >= 3).count();

	println!("Day 16 part 1: {}", part1);
}

fn part1(before: State, after: State, inst: Inst) -> usize {
	[
		addr(before, inst) == after,
		addi(before, inst) == after,
		mulr(before, inst) == after,
		muli(before, inst) == after,
		banr(before, inst) == after,
		bani(before, inst) == after,
		borr(before, inst) == after,
		bori(before, inst) == after,
		setr(before, inst) == after,
		seti(before, inst) == after,
		gtir(before, inst) == after,
		gtri(before, inst) == after,
		gtrr(before, inst) == after,
		eqir(before, inst) == after,
		eqri(before, inst) == after,
		eqrr(before, inst) == after,
	].iter().filter(|&&x| x).count()
}

fn addr(state: State, (_, a, b, c): Inst) -> State {
	let mut state = state;
	state[c as usize] = state[a as usize] + state[b as usize];
	state
}

fn addi(state: State, (_, a, b, c): Inst) -> State {
	let mut state = state;
	state[c as usize] = state[a as usize] + b;
	state
}

fn mulr(state: State, (_, a, b, c): Inst) -> State {
	let mut state = state;
	state[c as usize] = state[a as usize] * state[b as usize];
	state
}

fn muli(state: State, (_, a, b, c): Inst) -> State {
	let mut state = state;
	state[c as usize] = state[a as usize] * b;
	state
}

fn banr(state: State, (_, a, b, c): Inst) -> State {
	let mut state = state;
	state[c as usize] = state[a as usize] & state[b as usize];
	state
}

fn bani(state: State, (_, a, b, c): Inst) -> State {
	let mut state = state;
	state[c as usize] = state[a as usize] & b;
	state
}

fn borr(state: State, (_, a, b, c): Inst) -> State {
	let mut state = state;
	state[c as usize] = state[a as usize] | state[b as usize];
	state
}

fn bori(state: State, (_, a, b, c): Inst) -> State {
	let mut state = state;
	state[c as usize] = state[a as usize] | b;
	state
}

fn setr(state: State, (_, a, _, c): Inst) -> State {
	let mut state = state;
	state[c as usize] = state[a as usize];
	state
}

fn seti(state: State, (_, a, _, c): Inst) -> State {
	let mut state = state;
	state[c as usize] = a;
	state
}

fn gtir(state: State, (_, a, b, c): Inst) -> State {
	let mut state = state;
	state[c as usize] = if a > state[b as usize] { 1 } else { 0 };
	state
}

fn gtri(state: State, (_, a, b, c): Inst) -> State {
	let mut state = state;
	state[c as usize] = if state[a as usize] > b { 1 } else { 0 };
	state
}

fn gtrr(state: State, (_, a, b, c): Inst) -> State {
	let mut state = state;
	state[c as usize] = if state[a as usize] > state[b as usize] { 1 } else { 0 };
	state
}

fn eqir(state: State, (_, a, b, c): Inst) -> State {
	let mut state = state;
	state[c as usize] = if a == state[b as usize] { 1 } else { 0 };
	state
}

fn eqri(state: State, (_, a, b, c): Inst) -> State {
	let mut state = state;
	state[c as usize] = if state[a as usize] == b { 1 } else { 0 };
	state
}

fn eqrr(state: State, (_, a, b, c): Inst) -> State {
	let mut state = state;
	state[c as usize] = if state[a as usize] == state[b as usize] { 1 } else { 0 };
	state
}
