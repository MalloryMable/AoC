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
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut min: (i32, i32) = (0, 0);
    let mut outline = 0;
    let mut points: Vec<(i32, i32)> = Vec::new();
    points.push((0, 0));
    
    let mut map: Vec<(i32, i32)> = reader.lines().filter_map(Result::ok).map(|line| {
        let split: Vec<&str> = line.split_whitespace().collect();
        let shift = split[1].parse::<i32>().ok().unwrap();
        outline += shift;
        match split[0] {
            "D" => y = y - shift,
            "U" => y = y + shift,
            "R" => x = x + shift,
            "L" => x = x - shift,
            _ => {
                eprintln!("Not a valid directional input");
                std::process::exit(1);
            },
        };
        let index = (x, y);
        if index.0 < min.0 {min.0 = index.0};
        if index.1 < min.1 {min.1 = index.1};

        index
    }).collect();
    points.append(&mut map);
    // Because iters are lazy and absolute min is needed map is not chained
    let points: Vec<(usize, usize)> =  points.iter().map(|(x, y)| {
        ((x - min.0) as usize, (y - min.1) as usize)
    }).collect();


    let mut prev_x: usize = 0;
    let mut prev_y: usize = 0;
    let mut sum_x: usize = 0;
    let mut sum_y: usize = 0;
    for (x, y) in points {
        sum_x += prev_x * y;
        sum_y += prev_y * x;
        prev_x = x;
        prev_y = y;
    }
    let diff = if sum_y > sum_x {sum_y - sum_x} else {sum_x - sum_y};
    println!("Total area: {}", (diff + outline as usize) / 2  + 1 );
}

