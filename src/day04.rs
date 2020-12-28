use regex::Regex;
use std::fs;

fn read_file() -> Result<Vec<String>, Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("assets/input04.txt")?;
    Ok(data_str
        .split("\n\n")
        .map(|s| s.to_string())
        .filter(|s| s.len() > 0)
        .collect())
}

pub fn find() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let data = read_file()?;
    let req_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut valids = 0;

    for passport in data.iter() {
        let fields = passport
            .split_whitespace()
            .map(|x| x.split(':').collect::<Vec<_>>().split_off(0)[0])
            .filter(|x| req_fields.contains(x))
            .collect::<Vec<_>>();

        if fields.len() >= 7 {
            valids += 1;
        }
    }

    Ok(valids.to_string())
}

pub fn find2() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let data = read_file()?;
    let eye_colours = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let mut valids = 0;

    for passport in data.iter() {
        let fields = passport
            .split_whitespace()
            .map(|x| x.split(':').collect::<Vec<_>>())
            .filter(|x| match x[0] {
                "byr" => {
                    x[1].parse::<u32>().unwrap() >= 1920 && x[1].parse::<u32>().unwrap() <= 2002
                }
                "iyr" => {
                    x[1].parse::<u32>().unwrap() >= 2010 && x[1].parse::<u32>().unwrap() <= 2020
                }
                "eyr" => {
                    x[1].parse::<u32>().unwrap() >= 2020 && x[1].parse::<u32>().unwrap() <= 2030
                }
                "hgt" => {
                    if let Some(cap) = Regex::new(r"^(\d+)(cm|in)$").unwrap().captures(x[1]) {
                        match &cap[2] {
                            "in" => {
                                cap[1].parse::<u32>().unwrap() >= 59
                                    && cap[1].parse::<u32>().unwrap() <= 76
                            }
                            "cm" => {
                                cap[1].parse::<u32>().unwrap() >= 150
                                    && cap[1].parse::<u32>().unwrap() <= 193
                            }
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
                "hcl" => Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(x[1]),
                "ecl" => eye_colours.contains(&x[1]),
                "pid" => Regex::new(r"^\d{9}$").unwrap().is_match(x[1]),
                _ => false,
            })
            .collect::<Vec<_>>();

        if fields.len() >= 7 {
            valids += 1;
        }
    }

    Ok(valids.to_string())
}
