use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const BUFFER_SIZE: usize = 14;

fn main() {
    // File hosts must exist in current path before this produces output
    // if let Ok(lines) = read_lines("./src/bin/test_06.txt") {
    if let Ok(lines) = read_lines("./src/bin/input_06.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut markers: Vec<usize> = Vec::new();
                let mut buffer: [&str; BUFFER_SIZE] = [""; BUFFER_SIZE];
                // Split returns an empty string at the start and end, so we skip one
                for (index, value) in ip.split("").enumerate().skip(1) {
                    let real_index = index - 1;
                    shift_buffer_values(&mut buffer);
                    buffer[BUFFER_SIZE - 1] = value;
                    if real_index >= BUFFER_SIZE && is_marker(buffer) {
                        // This should instead be the real_index + 1, so the actual index
                        markers.push(index);
                    }
                }
                println!("Markers: {:?}", markers);
            }
        }
    }
}

fn shift_buffer_values(buffer: &mut [&str; BUFFER_SIZE]) {
    for i in 0..(BUFFER_SIZE - 1) {
        buffer[i] = buffer[i + 1];
    }
}

fn is_marker(buffer: [&str; BUFFER_SIZE]) -> bool {
    let mut set = HashSet::new();
    return buffer.iter().fold(true, |acc, value| {
        if !acc {
            return false;
        }

        if set.contains(value) {
            return false;
        }

        set.insert(value);
        return acc;
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
