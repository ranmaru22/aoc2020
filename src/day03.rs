use std::fs;

fn read_file() -> Result<Vec<String>, Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("assets/input03.txt")?;
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
        if let Some(c) = line.chars().nth((3 * i) % line.len()) {
            if c == '#' {
                trees += 1;
            }
        }
    }

    Ok(trees.to_string())
}

pub fn find2() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let data = read_file()?;
    let mut trees: Vec<u64> = Vec::with_capacity(5);
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for (right, down) in slopes {
        let mut current_trees = 0;
        for (i, line) in data.iter().step_by(down).enumerate() {
            if let Some(c) = line.chars().nth((right * i) % line.len()) {
                if c == '#' {
                    current_trees += 1;
                }
            }
        }
        trees.push(current_trees);
    }

    Ok(trees.iter().fold(1, |acc, val| acc * val).to_string())
}
