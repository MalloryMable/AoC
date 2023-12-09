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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} filepath", args[0]);
        std::process::exit(1);
    }
    
    let mut seed_values: Vec<(u64, u64)> = Vec::new();

    let reader = reader_from_path(&args[1]);
    let mut lines = reader.lines().filter_map(Result::ok);
    if let Some(line) = lines.next() {
        let line = &line[7..];
        let seeds = line.split_whitespace();
        
        for seed in seeds{  
            if let Ok(seed) = seed.parse::<u64>(){
                seed_values.push((seed, 0));
            }
        }
    }

    // Skip two known lines
    lines.next();
    lines.next();
    // If I'm going to have to check for a bad line every time I want it to be quick
    let mut skip = false;
    for line in lines{
        if line.is_empty() {
            for (_, seed) in seed_values.iter_mut().enumerate(){
                seed.0 = seed.1;
            }
            skip = true;
            continue;
        }

        if skip {
            skip = false;
            continue;
        }

        let values: Vec<u64> = line.split_whitespace()
            .filter_map(|entry| entry.parse::<u64>().ok())
            .collect();
        
        let (dest, source, range) = (values[0], values[1], values[2]);
        for (_, seed_values) in seed_values.iter_mut().enumerate(){
            if source <= seed_values.0 && seed_values.0 < (source + range ) {
                let delta = seed_values.0 - source;
                seed_values.1 = delta + dest;
            } 
        }
    }
    println!("Minimum final value: {}", seed_values.iter()
        .map(|(_, x)| x).fold(u64::MAX, |min, &x| if x < min {x} else {min}));
}

