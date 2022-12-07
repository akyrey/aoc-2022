use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut elves_calories = Vec::<Vec<i32>>::new();
    elves_calories.push(Vec::new());
    let mut index: usize = 0;
    // File hosts must exist in current path before this produces output
    // if let Ok(lines) = read_lines("./input/test_01.txt") {
    if let Ok(lines) = read_lines("./input/input_01.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if !ip.is_empty() {
                    // Add value to current vector
                    elves_calories[index].push(ip.parse::<i32>().unwrap());
                } else {
                    // Create new vector
                    elves_calories.push(Vec::new());
                    index = index + 1;
                }
            }
        }
    }

    let sum: i32 = elves_calories
        .iter()
        .map(|snack| snack.iter().sum())
        .fold(Vec::new(), |mut max_three, item: i32| {
            let index = max_three
                .iter()
                .position(|&current| item > current)
                .unwrap_or_else(|| max_three.len());

            max_three.insert(index, item);

            return max_three.into_iter().take(3).collect::<Vec<i32>>();
        })
        .into_iter()
        .sum();
    println!(
        "Elves: {}, calories sum for top three: {}",
        elves_calories.len(),
        sum
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
