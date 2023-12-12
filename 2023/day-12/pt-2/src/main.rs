use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};

#[derive(PartialEq, Clone)]
enum Position {
    KnownTrue,
    KnownFalse,
    Unknown,
}

fn reader_from_path(relative_path: &str) -> BufReader<File> {
    let absolute_path = {
        match canonicalize(Path::new(relative_path)){
            Ok(file) => file,
            Err(_) => {
                eprintln!("invalid file path: {}", relative_path);
                std::process::exit(1);
            },
        }
    };

    let file = {
        match File::open(absolute_path){
            Ok(file) => file,
            Err(_) => {
                eprintln!("file failed to open");
                std::process::exit(1);
            }
        }
    };

    BufReader::<File>::new(file)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: {} filepath", args[0]);
        std::process::exit(1);
    }
    
    let reader = reader_from_path(&args[1]);

    let lines: Vec<(Vec<Position>, Vec<u16>, u16, u16)> = reader.lines()
        .filter_map(Result::ok).map( |line| {
        let line: Vec<&str> = line.split_whitespace().collect();
        
        let parsed_num: Vec<u16> = line[1].split(',')
            .map(|x| x.parse::<u16>().unwrap()).collect();

        let mut unknown_count = 0;
        let mut true_count = 0;
        let parsed_str: Vec<Position> = line[0].chars().map( |c| {
            match c {
                '.' => Position::KnownFalse,
                '#' => { true_count += 1; Position::KnownTrue },
                '?' => { unknown_count += 1; Position::Unknown},
                _ => { eprintln!("invalid string input");
                    std::process::exit(1);
                },
            }
        }).collect();

        
        // let mut extended_num: Vec<u16> = Vec::new();
        // let mut extended_str: Vec<Position> = Vec::new();

        // for _ in 0..4{
        //     extended_num.extend(&parsed_num);
        //     extended_str.extend(parsed_str.iter().cloned());
        //     extended_str.push(Position::Unknown);
        // }
        // extended_num.extend(&parsed_num);
        // extended_str.extend(parsed_str.iter().cloned());
    
        // (extended_str, extended_num, unknown_count * 5 + 4, true_count * 5)
         (parsed_str, parsed_num, unknown_count, true_count)


    }).collect();
    
    let mut sum = 0;
    
    for line in lines {

        let needed_ones = line.1.clone().iter().sum::<u16>() - line.3;

        let total_bits = line.2 as u64;

        let min_value = (1u64 << needed_ones) - 1; 
        let max_value = min_value << (total_bits - needed_ones as u64);         

        for combo in min_value..=max_value {
            if combo.count_ones() != needed_ones as u32 {
                continue;
            } 
            let mut combo_index = 0;
            let mut continuous: bool = false;
            let mut line_1 = line.1.clone();
            let mut current_group = line_1.get(0);
            let mut cont_count = 0;
            let mut valid = true;

            for pos in &line.0 {
                if pos == &Position::Unknown {
                    if combo &(1<< combo_index) != 0 {
                        cont_count += 1;
                        if current_group == None || current_group.unwrap() < &cont_count{
                            valid = false;
                            break;
                        }
                        continuous = true;
                    } else {
                    if continuous {
                        if current_group != Some(&cont_count){
                            valid = false;
                            break;
                        }
                        line_1.remove(0);
                        current_group = line_1.get(0);
                        
                        continuous = false;
                        cont_count = 0;
                    }
                }
                combo_index += 1;
            } else if pos == &Position::KnownTrue {
                cont_count += 1;
                
                if current_group == None || current_group.unwrap() < &cont_count{
                    valid = false;
                    break;
                }
                
                continuous = true;
            
            } else {
                if continuous {
                    if current_group != Some(&cont_count){
                        valid = false;    
                        break;
                    }
                        line_1.remove(0);
                        current_group = line_1.get(0);

                        continuous = false;
                        cont_count = 0;
                    }
                }
            }

            if continuous{
                if line_1.get(0) != Some(&cont_count){
                    continue;
                } else {
                    line_1.remove(0);
                }
            }
            if valid && line_1.is_empty() {sum += 1;}
        }
    }
    print!{"\n{}", sum};
}


