use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
  // hardcoded in because I'm lazy
  // NOTE: Implement enviroment variables such as ~ 
  let path: &Path = Path::new("/home/mallory/input.txt");
  if let Ok(file) = File::open(&path){
    let buffer  = BufReader::<File>::new(file);

    // While often '.?*' is useful for anchors opposite to the direction they produce
    // weird.
    // got this working and it output backwards. Not sure why flipping works but it did?
    
    // seperate regex means only 1 true case can satisify both
    let front_re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine).?*$").unwrap();
    let back_re = Regex::new(r"^.?*(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let sum: u16 = buffer.lines()
      .filter_map(Result::ok)
      .map(|line| {
        if let Some(front_cap) = front_re.captures(&line) {
            if let Some(back_cap) = back_re.captures(&line) {  
                if let Ok(num) = format!("{}{}", word_to_number(&front_cap[1]), word_to_number(&back_cap[1])).parse::<u16>() {
                    return num;
                }
            }        
        }
        // Error return:
        0
      })
      .fold(0, |acc, x| acc + x);
      println!("The sum of cordinates is: {}", sum)
    } else {
      eprintln!("This is not a valid file");
    }
  }

// Returned as string because they will be concatanated in a moment and length 1
fn word_to_number(word: &str) -> char{
  match word {
    "one" => '1',
    "two" => '2',
    "three" => '3',
    "four" => '4',
    "five" => '5',
    "six" => '6',
    "seven" => '7',
    "eight" => '8',
    "nine" => '9',
    _ => word.chars().next().unwrap_or('0'), // If the word does not match any number
    }
}
