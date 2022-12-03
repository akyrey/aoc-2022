use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum = 0;
    // File hosts must exist in current path before this produces output
    // if let Ok(lines) = read_lines("./src/bin/test_03.txt") {
    if let Ok(lines) = read_lines("./src/bin/input_03.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let chars = ip.chars().collect::<Vec<char>>();
                let rucksacks: Vec<Vec<char>> = chars.chunks(chars.len() / 2).map(|c| c.into()).collect();
                let common_char = find_common_char(&rucksacks[0], &rucksacks[1]);
                if let Some(item_type) = common_char {
                    sum = sum + get_item_priority(item_type);
                }
            }
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

fn find_common_char<'a>(first_rucksack: &Vec<char>, second_rucksack: &'a Vec<char>) -> Option<&'a char> {
    return second_rucksack.iter().find(|current| first_rucksack.contains(&current));
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
