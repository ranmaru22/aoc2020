use advent2020::day01;
use advent2020::day02;
use advent2020::day03;
use advent2020::day04;
use advent2020::day05;
use advent2020::day06;
use advent2020::day07;
use advent2020::day08;
use advent2020::day09;
use advent2020::day10;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let day = args.get(1)
        .map(|s| s.as_str())
        .unwrap_or("None");

    let result = match day {
        "day01a" => day01::find().ok().unwrap(),
        "day01b" => day01::find2().ok().unwrap(),
        "day02a" => day02::find().ok().unwrap(),
        "day02b" => day02::find2().ok().unwrap(),
        "day03a" => day03::find().ok().unwrap(),
        "day03b" => day03::find2().ok().unwrap(),
        "day04a" => day04::find().ok().unwrap(),
        "day04b" => day04::find2().ok().unwrap(),
        "day05a" => day05::find().ok().unwrap(),
        "day05b" => day05::find2().ok().unwrap(),
        "day06a" => day06::find().ok().unwrap(),
        "day06b" => day06::find2().ok().unwrap(),
        "day07a" => day07::find().ok().unwrap(),
        "day07b" => day07::find2().ok().unwrap(),
        "day08a" => day08::find().ok().unwrap(),
        // "day08b" => day08::find2().ok().unwrap(),
        "day09a" => day09::find().ok().unwrap(),
        "day09b" => day09::find2().ok().unwrap(),
        "day10a" => day10::find().ok().unwrap(),
        _ => "Invalid problem".to_string()
    };

    println!("{}", result);
}
