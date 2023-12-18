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

enum Direction {
    Down,
    Up,
    Right,
    Left,
}

#[derive(Debug, Clone)]
enum Type {
    Empty,
    VertSplit,
    HoriSplit,
    RightTurn,
    LeftTurn,
}



fn traverse_map((y, x): (usize, usize), direction: Direction, 
    map: &mut Vec<Vec<(Type, bool)>>, seen_split: &mut Vec<(usize, usize)>) {
    
    match direction {
        Direction::Down => {
            for j in (y + 1)..map.len() {
                map[j][x].1 = true;
                match map[j][x].0 {
                    Type::RightTurn => return traverse_map((j, x), Direction::Left,
                        map, seen_split),
                    Type::LeftTurn => return traverse_map((j, x), Direction::Right,
                        map, seen_split),
                    Type::HoriSplit => {
                        if seen_split.contains(&(j, x)) {return;};
                        seen_split.push((j,x));
                        traverse_map((j, x), Direction::Left, map, seen_split);
                        return traverse_map((j, x), Direction::Right, map, seen_split);
                    },
                    _ => {},

                }
            }
        },
        Direction::Up => {
            for j in (0..y).rev() {
                map[j][x].1 = true;
                match map[j][x].0 {
                    Type::RightTurn => return traverse_map((j, x), Direction::Right, 
                        map, seen_split),
                    Type::LeftTurn => return traverse_map((j, x), Direction::Left, 
                        map, seen_split),
                    Type::HoriSplit => {
                        if seen_split.contains(&(j,x)) {return;};
                        seen_split.push((j, x));
                        traverse_map((j, x), Direction::Left, map, seen_split);
                        return traverse_map((j, x), Direction::Right, map, seen_split);
                    },
                    _ => {},

                }
            }
        },
        Direction::Right => {
            for i in (x + 1)..map[y].len() {
                map[y][i].1 = true;
                match map[y][i].0 {
                    Type::RightTurn => return traverse_map((y, i), Direction::Up, 
                        map, seen_split),
                    Type::LeftTurn => return traverse_map((y, i), Direction::Down, 
                        map, seen_split),
                    Type::VertSplit => {
                        if seen_split.contains(&(y, i)) {return;};
                        seen_split.push((y, i));
                        traverse_map((y, i), Direction::Up, map, seen_split);
                        return traverse_map((y, i), Direction::Down, map, seen_split);
                    },
                    _ => {},

                }
            }
        },
        Direction::Left => {
            for i in (0..x).rev() {
                map[y][i].1 = true;
                match map[y][i].0 {
                    Type::RightTurn => return traverse_map((y, i), Direction::Down, 
                        map, seen_split),
                    Type::LeftTurn => return traverse_map((y, i), Direction::Up, 
                        map, seen_split),
                    Type::VertSplit => {
                        if seen_split.contains(&(y, i)) {return;};
                        seen_split.push((y, i));
                        traverse_map((y, i), Direction::Up, map, seen_split);
                        return traverse_map((y, i), Direction::Down, map, seen_split);
                    },
                    _ => {},

                }
            }
        },
    }    
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} filepath", args[0]);
        std::process::exit(1);
    }
    
    let reader = reader_from_path(&args[1]);

    let map: Vec<Vec<(Type, bool)>> = reader.lines().filter_map(Result::ok).map(|line| {
        line.chars().map(|c| {
            match c {
                '.' => (Type::Empty, false),
                '|' => (Type::VertSplit, false),
                '-' => (Type::HoriSplit, false),
                '/' => (Type::RightTurn, false),
                '\\' => (Type::LeftTurn, false),
                _ => { eprintln!("Invalid map char");
                    std::process::exit(1);
                }
            }
        }).collect()
    }).collect();


    let mut max = 0;
    for i in 0..map[0].len(){
        let mut map_clone = map.clone();
        
        map_clone[0][i].1 = true;
        let mut seen_split = Vec::new(); 

         match map_clone[0][i].0 {
            Type::RightTurn => traverse_map((0, i), Direction::Left, 
                &mut map_clone, &mut seen_split),
            Type::LeftTurn => traverse_map((0, i), Direction::Right, 
                &mut map_clone, &mut seen_split),
            Type::HoriSplit => {
                seen_split.push((0, i));
                traverse_map((0, i), Direction::Left, &mut map_clone, &mut seen_split);
                traverse_map((0, i), Direction::Right, &mut map_clone, &mut seen_split);
            },
            _ => traverse_map((0, i), Direction::Down, &mut map_clone, &mut seen_split),
        }

        let mut sum = 0;
        for line in map_clone {
            for c in line {
                if c.1 {sum +=1;};
            }
        }
        if sum > max {max = sum};

        // New cloned ma
        let mut map_clone = map.clone();
        let end = map.len() - 1;
        map_clone[end][i].1 = true;
        let mut seen_split = Vec::new(); 

         match map_clone[end][i].0 {
            Type::RightTurn => traverse_map((end, i), Direction::Right, 
                &mut map_clone, &mut seen_split),
            Type::LeftTurn => traverse_map((end, i), Direction::Left, 
                &mut map_clone, &mut seen_split),
            Type::HoriSplit => {
                seen_split.push((end, i));
                traverse_map((end, i), Direction::Left, &mut map_clone, &mut seen_split);
                traverse_map((end, i), Direction::Right, &mut map_clone, &mut seen_split);
            },
            _ => traverse_map((end, i), Direction::Up, &mut map_clone, &mut seen_split),
        }

        let mut sum = 0;
        for line in map_clone {
            for c in line {
                if c.1 {sum +=1;};
            }
        }
        if sum > max {max = sum};

    }

    for j in 0..map[0].len(){
        
        let mut map_clone = map.clone();
        
        map_clone[j][0].1 = true;
        let mut seen_split = Vec::new(); 

         match map_clone[j][0].0 {
            Type::RightTurn => traverse_map((j, 0), Direction::Up, 
                &mut map_clone, &mut seen_split),
            Type::LeftTurn => traverse_map((j, 0), Direction::Down, 
                &mut map_clone, &mut seen_split),
            Type::HoriSplit => {
                seen_split.push((j, 0));
                traverse_map((j, 0), Direction::Down, &mut map_clone, &mut seen_split);
                traverse_map((j, 0), Direction::Up, &mut map_clone, &mut seen_split);
            },
            _ => traverse_map((j, 0), Direction::Right, &mut map_clone, &mut seen_split),
        }

        let mut sum = 0;
        for line in map_clone {
            for c in line {
                if c.1 {sum +=1;};
            }
        }
        if sum > max {max = sum};

        let mut map_clone = map.clone();
        let end = map[0].len() - 1;
        map_clone[j][end].1 = true;
        let mut seen_split = Vec::new(); 

         match map_clone[j][end].0 {
            Type::RightTurn => traverse_map((j, end), Direction::Up, 
                &mut map_clone, &mut seen_split),
            Type::LeftTurn => traverse_map((j, end), Direction::Down, 
                &mut map_clone, &mut seen_split),
            Type::HoriSplit => {
                seen_split.push((j, end));
                traverse_map((j, end), Direction::Up, &mut map_clone, &mut seen_split);
                traverse_map((j, end), Direction::Down, &mut map_clone, &mut seen_split);
            },
            _ => traverse_map((j, end), Direction::Left, &mut map_clone, &mut seen_split),
        }

        let mut sum = 0;
        for line in map_clone {
            for c in line {
                if c.1 {sum +=1;};
            }
        }
        if sum > max {max = sum};

    }
    println!("{max}");
}
