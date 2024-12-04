
use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};
use regex::Regex;

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

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    println!("{}", reader.lines()
        .filter_map(Result::ok)
        .flat_map(|line| { // without flat_map we have to sum each line
            re.captures_iter(&line)
                .map(|cap| {
                    let num1 = cap[1].parse::<u32>().unwrap();
                    let num2 = cap[2].parse::<u32>().unwrap();
                    num1 * num2
                })
                .collect::<Vec<u32>>()
        })
        .sum::<u32>()
    );
}

