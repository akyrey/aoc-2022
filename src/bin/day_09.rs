use std::{collections::HashSet, str::FromStr};

use anyhow::Result;

// const INPUT_FILE: &str = "./input/test_09.txt";
const INPUT_FILE: &str = "./input/input_09.txt";

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn simple_move(&mut self, direction: &Direction) {
        match direction {
            Direction::Up(m) => self.0 = self.0 - m,
            Direction::Right(m) => self.1 = self.1 + m,
            Direction::Down(m) => self.0 = self.0 + m,
            Direction::Left(m) => self.1 = self.1 - m,
        };
    }

    fn distance(&self, other: &Self) -> f64 {
        return f64::sqrt(
            f64::from((other.0 - self.0).pow(2)) + f64::from((other.1 - self.1).pow(2)),
        );
    }
}

#[derive(Clone, Debug)]
struct Rope {
    head: Point,
    tail: Point,
    visited: HashSet<Point>,
}

impl Rope {
    fn new() -> Self {
        return Rope {
            head: Point(0, 0),
            tail: Point(0, 0),
            visited: HashSet::from([Point(0, 0)]),
        };
    }

    fn follow_head(&mut self) {
        while self.tail.distance(&self.head) >= 2.0 {
            // Same x
            if self.tail.0 == self.head.0 {
                if self.head.1 > self.tail.1 {
                    self.tail.1 += 1;
                } else {
                    self.tail.1 -= 1;
                }
            }
            // Same y
            else if self.tail.1 == self.head.1 {
                if self.head.0 > self.tail.0 {
                    self.tail.0 += 1;
                } else {
                    self.tail.0 -= 1;
                }
            } else {
                if self.head.0 > self.tail.0 && self.head.1 > self.tail.1 {
                    self.tail.0 += 1;
                    self.tail.1 += 1;
                } else if self.head.0 < self.tail.0 && self.head.1 < self.tail.1 {
                    self.tail.0 -= 1;
                    self.tail.1 -= 1;
                } else if self.head.0 > self.tail.0 && self.head.1 < self.tail.1 {
                    self.tail.0 += 1;
                    self.tail.1 -= 1;
                } else if self.head.0 < self.tail.0 && self.head.1 > self.tail.1 {
                    self.tail.0 -= 1;
                    self.tail.1 += 1;
                }
            }
            self.visited.insert(self.tail.clone());
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
            acc.head.simple_move(&direction);
            acc.follow_head();
            return acc;
        });

    // println!("Rope: {:?}", res);
    println!("Tail unique position visited: {}", res.visited.len());

    return Ok(());
}
