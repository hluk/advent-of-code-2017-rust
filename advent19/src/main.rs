use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn solution(input: &String) -> (String, usize) {
    let mut result = String::new();

    let maze : Vec<Vec<char>> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
        ;

    let mut x = maze[0].iter().position(|&c| c == '|').unwrap() as i32;
    let mut y = 0i32;
    let mut d = Direction::Down;
    let mut steps = 0usize;

    macro_rules! mz {
        ($x:expr, $y:expr) => (
            if 0 <= $y && ($y as usize) < maze.len() && 0 <= $x && ($x as usize) < maze[$y as usize].len() {
                maze[$y as usize][$x as usize]
            } else {
                ' '
            }
        )
    }

    loop {
        steps += 1;

        match d {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        };

        if mz!(x, y) == ' ' {
            return (result, steps);
        }

        let c2 = mz!(x, y);
        println!("{} {} {}", x, y, c2);

        if 'A' <= c2 && c2 <= 'Z' {
            result.push(c2);
        } else if c2 == '+' {
            if d != Direction::Left && mz!(x + 1, y + 0) != ' ' {
                assert!(mz!(x + 1, y + 0) != '|');
                d = Direction::Right;
            } else if d != Direction::Right && mz!(x - 1, y + 0) != ' ' {
                assert!(mz!(x - 1, y + 0) != '|');
                d = Direction::Left;
            } else if d != Direction::Up && mz!(x + 0, y + 1) != ' ' {
                assert!(mz!(x + 0, y + 1) != '-');
                d = Direction::Down;
            } else if d != Direction::Down && mz!(x + 0, y - 1) != ' ' {
                assert!(mz!(x + 0, y - 1) != '-');
                d = Direction::Up;
            } else {
                assert!(false);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = String::new()
             + "    |          \n"
             + "    |  +--+    \n"
             + "    A  |  C    \n"
             + "F---|----E|--+ \n"
             + "    |  |  |  D \n"
             + "    +B-+  +--+ \n";

        assert_eq!(solution(&input), (String::from("ABCDEF"), 38));
    }
}

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    let s = solution(&input);
    println!("solution: {} {}", s.0, s.1);
}
