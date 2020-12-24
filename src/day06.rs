use std::fs;
use std::collections::HashSet;

fn read_file() -> Result<Vec<String>, Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("../assets/input06.txt")?;
    Ok(data_str
       .split("\n\n")
       .map(|s| s.to_string())
       .filter(|s| s.len() > 0)
       .collect())
}

pub fn find() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let data = read_file()?;
    let mut sum = 0;

    for answers in data {
        let mut uniques: HashSet<char> = HashSet::with_capacity(answers.len());
        for c in answers.chars().filter(|c| !c.is_whitespace()) {
            uniques.insert(c);
        }
        sum += uniques.len();
    }

    Ok(sum.to_string())
}
