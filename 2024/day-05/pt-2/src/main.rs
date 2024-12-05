use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};

fn reader_from_path(relative_path: &str) -> BufReader<File> {
    let absolute_path = canonicalize(Path::new(relative_path))
        .expect("Invalid file path");
    
    let file = File::open(absolute_path)
        .expect("File failed to open");
        
    BufReader::new(file)
}

struct Pages {
    rules: Vec<(u32, u32)>,
    swaps: u32, 
}

impl Pages {
   fn sort(&mut self, nums: &mut Vec<u32>) -> bool {
       for (first, second) in &self.rules {
           if let Some(pos) = nums.iter().position(|&x| x == *second) {
               if let Some(swap_pos) = nums[pos+1..].iter().position(|&x| x == *first) {
                   self.swaps += 1;
                   nums.swap(pos, swap_pos + pos + 1);
                   self.sort(nums);
                   return true;
               }
           }
       }
       false
   }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Missing input file");
        std::process::exit(1);
    }

    let reader = reader_from_path(&args[1]);

    let mut rules = Vec::new();
    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() { break; } // could be while !line.is_empty() but why delcare line
        let (a, b) = line.split_once('|')
            .expect("Incorectly formmated ordering rule");
        
        rules.push((
            a.trim().parse::<u32>().unwrap(),
            b.trim().parse::<u32>().unwrap()
        ))
    }

    let mut pages = Pages { rules, swaps: 0}; 

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
    println!("Middle page sum: {}. Swaps: {}", total, pages.swaps);
}

