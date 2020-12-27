use std::fs;
use itertools::Itertools;

fn read_file() -> Result<Vec<usize>, Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("../assets/input09.txt")?;
    Ok(data_str
       .split('\n')
       .filter(|s| s.len() > 0)
       .map(|s| s.parse::<_>().unwrap())
       .collect())
}

fn is_valid_sum(range: &[usize]) -> bool {
    if let Some(num) = range.last() {
        let sums = &range[0..range.len() - 1].iter().combinations(2).map(|s| s[0] + s[1]).collect::<Vec<usize>>();
        sums.contains(&num)
    } else { false }
}

pub fn find() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let data = read_file()?;
    let mut result: usize = 0;

    for (i, num) in data.iter().enumerate().skip(25) {
        if !is_valid_sum(&data[i - 25..=i]) {
            result = *num;
            break;
        }
    }
    Ok(result.to_string())
}
