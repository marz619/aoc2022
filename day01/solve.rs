use std::io::Read;

fn parse_list(list: String) -> Vec<Vec<i32>> {
    list.split("\n\n").map(|inventory| {
        inventory.lines()
            .map(|item| item.parse::<i32>().unwrap())
            .collect()
    }).collect()
}

fn sum(inventory: &Vec<i32>) -> i32 {
    inventory.iter().sum()
}

fn sums(inventories: Vec<Vec<i32>>) -> Vec<i32> {
    inventories.iter().map(sum).collect()
}

fn max(calories: Vec<i32>) -> i32 {
    *calories.iter().max().unwrap()
}

fn part1() {
    let input = read_input();
    print!("{}\n", max(sums(parse_list(input))));
}

fn part2() {
    let input = read_input();
    let mut total_per_elf = sums(parse_list(input));
    total_per_elf.sort_by(|a, b| b.cmp(a));
    print!("{}\n", sum(&total_per_elf[0..3].to_vec()));
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
