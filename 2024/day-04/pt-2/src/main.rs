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
}

impl Grid {
    fn new(reader: &mut impl BufRead) -> Grid {
        let data: Vec<Vec<char>> = reader
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.to_uppercase().chars().collect())
            .collect();
        
        Grid { data }
    }

    fn find_mas_patterns(&self) -> usize {
        let mut count = 0;

        // For each position in the grid
        for row in 1..self.data.len()-1 {  // Skip edges since we need space for diagonals
            for col in 1..self.data.first().unwrap().len()-1 { // WARN: could miscount cols
                if self.data[row][col] == 'A' {
                    if self.valid_pattern(row, col) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn valid_pattern(&self, row: usize, col: usize) -> bool {
        let (left_row, top_col, right_row, bot_col) = (row - 1, col - 1, row + 1, col + 1);
        
        let diagonals = [
            (self.data[left_row][top_col], self.data[right_row][bot_col]), // Top-left to bottom-right
            (self.data[right_row][top_col], self.data[left_row][bot_col]), // Top-right to bottom-left
        ];

        diagonals.iter().all(|&(first, second)| {
            (first == 'M' && second == 'S') || (first == 'S' && second == 'M')
        })
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
    let count = grid.find_mas_patterns();
    println!("Found MAS patterns: {}", count);
}

