fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let day = args.get(1).map(|s| s.as_str()).unwrap_or("None");

    let result = match day {
        "day01a" => advent2020::day01::find().ok().unwrap(),
        "day01b" => advent2020::day01::find2().ok().unwrap(),
        "day02a" => advent2020::day02::find().ok().unwrap(),
        "day02b" => advent2020::day02::find2().ok().unwrap(),
        "day03a" => advent2020::day03::find().ok().unwrap(),
        "day03b" => advent2020::day03::find2().ok().unwrap(),
        "day04a" => advent2020::day04::find().ok().unwrap(),
        "day04b" => advent2020::day04::find2().ok().unwrap(),
        "day05a" => advent2020::day05::find().ok().unwrap(),
        "day05b" => advent2020::day05::find2().ok().unwrap(),
        "day06a" => advent2020::day06::find().ok().unwrap(),
        "day06b" => advent2020::day06::find2().ok().unwrap(),
        "day07a" => advent2020::day07::find().ok().unwrap(),
        "day07b" => advent2020::day07::find2().ok().unwrap(),
        "day08a" => advent2020::day08::find().ok().unwrap(),
        // "day08b" => advent2020::day08::find2().ok().unwrap(),
        "day09a" => advent2020::day09::find().ok().unwrap(),
        "day09b" => advent2020::day09::find2().ok().unwrap(),
        "day10a" => advent2020::day10::find().ok().unwrap(),
        "day10b" => advent2020::day10::find2().ok().unwrap(),
        "day11a" => advent2020::day11::find().ok().unwrap(),
        "day11b" => advent2020::day11::find2().ok().unwrap(),
        "day12a" => advent2020::day12::find().ok().unwrap(),
        "day12b" => advent2020::day12::find2().ok().unwrap(),
        _ => advent2020::day13::find().ok().unwrap(),
    };

    println!("{}", result);
}
