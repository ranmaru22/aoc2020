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

fn sum_until_mod(start_value: usize, offset: usize, until: usize, step: usize) -> usize {
    let mut end_value = start_value;
    loop {
        if (end_value + offset) % until == 0 {
            break end_value;
        }
        end_value += step;
    }
}

fn check_schedule(schedule: &Vec<i32>) -> usize {
    let mut sum = schedule[0] as usize;
    let mut primes = vec![sum];
    for (i, &bus) in schedule.iter().enumerate().skip(1) {
        if bus != 0 {
            sum = sum_until_mod(sum, i, bus as usize, primes.iter().product());
            primes.push(bus as usize);
        }
    }
    sum
}

pub fn find2() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("assets/input13.txt")?;
    let mut split = data_str.splitn(2, '\n');

    split.next();
    let schedule = split.next()
                        .unwrap()
                        .split(',')
                        .map(|x| match x.trim().parse::<i32>() {
                            Ok(num) => num,
                            Err(_) => 0,
                        }).collect::<Vec<_>>();

    let ans = check_schedule(&schedule);

    Ok(ans.to_string())
}
