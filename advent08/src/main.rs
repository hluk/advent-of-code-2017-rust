use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

struct Registers {
    values: HashMap<String, i32>,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            values: HashMap::new(),
        }
    }

    pub fn value(&self, register: &str) -> i32 {
        self.values.get(register).cloned().unwrap_or(0)
    }

    pub fn set_value(&mut self, register: &str, value: i32) {
        self.values.insert(String::from(register), value);
    }

    pub fn max_value(&self) -> i32 {
        *self.values.iter().max_by_key(|&(_, v)| v).unwrap().1
    }
}

fn is_true(register: &str, operator: &str, value: i32, registers: &Registers) -> bool {
    let r = registers.value(register);
    if operator == "<"  { return r <  value };
    if operator == ">"  { return r >  value };
    if operator == "<=" { return r <= value };
    if operator == ">=" { return r >= value };
    if operator == "==" { return r == value };
    if operator == "!=" { return r != value };
    assert!(false);
    return false;
}

fn execute_instruction(instruction: &str, registers: &mut Registers, max: &mut i32) {
    let tokens : Vec<_> = instruction.split(' ').collect();

    let iftext = tokens[3];
    assert_eq!(iftext, "if");

    let register2 = tokens[4];
    let operator = tokens[5];
    let value2 = tokens[6].parse::<i32>().unwrap();
    if is_true(register2, operator, value2, registers) {
        let register1 = tokens[0];
        let action = tokens[1];
        let value1 = tokens[2].parse::<i32>().unwrap();
        let mut r = registers.value(register1);
        if action == "inc" {
            r += value1;
        } else if action == "dec" {
            r -= value1;
        } else {
            assert!(false);
        }
        registers.set_value(register1, r);

        if *max < r {
            *max = r;
        }
    }
}

fn solution1(input: String) -> (i32, i32) {
    let instructions = input.split('\n')
        .filter(|x| !x.is_empty());

    let mut registers = Registers::new();
    let mut max = 0i32;
    instructions.for_each(|instruction| execute_instruction(instruction, &mut registers, &mut max));

    return (registers.max_value(), max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_instruction() {
        let mut registers = Registers::new();
        assert_eq!(registers.value("a"), 0);

        execute_instruction("b inc 5 if a > 1", &mut registers);
        assert_eq!(registers.value("a"), 0);
        assert_eq!(registers.value("b"), 0);

        execute_instruction("a inc 1 if b < 5", &mut registers);
        assert_eq!(registers.value("a"), 1);
        assert_eq!(registers.value("b"), 0);

        execute_instruction("c dec -10 if a >= 1", &mut registers);
        assert_eq!(registers.value("a"), 1);
        assert_eq!(registers.value("b"), 0);
        assert_eq!(registers.value("c"), 10);

        execute_instruction("c inc -20 if c == 10", &mut registers);
        assert_eq!(registers.value("a"), 1);
        assert_eq!(registers.value("b"), 0);
        assert_eq!(registers.value("c"), -10);

        assert_eq!(registers.max_value(), 1);
    }
}

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    let (s, max) = solution1(input);
    println!("solution: {} {}", s, max);
}
