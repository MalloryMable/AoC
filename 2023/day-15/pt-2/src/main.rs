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
    
    let mut map: Vec<Vec<(&str, u8)>> = vec![Vec::new(); 256];
    

    let reader = reader_from_path(&args[1]);
    let line = reader.lines().find_map(Result::ok)
        .expect("File contains no lines");
    
    for instruct in  line.split(',') {
        let mut hasher: usize = 0;
        let mut add = false;
        for c in instruct.chars() {
            if c == '-' {
                break;
            } else if c == '=' {
                add = true;
                break;
            }
            hasher += (c as u8) as usize;
            hasher *= 17;
            hasher %= 256;
        }
        if add {
            let split: Vec<&str> = instruct.split('=').collect();
            let lense = (split[0], split[1].parse::<u8>().ok().unwrap());
            
            let mut removable: Option<usize> = None;
            for (i, contents) in map[hasher].iter().enumerate(){
                if contents.0 == lense.0 {
                    removable = Some(i);
                }
            }

            if let Some(i) = removable {
                map[hasher][i].1 = lense.1;
            } else {
                map[hasher].push(lense);
            }
        } else {
            let label = &instruct[..instruct.len() - 1];

            let mut removable: Option<usize> = None;
            for (i, lense) in map[hasher].iter().enumerate(){
                if lense.0 == label {
                    removable = Some(i);
                }
            }

            if let Some(i) = removable {
                map[hasher].remove(i);
            }
        }
    }

    let mut sum = 0;
    for (j, lenses) in map.iter().enumerate() {
        for (i, lense) in lenses.iter().enumerate() {
            sum += (j + 1) * (i + 1) * lense.1 as usize;
        }
    }
    println!("{}", sum);
}
