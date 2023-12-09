use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};

fn reader_from_path(relative_path: &str) -> BufReader<File> {
    let absolute_path = {
        match canonicalize(Path::new(relative_path)){
            Ok(file) => file,
            Err(_) => {
                eprintln!("Invalid file path: {}", relative_path);
                std::process::exit(1);
            },
        }
    };

    let file = {
        match File::open(absolute_path){
            Ok(file) => file,
            Err(_) => {
                eprintln!("File failed to open");
                std::process::exit(1);
            }
        }
    };

    BufReader::<File>::new(file)
}

fn get_next(values: Vec<i32>) -> i64 {
    
    let mut values = values.iter();
    let mut tester = values.clone();
    if tester.all(|v| v == &0){
        return 0;
    }
    let final_value = match values.next() {
        Some(x) => x,
        None => {
            eprintln!("Unreachable error");
            std::process::exit(1);
        },
    };
    let mut prev_value = *final_value;
    let mut derivative = Vec::new();

    for value in values {
        derivative.push(prev_value - value);
        prev_value = *value;
    } 
    
    *final_value as i64 + get_next(derivative)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} filepath", args[0]);
        std::process::exit(1);
    }

    let reader = reader_from_path(&args[1]);
    let sum: i64 = reader.lines().filter_map(Result::ok).map(|line| {
      
        let numbers: Vec<i32> = line.split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok()).rev().collect();


        get_next(numbers)

    }).sum();
    println!("{}", sum)
}
