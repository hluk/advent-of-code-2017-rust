use std::fs::File;
use std::io::prelude::*;

const REGISTER_COUNT : usize = 'h' as usize - 'a' as usize + 2;

#[derive(Clone, Copy, Debug)]
enum Argument {
    Register(char),
    Value(i64),
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Set(Argument, Argument),
    Sub(Argument, Argument),
    Mul(Argument, Argument),
    Jnz(Argument, Argument),
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
                "set" => Instruction::Set(arg!(), arg!()),
                "sub" => Instruction::Sub(arg!(), arg!()),
                "mul" => Instruction::Mul(arg!(), arg!()),
                "jnz" => Instruction::Jnz(arg!(), arg!()),
                _ => { assert!(false); Instruction::Set(Argument::Value(0), Argument::Value(0)) },
            }
        }).collect()
}

fn solution1(program: &Vec<Instruction>) -> usize {
    let mut registers = [0i64; REGISTER_COUNT];
    let mut program_counter = 0i64;

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

    let mut mul_count = 0usize;
    while 0 <= program_counter && (program_counter as usize) < program.len() {
        let instruction = program[program_counter as usize];
        //println!("{:?} {}: {:?}", registers, program_counter, instruction);
        match instruction {
            Instruction::Set(a, b) => {
                reg!(a, val!(b));
            },

            Instruction::Sub(a, b) => {
                reg!(a, val!(a) - val!(b));
            },

            Instruction::Mul(a, b) => {
                mul_count += 1;
                reg!(a, val!(a) * val!(b));
            },

            Instruction::Jnz(a, b) => {
                if val!(a) != 0 {
                    program_counter += val!(b) - 1;
                }
            },
        }
        program_counter += 1;
    }

    return mul_count;
}

fn is_prime(x: i64) -> bool {
    if x < 2 {
        return false;
    }

    let r = x / 2;
    let mut i = 2;

    while i < r {
        if x % i == 0 {
            return false;
        }
        i += 1;
    }

    return true;
}

fn solution2() -> i64 {
    let mut b = 99 * 100 + 100000;
    let c = b + 17000;
    let mut h = 0i64;

    while b <= c {
        if !is_prime(b) {
            h += 1;
        }
        b += 17;
    }

    return h;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
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

    let s2 = solution2();
    println!("solution 2: {}", s2);
}
