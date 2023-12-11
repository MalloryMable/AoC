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
    
    let mut offset: i32 = 0;
    let mut galaxies: Vec<(i32, i32)> = Vec::new();

    let reader = reader_from_path(&args[1]);
    let mut length = 0;
    let mut y_array: Vec<i32> = Vec::new();

    for (x, line) in reader.lines().filter_map(Result::ok).enumerate() {
        let mut expanding: bool = true;
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                expanding = false;
                let y = y as i32;
                galaxies.push((x as i32 + offset, y));
                if ! y_array.contains(&y) {
                    y_array.push(y);
                }
            }
        }
        if expanding {
            offset += 1;
            length = line.len();
        }
    }

    let filtered_columns: Vec<i32> = (0..length as i32).into_iter()
        .filter(|x| !y_array.contains(x)).collect();

    for (offset, y) in filtered_columns.iter().enumerate() { 
        for galaxy in &mut galaxies {
            if &galaxy.1 > &(y + offset as i32) {
                galaxy.1 += 1;
            }
        }
    }

    let mut sum = 0;
    
    for (offset, (x1, y1)) in galaxies.iter().enumerate(){
        for (x2, y2) in &galaxies[offset + 1..] {
            let prev_sum = sum;
            sum += (x2-x1).abs() + (y2-y1).abs();
        }
    }
    println!("{}", sum);
}
