use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::collections::HashMap;
// TODO: Remove
use std::{print, println, eprintln};

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
    
    let re = Regex::new(r"^(broadcast|([&%])(\w+)) -> (.*)$").unwrap();

    let reader = reader_from_path(&args[1]);
    for line in reader.lines().filter_map(Result::ok) {
        let caps = re.captures(&line).unwrap();
        if let Some(symbol) = caps[2] {
            println!("wah");
        } else {
            println!("broadcast");
        }
    }

}
