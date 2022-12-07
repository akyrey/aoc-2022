use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut index = 0;
    let mut group = Vec::<char>::new();
    let mut sum = 0;
    // File hosts must exist in current path before this produces output
    // if let Ok(lines) = read_lines("./input/test_03.txt") {
    if let Ok(lines) = read_lines("./input/input_03.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let remainder = index % 3;
                if remainder == 0 {
                    group = ip.chars().collect::<Vec<char>>();
                } else {
                    group = group
                        .into_iter()
                        .filter(|value| ip.chars().collect::<Vec<char>>().contains(value))
                        .collect::<HashSet<char>>()
                        .into_iter()
                        .collect::<Vec<char>>();
                }
                if group.len() == 1 {
                    sum = sum + get_item_priority(&group[0]);
                }
            }
            index = index + 1;
        }
    }

    println!("Priority sum: {}", sum);
}

fn get_item_priority(value: &char) -> u32 {
    let int_value = *value as u32;
    if int_value < 97 {
        return int_value - (65 - 27);
    }

    return int_value - 96;
}

// fn find_common_char<'a>(
//     first_rucksack: &Vec<char>,
//     second_rucksack: &'a Vec<char>,
// ) -> Option<&'a char> {
//     return second_rucksack
//         .iter()
//         .find(|current| first_rucksack.contains(&current));
// }

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
