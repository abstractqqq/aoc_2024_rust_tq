use crate::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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
    trail: HashSet<(usize, usize, Direction)>,
    oob: bool,
    in_loop: bool,
}

impl Guard {

    fn new(x: usize, y:usize) -> Self {
        let mut hs = HashSet::new();
        hs.insert((x, y, Direction::new()));
        Guard {
            x: x,
            y: y,
            direction: Direction::new(),
            trail: hs,
            oob: false,
            in_loop: false,
        }
    }

    fn get_direction(&self) -> Direction {
        self.direction.clone()
    }

    fn turn(&mut self) {
        self.direction = self.direction.turn();
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
            // reachable.

            // Stuck in a loop is equivalent to being in the same position
            // with the same direction
            if self.trail.contains(&(x, y, self.get_direction())) {
                self.in_loop = true;
            }

            // Update coordinates and trail
            self.x = x;
            self.y = y;
            self.trail.insert((x, y, self.get_direction()));
        } else {
            self.turn();
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
            let only_positions_hashset:HashSet<(usize, usize)> = 
                HashSet::from_iter(
                    guard.trail.iter().map(|(x, y, _)| (*x, *y))
                );

            break only_positions_hashset.len()
        }
    };

    Ok(distinct_places)
}

fn check_loop_or_oob(map: &[[bool;130];130], mut guard: Guard) -> bool {
    loop {
        guard.check_and_move(&map);
        if guard.oob {
            break false
        }
        if guard.in_loop {
            break true
        }
    }
}

pub fn d6_part2_solution(input_path: &str) -> Result<usize, Error> {
    // True if walkable
    let (mut map, starting_pos) = load_map(input_path)?;
    let (x, y) = starting_pos;

    // Could there be more reduction we can do? E.g. no need to check lots of walkable positions?
    let mut will_loop_count = 0usize; 
    for i in 0..130 {
        for j in 0..130 {
            // Walkable and not initial position
            if map[i][j] && (i != x || j != y) {
                map[i][j] = false;
                will_loop_count += check_loop_or_oob(&map, Guard::new(x, y)) as usize;
                map[i][j] = true; // return it back
            } else {
                continue; // skip existing obstacles
            }
        }
    }

    Ok(will_loop_count)
}