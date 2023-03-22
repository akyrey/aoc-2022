use std::{collections::HashSet, str::FromStr};

use anyhow::Result;

const KNOTS: usize = 10;
// const INPUT_FILE: &str = "./input/test_09.txt";
// const INPUT_FILE: &str = "./input/test_09_2.txt";
const INPUT_FILE: &str = "./input/input_09.txt";

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn simple_move(&mut self, direction: &Direction) {
        match direction {
            Direction::Up(m) => self.x = self.x - m,
            Direction::Right(m) => self.y = self.y + m,
            Direction::Down(m) => self.x = self.x + m,
            Direction::Left(m) => self.y = self.y - m,
        };
    }

    fn distance(&self, other: &Self) -> f64 {
        return f64::sqrt(
            f64::from((other.x - self.x).pow(2)) + f64::from((other.y - self.y).pow(2)),
        );
    }
}

#[derive(Clone, Debug)]
struct Rope {
    knots: Vec<Point>,
    visited: HashSet<Point>,
}

impl Rope {
    fn new() -> Self {
        return Rope {
            knots: vec![Point { x: 0, y: 0 }; KNOTS],
            visited: HashSet::from([Point { x: 0, y: 0 }]),
        };
    }

    fn follow_head(&mut self, direction: &Direction) {
        let (coordinate, multiplier, count) = match direction {
            Direction::Up(m) => (0, -1, m),
            Direction::Right(m) => (1, 1, m),
            Direction::Down(m) => (0, 1, m),
            Direction::Left(m) => (1, -1, m),
        };
        for _ in 0..*count {
            if coordinate == 0 {
                self.knots[0].x += multiplier;
            } else {
                self.knots[0].y += multiplier;
            }
            for i in 0..self.knots.len() - 1 {
                while self.knots[i + 1].distance(&self.knots[i]) >= 2.0 {
                    // Same x
                    if self.knots[i + 1].x == self.knots[i].x {
                        if self.knots[i].y > self.knots[i + 1].y {
                            self.knots[i + 1].y += 1;
                        } else {
                            self.knots[i + 1].y -= 1;
                        }
                    }
                    // Same y
                    else if self.knots[i + 1].y == self.knots[i].y {
                        if self.knots[i].x > self.knots[i + 1].x {
                            self.knots[i + 1].x += 1;
                        } else {
                            self.knots[i + 1].x -= 1;
                        }
                    } else {
                        if self.knots[i].x > self.knots[i + 1].x
                            && self.knots[i].y > self.knots[i + 1].y
                        {
                            self.knots[i + 1].x += 1;
                            self.knots[i + 1].y += 1;
                        } else if self.knots[i].x < self.knots[i + 1].x
                            && self.knots[i].y < self.knots[i + 1].y
                        {
                            self.knots[i + 1].x -= 1;
                            self.knots[i + 1].y -= 1;
                        } else if self.knots[i].x > self.knots[i + 1].x
                            && self.knots[i].y < self.knots[i + 1].y
                        {
                            self.knots[i + 1].x += 1;
                            self.knots[i + 1].y -= 1;
                        } else if self.knots[i].x < self.knots[i + 1].x
                            && self.knots[i].y > self.knots[i + 1].y
                        {
                            self.knots[i + 1].x -= 1;
                            self.knots[i + 1].y += 1;
                        }
                    }
                    let tail = self.knots[self.knots.len() - 1].clone();
                    // if self.visited.get(&tail).is_none() {
                    //     println!(
                    //         "Tail visited: {:?}",
                    //         self.knots[self.knots.len() - 1].clone()
                    //     );
                    // }
                    self.visited.insert(tail);
                }
            }
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32),
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("bad motion from str"));
        }

        match parts[0] {
            "U" => return Ok(Direction::Up(parts[1].parse::<i32>()?)),
            "R" => return Ok(Direction::Right(parts[1].parse::<i32>()?)),
            "D" => return Ok(Direction::Down(parts[1].parse::<i32>()?)),
            "L" => return Ok(Direction::Left(parts[1].parse::<i32>()?)),
            _ => return Err(anyhow::anyhow!("unknown direction")),
        }
    }
}

fn main() -> Result<()> {
    let res = std::fs::read_to_string(INPUT_FILE)?
        .lines()
        .filter_map(|x| x.parse::<Direction>().ok())
        .fold(Rope::new(), |mut acc, direction| {
            // acc.knots[0].simple_move(&direction);
            acc.follow_head(&direction);
            // println!("Moved: {:?}", acc);
            return acc;
        });

    // println!("Rope: {:?}", res);
    println!("Tail unique position visited: {}", res.visited.len());

    return Ok(());
}
