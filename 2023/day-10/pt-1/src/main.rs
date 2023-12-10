use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};

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

#[derive(PartialEq)]
enum PipeType {
    Start,
    Vertical,
    Horiziontal,
    LBend,
    JBend,
    SevBend,
    FBend,
    Ground,
}

impl PipeType {
    fn get_next(&self, prev_direction: Direction, (x, y): (usize, usize)) 
        -> ((usize,usize), Direction)
    {
        match self{
            PipeType::Vertical => match prev_direction {
                Direction::North => ((x - 1, y), Direction::North),
                Direction::South => ((x + 1, y), Direction::South),
                _ => {
                   eprintln!("Invalid starting direction");
                   std::process::exit(1);
                }
            },
            PipeType::Horiziontal => match prev_direction {
                Direction::East => ((x,  y + 1), Direction::East),
                Direction::West => ((x, y - 1), Direction::West),
                _ => {
                    eprintln!("Invalid starting direction");
                    std::process::exit(1);
                }
            },
            PipeType::LBend => match prev_direction {
                Direction::South => ((x, y + 1), Direction::East),
                Direction::West => ((x - 1, y), Direction::North),
                _ => {
                    eprintln!("Invalid starting direction");
                    std::process::exit(1);
                }
            },
            PipeType::JBend => match prev_direction {
                Direction::South => ((x, y - 1), Direction::West),
                Direction::East => ((x - 1, y), Direction::North),
                _ => {
                   eprintln!("Invalid starting direction");
                   std::process::exit(1);
                }
            },
            PipeType::SevBend => match prev_direction {
                Direction::North => ((x, y - 1), Direction::West),
                Direction::East => ((x + 1, y), Direction::South),
               _ => {
                   eprintln!("Invalid starting direction");
                   std::process::exit(1);
               }
            },
            PipeType::FBend => match prev_direction {
                Direction::North => ((x, y + 1), Direction::East),
                Direction::West => ((x + 1, y), Direction::South),
                _ => {
                   eprintln!("Invalid starting direction");
                   std::process::exit(1);
                }
            },
            _ => {
                eprintln!("Invalid location");
                std::process::exit(1);
            }
        }
        


    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} filepath", args[0]);
        std::process::exit(1);
    }
    
    let reader = reader_from_path(&args[1]);
    
    let mut map: Vec<Vec<PipeType>> = Vec::new();
    let mut current_index: (usize, usize) = (0, 0);

    for (x, line) in reader.lines().filter_map(Result::ok).enumerate() {
        let mut line_vec: Vec<PipeType> = Vec::new();

        for (y, c) in line.chars().enumerate() {
            let pipe_type = match c {
                'S' => {
                    println!("Start: {}, {}", x, y);
                    current_index = (x, y);
                    PipeType::Start
                },
                '|' => PipeType::Vertical,
                '-' => PipeType::Horiziontal,
                'L' => PipeType::LBend,
                'J' => PipeType::JBend,
                '7' => PipeType::SevBend,
                'F' => PipeType::FBend,
                '.' => PipeType::Ground,
                _ => {
                    eprintln!("Invalid pipe character: {}", c);
                    std::process::exit(1);
                }
            };
            line_vec.push(pipe_type);
        }

        map.push(line_vec);
    }
    
    // 
    let mut steps: u32 = 1;
    let mut direction = Direction::South;
    // *This* is what happens when your language doesn't have a do while
    let mut next_step: ((usize, usize), Direction) = ((current_index.0 + 1,
            current_index.1), Direction::South);
    current_index = next_step.0;

    while map[current_index.0][current_index.1] != PipeType::Start {
        let current_pipe = &map[current_index.0][current_index.1];
        steps += 1;
        next_step = current_pipe.get_next(direction, current_index);
        direction = next_step.1;
        current_index = next_step.0;
    }
    println!("Step count: {}", steps / 2);
}

