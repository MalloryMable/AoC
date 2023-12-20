use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::collections::HashMap;

enum Var {
    X,
    M,
    A,
    S,
}

struct Instruct {
    var: Var,
    gt: bool, // Short for greater than
    value: u16,
    result: (Option<bool>, Option<String>),
}

impl Instruct {
    fn process(&self, input: (u16, u16, u16, u16)) -> (Option<bool>, Option<String>) {
        let var = match self.var {
            Var::X => input.0,
            Var::M => input.1,
            Var::A => input.2,
            Var::S => input.3,
        };

        let rtn_result = if self.gt {var > self.value} else {var < self.value};
        if rtn_result {return self.result.clone();}
        (None, None)
    }
}

fn run_instruct (name: &String, 
    map: &HashMap<String, (Vec<Instruct>, (Option<bool>, Option<String>))>,
    input: (u16, u16, u16, u16)) -> bool {
    let instructions = map.get(name).unwrap();
    for (_, instruct) in instructions.0.iter().enumerate() {
        let result = instruct.process(input);
        if let Some(x) = result.0 {
            return x;
        };
        if let Some(name) = result.1 {
            return run_instruct(&name, map, input);
        }
    }
    if let Some(x) = instructions.1.0 {
        return x;
    };
    if let Some(name) = &instructions.1.1 {
        return run_instruct(&name, map, input);
    }
    // unreachable
    false

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

fn parse_return(input:&str) -> (Option<bool>, Option<String>) {
    if input == "A" {
        (Some(true), None)
    } else if input == "R" {
        (Some(false), None)
    } else {
        (None, Some(String::from(input)))
    }
}

fn parse_cap(input:&str) -> u16 {
    input.parse::<u16>().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} filepath", args[0]);
        std::process::exit(1);
    }
    
    let re_instruct = Regex::new(r"^(\w+)\{(.*),(\w+)\}$").unwrap();
    let re_var = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    
    let mut read_instruct = true;

    let mut map: HashMap<String, (Vec<Instruct>, (Option<bool>, Option<String>))> = 
        HashMap::new();
    let mut count: u32 = 0;

    let reader = reader_from_path(&args[1]);

    for line in reader.lines().filter_map(Result::ok){
        if line == "" {
            read_instruct = false;
            continue;
        }
        if read_instruct {
            let cap = re_instruct.captures(&line).unwrap();
            let structs: Vec<Instruct> = cap[2].split(',').map( |instruct|
             {
                let mut chared = instruct[..2].chars();
                let split: Vec<&str> = instruct[2..].split(':').collect();
                Instruct{
                    var: match chared.next().unwrap() {
                        'x' => Var::X,
                        'm' => Var::M,
                        'a' => Var::A,
                        's' => Var::S,
                        _ => {eprintln!("Invalid variable in instructions");
                            std::process::exit(1);
                        }
                    },

                    gt: if chared.next().unwrap() == '>' {true} else {false},
                    value: parse_cap(split[0]),
                    result: parse_return(split[1]),
                }
            }).collect();
            
            map.insert(String::from(&cap[1]), (structs, parse_return(&cap[3])));
        } else {
            let caps = re_var.captures(&line).unwrap();
            
            let input = (parse_cap(&caps[1]), parse_cap(&caps[2]), 
                parse_cap(&caps[3]), parse_cap(&caps[4]));
            if run_instruct(&String::from("in"), &map, input) {
                count+= (input.0 + input.1 + input.2 + input.3) as u32;
            };
            
        }
    }
    println!("Aproved count: {}", count);
}

