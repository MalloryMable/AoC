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
    
    let re_trim = Regex::new(r"\s{2,}").unwrap();
    
    // First time I've ever done too much in one line in rust
    let times = re_trim.replace_all(&lines[0][5..].trim(), " ");
    let times:Vec<u16> = times.split_whitespace()
        .filter_map(|x| x.parse::<u16>().ok()).collect();
    let distances = re_trim.replace_all(&lines[1][9..].trim(), " ");
    let distances: Vec<u16> = distances.split_whitespace()
        .filter_map(|x| x.parse::<u16>().ok()).collect();
    
    let pairs = vec![(times[0], distances[0]),(times[1], distances[1]),
        (times[2], distances[2]), (times[3], distances[3])];
    let product = pairs.iter()
            .map(|(time, distance)| 
        {
            let time = *time as f32;
            let distance = *distance as f32;
            let quad_root = ( time * time - (4.0*distance) ).sqrt();
            let quad_plus =  ( (time + quad_root) / 2.0 ).ceil() as u32;
            let quad_minus =  ( (time - quad_root) / 2.0  ).ceil() as u32;
            quad_plus - quad_minus // Could add one and take quad_plus floor. 1 less opp 
        }).fold(1, |x, acc| x*acc);
    println!("{}", product);
}
