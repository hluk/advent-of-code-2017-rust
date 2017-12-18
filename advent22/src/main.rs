use std::fs::File;
use std::io::prelude::*;

use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn reverse(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Copy, Clone)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

fn solution1(input: &String, bursts: usize) -> usize {
    let mut count = 0usize;

    let mut infected : HashSet<(i32, i32)> = HashSet::new();
    let mut width = 0usize;
    let mut height = 0usize;
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(y, line)| {
            height = y + 1;
            line
                .chars()
                .enumerate()
                .for_each(|(x, c)| {
                    width = x + 1;
                    if c == '#' {
                        infected.insert((x as i32, y as i32));
                    }
                })
        });

    let mut direction = Direction::Up;
    let mut x = (width / 2) as i32; 
    let mut y = (height / 2) as i32; 
    for _ in 0..bursts {
        if infected.insert((x, y)) {
            count += 1;
            direction = direction.turn_left();
        } else {
            infected.remove(&(x, y));
            direction = direction.turn_right();
        }

        match direction {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        };
    }

    return count;
}

fn solution2(input: &String, bursts: usize) -> usize {
    let mut count = 0usize;

    let mut states : HashMap<(i32, i32), State> = HashMap::new();
    let mut width = 0usize;
    let mut height = 0usize;
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(y, line)| {
            height = y + 1;
            line
                .chars()
                .enumerate()
                .for_each(|(x, c)| {
                    width = x + 1;
                    if c == '#' {
                        states.insert((x as i32, y as i32), State::Infected);
                    }
                })
        });

    let mut direction = Direction::Up;
    let mut x = (width / 2) as i32; 
    let mut y = (height / 2) as i32; 
    for _ in 0..bursts {
        let pos = (x, y);
        let state = states.get(&pos).unwrap_or(&State::Clean).clone();
        direction = match state {
            State::Clean => direction.turn_left(),
            State::Weakened => direction,
            State::Infected => direction.turn_right(),
            State::Flagged => direction.reverse(),
        };

        match state {
            State::Clean => states.insert(pos, State::Weakened),
            State::Weakened => {
                count += 1;
                states.insert(pos, State::Infected)
            },
            State::Infected => states.insert(pos, State::Flagged),
            State::Flagged => states.remove(&pos),
        };

        match direction {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        };
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1_41() {
        let input = String::from("\
            ..#\n\
            #..\n\
            ...\n\
            ");
        assert_eq!(solution1(&input, 70), 41);
    }

    #[test]
    fn test_solution1_10000() {
        let input = String::from("\
            ..#\n\
            #..\n\
            ...\n\
            ");
        assert_eq!(solution1(&input, 10000), 5587);
    }

    #[test]
    fn test_solution2_100() {
        let input = String::from("\
            ..#\n\
            #..\n\
            ...\n\
            ");
        assert_eq!(solution2(&input, 100), 26);
    }

    #[test]
    fn test_solution2_10000000() {
        let input = String::from("\
            ..#\n\
            #..\n\
            ...\n\
            ");
        assert_eq!(solution2(&input, 10000000), 2511944);
    }
}

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    let s1 = solution1(&input, 10000);
    println!("solution1: {}", s1);

    let s2 = solution2(&input, 10000000);
    println!("solution2: {}", s2);
}
