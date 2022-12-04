use std::io::Read;

const LOWER_ASCII_INDEX: i32 = 'a' as i32 - 1;
const UPPER_ASCII_INDEX: i32 = 'A' as i32 - 103;

fn priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - LOWER_ASCII_INDEX,
        'A'..='Z' => c as i32 + UPPER_ASCII_INDEX,
        _ => panic!("char out of range: {}\n", c),
    }
}

fn contains2(first: String, second: String) -> i32 {
    for c in first.chars() {
        if second.contains(c) {
            return priority(c);
        }
    }
    0
}

fn contains3(first: String, second: String, third: String) -> i32 {
    for c in first.chars() {
        if second.contains(c) && third.contains(c) {
            return priority(c);
        }
    }
    0
}

fn rucksack_halves() -> Vec<(String, String)> {
    read_input()
        .lines()
        .map(|line| {
            let rucksack = String::from(line);
            let len = rucksack.len();
            let (first, last) = rucksack.split_at(len / 2);
            (String::from(first), String::from(last))
        })
        .collect()
}

fn rucksack_triplets() -> Vec<(String, String, String)> {
    read_input()
        .lines()
        .map(|line| String::from(line))
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|ss| (ss[0].clone(), ss[1].clone(), ss[2].clone()))
        .collect()
}

fn part1() {
    let priority_sum: i32 = rucksack_halves()
        .iter()
        .map(|(f, s)| contains2(f.to_string(), s.to_string()))
        .sum();
    println!("{}", priority_sum);
}

fn part2() {
    let priority_sum: i32 = rucksack_triplets()
        .iter()
        .map(|(f, s, t)| contains3(f.to_string(), s.to_string(), t.to_string()))
        .sum();
    println!("{}", priority_sum);
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
