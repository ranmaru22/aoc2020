use advent2020::day01;
use advent2020::day02;

fn main() {
    if let Ok(ans01) = day01::find() {
        println!("Day 1, Part 1: {}", ans01);
    }

    if let Ok(ans02) = day01::find2() {
        println!("Day 1, Part 2: {}", ans02);
    }

    if let Ok(ans01) = day02::find() {
        println!("Day 2, Part 1: {}", ans01);
    }

    if let Ok(ans02) = day02::find2() {
        println!("Day 2, Part 2: {}", ans02);
    }
}
