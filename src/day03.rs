use std::fs;

fn read_file() -> Result<Vec<String>, Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("../assets/input03.txt")?;
    Ok(data_str
       .split('\n')
       .map(|s| s.to_string())
       .filter(|s| s.len() > 0)
       .collect())
}

pub fn find() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let data = read_file()?;
    let mut trees = 0;

    for (i, line) in data.iter().enumerate() {
        if let Some(c) = line.chars().nth((3 * i + 1) % line.len()) {
            if c == '#' {
                trees += 1;
            }
        }
    }

    Ok(trees.to_string())
}
