use anyhow::Result;
use std::str::FromStr;
//
// const INPUT_FILE: &str = "./input/test_10.txt";
const INPUT_FILE: &str = "./input/input_10.txt";

#[derive(Debug)]
struct Program {
    cycle: i32,
    x: i32,
    drawing: String,
}

impl Default for Program {
    fn default() -> Self {
        Self {
            cycle: 0,
            x: 1,
            drawing: String::from("#"),
        }
    }
}

impl Program {
    fn draw_pixel(&mut self) {
        let armonized_cycle = self.cycle % 40;
        if self.cycle % 40 == 0 {
            self.drawing.push_str("\n");
        }
        if self.x == armonized_cycle
            || self.x - 1 == armonized_cycle
            || self.x + 1 == armonized_cycle
        {
            self.drawing.push_str("#");
        } else {
            self.drawing.push_str(".");
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
            acc.draw_pixel();
            match instruction {
                Instruction::Addx(x) => {
                    acc.cycle += 1;
                    // println!("Current: {}, adding {}, new total: {}", acc.x, x, acc.x + x);
                    acc.x += x;
                    acc.draw_pixel();
                }
                Instruction::Noop => (),
            };

            return acc;
        });

    println!("{}", res.drawing);

    return Ok(());
}
