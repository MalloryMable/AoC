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
    c == '*' }

fn adjacent_check (start: usize,end: usize, line: &str) -> Option<usize> {
    if let Some( first_char ) = line.chars().nth(start) {
        if let Some( second_char ) = line.chars().nth(end-1) {
            if pass_check(first_char){
                return Some( start ); //return 0;
            } else if pass_check(second_char) {
                return Some( end - 1 ); // return end - start
            }
        }
    }
    return None;
}

fn line_check (substring: &str) -> Option<usize> {
    for (index, c) in substring.chars().enumerate() {
        if pass_check(c) {
           return Some( index );
        }
    }
    return None;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} filepath", args[0]);
        std::process::exit(1);
    }

    let mut file_indicies: Vec<Vec<(u8, u8)>> = vec![];
   
    // Very Silly Way to Do This
    let mut line_count: u8  = 0;

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
        // We don't enumerate because we have to update line count each time anway
        line_count += 1;
    }
    line_count -= 1;

    let mut file_indicies = file_indicies.iter();

    
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

    let mut cogs: Vec<((u8, u8), u32)> = Vec::new();
    
    // NOTE: Observed only 1 cog per number so we can safely only add the first cog 

    // NOTE: Opening logic is different 
    if let Some( current_indicies ) = file_indicies.next() { 
        for index_pair in current_indicies.iter() {
     
            let (start, end) = get_surounding_indecies(index_pair, &current_line.len());
                
            if let Some(index) = adjacent_check(start, end, &current_line){
                if let Ok(num) = 
                    current_line[index_pair.0 as usize..index_pair.1 as usize]
                        .parse::<u32>()
                {
                    cogs.push(((0, index as u8), num )); 
                } 
 
            }

            if let Some(index) = line_check(&next_line[start..end]) {
                if let Ok(num) = 
                    current_line[index_pair.0 as usize..index_pair.1 as usize]
                        .parse::<u32>() 
                {
                    let cog_index: u8 = (start + index) as u8;
                    cogs.push( ((1, cog_index), num) );
                }
            }
        }
    }

    for (line_index, line) in lines.enumerate(){
        let line_index = line_index as u8 + 1;
        let previous_line = current_line;
        current_line = next_line;
        next_line = line;

        if let Some( current_indicies ) = file_indicies.next() { 
            for index_pair in current_indicies.iter() {
                let (start, end) = get_surounding_indecies(
                        index_pair, &current_line.len()
                    );

                if let Some(index) = adjacent_check(start, end, &current_line){
                    if let Ok(num) = 
                        current_line[index_pair.0 as usize..index_pair.1 as usize]
                            .parse::<u32>()
                    {
                        cogs.push(((line_index, index as u8), num )); 
                    } 
                }

                if let Some(index) = line_check(&previous_line[start..end]) {
                    if let Ok(num) = 
                        current_line[index_pair.0 as usize..index_pair.1 as usize]
                            .parse::<u32>() 
                    {
                        let cog_index: u8 = (start + index) as u8;
                        cogs.push(((line_index - 1, cog_index), num));
                    }
                }

                if let Some(index) = line_check(&next_line[start..end]) {
                    if let Ok(num) = 
                        current_line[index_pair.0 as usize..index_pair.1 as usize]
                            .parse::<u32>() 
                    {
                        let cog_index: u8 = (start + index) as u8;
                        cogs.push(( (line_index + 1, cog_index), num ));
                    }
                } 
            }
        }
    }

    if let Some( current_indicies ) = file_indicies.next() { 
        for index_pair in current_indicies.iter() {
            
            let (start, end) = get_surounding_indecies(index_pair, &next_line.len());
                
            if let Some(index) = adjacent_check(start, end, &next_line){
                if let Ok(num) = 
                    next_line[index_pair.0 as usize..index_pair.1 as usize]
                        .parse::<u32>()
                {
                    cogs.push(((line_count, index as u8), num )); 
                } 
 
            }

            if let Some(index) = line_check(&current_line[start..end]) {
                if let Ok(num) = 
                    next_line[index_pair.0 as usize..index_pair.1 as usize]
                        .parse::<u32>() 
                {
                    let cog_index: u8 = (start + index) as u8;
                    cogs.push(( (line_count - 1, cog_index), num) );
                }
            }   
        }
    }

    // O(n^2) twice. All gear scores calculated twice. Bad program
    // I will *never* do an adjacency check on a cog.
    let mut sum: u32 = 0;
    for (cog, num) in &cogs{
        for(inner_cog, inner_num) in &cogs {
            if cog == inner_cog && inner_num != num {
                sum += num * inner_num;
            }
        }
    }
    println!("Final sum: {}", sum / 2);
}

