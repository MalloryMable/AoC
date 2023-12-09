use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No file path provided");
        std::process::exit(1);
    }

    let absolute_path = match canonicalize(Path::new(&args[1])) 
    {
        Ok(path) => path,
        Err(_) => {
            eprintln!("Invalid path: {}", &args[1]);
            std::process::exit(1);
        }
    };

    let file = match File::open(absolute_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("File failed to open");
            std::process::exit(1);
        }
    };
    
    let re_drop_intro = Regex::new(r"Card\s+\d+: ").unwrap();
    let re_trim = Regex::new(r"\s{2,}").unwrap();

    
    let mut reps: Vec<u8> = Vec::new();

    let reader = BufReader::<File>::new(file);
    let sum: usize = reader.lines().filter_map(Result::ok)
        .map(|line|{
            let mut score: u8 = 0;

            let dropped_line = re_drop_intro.replace(&line, "");
            let trimmed_line = re_trim.replace_all(&dropped_line.trim(), " ");

           
            let (winning_nums, my_nums) = match trimmed_line.split_once(" | "){
                Some(split) => split,
                None => {
                    eprintln!("Line split failed");
                    std::process::exit(1);
                }
             };

            // O(n^2) :(
            for num in my_nums.split_whitespace(){
                for winner in winning_nums.split_whitespace(){
                    if let Ok(num) = num.parse::<u8>(){
                        if let Ok(winner) = winner.parse::<u8>(){
                            if winner == num {
                                score += 1;
                            } 
                        }  
                    } 
                }
            }
            
            let card_count: usize = 1 + reps.len();

            for remaining in reps.iter_mut() {
                *remaining -= 1;
            }

            reps.extend(std::iter::repeat(score).take(card_count));

            reps.retain(|&x| x > 0);

            card_count

        }).sum();

    println!("Sum of winning numbers: {}", sum);
}
