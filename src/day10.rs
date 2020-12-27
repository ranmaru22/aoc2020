use std::fs;

fn read_file() -> Result<Vec<isize>, Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("../assets/input10.txt")?;
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
            _ => ()
        }
        cur_joltage = *adaptor;
    }
    Ok((diffs.0 * diffs.1).to_string())
}
