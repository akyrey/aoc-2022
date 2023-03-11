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
    Top,
    Left,
    Right,
    Bottom,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Top,
            Direction::Left,
            Direction::Right,
            Direction::Bottom,
        ];
        DIRECTIONS.iter()
    }
}

fn main() {
    // let mut matrix: Matrix = Matrix::new(create_matrix("./input/test_08.txt"));
    let mut matrix: Matrix = Matrix::new(create_matrix("./input/input_08.txt"));

    let scenic_score: usize = highest_scenic_score(&mut matrix);

    println!("The highest scenic score is {}", scenic_score);
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

fn highest_scenic_score(matrix: &mut Matrix) -> usize {
    let mut scenic_scores: Vec<usize> = Vec::new();

    for col in 0..matrix.cols {
        for row in 0..matrix.rows {
            let scenic_score = count_tree_visible(matrix, col, row);
            println!("Scenic score {}", scenic_score);
            scenic_scores.push(scenic_score);
        }
    }

    match scenic_scores.iter().max() {
        Some(&max) => max,
        None => 0,
    }
}

fn count_tree_visible(matrix: &mut Matrix, col: usize, row: usize) -> usize {
    if col == 0 || row == 0 || col == matrix.cols - 1 || row == matrix.rows - 1 {
        return 0;
    }

    let mut count: usize = 1;
    for direction in Direction::iterator() {
        let visibles = count_visible_from(matrix, col, row, direction);
        // println!("Viewing {} from {:?}", visibles, direction);
        count *= visibles;
    }

    return count;
}

fn count_visible_from(matrix: &mut Matrix, col: usize, row: usize, direction: &Direction) -> usize {
    let current_tree_height = matrix.values[col][row];
    // println!("Compare {:?}", direction);
    match direction {
        Direction::Right => {
            let mut first_equal_or_higher = true;
            return ((row + 1)..(matrix.rows))
                .take_while(|&check| {
                    let result = first_equal_or_higher;
                    if current_tree_height <= matrix.values[col][check] {
                        first_equal_or_higher = false;
                    }
                    return result;
                })
                .collect::<Vec<usize>>()
                .len();
        }
        Direction::Left => {
            let mut first_equal_or_higher = true;
            return (0..row)
                .rev()
                .take_while(|&check| {
                    let result = first_equal_or_higher;
                    if current_tree_height <= matrix.values[col][check] {
                        first_equal_or_higher = false;
                    }
                    return result;
                })
                .collect::<Vec<usize>>()
                .len();
        }
        Direction::Bottom => {
            let mut first_equal_or_higher = true;
            return ((col + 1)..(matrix.cols))
                .take_while(|&check| {
                    let result = first_equal_or_higher;
                    if current_tree_height <= matrix.values[check][row] {
                        first_equal_or_higher = false;
                    }
                    return result;
                })
                .collect::<Vec<usize>>()
                .len();
        }
        Direction::Top => {
            let mut first_equal_or_higher = true;
            return (0..col)
                .rev()
                .take_while(|&check| {
                    let result = first_equal_or_higher;
                    if current_tree_height <= matrix.values[check][row] {
                        first_equal_or_higher = false;
                    }
                    return result;
                })
                .collect::<Vec<usize>>()
                .len();
        }
    }
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
