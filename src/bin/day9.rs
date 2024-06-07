use std::fs;

fn main() {
	let file = fs::read_to_string("input/day9.txt").expect("Should have read file");
    
    let split = file.split(" players; last marble is worth ").collect::<Vec<_>>();
    let players = split[0].parse::<usize>().unwrap();
    let last_marble = split[1].replace(" points\n", "").parse::<usize>().unwrap();

    let part1 = day9(players, last_marble);
    println!("Day 9 part 1: {}", part1);
    let part2 = day9(players, last_marble*100);
    println!("Day 9 part 2: {}", part2);
}

fn day9(players: usize, last_marble: usize) -> usize {
    let mut scores = (0..players).map(|_| 0).collect::<Vec<_>>();
    let mut circle = Vec::new();
    circle.push((0, 0, 0));

    let mut curr = 0;

    let mut next_marble = 1;

    let mut player = 1;

    while next_marble <= last_marble {
        if next_marble % 23 == 0 {
            
            scores[player] += next_marble;
            
            let mut rcurr = curr;
            for _ in 0..7 {
                rcurr = circle[rcurr].1;
            }
            curr = circle[rcurr].2;

            scores[player] += circle[rcurr].0;

            let iprev = circle[rcurr].1;
            let inext = circle[rcurr].2;
            
            let prev = circle[iprev];
            circle[iprev] = (prev.0, prev.1, inext);

            let next = circle[inext];
            circle[inext] = (next.0, iprev, next.2);
        } else {
            let iprev = circle[curr].2;
            let inext = circle[iprev].2;
    
            let inew = circle.len();
            circle.push((next_marble, iprev, inext));
            
            let prev = circle[iprev];
            circle[iprev] = (prev.0, prev.1, inew);
    
            let next = circle[inext];
            circle[inext] = (next.0, inew, next.2);

            curr = inew;
        }

        next_marble += 1;
        player += 1;
        player %= players;
    }

    *scores.iter().max().unwrap()
}