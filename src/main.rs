use std::{env, path::Path};

mod day01;

fn main() {
    let arg = env::args().nth(1).expect("Usage: ./aoc2024 <day>");

    let day_number = arg[..arg.len() - 1]
        .parse::<usize>()
        .map(|n| {
            if n < 10 {
                format!("0{}", n)
            } else {
                format!("{}", n)
            }
        })
        .expect("Invalid argument: The day must be formatted as 1a or 12b");

    let input = parse_lines(format!("./data/day{}.txt", day_number));

    match arg.as_str() {
        "1a" => println!("{}", day01::part1(input)),
        "1b" => println!("{}", day01::part2(input)),
        // "2a" => println!("{}", day02::part1(input)),
        // "2b" => println!("{}", day02::part2(input)),
        // "3a" => println!("{}", day03::part1(input)),
        // "3b" => println!("{}", day03::part2(input)),
        // "4a" => println!("{}", day04::part1(input)),
        // "4b" => println!("{}", day04::part2(input)),
        // "5a" => println!("{}", day05::part1(input)),
        // "5b" => println!("{}", day05::part2(input)),
        // "6a" => println!("{}", day06::part1(input)),
        // "6b" => println!("{}", day06::part2(input)),
        // "7a" => println!("{}", day07::part1(input)),
        // "7b" => println!("{}", day07::part2(input)),
        // "8a" => println!("{}", day08::part1(input)),
        // "8b" => println!("{}", day08::part2(input)),
        // "9a" => println!("{}", day09::part1(input)),
        // "9b" => println!("{}", day09::part2(input)),
        // "10a" => println!("{}", day10::part1(input)),
        // "10b" => println!("{}", day10::part2(input)),
        // "11a" => println!("{}", day11::part1(input)),
        // "11b" => println!("{}", day11::part2(input)),
        // "12a" => println!("{}", day12::part1(input)),
        // "12b" => println!("{}", day12::part2(input)),
        // "13a" => println!("{}", day13::part1(input)),
        // "13b" => println!("{}", day13::part2(input)),
        // "14a" => println!("{}", day14::part1(input)),
        // "14b" => println!("{}", day14::part2(input)),
        // "15a" => println!("{}", day15::part1(input)),
        // "15b" => println!("{}", day15::part2(input)),
        // "16a" => println!("{}", day16::part1(input)),
        // "16b" => println!("{}", day16::part2(input)),
        // "17a" => println!("{}", day17::part1(input)),
        // "17b" => println!("{}", day17::part2(input)),
        // "18a" => println!("{}", day18::part1(input)),
        // "18b" => println!("{}", day18::part2(input)),
        // "19a" => println!("{}", day19::part1(input)),
        // "19b" => println!("{}", day19::part2(input)),
        // "20a" => println!("{}", day20::part1(input)),
        // "20b" => println!("{}", day20::part2(input)),
        // "21a" => println!("{}", day21::part1(input)),
        // "21b" => println!("{}", day21::part2(input)),
        // "22a" => println!("{}", day22::part1(input)),
        // "22b" => println!("{}", day22::part2(input)),
        // "23a" => println!("{}", day23::part1(input)),
        // "23b" => println!("{}", day23::part2(input)),
        // "24a" => println!("{}", day24::part1(input)),
        // "24b" => println!("{}", day24::part2(input)),
        // "25a" => println!("{}", day25::part1(input)),
        // "25b" => println!("{}", day25::part2(input)),
        _ => panic!("Invalid day: {}", arg),
    }
}

fn parse_lines(path: impl AsRef<Path>) -> Vec<String> {
    std::fs::read_to_string(path.as_ref())
        .unwrap_or_else(|e| panic!("Error reading file {:?}: {}", path.as_ref(), e))
        .lines()
        .map(ToOwned::to_owned)
        .collect()
}
