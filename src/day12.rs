use std::fs;

#[derive(Copy, Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    pub fn as_char(&self) -> char {
        use self::Direction::*;
        match self {
            N => 'N',
            E => 'E',
            S => 'S',
            W => 'W',
        }
    }
}

impl std::ops::Add<isize> for Direction {
    type Output = Self;

    fn add(self, rhs: isize) -> Self {
        use self::Direction::*;
        match self {
            N => match rhs {
                90 => E,
                180 => S,
                270 => W,
                _ => panic!("Invalid argument"),
            },
            E => match rhs {
                90 => S,
                180 => W,
                270 => N,
                _ => panic!("Invalid argument"),
            },
            S => match rhs {
                90 => W,
                180 => N,
                270 => E,
                _ => panic!("Invalid argument"),
            },
            W => match rhs {
                90 => N,
                180 => E,
                270 => S,
                _ => panic!("Invalid argument"),
            },
        }
    }
}

impl std::ops::Sub<isize> for Direction {
    type Output = Self;

    fn sub(self, rhs: isize) -> Self {
        use self::Direction::*;
        match self {
            N => match rhs {
                90 => W,
                180 => S,
                270 => E,
                _ => panic!("Invalid argument"),
            },
            E => match rhs {
                90 => N,
                180 => W,
                270 => S,
                _ => panic!("Invalid argument"),
            },
            S => match rhs {
                90 => E,
                180 => N,
                270 => W,
                _ => panic!("Invalid argument"),
            },
            W => match rhs {
                90 => S,
                180 => E,
                270 => N,
                _ => panic!("Invalid argument"),
            },
        }
    }
}

impl std::ops::SubAssign<isize> for Direction {
    fn sub_assign(&mut self, rhs: isize) {
        *self = *self - rhs;
    }
}

impl std::ops::AddAssign<isize> for Direction {
    fn add_assign(&mut self, rhs: isize) {
        *self = *self + rhs;
    }
}

struct Instruction {
    dir: char,
    steps: isize,
}

impl Instruction {
    pub fn from(s: &str) -> Self {
        let dir = s.as_bytes()[0] as char;
        let steps = s[1..].parse::<isize>().expect("Invalid number");

        Self { dir, steps }
    }
}

struct Ship {
    pos: (isize, isize),
    dir: Direction,
    stack: Vec<Instruction>
}

impl Ship {
    pub fn new(stack: Vec<Instruction>) -> Self {
        Self {
            pos: (0, 0),
            dir: Direction::E,
            stack
        }
    }

    pub fn get_instruction_with_abs_dir(&self, i: Instruction) -> Instruction {
        match i.dir {
            'F' => Instruction { dir: self.dir.as_char(), steps: i.steps },
            'B' => Instruction { dir: (self.dir + 180).as_char(), steps: i.steps },
            _ => i,
        }
    }

    pub fn move_ship(&mut self, i: Instruction) {
        match i.dir {
            'N' => self.pos.1 -= i.steps,
            'S' => self.pos.1 += i.steps,
            'E' => self.pos.0 -= i.steps,
            'W' => self.pos.0 += i.steps,
            'R' => self.dir += i.steps,
            'L' => self.dir -= i.steps,
            'F' | 'B' => self.move_ship(self.get_instruction_with_abs_dir(i)),
            x => panic!("Invalid instruction: {}", x),
        }
    }

    pub fn process_stack(&mut self) {
        while let Some(next) = self.stack.pop() {
            self.move_ship(next);
        }
    }

    pub fn manhattan(&self) -> isize {
        self.pos.0.abs() + self.pos.1.abs()
    }
}

fn read_file() -> Result<Vec<Instruction>, Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("assets/input12.txt")?;
    Ok(data_str
       .split('\n')
       .filter(|s| s.len() > 0)
       .map(|s| Instruction::from(s))
       .rev()
       .collect())
}

pub fn find() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let data = read_file()?;
    let mut ship = Ship::new(data);

    ship.process_stack();

    Ok(ship.manhattan().to_string())
}
