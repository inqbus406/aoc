use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut machine = Machine::from_file("input/day17.txt")?;
    let outputs = machine.run();
    print!("{}", outputs[0]);
    for output in &outputs[1..] {
        print!(",{}", output);
    }

    // dbg!(&machine);

    Ok(())
}

#[derive(Debug)]
struct Machine {
    reg_a: i32,
    reg_b: i32,
    reg_c: i32,
    program: Vec<u8>,
}

impl Machine {
    fn from_file(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let lines = reader.lines();

        let mut reg_a = 0;
        let mut reg_b = 0;
        let mut reg_c = 0;
        let mut program = Vec::new();

        for line in lines {
            let Ok(line) = line else {
                continue;
            };
            if line.is_empty() {
                continue;
            }
            let tokens: Vec<&str> = line.split_whitespace().collect();
            match tokens[0] {
                "Register" => match tokens[1] {
                    "A:" => reg_a = tokens.last().unwrap().parse::<i32>().unwrap(),
                    "B:" => reg_b = tokens.last().unwrap().parse::<i32>().unwrap(),
                    "C:" => reg_c = tokens.last().unwrap().parse::<i32>().unwrap(),
                    _   => unreachable!(),
                }
                "Program:" => {
                    program = tokens[1].split(',').map(|s| s.parse::<u8>().unwrap()).collect();
                }
                _ => unreachable!(),
            }
        }

        Ok(Self { reg_a, reg_b, reg_c, program })
    }

    fn run(&mut self) -> Vec::<i32> {
        let mut outputs = Vec::new();
        let mut instruction_pointer = 0;

        while instruction_pointer < self.program.len() - 1 {
            // dbg!(instruction_pointer);
            // dbg!(&self.reg_a);
            let instruction = Instruction::from_val(self.program[instruction_pointer]).unwrap();
            let literal_operand = self.program[instruction_pointer + 1];
            let combo_operand = self.combo_operand(literal_operand);

            match instruction {
                Instruction::Adv => {
                    self.reg_a = self.reg_a / 2_i32.pow(combo_operand as u32);
                },
                Instruction::Bxl => {
                    self.reg_b = self.reg_b ^ literal_operand as i32;
                },
                Instruction::Bst => {
                    self.reg_b = combo_operand % 8;
                },
                Instruction::Jnz => {
                    if self.reg_a != 0 {
                        instruction_pointer = literal_operand as usize;
                        continue;
                    }
                },
                Instruction::Bxc => {
                    self.reg_b = self.reg_b ^ self.reg_c;
                },
                Instruction::Out => {
                    let result = combo_operand % 8;
                    outputs.push(result);
                },
                Instruction::Bdv => {
                    self.reg_b = self.reg_a / 2_i32.pow(combo_operand as u32);
                },
                Instruction::Cdv => {
                    self.reg_c = self.reg_a / 2_i32.pow(combo_operand as u32);
                },
            }

            instruction_pointer += 2;
        }

        outputs
    }

    fn combo_operand(&self, num: u8) -> i32 {
        match num {
            x @ 0..=3 => x as i32,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!(),
        }
    }
}

enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn from_val(val: u8) -> Option<Self> {
        match val {
            0 => Some(Instruction::Adv),
            1 => Some(Instruction::Bxl),
            2 => Some(Instruction::Bst),
            3 => Some(Instruction::Jnz),
            4 => Some(Instruction::Bxc),
            5 => Some(Instruction::Out),
            6 => Some(Instruction::Bdv),
            7 => Some(Instruction::Cdv),
            _ => None,
        }
    }
}
