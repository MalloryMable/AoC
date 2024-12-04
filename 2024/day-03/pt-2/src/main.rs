use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};
use regex::Regex;

fn reader_from_path(relative_path: &str) -> BufReader<File> {
    let absolute_path = canonicalize(Path::new(relative_path))
        .expect("Invalid file path");
    
    let file = File::open(absolute_path)
        .expect("File failed to open");
        
    BufReader::new(file)
}

struct Parser {
    re: Regex,
    parsing: bool,
}

impl Parser{
    fn new() -> Self {
        Self {
            re: Regex::new(r"mul\((\d+),(\d+)\)").unwrap(),
            parsing: true,
        }
    }


    fn parse_segment(&self, input: &str) -> u32 {
        self.re.captures_iter(input)
            .map(|cap| {
                let num1 = cap[1].parse::<u32>().unwrap();
                let num2 = cap[2].parse::<u32>().unwrap();
                num1 * num2
            })
            .sum::<u32>()
    }


    // back to back do() statements are split unecessiarly but my solutions are inellegent
    fn process_line(&mut self, line: &str) -> u32 {
        let mut sum = 0;
        let mut current_pos = 0;

        while let Some(next_do) = line[current_pos..].find("do()") {
            let absolute_do_pos = current_pos + next_do;
        
            // Search in the segment between current position and next "do()"
            if let Some(next_dont) = line[current_pos..absolute_do_pos].find("don't()") {
                if self.parsing {
                    sum += self.parse_segment(&line[current_pos..current_pos + next_dont]);
                }
            } else if self.parsing { // accounts for where the beggining of a line isn't parsed
                sum += self.parse_segment(&line[current_pos..absolute_do_pos]);
            }
            self.parsing = true;
            current_pos = absolute_do_pos + 4; // Move past "do()"
        }

        // Handle the remaining part of the line
        if let Some(next_dont) = line[current_pos..].find("don't()") {
            sum += self.parse_segment(&line[current_pos..current_pos + next_dont]);
            self.parsing = false;
        } else {
            sum += self.parse_segment(&line[current_pos..]);
        }

        sum
    }

}

fn main() {
    let input_file = env::args()
        .nth(1)
        .unwrap_or_else( || { 
            eprintln!("Missing input file");
            std::process::exit(1); 
        });

    let reader = reader_from_path(&input_file);
    let mut parser = Parser::new();


    println!("Sum: {}", reader.lines()
        .filter_map(Result::ok)
        .map(|line| parser.process_line( &line))
        .sum::<u32>()
    );
}

