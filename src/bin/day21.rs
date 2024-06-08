use std::fs;

type Inst = (String, u32, u32, u32);
type State = [u32; 6];

fn main() {
	let file = fs::read_to_string("input/day21.txt").expect("Should have read file");

	let lines = file.lines().collect::<Vec<_>>();

	let instruction_pointer: usize = lines[0].replace("#ip ", "").parse::<usize>().unwrap();

	let instructions: Vec<(String, u32, u32, u32)> = lines[1..].into_iter().map(|x| {
		let split = x.split(" ").collect::<Vec<_>>();
		(split[0].to_string(), split[1].parse::<u32>().unwrap(), split[2].parse::<u32>().unwrap(), split[3].parse::<u32>().unwrap())
	}).collect::<Vec<_>>();

	day21(instruction_pointer, instructions.clone(), [0, 0, 0, 0, 0, 0]);
}

fn day21(instruction_pointer: usize, instructions: Vec<(String, u32, u32, u32)>, state: State) {
	let mut state = state;

	let mut set = Vec::new();

	loop {
		if state[instruction_pointer] == 28 {
			if set.len() == 0 {
				println!("Day 21 part 1: {}", state[5]);
			}
			if set.contains(&state[5]) {
				println!("Day 21 part 2: {}", set.last().unwrap());
				break;
			}
			set.push(state[5]);
		}

		let instruction = instructions[state[instruction_pointer] as usize].clone();

		state = match instruction.0.as_str() {
			"addr" => addr(state, instruction),
			"addi" => addi(state, instruction),
			"mulr" => mulr(state, instruction),
			"muli" => muli(state, instruction),
			"banr" => banr(state, instruction),
			"bani" => bani(state, instruction),
			"borr" => borr(state, instruction),
			"bori" => bori(state, instruction),
			"setr" => setr(state, instruction),
			"seti" => seti(state, instruction),
			"gtir" => gtir(state, instruction),
			"gtri" => gtri(state, instruction),
			"gtrr" => gtrr(state, instruction),
			"eqir" => eqir(state, instruction),
			"eqri" => eqri(state, instruction),
			"eqrr" => eqrr(state, instruction),
			_ => unreachable!("Unknown instruction: {}", instruction.0),
		};

		if state[instruction_pointer] + 1 >= instructions.len() as u32 {
			break;
		}
		
		state[instruction_pointer] += 1;
	}
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
