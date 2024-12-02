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
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Missing input file");
        std::process::exit(1);
    }

    let reader = reader_from_path(&args[1]);

    let (mut col1, mut col2): (Vec<u32>, Vec<u32>) = reader.lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            let mut nums = line.split_whitespace();
            Some((
                nums.next().unwrap().parse::<u32>().unwrap(),
                nums.next().unwrap().parse::<u32>().unwrap()
            ))
        })
        .unzip();

    col1.sort();
    col2.sort();

    println!("Sum: {}", col1.iter()
        .zip(col2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u32>()
    );
}

