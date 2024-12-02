use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};
use std::ops::Bound::{Included, Unbounded};


fn reader_from_path(relative_path: &str) -> BufReader<File> {
    let absolute_path = canonicalize(Path::new(relative_path))
        .expect("Invalid file path");
    
    let file = File::open(absolute_path)
        .expect("File failed to open");
        
    BufReader::new(file)
}

// Uses the first and last position in a sorted list to get column 2 count
fn count_shared(target: u32, sorted_vec: &[u32]) -> usize {
    let start = sorted_vec.partition_point(|&x| x < target);
    let end = sorted_vec.partition_point(|&x| x <= target);
    end - start
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Missing input file");
        std::process::exit(1);
    }

    let reader = reader_from_path(&args[1]);

    let (col1, mut col2): (Vec<u32>, Vec<u32>) = reader.lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            let mut nums = line.split_whitespace();
            Some((
                nums.next().unwrap().parse::<u32>().unwrap(),
                nums.next().unwrap().parse::<u32>().unwrap()
            ))
        })
        .unzip();

    col2.sort();

    println!("Similarity score: {}", col1.iter()
        .map(|&num| num as usize * count_shared(num, &col2))
        .sum::<usize>()
    );
}

