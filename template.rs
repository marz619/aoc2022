use std::io::Read;

fn part1() {
    let input = read_input();
    panic!("part1 not implemented!")
}

fn part2() {
    let input = read_input();
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
