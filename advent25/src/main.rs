use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

extern crate bit_vec;
use bit_vec::BitVec;

extern crate regex;
use regex::Regex;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum CursorMoveDirection {
    Left,
    Right,
}

impl CursorMoveDirection {
    fn from_string(text: &str) -> CursorMoveDirection {
        if text == "left" {
            CursorMoveDirection::Left
        } else {
            assert_eq!(text, "right");
            CursorMoveDirection::Right
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    write_value : bool,
    move_direction : CursorMoveDirection,
    next_state : char,
}

fn solution1(input: &String) -> usize {
    let mut lines = input.split('\n');

    let start_line = lines.next().unwrap();
    let start_re = Regex::new(r"^Begin in state ([A-Z])\.$").unwrap();
    let start = start_re
        .captures_iter(start_line)
        .next()
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .chars()
        .nth(0)
        .unwrap();

    let steps_line = lines.next().unwrap();
    let steps_re = Regex::new(r"^Perform a diagnostic checksum after (\d+) steps\.$").unwrap();
    let steps = steps_re
        .captures_iter(steps_line)
        .next()
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u32>()
        .unwrap();

    let mut states : HashMap<(char, bool), State> = HashMap::new();
    let state_name_re = Regex::new("^In state (?P<state>[A-Z]):").unwrap();
    let state_re = Regex::new("\
        \\s*If the current value is (?P<read_value>[01]):\n\
        \\s*- Write the value (?P<write_value>[01])\\.\n\
        \\s*- Move one slot to the (?P<move_direction>left|right)\\.\n\
        \\s*- Continue with state (?P<next_state>[A-Z])\\.").unwrap();
    input
        .split("\n\n")
        .skip(1)
        .for_each(|x| {
            let state_name = state_name_re
                .captures_iter(x)
                .next()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .nth(0)
                .unwrap();

            for captures in state_re.captures_iter(x) {
                let read_value = &captures["read_value"];
                let write_value = &captures["write_value"];
                let move_direction = &captures["move_direction"];
                let next_state = &captures["next_state"];
                let state = State{
                    write_value: write_value == "1",
                    move_direction: CursorMoveDirection::from_string(move_direction),
                    next_state: next_state.chars().next().unwrap(),
                };
                states.insert((state_name, read_value == "1"), state);
            }
        });

    const TAPE_SIZE : usize = 1024*1024;
    let mut tape = BitVec::from_elem(TAPE_SIZE, false);
    let mut cursor_position = TAPE_SIZE / 2;
    let mut current_state = start;

    (0..steps)
        .for_each(|_| {
            let read_value = tape[cursor_position];
            let state = states[&(current_state, read_value)];

            tape.set(cursor_position, state.write_value);

            if state.move_direction == CursorMoveDirection::Left {
                assert!(cursor_position != 0);
                cursor_position -= 1;
            } else {
                cursor_position += 1;
                assert!(cursor_position != TAPE_SIZE);
            }

            current_state = state.next_state;
        });

    return tape
        .iter()
        .filter(|&x| x)
        .count();
}

fn read_file(file_path: &str) -> String {
    let mut input_file = File::open(file_path).expect("file not found");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    return input;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let input = read_file("test.txt");
        assert_eq!(solution1(&input), 3);
    }
}

fn main() {
    let input = read_file("input.txt");
    let s1 = solution1(&input);
    println!("solution 1: {}", s1);
}
