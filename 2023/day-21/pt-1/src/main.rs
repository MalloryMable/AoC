use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

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
    let mut current_position:HashSet<(usize, usize)> = HashSet::new();
    
    let reader = reader_from_path(&args[1]);
    let map: Vec<Vec<bool>> = reader.lines().filter_map(Result::ok)
        .enumerate().map(|(j, line)| {
        line.chars().enumerate().map(|(i, c)| {
            // (walkable, visited)
            return match c {
                '.' => true,
                '#' => false,
                'S' => {
                    current_position.insert((i,j));
                    true
                },
                _ => { 
                    eprintln!("Invalid map item");
                    std::process::exit(1);
                },
            };
        }).collect()
    }).collect();



    let max_y = map.len() - 1;
    let max_x = map[0].len() - 1;

    for _ in 0..64 {
        let mut new_pos: HashSet<(usize, usize)> = HashSet::new();
        for (y, x) in current_position {
            if y > 0 {
                let dy = y - 1;
                if map[dy][x] == true { // map[dy][x].1 == false {
                    new_pos.insert((dy, x));
                }
            }
            if x > 0 {
                let dx = x - 1;
                if map[y][dx] == true { // && map[y][dx].1 == false {
                    new_pos.insert((y, dx));
                }
            } 
            if y < max_y {
                let dy = y + 1;
                if map[dy][x] == true { // && map[dy][x].1 == false {
                    new_pos.insert((dy, x));
                }
            }
            if x < max_x {
                let dx = x + 1;
                if map[y][dx] == true { // && map[y][dx].1 == false {
                    new_pos.insert((y, dx));
                }
            }
            
       }
       current_position = new_pos;
    }

    println!("{}", current_position.len());
}
