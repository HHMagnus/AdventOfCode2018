use std::{collections::{HashMap, HashSet}, fs};

type Inst = (u32, u32, u32, u32);
type State = [u32; 4];


fn main() {
	let file = fs::read_to_string("input/day16.txt").expect("Should have read file");
	
	let inputsets = file.split("\n\n\n\n").collect::<Vec<_>>();

	let part1input = inputsets[0].split("\n\n").map(|x| {
		let x = x.lines().collect::<Vec<_>>();
		let x1 = x[0].replace("Before: [", "").replace("]", "").split(", ").map(|y| y.parse::<u32>().unwrap()).collect::<Vec<_>>();
		let x2 = x[1].split(" ").map(|y| y.parse::<u32>().unwrap()).collect::<Vec<_>>();
		let x3 = x[2].replace("After:  [", "").replace("]", "").split(", ").map(|y| y.parse::<u32>().unwrap()).collect::<Vec<_>>();
		([x1[0], x1[1], x1[2], x1[3]], (x2[0], x2[1], x2[2], x2[3]), [x3[0], x3[1], x3[2], x3[3]])
	}).collect::<Vec<_>>();

	let tests = part1input.into_iter().map(|(before, (op, a, b, c), after)| (op, part1(before, after, (op, a, b, c)))).collect::<Vec<_>>();

	let part1 = tests.iter().filter(|(_, x)| x.len() >= 3).count();

	println!("Day 16 part 1: {}", part1);

	let mut pos = HashMap::new();

	for (op, poss) in tests {
		for i in poss {
			pos.entry(op).or_insert(Vec::new()).push(i);
		}
	}

	let mut map = HashMap::new();

	while map.len() != 16 {
		let next = pos.iter().find(|x| x.1.iter().collect::<HashSet<_>>().len() == 1).unwrap();
		let curr = next.1[0];
		*map.entry(*next.0).or_default() = curr;
		
		pos = pos.into_iter().map(|(x, v)| (x, v.into_iter().filter(|&x| x != curr).collect::<Vec<_>>())).filter(|(_, v)| v.len() > 0).collect::<HashMap<_,_>>();
	}

	let part2input = inputsets[1].lines().map(|x| {
		let k = x.split(" ").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
		(k[0], k[1], k[2], k[3])
	}).collect::<Vec<_>>();

	let part2 = part2input.into_iter().fold([0, 0, 0, 0], |state, op| {
		let realop = map[&op.0];
		match realop {
			0 => addr(state, op),
			1 => addi(state, op),
			2 => mulr(state, op),
			3 => muli(state, op),
			4 => banr(state, op),
			5 => bani(state, op),
			6 => borr(state, op),
			7 => bori(state, op),
			8 => setr(state, op),
			9 => seti(state, op),
			10 => gtir(state, op),
			11 => gtri(state, op),
			12 => gtrr(state, op),
			13 => eqir(state, op),
			14 => eqri(state, op),
			15 => eqrr(state, op),
			_ => unreachable!("Too long op"),
		}
	})[0];

	println!("Day 16 part 2: {}", part2);
}


fn part1(before: State, after: State, inst: Inst) -> Vec<usize> {
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
	].into_iter().enumerate().flat_map(|(i, b)| if b { Some(i) } else { None }).collect::<Vec<_>>()
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
