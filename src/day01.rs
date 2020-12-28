use std::fs;

pub fn find() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let data = fs::read_to_string("assets/input01.txt")?
        .split('\n')
        .map(|r| r.parse::<u64>().unwrap_or_default())
        .collect::<Vec<_>>();

    for i in 0..data.len() {
        for j in i..data.len() {
            if data[i] + data[j] == 2020 {
                return Ok((data[i] * data[j]).to_string());
            }
        }
    }
    Ok("None".to_string())
}

pub fn find2() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let data = fs::read_to_string("../assets/input01.txt")?
        .split('\n')
        .map(|r| r.parse::<u64>().unwrap_or_default())
        .collect::<Vec<_>>();

    for i in 0..data.len() {
        for j in i..data.len() {
            for k in j..data.len() {
                if data[i] + data[j] + data[k] == 2020 {
                    return Ok((data[i] * data[j] * data[k]).to_string());
                }
            }
        }
    }
    Ok("None".to_string())
}
