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

fn is_valid_sequence(nums: &[i32]) -> bool {
    if nums.len() < 2 {
        return false;
    }
    
    // Check direction using first two numbers
    let is_ascending = nums[1] > nums[0];
    
    nums.windows(2).all(|pair| {
        let diff = if is_ascending {
            pair[1] - pair[0]
        } else {
            pair[0] - pair[1]
        };
        diff >= 1 && diff <= 3
    })
}

fn check_sequence(nums: &[i32]) -> bool {
    // First check if sequence is valid without removal
    if is_valid_sequence(nums) {
        return true;
    }
    
    // Try removing each number and check if any resulting sequence is valid
    let mut modified = Vec::with_capacity(nums.len() - 1); // declares a single vector
    
    (0..nums.len()).any(|skip_idx| {
        modified.clear(); // clears vectors
        modified.extend_from_slice(&nums[..skip_idx]); // appends start slice to empty vector
        modified.extend_from_slice(&nums[skip_idx + 1..]); // appends end slice to vector
        is_valid_sequence(&modified) // checks moddified vector
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

