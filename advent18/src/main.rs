use std::fs::File;
use std::io::prelude::*;

use std::collections::VecDeque;

const REGISTER_COUNT : usize = 'z' as usize - 'a' as usize + 2;

#[derive(Clone, Copy, Debug)]
enum Argument {
    Register(char),
    Value(i64),
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Snd(Argument),
    Set(Argument, Argument),
    Add(Argument, Argument),
    Mul(Argument, Argument),
    Mod(Argument, Argument),
    Rcv(Argument),
    Jgz(Argument, Argument),
}

struct Program {
    registers: [i64; REGISTER_COUNT],
    program_counter: i64,
}

impl Program {
    fn new() -> Program {
        return Program {
            registers: [0; REGISTER_COUNT],
            program_counter: 0,
        }
    }

    fn run(&mut self, instructions: &Vec<Instruction>, send: &mut VecDeque<i64>) -> char {
        macro_rules! reg {
            ($e:expr, $f:expr) => (
                match $e {
                    Argument::Register(x) => self.registers[x as usize - 'a' as usize] = $f,
                    Argument::Value(_) => { assert!(false); self.registers[REGISTER_COUNT - 1] = $f },
                };
            )
        }

        macro_rules! val {
            ($e:expr) => (
                match $e {
                    Argument::Register(x) => self.registers[x as usize - 'a' as usize],
                    Argument::Value(a) => a,
                }
            )
        }

        loop {
            let instruction = instructions[self.program_counter as usize];
            println!("{:?} {}: {:?}", self.registers, self.program_counter, instruction);
            self.program_counter += 1;
            match instruction {
                Instruction::Snd(a) => {
                    send.push_back(val!(a));
                },

                Instruction::Rcv(a) => {
                    return match a {
                        Argument::Register(x) => x,
                        Argument::Value(_) => { assert!(false); 'a' },
                    }
                },

                Instruction::Set(a, b) => {
                    reg!(a, val!(b));
                },

                Instruction::Add(a, b) => {
                    reg!(a, val!(a) + val!(b));
                },

                Instruction::Mul(a, b) => {
                    reg!(a, val!(a) * val!(b));
                },

                Instruction::Mod(a, b) => {
                    reg!(a, val!(a) % val!(b));
                },

                Instruction::Jgz(a, b) => {
                    if val!(a) > 0 {
                        self.program_counter += val!(b) - 1;
                    }
                },
            }
        }
    }

    fn set_register(&mut self, register: char, value: i64) {
        self.registers[register as usize - 'a' as usize] = value;
    }
}

fn argument(arg: &str) -> Argument {
    let c = arg.chars().next().unwrap();
    if 'a' <= c && c <= 'z' {
        return Argument::Register(c);
    }

    return Argument::Value(arg.parse::<i64>().unwrap());
}

fn parse(input: String) -> Vec<Instruction> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|x| {
            let mut tokens = x.split(' ');

            macro_rules! arg {
                () => ( argument(tokens.next().unwrap()) )
            }

            let instruction = tokens.next().unwrap();
            match instruction {
                "snd" => Instruction::Snd(arg!()),
                "rcv" => Instruction::Rcv(arg!()),
                "set" => Instruction::Set(arg!(), arg!()),
                "add" => Instruction::Add(arg!(), arg!()),
                "mul" => Instruction::Mul(arg!(), arg!()),
                "mod" => Instruction::Mod(arg!(), arg!()),
                "jgz" => Instruction::Jgz(arg!(), arg!()),
                _ => { assert!(false); Instruction::Rcv(Argument::Value(0)) },
            }
        }).collect()
}

fn solution1(program: &Vec<Instruction>) -> i64 {
    let mut registers = [0i64; REGISTER_COUNT];

    macro_rules! reg {
        ($e:expr, $f:expr) => (
            match $e {
                Argument::Register(x) => registers[x as usize - 'a' as usize] = $f,
                Argument::Value(_) => { assert!(false); registers[REGISTER_COUNT - 1] = $f },
            };
        )
    }

    macro_rules! val {
        ($e:expr) => (
            match $e {
                Argument::Register(x) => registers[x as usize - 'a' as usize],
                Argument::Value(a) => a,
            }
        )
    }

    let mut program_counter = 0i64;
    let mut played = -1i64;
    loop {
        let instruction = program[program_counter as usize];
        println!("{:?} {}: {:?}", registers, program_counter, instruction);
        match instruction {
            Instruction::Snd(a) => {
                played = val!(a);
            },

            Instruction::Rcv(a) => {
                if val!(a) != 0 {
                    return played;
                }
            },

            Instruction::Set(a, b) => {
                reg!(a, val!(b));
            },

            Instruction::Add(a, b) => {
                reg!(a, val!(a) + val!(b));
            },

            Instruction::Mul(a, b) => {
                reg!(a, val!(a) * val!(b));
            },

            Instruction::Mod(a, b) => {
                reg!(a, val!(a) % val!(b));
            },

            Instruction::Jgz(a, b) => {
                if val!(a) > 0 {
                    program_counter += val!(b) - 1;
                }
            },
        }
        program_counter += 1;
    }
}

fn solution2(program: &Vec<Instruction>) -> usize {
    let mut p0 = Program::new();
    let mut p1 = Program::new();
    p0.set_register('p', 0);
    p1.set_register('p', 1);

    let mut count = 0usize;
    let mut send_to0 : VecDeque<i64> = VecDeque::new();
    let mut send_to1 : VecDeque<i64> = VecDeque::new();
    let mut r0 : Option<char> = None;
    let mut r1 : Option<char> = None;

    while r0.is_none() || r1.is_none() {
        if r0.is_none() {
            r0 = Some(p0.run(&program, &mut send_to1));
        }

        if r1.is_none() {
            r1 = Some(p1.run(&program, &mut send_to0));
        }

        if r0.is_some() {
            match send_to0.pop_front() {
                Some(x) => {
                    count +=1;
                    p0.set_register(r0.unwrap(), x);
                    r0 = None;
                },

                None => (),
            }
        }

        if r1.is_some() {
            match send_to1.pop_front() {
                Some(x) => {
                    p1.set_register(r1.unwrap(), x);
                    r1 = None;
                },

                None => (),
            }
        }
    }

    return count + send_to0.iter().count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let input = String::from("\
            set a 1\n\
            add a 2\n\
            mul a a\n\
            mod a 5\n\
            snd a\n\
            set a 0\n\
            rcv a\n\
            jgz a -1\n\
            set a 1\n\
            jgz a -2\n\
            ");

        let program = parse(input);
        assert_eq!(solution1(&program), 4);
    }

    #[test]
    fn test_solution2() {
        {
            let program = vec![
                Instruction::Rcv(Argument::Register('a')),
            ];
            assert_eq!(solution2(&program), 0);
        }

        {
            let program = vec![
                Instruction::Snd(Argument::Value(1)),
                Instruction::Rcv(Argument::Register('a')),
                Instruction::Rcv(Argument::Register('b')),
            ];
            assert_eq!(solution2(&program), 1);
        }

        {
            let program = vec![
                Instruction::Snd(Argument::Value(1)),
                Instruction::Snd(Argument::Value(2)),
                Instruction::Rcv(Argument::Register('a')),
                Instruction::Rcv(Argument::Register('b')),
                Instruction::Rcv(Argument::Register('c')),
            ];
            assert_eq!(solution2(&program), 2);
        }

        {
            let program = vec![
                Instruction::Snd(Argument::Value(1)),
                Instruction::Rcv(Argument::Register('a')),
                Instruction::Snd(Argument::Value(2)),
                Instruction::Rcv(Argument::Register('b')),
                Instruction::Rcv(Argument::Register('c')),
            ];
            assert_eq!(solution2(&program), 2);
        }
    }
}

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    let program = parse(input);

    let s1 = solution1(&program);
    println!("solution 1: {}", s1);

    let s2 = solution2(&program);
    println!("solution 2: {}", s2);
}
