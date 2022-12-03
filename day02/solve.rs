use std::io::Read;
use std::str::FromStr;

// Coding
//
// A -> Rock     <- X
// B -> Paper    <- Y
// C -> Scissors <- Z

// Scoring
//
// Rock     -> 1
// Paper    -> 2
// Scissors -> 3
//
// Win  -> 6
// Draw -> 3
// Loss -> 0

fn move_value(c: char) -> i32 {
    match c {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => panic!("invalid char {}\n", c),
    }
}

fn round_value(op_value: i32, my_value: i32) -> i32 {
    match op_value - my_value {
        -1 | 2 => 6,  // win
        0 => 3,       // draw
        1 | -2 => 0,  // loss
        _ => panic!("invalid move value(s) op={} - my={}\n", op_value, my_value),
    }
}

fn round_score(op_move: char, my_move: char) -> i32 {
    let op_value = move_value(op_move);
    let my_value = move_value(my_move);
    round_value(op_value, my_value) + my_value
}

fn part1() {
    let input = read_input();
    let score: i32 = input.lines()
        .map(|line| {
            let mut split = line.split(' ');
            let op = char::from_str(split.next().unwrap()).unwrap();
            let me = char::from_str(split.next().unwrap()).unwrap();
            round_score(op, me)
        })
        .sum();
    print!("{}\n", score);
}

fn part2() {
    let input = read_input();
    print!("{:?}\n", input);
    panic!("part2 not implemented!")
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    input
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        part1();
    } else if args[1] == "2" {
        part2();
    } else {
        panic!("invalid argument");
    }
}
