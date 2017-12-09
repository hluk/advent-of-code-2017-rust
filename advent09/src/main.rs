use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
struct State {
    level: u32,
    score: u32,
    escape: bool,
    garbage: bool,
    garbage_count: u32,
}

fn solution1(input: String) -> State {
    let mut state = State{
        level: 0,
        score: 0,
        escape: false,
        garbage: false,
        garbage_count: 0,
    };

    input.chars().for_each(|c| {
        if state.escape {
            state.escape = false;
        } else {
            if state.garbage && c != '!' && c != '>' {
                state.garbage_count += 1;
            }

            match c {
                '!' => state.escape = true,
                '{' => if !state.garbage { state.level += 1; state.score += state.level; },
                '}' => if !state.garbage { state.level -= 1 },
                '<' => state.garbage = true,
                '>' => state.garbage = false,
                _ => (),
            }
        }
    });

    return state;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solution1(String::from("{}")).score, 1);
        assert_eq!(solution1(String::from("{{{}}}")).score, 1 + 2 + 3);
        assert_eq!(solution1(String::from("{{},{}}")).score, 1 + 2 + 2);
        assert_eq!(solution1(String::from("{{{},{},{{}}}}")).score, 1 + 2 + 3 + 3 + 3 + 4);
        assert_eq!(solution1(String::from("{<a>,<a>,<a>,<a>}")).score, 1);
        assert_eq!(solution1(String::from("{{<ab>},{<ab>},{<ab>},{<ab>}}")).score, 1 + 2 + 2 + 2 + 2);
        assert_eq!(solution1(String::from("{{<!!>},{<!!>},{<!!>},{<!!>}}")).score, 1 + 2 + 2 + 2 + 2);
        assert_eq!(solution1(String::from("{{<a!>},{<a!>},{<a!>},{<ab>}}")).score, 1 + 2);

        assert_eq!(solution1(String::from("<>")).garbage_count, 0);
        assert_eq!(solution1(String::from("<random characters>")).garbage_count, 17);
        assert_eq!(solution1(String::from("<<<<>")).garbage_count, 3);
        assert_eq!(solution1(String::from("<{!>}>")).garbage_count, 2);
        assert_eq!(solution1(String::from("<!!>")).garbage_count, 0);
        assert_eq!(solution1(String::from("<!!!>>")).garbage_count, 0);
        assert_eq!(solution1(String::from("<{o\"i!a,<{i<a>")).garbage_count, 10);
    }
}

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    let s = solution1(input);
    println!("solution: {} {}", s.score, s.garbage_count);
}
