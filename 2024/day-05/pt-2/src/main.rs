use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn reader_from_path(relative_path: &str) -> BufReader<File> {
    let absolute_path = canonicalize(Path::new(relative_path))
        .expect("Invalid file path");
    
    let file = File::open(absolute_path)
        .expect("File failed to open");
        
    BufReader::new(file)
}

struct Pages {
    rules: HashMap<u32, Vec<u32>>,
}

impl Pages {
    fn sort(&self, nums: &mut Vec<u32>) -> bool {
        let mut swapped = false;

        let len = nums.len();
        for i in 0..len {
            for j in 0..len - i - 1 { 
                let first = nums[j];
                let second = nums[j + 1];

                if let Some(lower_vec) = self.rules.get(&first) {
                    if lower_vec.contains(&second) {
                        swapped = true;
                        nums.swap(j,j + 1);
                    }
                } 
            }
        }
        swapped
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Missing input file");
        std::process::exit(1);
    }

    let reader = reader_from_path(&args[1]);

    let mut lines = reader.lines();
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();


    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() { break; } // could be while !line.is_empty() but why delcare line
        let (low, high) = line.split_once('|')
            .expect("Incorectly formmated ordering rule");
        
        let low: u32 = low.parse::<u32>().unwrap();
        let high: u32 = high.parse::<u32>().unwrap();
        
        rules.entry(high).or_insert_with(Vec::new).push(low);
    }

    let pages = Pages { rules, }; 

    let total: u32 = lines
        .filter_map(Result::ok)
        .map(|line| {
            let mut nums: Vec<u32> = line.split(',')
                .filter_map(|n| n.parse::<u32>().ok())
                .collect();

            if pages.sort(&mut nums) {
                nums[nums.len() / 2]
            } else {
                0
            }
        })
        .sum();
    println!("Middle page sum: {}", total);
}

