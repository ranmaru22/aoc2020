use std::fs;

fn read_file() -> Result<Vec<String>, Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("../assets/input05.txt")?;
    Ok(data_str
       .split('\n')
       .map(|s| s.to_string())
       .filter(|s| s.len() > 0)
       .collect())
}

fn bin_split(range: Vec<u32>, ptable: &str) -> u32 {
    if range.len() == 1 {
       range[0]
    } else {
        let pivot = range[range.len() / 2];
        match ptable.chars().nth(0) {
            Some('F') | Some('L') => bin_split((range[0]..pivot).collect(), &ptable[1..]),
            Some('B') | Some('R') => bin_split((pivot..=*range.last().unwrap()).collect(), &ptable[1..]),
            _ => 0
        }
    }
}

pub fn find() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let seats = read_file()?;
    let mut highest_id = 0;

    for seat in seats.iter() {
        let (rtable, ctable) = (&seat[0..=6], &seat[7..]);
        let row = bin_split((0..=127).collect(), rtable);
        let col = bin_split((0..=7).collect(), ctable);
        let id = row * 8 + col;

        if id > highest_id {
            highest_id = id
        };
    }

    Ok(highest_id.to_string())
}
