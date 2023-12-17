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
    
    let reader = reader_from_path(&args[1]);
    let line = reader.lines().find_map(Result::ok)
        .expect("File contains no lines");
    
    let sum: u32 = line.split(',').map(|instruct| {
        let mut hasher: u16 = 0;
        for c in instruct.chars() {
            hasher += (c as u8) as u16;
            hasher *= 17;
            hasher %= 256;
        }
        hasher as u32
    }).sum();
    println!("{}", sum);
}
