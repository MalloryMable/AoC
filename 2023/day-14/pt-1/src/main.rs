use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};

#[derive(PartialEq, Clone)]
enum Ground {
    Rounded,
    Blocky,
    Empty,
}

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

fn get_ground(c: char) -> Ground{
    
    match c {
        'O' => Ground::Rounded,
        '#' => Ground::Blocky,
        '.' => Ground::Empty,
        _ => {
            eprintln!("Invalid char parsed in ");
            std::process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} filepath", args[0]);
        std::process::exit(1);
    }
    
    let reader = reader_from_path(&args[1]);
    
    let mut map: Vec<Vec<Ground>> = Vec::new();
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    for _ in 0..lines[0].len() {
        map.push(Vec::new()); 
    }

    for line in lines {
        for (i, c) in line.chars().enumerate(){
            map[i].push(get_ground(c));
        }
    }

    let mut sum = 0;

    let observed_map = map.clone();
    for (j, line) in observed_map.iter().enumerate() {
        
        let mut roll_count = 0;        
        let mut begin = 0;
        
        for (i, c) in line.iter().enumerate() {
                    if c == &Ground::Rounded {
                map[j][i] = Ground::Empty;
                roll_count += 1;
            } else if c == &Ground::Blocky {
                for count in 0..roll_count {
                    let index = count + begin;
                    map[j][index] = Ground::Rounded;
                    sum += map.len() - index;
                }
                roll_count = 0;
                begin = i + 1;
            }
        }
        for count in 0..roll_count {
            let index = count + begin;
            map[j][index] = Ground::Rounded;
            sum += map.len() - index;
        }
    }
    

    println!("{sum}");
}

