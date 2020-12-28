use std::fs;
use std::collections::HashMap;

fn read_file() -> Result<Vec<isize>, Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("assets/input10.txt")?;
    Ok(data_str
        .split('\n')
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<_>().unwrap())
        .collect())
}

pub fn find() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let mut data = read_file()?;
    data.sort();

    let mut diffs = (0, 1);
    let mut cur_joltage = 0;

    for adaptor in data.iter() {
        match adaptor {
            n if n - cur_joltage == 3 => diffs.1 += 1,
            n if n - cur_joltage == 1 => diffs.0 += 1,
            _ => (),
        }
        cur_joltage = *adaptor;
    }
    Ok((diffs.0 * diffs.1).to_string())
}

fn find_connections(data: &Vec<isize>) -> isize {
    let mut memo: HashMap<isize, isize> = HashMap::new();
    memo.insert(*data.last().unwrap(), 1);

    for adaptor in data.iter().rev().skip(1) {
        let mut connections = 0;
        if let Some(plus_one) = memo.get(&(adaptor + 1)) {
            connections += plus_one;
        }

        if let Some(plus_two) = memo.get(&(adaptor + 2)) {
            connections += plus_two;
        }

        if let Some(plus_three) = memo.get(&(adaptor + 3)) {
            connections += plus_three;
        }

        memo.insert(*adaptor, connections);
    }

    *memo.get(&(0 as isize)).unwrap()
}

pub fn find2() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let mut data = read_file()?;
    data.sort();
    data.insert(0, 0);
    data.push(data.last().unwrap() + 3);

    let arrangements = find_connections(&data);

    Ok(arrangements.to_string())
}
