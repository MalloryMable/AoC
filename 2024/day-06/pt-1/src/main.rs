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

#[derive(PartialEq, Clone, Copy)]
enum MapTile {
    Unvisited,
    Visited,
    Blocked,
}

struct Guard<'a> {
    map:&'a mut Vec<Vec<MapTile>>,
    pos: (usize, usize),
    direction: [(i32, i32); 4],
    facing: usize,
    bound: (usize, usize),
    visitied: u32,
}

impl<'a> Guard<'a> {
    fn new (map: &'a mut Vec<Vec<MapTile>>, pos: (usize, usize)) -> Self {
        let bound = (map.len() - 1, map[0].len() - 1);

        Self {
            map,
            pos,
            // North(0,-1), East(1,0), South(0,1), West(-1,0)
            direction: [(0, -1), (1, 0), (0, 1), (-1, 0)],
            facing: 0,
            // Lower bound is aways 0. -1 for similarity in walk
            bound,
            visitied: 1,
        }
    }
    
    fn turn(&mut self) {
        self.facing = (self.facing + 1) % 4;
    }

    fn look_ahead(&self) -> Option<(usize, usize)> {
        let (dx, dy) = self.direction[self.facing];
        let new_x = self.pos.0 as i32 + dx;
        let new_y = self.pos.1 as i32 + dy;
        
        if new_x < 0 || new_y < 0 ||
            new_x > self.bound.0 as i32 || 
            new_y > self.bound.1 as i32 {
                return None;
        }

        Some((new_x as usize, new_y as usize))
    }

    fn walk(&mut self) -> bool {
        loop {
            let Some(pos) = self.look_ahead() else {
                return false;
            };

            match self.map[pos.1][pos.0] {
                MapTile::Blocked => {
                    self.turn();
                    continue;
                },
                MapTile::Unvisited => {
                    self.map[pos.1][pos.0] = MapTile::Visited;
                    self.pos = pos;
                    self.visitied += 1;
                    return true;
                },
                MapTile::Visited => {
                    self.pos = pos;
                    return true;
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Missing input file");
        std::process::exit(1);
    }

    // rather than declaring this here can I make this work with lifetimes?
    let mut start_pos: (usize, usize) = (0, 0);

    let reader = reader_from_path(&args[1]);
    let mut map: Vec<Vec<MapTile>> = reader.lines()
        .enumerate()
        .filter_map(|(y, line)| line.ok().map(|l| (y,l))) 
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                match char {
                    '.' => MapTile::Unvisited,
                    '#' => MapTile::Blocked,
                    '^' => {
                        start_pos = (x, y);
                        MapTile::Visited
                    },
                    _ => {
                        eprintln!("Invalid map marker found at:({x}, {y})");
                        std::process::exit(1);
                    }
                }
            })
            .collect()
        })
        .collect();

    let mut guard: Guard = Guard::new(&mut map, start_pos);

    while guard.walk() {}
    
    guard.print();

    // One more is added as the guard leaves
    println!("Positions Visted: {}",  guard.visitied);
}

