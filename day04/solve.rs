use std::env;
use std::io::Read;
use std::ops::RangeInclusive;

fn read_ranges() -> Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    read_input()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|r| {
                    let (s, e) = r.split_once('-').unwrap();
                    s.parse().unwrap()..=e.parse().unwrap()
                })
                .collect::<Vec<RangeInclusive<i32>>>()
                .chunks(2)
                .map(|rr| (rr[0].clone(), rr[1].clone()))
                .collect::<Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>>()
        })
        .flatten()
        .collect()
}

fn contains(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> bool {
    r2.contains(&r1.start()) && r2.contains(&r1.end())
}

fn ranges_contained(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> bool {
    contains(r1, r2) || contains(r2, r1)
}

fn part1() {
    let contained_sum: i32 = read_ranges()
        .iter()
        .filter(|(r1, r2)| ranges_contained(r1, r2))
        .count() as i32;
    println!("{}", contained_sum);
}

fn overlaps(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> bool {
    &r1.start() <= &r2.end() && &r2.start() <= &r1.end()
}

fn ranges_overlap(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> bool {
    overlaps(r1, r2) || overlaps(r2, r1)
}

fn part2() {
    let overlap_sum: i32 = read_ranges()
        .iter()
        .filter(|(r1, r2)| ranges_overlap(r1, r2))
        .count() as i32;
    println!("{}", overlap_sum);
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    input
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => match &args[1][..] {
            "1" => part1(),
            "2" => part2(),
            _ => panic!("invalid argument {}\n", args[1]),
        },
        _ => panic!(
            "invalid number of arguments {}; must provide 2\n",
            args.len()
        ),
    }
}
