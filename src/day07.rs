use std::fs;
use regex::Regex;

fn read_file() -> Result<Vec<String>, Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("../assets/input07.txt")?;
    Ok(data_str
       .split('\n')
       .map(|s| s.to_string())
       .filter(|s| s.len() > 0)
       .collect())
}

fn collect_matches(bag: &str, rules: &Vec<String>) -> Vec<String> {
    rules.iter()
        .filter(|r| r.contains(bag))
        .map(|r| r.split_whitespace().take(2).collect::<Vec<_>>().join(" "))
        .filter(|b| b != bag)
        .collect()
}

fn collect_matches_with_count(bag: &str, rules: &Vec<String>) -> Vec<(usize, String)> {
    let mut matches: Vec<(usize, String)> = Vec::new();
    let re = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
    if let Some(contents) = rules.iter().find(|r| r.starts_with(bag)) {
        let query = contents.split("contain").last().unwrap();
        for cap in re.captures_iter(query) {
            matches.push((cap[1].parse().unwrap(), cap[2].to_owned()));
        }
    }
    matches
}

fn sum_up_counts(bag: &str, rules: &Vec<String>) -> usize {
    let contents = collect_matches_with_count(bag, rules);
    match contents.len() {
        0 => 1,
        _ => contents.iter()
            .map(|(n, next_bag)| match sum_up_counts(next_bag, rules) {
                1 => *n,
                ref x => n + n * x,
            })
            .fold(0, |acc, val| acc + val)
    }
}

pub fn find() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let data = read_file()?;

    let mut to_check = collect_matches("shiny gold", &data);
    let mut checked: Vec<String> = Vec::new();

    while let Some(bag) = to_check.pop() {
        if checked.contains(&bag) { continue; }
        let mut batch = collect_matches(&bag, &data);
        to_check.append(&mut batch);
        checked.push(bag);
    }

    Ok(checked.len().to_string())
}

pub fn find2() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let data = read_file()?;
    let num_bags = sum_up_counts("shiny gold", &data);

    Ok(num_bags.to_string())
}
