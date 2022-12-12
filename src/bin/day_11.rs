use std::{fs::read_to_string, str::FromStr};

use anyhow::Result;

const ROUNDS: u32 = 20;

struct Monkey {
    items: Vec<u32>,
    operation: Box<dyn Fn(u32) -> u32>,
    test: Box<dyn Fn(u32) -> bool>,
    positive_result: usize,
    negative_result: usize,
    inspections: u32,
}

impl Monkey {
    fn new(
        items: Vec<u32>,
        operation: Box<dyn Fn(u32) -> u32>,
        test: Box<dyn Fn(u32) -> bool>,
        positive_result: usize,
        negative_result: usize,
    ) -> Self {
        return Monkey {
            items,
            operation,
            test,
            positive_result,
            negative_result,
            inspections: 0,
        };
    }

    fn turn(&mut self) -> Vec<(usize, u32)> {
        let mut result: Vec<(usize, u32)> = Vec::new();

        for worry_level in self.items.iter() {
            self.inspections += 1;
            let new_worry_level = self.human_relief((*self.operation)(*worry_level));
            if (*self.test)(new_worry_level) {
                result.push((self.positive_result, new_worry_level));
            } else {
                result.push((self.negative_result, new_worry_level));
            }
        }
        self.items.clear();

        return result;
    }

    fn human_relief(&self, worry_level: u32) -> u32 {
        return worry_level / 3;
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        // Monkey 0:
        //   Starting items: 79, 98
        //   Operation: new = old * 19
        //   Test: divisible by 23
        //     If true: throw to monkey 2
        //     If false: throw to monkey 3
        let mut items = Vec::<u32>::new();
        let mut operation = get_operation(String::from("+ 0"));
        let mut test = get_test(String::from("1"));
        let mut positive_result = 0;
        let mut negative_result = 0;

        for v in s.split("\n").into_iter() {
            if v.trim().starts_with("Starting items: ") {
                items = v
                    .trim()
                    .replace("Starting items: ", "")
                    .split(", ")
                    .into_iter()
                    .filter_map(|x| x.parse::<u32>().ok())
                    .collect::<Vec<u32>>();
            } else if v.trim().starts_with("Operation: new = old ") {
                operation = get_operation(v.trim().replace("Operation: new = old ", ""));
            } else if v.trim().starts_with("Test: divisible by ") {
                test = get_test(v.trim().replace("Test: divisible by ", ""));
            } else if v.trim().starts_with("If true: ") {
                positive_result = v
                    .trim()
                    .replace("If true: throw to monkey ", "")
                    .parse::<usize>()
                    .unwrap();
            } else if v.trim().starts_with("If false: ") {
                negative_result = v
                    .trim()
                    .replace("If false: throw to monkey ", "")
                    .parse::<usize>()
                    .unwrap();
            }
        }

        return Ok(Monkey::new(
            items,
            operation,
            test,
            positive_result,
            negative_result,
        ));
    }
}

fn main() -> Result<()> {
    // if let Ok(file) = read_to_string("./input/test_11.txt") {
    if let Ok(file) = read_to_string("./input/input_11.txt") {
        let mut monkeys = file
            .split("\n\n")
            .into_iter()
            .filter_map(|monkey| monkey.parse::<Monkey>().ok())
            .collect::<Vec<Monkey>>();
        // println!("Result {:?}", result);

        for _ in 0..ROUNDS {
            for m in 0..monkeys.len() {
                let change = monkeys[m].turn();
                for (index, item) in change {
                    monkeys[index].items.push(item);
                }
            }
            for (i, monkey) in monkeys.iter().enumerate() {
                println!("Monkey {} has items {:?}", i, monkey.items);
            }
        }

        // Multiplied max values by hand
        let mut inspections = Vec::<u32>::new();
        for m in 0..monkeys.len() {
            inspections.push(monkeys[m].inspections);
        }

        inspections.sort();
        inspections.reverse();
        let max_two = &inspections[0..2];

        println!("Result: {}", max_two[0] * max_two[1]);
    }

    return Ok(());
}

fn get_operation(value: String) -> Box<dyn Fn(u32) -> u32> {
    let split = value.split(" ").collect::<Vec<&str>>();
    if let Ok(num) = split[1].parse::<u32>() {
        if split[0] == "+" {
            return Box::new(move |b| b + num);
        }

        return Box::new(move |b| b * num);
    }

    // Means that the second operator was "old"
    if split[0] == "+" {
        return Box::new(move |b| b + b);
    }

    return Box::new(move |b| b * b);
}

fn get_test(value: String) -> Box<dyn Fn(u32) -> bool> {
    let value = value.parse::<u32>().unwrap();
    return Box::new(move |b| b % value == 0);
}
