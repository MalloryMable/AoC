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
    
    let mut map: Vec<Vec<( PipeType, bool )>> = Vec::new();
    let mut current_index: (usize, usize) = (0, 0);

    for (x, line) in reader.lines().filter_map(Result::ok).enumerate() {
        let mut line_vec: Vec<(PipeType, bool)> = Vec::new();

        for (y, c) in line.chars().enumerate() {
            let pipe_type = match c {
                'S' => {
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
            line_vec.push(( pipe_type, false ));
        }

        map.push(line_vec);
    }
    
    // 
    let mut steps: u32 = 1;

    let mut pipe: Vec<(usize, usize)> = Vec::new();
    // Hardcoded for my inputs feel free to change for yours
    // Can be automated by making sure ending direction is or isn't the same as starting
    pipe.push(current_index);
        
    let mut direction = Direction::South;
    // *This* is what happens when your language doesn't have a do while
    let mut next_step: ((usize, usize), Direction) = ((current_index.0 + 1,
            current_index.1), Direction::South);
    // (min:(x,y), max(x, y))
    let mut min_max: ((usize, usize), (usize, usize)) = (current_index, current_index);
    current_index = next_step.0;
    


    while map[current_index.0][current_index.1].0 != PipeType::Start {
        let (x,y) = current_index;
        pipe.push(current_index);
        let current_pipe = &map[x][y].0;


        steps += 1;
        
        // min
        if x < min_max.0.0 {min_max.0.0 = x;}
        if y < min_max.0.1 {min_max.0.1 = y;}
        // max
        if x > min_max.1.0 {min_max.1.0 = x;}
        if y > min_max.1.1 {min_max.1.1 = y;}

        next_step = current_pipe.get_next(direction, current_index);
        direction = next_step.1;
        current_index = next_step.0;
        map[x][y].1 = true;


    }

    let mut area = 0;

    for x in min_max.0.0..=min_max.1.0 {
        for y in min_max.0.1..=min_max.1.1 {
            let mut count = 0;
            for i in 0..pipe.len() {
                let j = i + 1;
                let (x1, y1) = pipe[i];
                let (x2, y2) = if j == pipe.len() {pipe[0]} else {pipe[j]};

                if (y1 <= y && y2 > y) || (y2 <= y && y1 > y) {
                    let vt = (y as i32 - y1 as i32) as f64 / (y2 as i32 - y1 as i32) as f64;
                    if x < (x1 as f64 + (vt * (x2 as f64 - x1 as f64))) as usize {
                        count += 1;
                    }
                }
            }

            if count % 2 == 1 {
                area += 1;
            }
        }
    }
    println!("Area: {}", area - steps/2 + 1);
}

