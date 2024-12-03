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



fn main() {
    let input_file = env::args()
        .nth(1)
        .unwrap_or_else( || { 
            eprintln!("Missing input file");
            std::process::exit(1); 
        });

    let reader = reader_from_path(&input_file);

    // regex mul\((\d+),(\d+)\) while has next, extract matches, parse to int, multiply, sum

    for line in reader.lines().find_map(Result::ok) {
        println!("{}", line);
    }
}
