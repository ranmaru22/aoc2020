use std::fs;

fn read_file() -> Result<(i32, Vec<i32>), Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("assets/input13.txt")?;
    let mut split = data_str.splitn(2, '\n');

    let earliest = split.next().unwrap().parse::<i32>().unwrap();
    let schedule = split.next()
                        .unwrap()
                        .split(',')
                        .filter(|&x| x != "x")
                        .map(|x| x.trim().parse::<i32>().unwrap())
                        .collect::<Vec<_>>();

    Ok((earliest, schedule))
}

pub fn find() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let (earliest, schedule) = read_file()?;

    let departures = schedule.iter().map(|&x| -(earliest % x) + x).collect::<Vec<_>>();
    let next_bus = departures.iter().min().unwrap();
    let index = departures.iter().position(|x| x == next_bus).unwrap();
    let ans = schedule[index] * next_bus;

    Ok(ans.to_string())
}
