use std::fs;

pub fn find() -> Result<u64, Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("../assets/input02.txt")?;
    let data = data_str.split('\n');
    let mut result = 0;

    for entry in data {
        let s = entry.split(&[' ', '-', ':'][..]).collect::<Vec<&str>>();
        match s.as_slice() {
            [min, max, char, _, pwd] => {
                let cnt = pwd.matches(char).count();
                if cnt >= min.parse()? && cnt <= max.parse()? {
                    result += 1;
                }
            },
            _ => {}
        }
    }
    Ok(result)
}

pub fn find2() -> Result<u64, Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("../assets/input02.txt")?;
    let data = data_str.split('\n');
    let mut result = 0;

    for entry in data {
        let s = entry.split(&[' ', '-', ':'][..]).collect::<Vec<&str>>();
        match s.as_slice() {
            [pos1, pos2, char, _, pwd] => {
                let i: usize = pos1.parse()?;
                let j: usize = pos2.parse()?;
                if let (Some(a), Some(b)) = (pwd.get(i-1..i), pwd.get(j-1..j)) {
                    if &a == char && &b != char || &a != char && &b == char {
                        result += 1;
                    }
                }
            },
            _ => {}
        }
    }
    Ok(result)
}
