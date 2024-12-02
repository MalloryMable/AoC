use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};


fn reader_from_path(relative_path: &str) -> BufReader<File> {
    // Could change to ? syntax but for now I like that this fails quickly
    let absolute_path = canonicalize(Path::new(relative_path))
        .expect("Invalid file path");
    
    let file = File::open(absolute_path)
        .expect("File failed to open");
        
    BufReader::new(file)
}


fn check_sequence(nums: &[i32]) -> bool {
    if nums.len() < 2 {
        return false;  
    }
    
    // Check direction using first two numbers
    let is_ascending = nums[1] > nums[0];
    
    nums.windows(2)
        .all(|pair| {
            let diff = if is_ascending {
                pair[1] - pair[0]
            } else {
                pair[0] - pair[1]
            };
            
            // ensures our step sizes is 1-3 in whatever direction is found in our initial check
            diff >= 1 && diff <= 3
            /* While an or statement feels more readable it adds lines or code or an awkward not
             * and this fales just as quickly. <= and >= are a bit awkward, but x > 0 && x < 4 is
             * less clear at a glance */ 
        })
}

fn process_line(line: String) -> u32 {
    
    let nums: Vec<i32> = line.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
        
    if check_sequence(&nums) {
        1
    } else {
        0
    }
}

fn main() {
    let input_file = env::args()
        .nth(1)
        // could ok_or_else and propogate an error but as process is killed type change is avoided
        .unwrap_or_else( || { 
            eprintln!("Missing input file");
            std::process::exit(1); 
        });

    let reader = reader_from_path(&input_file);
    
    println!("Passed levels: {}", reader
        .lines()
        .filter_map(Result::ok)
        .map(process_line)
        .sum::<u32>()
    );
}

