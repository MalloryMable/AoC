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

fn evaluate(target: u64, list: &[u64]) -> Option<u64> {
    let len = list.len();
    
    // Iterate through all possible combinations of add/multiply
    for mask in 0..(1 << len) {
        let mut current = 1;
        
        // Compute combination based on bit mask
        for (i, &num) in list.iter().enumerate() {
            if mask & (1 << i) != 0 {
                current *= num;
            } else {
                current += num;
            }
        }
        
        // Check if this combination matches target
        if current == target {
            return Some(target);
        }
    }
    
    None
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Missing input file");
        std::process::exit(1);
    }

    let reader = reader_from_path(&args[1]);

    let sum: u64 = reader.lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            let (target_str, list_str) = line.split_once(':')?;

            let target: u64 = target_str.trim().parse::<u64>().ok()?;

            let list: Vec<u64> = list_str
                .split_whitespace()
                .filter_map(|num| num.parse::<u64>().ok())
                .collect();

            evaluate(target, &list) 
        })
        .sum();

    println!("Sum of valid combinations: {}", sum);

}


