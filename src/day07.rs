use std::fs;
use std::collections::HashSet;
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
