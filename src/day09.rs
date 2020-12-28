use itertools::Itertools;
use std::fs;

fn read_file() -> Result<Vec<usize>, Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("assets/input09.txt")?;
    Ok(data_str
        .split('\n')
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<_>().unwrap())
        .collect())
}

fn is_valid_sum(range: &[usize]) -> bool {
    if let Some(num) = range.last() {
        let sums = &range[0..range.len() - 1]
            .iter()
            .combinations(2)
            .map(|s| s[0] + s[1])
            .collect::<Vec<usize>>();
        sums.contains(&num)
    } else {
        false
    }
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

pub fn find2() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let data = read_file()?;
    let num: usize = find().unwrap().parse().unwrap();
    let filtered_data = data.iter().filter(|n| *n < &num).collect::<Vec<_>>();
    let mut result = 0;

    'outer: for i in 0..filtered_data.len() {
        for j in i..filtered_data.len() {
            let slice = &data[i..=j];
            let sum = slice.iter().fold(0, |acc, val| acc + val);
            if sum == num {
                result = slice.iter().min().unwrap() + slice.iter().max().unwrap();
                break 'outer;
            } else if sum > num {
                continue 'outer;
            }
        }
    }
    Ok(result.to_string())
}
