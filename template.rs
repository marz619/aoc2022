use std::env;
use std::io::Read;

fn part1() {
    let input = read_input();
    println!("{:?}", input);
    panic!("part1 not implemented!")
}

fn part2() {
    let input = read_input();
    println!("{:?}", input);
    panic!("part2 not implemented!")
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
