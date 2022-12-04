use std::io::Read;

fn move_value(c: char) -> i32 {
    match c {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => panic!("invalid char {}\n", c),
    }
}

fn round_value(op: i32, me: i32) -> i32 {
    match op - me {
        -1 | 2 => 6,  // win
        0 => 3,       // draw
        1 | -2 => 0,  // loss
        _ => panic!("invalid move value(s) op={} - my={}\n", op, me),
    }
}

fn round_score(op: i32, me: i32) -> i32 {
    round_value(op, me) + me
}

fn rounds() -> Vec<(i32, i32)> {
    read_input()
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            (
                move_value(chars.nth(0).unwrap()),
                move_value(chars.nth(1).unwrap())
            )
        })
        .collect()
}

fn shape(op: i32, scenario: i32) -> i32 {
    let opts: Vec<i32> = vec![1, 2, 3];
    match scenario {
        2 => op,  // draw
        1 => *opts.iter().filter(|&x| round_value(op, *x) == 0).last().unwrap(), // lose
        3 => *opts.iter().filter(|&x| round_value(op, *x) == 6).last().unwrap(), // win
        _ => panic!("invalid move me={}\n", scenario),
    }
}

fn part1() {
    let score: i32 = rounds()
        .iter()
        .map(|(op, me)| {
            round_score(*op, *me)
        })
        .sum();
    print!("{}\n", score);
}

fn part2() {
    let score: i32 = rounds()
        .iter()
        .map(|(op, me)| {
            round_score(*op, shape(*op, *me))
        })
        .sum();
    print!("{}\n", score);
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
