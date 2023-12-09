use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufRead, BufReader};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} filepath", args[0]);
        std::process::exit(1);
    }
    //next time I'm just going to allow this to return errors to take advantage of fmt tricks
    if let Ok(absolute_path) = canonicalize(Path::new(&args[1])){
        if let Ok(file) = File::open(absolute_path) {
            let buffer = BufReader::<File>::new(file);

            let sum: u32 = buffer.lines().filter_map(Result::ok)
            .map(|line| {
                let line: Vec<&str> = line.split(':').collect();
                let mut possible_set =(0, 0, 0);
                
                let rounds: Vec<&str> = line[1].split(';').collect();
                for round in rounds {
                    let pulls: Vec<&str> = round.split(',').collect();
                    for pull in pulls {
                        let split: Vec<&str> = pull[1..].split_whitespace().collect();
                        if let Ok(value) = split[0].parse::<u8>(){ 
                            match split[1].len() {
                                3 => {
                                    if value > possible_set.0{
                                        possible_set.0 = value;
                                    }
                                },
                                4 => {
                                    if value > possible_set.1{
                                        possible_set.1 = value;
                                    }
                                },
                                5 => {
                                    if value > possible_set.2{
                                        possible_set.2 = value;
                                    }
                                },
                                // This is bad practice
                                _ => todo!(),
                            }
                        }
                    }
                }
                possible_set.0 as u32 * possible_set.1 as u32 * possible_set.2 as u32

            })
            .fold(0, |acc, x| acc + x);

            println!("Sum of possible sets: {}", sum);
        } 
    }
}

