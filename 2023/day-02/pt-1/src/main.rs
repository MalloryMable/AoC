use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufRead, BufReader};

enum Color {
    Blue(u8),
    Red(u8),
    Green(u8),
}
impl Color{  
    fn impossible_check (&self) -> bool{
        match *self {
            Color::Red(x) => x > 12,
            Color::Blue(x) => x > 14,
            Color::Green(x) => x > 13,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} filepath", args[0]);
        std::process::exit(1);
    }

    if let Ok(absolute_path) = canonicalize(Path::new(&args[1])){
        if let Ok(file) = File::open(absolute_path) {
            let buffer = BufReader::<File>::new(file);

            let sum = buffer.lines().filter_map(Result::ok)
            .map(|line| {
                let line: Vec<&str> = line.split(':').collect();

                let rounds: Vec<&str> = line[1].split(';').collect();
                for round in rounds {
                    let pulls: Vec<&str> = round.split(',').collect();
                    for pull in pulls {
                        let split: Vec<&str> = pull[1..].split_whitespace().collect();
                        if let Ok(value) = split[0].parse::<u8>(){ 
                            let color = {
                                match split[1].len() {
                                    3 => Color::Red(value),
                                    4 => Color::Blue(value),
                                    5 => Color::Green(value),
                                    _ => todo!(),
                                }
                            };
                            // returns true if test is impossible
                            if color.impossible_check() {
                                return 0;
                            }
                        }
                    }
                }
                // returns game number if the game 
                if let Ok(game_number) = &line[0][5..].parse::<u16>(){
                    return *game_number;
                }
                // default
                0
            })
            .fold(0, |acc, x| acc + x);

            println!("Sum of possible game IDs: {}", sum);
        } 
    }
}

