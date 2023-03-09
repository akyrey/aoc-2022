use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::slice::Iter;

#[derive(Debug)]
struct Matrix {
    values: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(values: Vec<Vec<u8>>) -> Self {
        let rows = values[0].len();
        let cols = values.len();
        return Matrix { values, rows, cols };
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Bottom,
    Top,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Right,
            Direction::Left,
            Direction::Bottom,
            Direction::Top,
        ];
        DIRECTIONS.iter()
    }
}

fn main() {
    // let mut matrix: Matrix = Matrix::new(create_matrix("./input/test_08.txt"));
    let mut matrix: Matrix = Matrix::new(create_matrix("./input/input_08.txt"));

    let visible_count: u32 = count_visible_trees(&mut matrix);

    println!(
        "This is the matrix: {:?} has {} visible trees",
        matrix, visible_count
    );
}

fn create_matrix(input_file: &str) -> Vec<Vec<u8>> {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(input_file) {
        return lines.into_iter().fold(Vec::new(), |mut acc, line| {
            if let Ok(current) = line {
                let new_vec: Vec<u8> = current
                    .split("")
                    .into_iter()
                    .filter_map(|value| value.parse::<u8>().ok())
                    .collect::<Vec<u8>>();
                acc.push(new_vec);
            }

            return acc;
        });
    }

    return Vec::new();
}

fn count_visible_trees(matrix: &mut Matrix) -> u32 {
    let mut sum: u32 = 0;

    for col in 0..matrix.cols {
        for row in 0..matrix.rows {
            if is_tree_visible(matrix, col, row) {
                sum = sum + 1;
            }
        }
    }

    return sum;
}

fn is_tree_visible(matrix: &mut Matrix, col: usize, row: usize) -> bool {
    if col == 0 || row == 0 || col == matrix.cols - 1 || row == matrix.rows - 1 {
        return true;
    }

    for direction in Direction::iterator() {
        if check_visible_from(matrix, col, row, direction) {
            println!("{} visible", matrix.values[col][row]);
            return true;
        }
    }

    println!("{} NOT visible", matrix.values[col][row]);
    return false;
}

fn check_visible_from(matrix: &mut Matrix, col: usize, row: usize, direction: &Direction) -> bool {
    let current_tree_height = matrix.values[col][row];
    // println!("Compare {:?}", direction);
    match direction {
        Direction::Right => {
            return ((row + 1)..(matrix.rows)).rev().fold(true, |acc, check| {
                if acc && !is_visible(current_tree_height, matrix.values[col][check]) {
                    return false;
                }

                return acc;
            });
        }
        Direction::Left => {
            return (0..row).fold(true, |acc, check| {
                if acc && !is_visible(current_tree_height, matrix.values[col][check]) {
                    return false;
                }

                return acc;
            });
        }
        Direction::Bottom => {
            return ((col + 1)..(matrix.cols)).rev().fold(true, |acc, check| {
                if acc && !is_visible(current_tree_height, matrix.values[check][row]) {
                    return false;
                }

                return acc;
            });
        }
        Direction::Top => {
            return (0..col).fold(true, |acc, check| {
                if acc && !is_visible(current_tree_height, matrix.values[check][row]) {
                    return false;
                }

                return acc;
            });
        }
    }
}

fn is_visible(current_tree_height: u8, other_tree_height: u8) -> bool {
    // println!("Compare {} with {}", current_tree_height, other_tree_height);
    return current_tree_height > other_tree_height;
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
