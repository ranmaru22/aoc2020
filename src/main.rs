use advent2020::day01;
use advent2020::day02;
use advent2020::day03;
use advent2020::day04;
use advent2020::day05;

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
        _ => "Invalid problem".to_string()
    };

    println!("{}", result);
}
