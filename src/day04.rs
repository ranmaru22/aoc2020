use std::fs;

fn read_file() -> Result<Vec<String>, Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("../assets/input04.txt")?;
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
