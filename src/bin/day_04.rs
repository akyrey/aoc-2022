use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut contained_sets = 0;
    // File hosts must exist in current path before this produces output
    // if let Ok(lines) = read_lines("./src/bin/test_04.txt") {
    if let Ok(lines) = read_lines("./src/bin/input_04.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let elves_assignments = ip.split(',').collect::<Vec<&str>>();
                if elves_assignments.len() != 2 {
                    panic!("The string should represent two assignments separated by comma");
                }
                let first_assignment = get_sections(elves_assignments[0]);
                let second_assignment = get_sections(elves_assignments[1]);
                if is_overlapping(first_assignment, second_assignment) {
                    contained_sets = contained_sets + 1;
                }
            }
        }
    }
    println!("Contained sets {}", contained_sets);
}

fn is_overlapping(first_assignment: (u32, u32), second_assignment: (u32, u32)) -> bool {
    for first_value in first_assignment.0..(first_assignment.1 + 1) {
        for second_value in second_assignment.0..(second_assignment.1 + 1) {
            if first_value == second_value {
                return true;
            }
        }
    }

    return false;
}

// fn is_contained(first_assignment: (u32, u32), second_assignment: (u32, u32)) -> bool {
//     return (first_assignment.0 >= second_assignment.0
//         && first_assignment.1 <= second_assignment.1)
//         || (second_assignment.0 >= first_assignment.0
//             && second_assignment.1 <= first_assignment.1);
// }

fn get_sections(value: &str) -> (u32, u32) {
    let array = value.split('-').collect::<Vec<&str>>();
    if array.len() != 2 {
        return (0, 0);
    }

    return (
        array[0]
            .parse::<u32>()
            .expect("This should have been a digit"),
        array[1]
            .parse::<u32>()
            .expect("This should have been a digit"),
    );
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
