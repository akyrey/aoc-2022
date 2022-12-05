use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn pop(&mut self) -> Option<T> {
        return self.stack.pop();
    }

    fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    // fn is_empty(&self) -> bool {
    //     return self.stack.is_empty();
    // }

    // fn length(&self) -> usize {
    //     return self.stack.len();
    // }

    fn peek(&self) -> Option<&T> {
        return self.stack.last();
    }
}

#[derive(Debug)]
struct Movement {
    size: u32,
    start: usize,
    end: usize,
}

impl Movement {
    fn new(values: Vec<u32>) -> Self {
        if values.len() != 3 {
            panic!("Cannot be here");
        }

        return Movement {
            size: values[0],
            start: (values[1] - 1) as usize,
            end: (values[2] - 1) as usize,
        };
    }
}

fn main() {
    let mut starting_stack_representation = Vec::<String>::new();
    let mut is_movement = false;
    let mut stacks = Vec::<Stack<String>>::new();
    // File hosts must exist in current path before this produces output
    // if let Ok(lines) = read_lines("./src/bin/test_05.txt") {
    if let Ok(lines) = read_lines("./src/bin/input_05.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    create_stacks_from_representation(
                        &mut stacks,
                        &mut starting_stack_representation,
                    );
                    is_movement = true;
                } else {
                    if !is_movement {
                        starting_stack_representation.push(ip);
                    } else {
                        if let Some(movement) = get_movement(ip) {
                            perform_movement(&mut stacks, movement);
                        }
                    }
                }
            }
        }
    }

    for value in stacks.iter() {
        println!("{:?}", value.peek());
    }
}

fn perform_movement(stacks: &mut Vec<Stack<String>>, movement: Movement) {
    let mut to_move = Vec::<String>::new();
    for _ in 0..movement.size {
        if let Some(moved) = stacks[movement.start].pop() {
            to_move.push(moved);
        }
    }
    for value in to_move.iter().rev() {
        stacks[movement.end].push(value.clone());
    }
}

fn get_movement(line: String) -> Option<Movement> {
    let result = line
        .split(" ")
        .into_iter()
        .filter_map(|value| value.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    if result.len() != 3 {
        return None;
    }

    return Some(Movement::new(result));
}

fn create_stacks_from_representation(
    stacks: &mut Vec<Stack<String>>,
    starting_stack_representation: &mut Vec<String>,
) {
    let reversed = starting_stack_representation
        .into_iter()
        .rev()
        .collect::<Vec<&mut String>>();
    let stack_positions = reversed
        .first()
        .unwrap()
        .split("")
        .enumerate()
        .filter_map(|(index, value)| {
            if value != "" && value != " " {
                return Some(index);
            } else {
                return None;
            }
        })
        .collect::<Vec<usize>>();
    for _ in stack_positions.iter().enumerate() {
        stacks.push(Stack::new());
    }
    reversed.into_iter().skip(1).for_each(|line| {
        let values = line.split("").collect::<Vec<&str>>();
        for (i, position) in stack_positions.iter().enumerate() {
            if values[*position] != " " {
                stacks[i].push(String::from(values[*position]));
            }
        }
    });
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
