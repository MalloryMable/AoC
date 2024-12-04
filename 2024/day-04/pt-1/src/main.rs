use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};


fn reader_from_path(relative_path: &str) -> BufReader<File> {
    let absolute_path = canonicalize(Path::new(relative_path))
        .expect("Invalid file path");
    
    let file = File::open(absolute_path)
        .expect("File failed to open");
        
    BufReader::new(file)
}


struct Grid {
    data: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    directions: [(i32, i32); 8],
}

impl Grid {
    fn new(reader: &mut impl BufRead) -> Grid {
        let data: Vec<Vec<char>> = reader
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.to_uppercase().chars().collect())
            .collect();
        
        let rows = data.len();
        // This means the first row must be the begginning of the grid or columns will miscount
        let cols = data.first().unwrap().len(); // Always an empty vector to fallback and count
       
        // All possible directions: horizontal, vertical, and diagonal
        let directions = [
            (0, 1),   // right
            (1, 0),   // down
            (1, 1),   // diagonal down-right
            (-1, 1),  // diagonal up-right
            (0, -1),  // left
            (-1, 0),  // up
            (-1, -1), // diagonal up-left
            (1, -1),  // diagonal down-left
        ];

        Grid { data, rows, cols, directions }
    }

    fn find_word(&self, target: &str) -> usize {
        let target_chars: Vec<char> = target.chars().collect();
        let mut count = 0;

        for row in 0..self.rows {
            for col in 0..self.cols {
                // Only check if first character matches
                if self.data[row][col] == target_chars[0] {
                    // Try all directions
                    for &(dx, dy) in &self.directions {
                        if self.check_direction(row, col, dx, dy, &target_chars) {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }

    fn check_direction(&self, start_row: usize, start_col: usize, dx: i32, dy: i32, target: &[char]) -> bool {
        let mut row = start_row as i32;
        let mut col = start_col as i32;

        for target_char in target {
            if row < 0 || row >= self.rows as i32 || col < 0 || col >= self.cols as i32 {
                return false;
            }

            if self.data[row as usize][col as usize] != *target_char {
                return false;
            }

            row += dx;
            col += dy;
        }

        true
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Missing input file");
        std::process::exit(1);
    }

    let mut reader = reader_from_path(&args[1]);
  
    let grid = Grid::new(&mut reader);

    println!("XMAS count: {}", grid.find_word("XMAS"));


}

