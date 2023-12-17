use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};

#[derive(Hash, Eq, PartialEq, Clone)]
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


// All parts of this are just different enough from one another to not method-ize well
fn rotate_map (map: &mut Vec<Vec<Ground>>) {
    // NOTE: While map.len() == map[0].len() different uses denote rows or columns

    // North
    for i in  0..map[0].len() {

        let mut roll_count = 0;        
        let mut begin = 0;

        for j in 0..map.len() {
            let c = &map[j][i];        
            if c == &Ground::Rounded {
                map[j][i] = Ground::Empty;
                roll_count += 1;
            } else if c == &Ground::Blocky {
                for count in 0..roll_count {
                    let index = count + begin;
                    map[index][i] = Ground::Rounded;
                }
                roll_count = 0;
                begin = j + 1;
            }
        }
        for count in 0..roll_count {
            let index = count + begin;
            map[index][i] = Ground::Rounded;
        }
    }

    // West
    for j in  0..map.len() {

        let mut roll_count = 0;        
        let mut begin = 0;

        for i in 0..map[0].len() {
            let c = &map[j][i];        
            if c == &Ground::Rounded {
                map[j][i] = Ground::Empty;
                roll_count += 1;
            } else if c == &Ground::Blocky {
                for count in 0..roll_count {
                    let index = count + begin;
                    map[j][index] = Ground::Rounded;
                }
                roll_count = 0;
                begin = i + 1;
            }
        }
        for count in 0..roll_count {
            let index = count + begin;
            map[j][index] = Ground::Rounded;
        }
    }

    // South
    for i in  0..map[0].len() {

        let mut roll_count = 0;        
        let mut begin = map.len();

        for not_j in 1..=map.len() {
            let j = map.len() - not_j;

            let c = &map[j][i];      
            if c == &Ground::Rounded {
                map[j][i] = Ground::Empty;
                roll_count += 1;
            } else if c == &Ground::Blocky {
                for count in 1..=roll_count {
                    let index = begin - count;
                    map[index][i] = Ground::Rounded;
                }
                roll_count = 0;
                begin = j;
            }
        }
        for count in 1..=roll_count {
            let index = begin - count;
            map[index][i] = Ground::Rounded;
        }
    }

    // East
    for j in  0..map.len() {

        let mut roll_count = 0;        
        let mut begin = map[0].len();

        for not_i in 1..=map[0].len() {
            let i = map.len() - not_i;

            let c = &map[j][i];        
            if c == &Ground::Rounded {
                map[j][i] = Ground::Empty;
                roll_count += 1;
            } else if c == &Ground::Blocky {
                for count in 1..=roll_count {
                    let index = begin - count;
                    map[j][index] = Ground::Rounded;
                }
                roll_count = 0;
                begin = i;
            }
            for count in 1..=roll_count {
                let index = begin - count;
                map[j][index] = Ground::Rounded;
            }
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
    for line in reader.lines().filter_map(Result::ok) {
        map.push( line.chars().map(|c| { 
            match c {
                'O' => Ground::Rounded,
                '#' => Ground::Blocky,
                '.' => Ground::Empty,
                _ => {
                    eprintln!("Invalid char parsed in ");
                    std::process::exit(1);
                }
            }
        }).collect());
    }

    let mut seen_hashes = HashMap::new();
    let mut remaining = 0; 

    for l in 0..1000000000 {
        rotate_map(&mut map);       

        let mut hasher = DefaultHasher::new();
        map.hash(&mut hasher);
        let hash = hasher.finish();
        if let Some(&start) = seen_hashes.get(&hash) {
            remaining = (1000000000 - (start + 1)) % (l - start);
            break;
        } else {
            seen_hashes.insert(hash, l);
        }
    }
    
    for _ in 0..remaining {
        rotate_map(&mut map);
    }
  
    let mut sum = 0;
    for (i, line) in map.iter().rev().enumerate() {
           let i = i + 1;
            for c in line {
               if c == &Ground::Rounded {sum += i};
           }
    }

    println!("{sum}");
}

