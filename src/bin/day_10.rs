use anyhow::Result;
use std::str::FromStr;
//
// const INPUT_FILE: &str = "./input/test_10.txt";
const INPUT_FILE: &str = "./input/input_10.txt";

#[derive(Debug)]
struct Program {
    cycle: i32,
    x: i32,
    signal_strength: i32,
}

impl Default for Program {
    fn default() -> Self {
        Self {
            cycle: 0,
            x: 1,
            signal_strength: 0,
        }
    }
}

impl Program {
    fn calc_signal_strength(&mut self) {
        let armonized_cycle = self.cycle - 20;
        if armonized_cycle == 0 || armonized_cycle % 40 == 0 {
            let signal_strength = self.cycle * self.x;
            self.signal_strength += signal_strength;
            println!(
                "Increment signal strength, current: {} = {} * {}, sum: {}",
                signal_strength, self.cycle, self.x, self.signal_strength
            );
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        if parts.len() < 1 {
            return Err(anyhow::anyhow!("bad instruction from str"));
        }

        match parts[0] {
            "addx" => return Ok(Instruction::Addx(parts[1].parse::<i32>()?)),
            "noop" => return Ok(Instruction::Noop),
            _ => return Err(anyhow::anyhow!("unknown instruction")),
        }
    }
}

fn main() -> Result<()> {
    let res = std::fs::read_to_string(INPUT_FILE)?
        .lines()
        .filter_map(|x| x.parse::<Instruction>().ok())
        .fold(Program::default(), |mut acc, instruction| {
            acc.cycle += 1;
            acc.calc_signal_strength();
            match instruction {
                Instruction::Addx(x) => {
                    acc.cycle += 1;
                    acc.calc_signal_strength();
                    // println!("Current: {}, adding {}, new total: {}", acc.x, x, acc.x + x);
                    acc.x += x;
                }
                Instruction::Noop => (),
            };

            return acc;
        });

    println!("Signal strength sum: {}", res.signal_strength);

    return Ok(());
}
