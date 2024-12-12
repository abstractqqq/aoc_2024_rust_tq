use crate::error::Error;
use std::fs::File;
use std::io::Read;

enum Direction<const N: usize> {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight
}

impl <const N: usize> Direction<N> {

    fn step(&self, i:usize, j:usize) -> Option<(usize, usize)> {
        match self {
            Direction::TopLeft => {
                if i >= 1 && j >= 1 {
                    Some((i - 1, j - 1))
                } else {
                    None
                }
            },
            Direction::Top => {
                if i >= 1 {
                    Some((i - 1, j))
                } else {
                    None
                }
            },
            Direction::TopRight => {
                if i >= 1 && j + 1 < N {
                    Some((i - 1, j + 1))
                } else {
                    None
                }
            },
            Direction::Left => {
                if j >= 1 {
                    Some((i, j - 1))
                } else {
                    None
                }
            },
            Direction::Right => {
                if j + 1 < N {
                    Some((i, j + 1))
                } else {
                    None
                }
            },
            Direction::BottomLeft => {
                if i + 1 < N && j >= 1 {
                    Some((i + 1, j - 1))
                } else {
                    None
                }
            },
            Direction::Bottom => {
                if i + 1 < N {
                    Some((i + 1, j))
                } else {
                    None
                }
            },
            Direction::BottomRight => {
                if i + 1 < N && j + 1 < N {
                    Some((i + 1, j + 1))
                } else {
                    None
                }
            },
        }


    }


}

#[derive(Clone, Copy, PartialEq)]
enum XMAS {
    X, 
    M,
    A,
    S,
    None
}

impl From<char> for XMAS {
    fn from(value: char) -> Self {
        match value {
            'X' => Self::X,
            'M' => Self::M,
            'A' => Self::A,
            'S' => Self::S,
            _ => Self::None
        }
    }
}

impl XMAS {

    fn next(&self) -> Self {
        match self {
            XMAS::X => Self::M,
            XMAS::M => Self::A,
            XMAS::A => Self::S,
            _ => Self::None 
        }
    }

    fn prev(&self) -> Self {
        match self {
            XMAS::M => Self::X,
            XMAS::A => Self::M,
            XMAS::S => Self::A,
            _ => Self::None 
        }
    }

    fn is_final(&self) -> bool {
        match self {
            XMAS::S => true,
            _ => false
        }
    }
}



// I know the map is size 140x140
fn load_map(data_string: String) -> [[XMAS; 140]; 140] {
    let mut data_map: [[XMAS; 140]; 140] = [[XMAS::None; 140]; 140];
    for (i, line) in data_string.lines().enumerate().take(140) {
        for (j, char) in line.chars().enumerate().take(140) {
            data_map[i][j] = char.into();
        }        
    }
    data_map
}

fn get_target_coords(map: &[[XMAS; 140]; 140], target: XMAS) -> Vec<(usize, usize, XMAS)> {
    let mut matches = Vec::new();
    for i in 0..140 {
        for j in 0..140 {
            if map[i][j] == target {
                matches.push((i, j, target));
            }
        }
    }
    matches
}

/// Given the center (i, j), go through each of its 8 directions to find XMAS
fn check_xmas_in_direction(
    map:&[[XMAS;140];140], 
    i:usize, 
    j:usize,
    direction: Direction<140>
) -> bool {

    let mut start = map[i][j];
    let mut x = i;
    let mut y = j;
    for _ in 0..3 { // Move 3 more steps, because we start with X
        let next = start.next();
        if let Some((u, v)) = direction.step(x, y) {
            if map[u][v] == next {
                start = next;
                x = u;
                y = v;
            } else {
                return false
            }
        } else {
            return false
        }
    }

    start.is_final()
}

/// Given the center (i, j), which is A, check if both the diagonals of the 3x3 square qualify
fn check_mas_in_square(
    map:&[[XMAS;140];140], 
    i:usize, 
    j:usize,
) -> bool {

    let top_left = Direction::<140>::TopLeft;
    let top_right = Direction::<140>::TopRight;
    let bottom_left = Direction::<140>::BottomLeft;
    let bottom_right = Direction::<140>::BottomRight;

    let mut count= 0usize;

    let letter_top_left = top_left.step(i, j);
    let letter_bottom_right = bottom_right.step(i, j);
    if let (Some((i, j)), Some((u, v))) = (letter_top_left, letter_bottom_right) {
        if (map[i][j] == XMAS::M && map[u][v] == XMAS::S) || (map[i][j] == XMAS::S && map[u][v] == XMAS::M) {
            count += 1;
        }
    }

    let letter_bottom_left = bottom_left.step(i, j);
    let letter_top_right = top_right.step(i, j);
    if let (Some((i, j)), Some((u, v))) = (letter_bottom_left, letter_top_right) {
        if (map[i][j] == XMAS::M && map[u][v] == XMAS::S) || (map[i][j] == XMAS::S && map[u][v] == XMAS::M) {
            count += 1;
        }
    }

    count >= 2
}


pub fn d4_part1_solution(input_path: &str) -> Result<usize, Error> {
    let mut f = File::open(input_path).map_err(|e| Error::IOError(e))?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).map_err(|e| Error::IOError(e))?;
    let map = load_map(buffer);
    let search_coords = get_target_coords(&map, XMAS::X);
    
    let result = search_coords
        .into_iter()
        .fold(0usize, |acc, coord| {
            let (i, j, _) = coord;
            acc + (
                check_xmas_in_direction(&map, i, j, Direction::Bottom) as usize
                + check_xmas_in_direction(&map, i, j, Direction::BottomLeft) as usize
                + check_xmas_in_direction(&map, i, j, Direction::BottomRight) as usize
                + check_xmas_in_direction(&map, i, j, Direction::Left) as usize
                + check_xmas_in_direction(&map, i, j, Direction::Right) as usize
                + check_xmas_in_direction(&map, i, j, Direction::TopLeft) as usize
                + check_xmas_in_direction(&map, i, j, Direction::Top) as usize
                + check_xmas_in_direction(&map, i, j, Direction::TopRight) as usize
            )
        });
    Ok(result)
    

}


pub fn d4_part2_solution(input_path: &str) -> Result<usize, Error> {
    let mut f = File::open(input_path).map_err(|e| Error::IOError(e))?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).map_err(|e| Error::IOError(e))?;
    let map = load_map(buffer);
    let search_coords = get_target_coords(&map, XMAS::A);
    
    let result = search_coords
        .into_iter()
        .fold(0usize, |acc, coord| {
            let (i, j, _) = coord;
            acc + check_mas_in_square(&map, i, j) as usize
        });
    Ok(result)
    

}