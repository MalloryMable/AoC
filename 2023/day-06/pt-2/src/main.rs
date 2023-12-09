use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};
use regex::Regex;

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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} filepath", args[0]);
        std::process::exit(1);
    }
    
    let lines: Vec<String>= reader_from_path(&args[1])
        .lines()
        .filter_map(Result::ok)
        .collect();
    
    let re_trim = Regex::new(r"\s+").unwrap();
    let time = re_trim.replace_all(&lines[0][5..].trim(), "");
    let time:f64 = match time.parse::<u64>() {
        Ok(x) => x as f64,
        Err(_) => {
            eprintln!("Error parsing time"); 
            std::process::exit(1);
        }
    };
    let distance = re_trim.replace_all(&lines[1][9..].trim(), "");
    let distance: f64 = match distance.parse::<u64>() {
        Ok(x) => x as f64,
        Err(_) => {
            eprintln!("Error parsing distance"); 
            std::process::exit(1);
        }
    }; 
    
    let quad_root = ( time * time - (4.0*distance) ).sqrt();
    let quad_plus =  ( (time + quad_root) / 2.0 ).floor() as u32;
    let quad_minus =  ( (time - quad_root) / 2.0  ).ceil() as u32;
    println!("Winning range: {}", quad_plus - quad_minus + 1);
}
