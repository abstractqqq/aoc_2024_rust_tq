use crate::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::collections::HashSet;

enum Direction {
    UP,
    LEFT,
    RIGHT, 
    DOWN
}

impl Direction {
    fn new() -> Self {
        Self::UP
    }

    fn turn(&self) -> Self {
        match self {
            Self::UP => Self::RIGHT,
            Self::LEFT => Self::UP,
            Self::RIGHT => Self::DOWN,
            Self::DOWN => Self::LEFT,
        }
    }
}


pub struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
    trail: HashSet<(usize, usize)>,
    oob: bool
}

impl Guard {

    fn new(x: usize, y:usize) -> Self {
        let mut hs = HashSet::new();
        hs.insert((x, y));
        Guard {
            x: x,
            y: y,
            direction: Direction::new(),
            trail: hs,
            oob: false
        }
    }

    fn check_and_move(&mut self, map: &[[bool; 130];130]) {

        if self.oob {
            return
        }

        // If either x or y is >= 130, it means we move out of bounds.
        let next_pos: (usize, usize) = match self.direction {
            Direction::UP => {
                if self.x >= 1 {
                    (self.x - 1, self.y)
                } else {
                    (9999, 9999) // oob
                }
            },
            Direction::LEFT => {
                if self.y >= 1 {
                    (self.x, self.y - 1)
                } else {
                    (9999, 9999)
                }
            },
            Direction::RIGHT => {
                if self.y + 1 >= 130 {
                    (9999, 9999)
                } else {
                    (self.x, self.y + 1)
                }

            },
            Direction::DOWN => {
                if self.x + 1 >= 130 {
                    (9999, 9999)
                } else {
                    (self.x + 1, self.y)
                }
            },
        };

        let (x, y) = next_pos;
        if x >= 130 || y >= 130 {
            self.oob = true;
            return
        }

        if map[x][y] { // is obstacle
            // reachable. Update coordinates and trail
            self.x = x;
            self.y = y;
            self.trail.insert((x, y));
        } else {
            self.direction = self.direction.turn();
        }
    }
}


/// Returns the (map, starting position of guard) 
fn load_map(input_path: &str) -> Result<([[bool;130];130], (usize, usize)), Error> {

    let f = File::open(input_path).map_err(|e| Error::IOError(e))?;
    let reader = BufReader::new(f);
    let mut x = 0usize;
    let mut y = 0usize;
    let mut walkable = [[true; 130]; 130];
    for (i, l) in reader.lines().enumerate() {
        if let Ok(line) = l {
            for (j, c) in line.chars().enumerate() {
                walkable[i][j] = c != '#';
                if c == '^' {
                    x = i;
                    y = j;
                }
            }
        } else {
            return Err(Error::Other("Input file is not correct.".into())) 
        }

    }
    //println!("{:?}", walkable);
    // println!("{:?}", (x,y));
    Ok((walkable, (x, y)))
}

pub fn d6_part1_solution(input_path: &str) -> Result<usize, Error> {
    // True if walkable
    let (map, starting_pos) = load_map(input_path)?;
    let (x, y) = starting_pos;
    let mut guard = Guard::new(x, y);
    
    let distinct_places = loop {
        guard.check_and_move(&map);
        if guard.oob {
            break guard.trail.len()
        }
    };

    Ok(distinct_places)
}

pub fn d6_part2_solution(input_path: &str) -> Result<usize, Error> {
    todo!()
}