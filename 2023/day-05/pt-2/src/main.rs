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
    
    let mut seed_values: Vec<(u64, u64)> = Vec::new();
    let mut new_seeds: Vec<(u64, u64)> = Vec::new();
    let reader = reader_from_path(&args[1]);
    let mut lines = reader.lines().filter_map(Result::ok);
    if let Some(line) = lines.next() {
        let line = &line[7..];
        let seeds = line.split_whitespace();

        let mut first_value: u64 = 0;
        for (index, seed ) in seeds.enumerate(){
            if let Ok(seed) = seed.parse::<u64>(){
                if index % 2 == 1 {
                    seed_values.push((first_value, seed));
                } else {
                    first_value = seed;
                }
                
            }
        }
    }

    // Skip two known lines
    lines.next();
    lines.next();

    let mut skip = false;
    for line in lines{
        if line.is_empty() {
            for (_, seed) in seed_values.iter_mut().enumerate(){
                if seed.1 > 0 {
                    new_seeds.push((seed.0, seed.1));
                }
            }
            seed_values = new_seeds;
            new_seeds = Vec::new(); 
            skip = true;
            continue;
        }

        if skip {
            skip = false;
            continue;
        }

        let values: Vec<u64> = line.split_whitespace()
            .filter_map(|entry| entry.parse::<u64>().ok())
            .collect();
        
        let (dest, source, range) = (values[0], values[1], values[2]);
        
        let mut held_range: Vec<(u64, u64)> = Vec::new();
        
        for (_, seed_range) in seed_values.iter_mut().enumerate(){
            let source_end = source + range;
            let origin_end = seed_range.0 + seed_range.1;

            if  seed_range.0 < source && source < origin_end { // source after origin
                // Since source starts after origin new orgin range is old minus source 
                let offset = source - seed_range.0;

                seed_range.1 = offset;
                
                if source_end < origin_end { // origin proper subset case
                    held_range.push((source_end, origin_end - source_end));
                    new_seeds.push((dest, range));
                    

                } else { // origin end <= source end 
                    new_seeds.push((dest, seed_range.1 - offset));
                }
            } else if source <= seed_range.0 && seed_range.0 < source_end { 
                // origin after source 

                let offset = seed_range.0 - source;

                if source_end < origin_end { // trailing origin case
                    seed_range.0 = source_end;
                    seed_range.1 = origin_end - source_end;
                    
                    new_seeds.push((dest + offset, range - offset));
                } else { // origin end <= source end (source perfect subset)
                    new_seeds.push((dest + offset, seed_range.1));
                   
                    seed_range.1 = 0;

                }
            }
        }
        for vector in held_range{ 
            seed_values.push(vector);
        }
    }

    let mut min = u64::MAX;
    for (position, _) in new_seeds{
        if position < min{
            min = position;
        }
    }
    println!("Lowest planting location: {}",min);
}

