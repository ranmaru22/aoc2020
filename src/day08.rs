use std::fs;

fn read_file() -> Result<Vec<String>, Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("../assets/input08.txt")?;
    Ok(data_str
       .split('\n')
       .map(|s| s.to_string())
       .filter(|s| s.len() > 0)
       .collect())
}

enum Instruction { NOP, ACC, JMP }

struct Asm {
    instruction: Instruction,
    value: i32,
}

impl Asm {
    pub fn from(inp: &str) -> Self {
        let inp: Vec<_> = inp.split_whitespace().collect();
        match inp[0] {
            "nop" => Self { instruction: Instruction::NOP, value: inp[1].parse().unwrap_or(0) },
            "acc" => Self { instruction: Instruction::ACC, value: inp[1].parse().unwrap_or(0) },
            "jmp" => Self { instruction: Instruction::JMP, value: inp[1].parse().unwrap_or(0) },
            _ => panic!("Invalid instruction")
        }
    }
}

pub fn find() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let data = read_file()?;

    let stack: Vec<Asm> = data.iter().map(|line| Asm::from(line)).collect();
    let mut acc: i32 = 0;
    let mut sp: usize = 0;
    let mut hist: Vec<usize> = Vec::new();

    while !hist.contains(&sp) {
        let next = &stack[sp];
        hist.push(sp);

        match next.instruction {
            Instruction::NOP => sp += 1,
            Instruction::ACC => {
                acc += next.value;
                sp += 1;
            },
            Instruction::JMP => sp = (sp as i32 + next.value) as usize,
        }
    }
    Ok(acc.to_string())
}

pub fn find2() -> Result<String, Box<dyn std::error::Error + 'static>> {
    unimplemented!();
}
