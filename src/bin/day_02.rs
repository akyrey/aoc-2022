use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
* A or X Rock
* B or Y Paper
* C or Z Scissor
*/

fn main() {
    let mut score = 0;
    // File hosts must exist in current path before this produces output
    // if let Ok(lines) = read_lines("./src/bin/test_02.txt") {
    if let Ok(lines) = read_lines("./src/bin/input_02.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if !ip.is_empty() {
                    let strategy = ip.split(" ").collect::<Vec<&str>>();
                    let play_i_should_do = my_play(strategy[0], strategy[1]);
                    score = score
                        + my_play_score(&play_i_should_do)
                        + game_result(strategy[0], &play_i_should_do);
                }
            }
        }
    }

    println!("Total score: {}", score);
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

fn game_result(opponent_play: &str, my_play: &str) -> i32 {
    return match my_play {
        "A" => match opponent_play {
            "A" => 3,
            "B" => 0,
            "C" => 6,
            _ => 0,
        },
        "B" => match opponent_play {
            "A" => 6,
            "B" => 3,
            "C" => 0,
            _ => 0,
        },
        "C" => match opponent_play {
            "A" => 0,
            "B" => 6,
            "C" => 3,
            _ => 0,
        },
        _ => 0,
    };
}

fn my_play_score(play: &str) -> i32 {
    return match play {
        // Choose Rock
        "A" => 1,
        // Choose Paper
        "B" => 2,
        // Choose Scissor
        "C" => 3,
        _ => 0,
    };
}

fn my_play(opponent_play: &str, result: &str) -> String {
    return match result {
        "X" => match opponent_play {
            "A" => String::from("C"),
            "B" => String::from("A"),
            "C" => String::from("B"),
            _ => String::from(""),
        },
        "Y" => opponent_play.to_string(),
        "Z" => match opponent_play {
            "A" => String::from("B"),
            "B" => String::from("C"),
            "C" => String::from("A"),
            _ => String::from(""),
        },
        _ => String::from(""),
    };
}
