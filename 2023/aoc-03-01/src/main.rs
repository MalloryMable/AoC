use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufRead, BufReader};

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

fn get_surounding_indecies (index_pair: &(u8, u8), line_len: &usize) -> (usize, usize) {
    let usize_index_1: usize = index_pair.1 as usize;
    (if index_pair.0 == 0 {0} else {index_pair.0 as usize - 1},
    if usize_index_1 == *line_len {usize_index_1} else {usize_index_1 + 1})
}

fn pass_check (c: char) -> bool {
    c == '*' || c == '@' ||
    c == '%' || c == '$' ||
    c == '-' || c == '/' ||
    c == '+' || c == '+' ||
    c == '=' || c == '&' ||
    c == '#'
}

fn adjacent_check (start: usize,end: usize, line: &str) -> bool {
    if let Some( first_char ) = line.chars().nth(start) {
        if let Some( second_char ) = line.chars().nth(end-1) {
            return pass_check(first_char) || pass_check(second_char);
        }
    }
    return true;
}

fn line_check (substring: &str) -> bool {
    for c in substring.chars(){
        if pass_check(c) {
           return true;
        }
    }
    return false;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} filepath", args[0]);
        std::process::exit(1);
    }

    let mut file_indicies: Vec<Vec<(u8, u8)>> = vec![];
    
    let reader = reader_from_path(&args[1]);
    for line in reader.lines().filter_map(Result::ok) {

        let mut line_indicies: Vec<(u8, u8)> = vec![];
        let mut sequential = false; 
        let mut start: u8 = 0;

        for (c_number, c) in line.char_indices(){
            let is_digit = c.is_digit(10);

            if sequential {
                if !is_digit {
                    sequential = false;
                    line_indicies.push((start, c_number as u8));
                } 
            } else {
                if is_digit {
                    sequential = true;
                    start = c_number as u8;
                }
            }
        }
        // Clean up for trailing digits
        if sequential {
            line_indicies.push((start, line.len() as u8));
        }
        file_indicies.push(line_indicies);
    }

    let mut file_indicies = file_indicies.iter();

    
    let mut sum = 0;
    // I hate that I have to do this, I spent 4 hours finding a better option
    // Saving as a vector takes up more space
    // I could try and learn how seek works, but it's A Lot
    let reader = reader_from_path(&args[1]);
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let mut current_line = { match lines.next() {
            Some(line) => line,
            None => { eprintln!("Second file open error");
                std::process::exit(1)
            },
        }   
    };

    let mut next_line = { match lines.next(){
        Some(line) => line,
        None => std::process::exit(1),
        }
    };

    // Note: Opening logic is different 
    if let Some( current_indicies ) = file_indicies.next() { 
        let line_sum = current_indicies.iter().map(|index_pair| {
     
            let (start, end) = get_surounding_indecies(index_pair, &current_line.len());
                
            if adjacent_check(start, end, &current_line)|| 
                line_check(&next_line[start..end]) {
                return match current_line[index_pair.0 as usize..index_pair.1 as usize]
                    .parse::<u32>() 
                {
                    Ok(num) => num,
                    Err(e) => { 
                        eprintln!("Error parsing number: {}", e);
                        std::process::exit(1);
                    },
                };
            }
            return 0;
        }).fold(0, |x, acc| acc + x); 

        sum += line_sum;
    }

    for line in lines{
        
        let previous_line = current_line;
        current_line = next_line;
        next_line = line;

        if let Some( current_indicies ) = file_indicies.next() { 
            let line_sum = current_indicies.iter().map(|index_pair| {
     
                let (start, end) = get_surounding_indecies(
                        index_pair, &current_line.len()
                    );

                if adjacent_check(start, end, &current_line)|| 
                    line_check(&previous_line[start..end]) ||
                    line_check(&next_line[start..end]) 
                {
                    return match current_line[index_pair.0 as usize..index_pair.1 as usize]
                        .parse::<u32>()
                    {
                        Ok(num) => num,
                        Err(e) => {
                            eprintln!("Error parsing number: {}", e);
                            std::process::exit(1);
                        },
                    };
                }

                return 0; 
            }).fold(0, |x, acc| acc + x); 

            sum += line_sum;
        }
    }

    if let Some( current_indicies ) = file_indicies.next() { 
        let line_sum = current_indicies.iter().map(|index_pair| {
     
            let (start, end) = get_surounding_indecies(index_pair, &next_line.len());
                
            if adjacent_check(start, end-1, &next_line) ||
                line_check(&current_line[start..end]) 
            {
                return match next_line[index_pair.0 as usize..index_pair.1 as usize]
                    .parse::<u32>()
                {
                    Ok(num) => num,
                    Err(e) => { 
                        eprintln!("Error parsing number: {}", e);
                        std::process::exit(1);
                    }
                };
            }
            
            return 0;
        }).fold(0, |x, acc| acc + x); 

        sum += line_sum; 
    }
     
   
    println!("{}", sum);
}

