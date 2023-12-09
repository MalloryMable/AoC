use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use regex::Regex;
// TODO: remove
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

enum Direction {
    Right,
    Left,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} filepath", args[0]);
        std::process::exit(1);
    }

    let re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
     
    // get it, because it's both a map and a hash map
    let mut map:HashMap< String, (String, String)> = HashMap::new();
    let mut pathing: Vec<Direction> = Vec::new();

    let reader = reader_from_path(&args[1]);
    let mut lines = reader.lines().filter_map(Result::ok);
 
    if let Some(line) = lines.next() {
        for c in line.chars() {
            if c == 'R' {
                pathing.push(Direction::Right);
            } else {
                pathing.push(Direction::Left);
            }
        }
    }
    
    // Skip
    lines.next();
    
    // Building map
    for line in lines {
        if let Some(line) = re.captures(&line) {
            map.insert(line[1].to_string(), (line[2].to_string(), line[3].to_string()));
        }
    }

    let mut position = "AAA".to_string();
    let mut searching = true;
    let mut steps = 0;

    while searching {
        for next_move in &pathing{
            let debug_direction = match next_move {
                &Direction::Left => "left",
                &Direction::Right => "right"
            };
            print!("{}: Starting at:{} Going: {}",steps, position, debug_direction); 
            if position == "ZZZ"{
                searching = false;
                break;
            }
            if let Some((left, right)) = map.get(&position) {
                println!("Going: ({}, {})", left.clone(), right.clone());
                position = match next_move {
                   &Direction::Left => left.clone(),
                   &Direction::Right => right.clone(),
                }
            }
            steps += 1;
        }
    }
    println!("Total steps: {}", steps);
}

