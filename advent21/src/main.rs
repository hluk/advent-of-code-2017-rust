use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

extern crate bit_vec;
use bit_vec::BitVec;

fn bit(x: u16, i: u16) -> u16 {
    (x >> i) & 1
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Square {
    value: u16,
    size: u8,
}

impl Square {
    fn from(input: &str) -> Square {
        let size = match input.chars().count() {
            5 => 2,
            11 => 3,
            19 => 4,
            _ => { assert!(false); 2 },
        };

        let value = input
            .split('/')
            .map(|x| {
                x
                    .chars()
                    .fold(0, |acc, x| (acc << 1) | (if x == '#' {1} else {0}))
            })
            .fold(0, |acc, x| (acc << size) | x);

        Square{
            value: value,
            size: size,
        }
    }

    fn flip(self) -> Square {
        let x = self.value;
        let s = self.size as u16;
        let result = (0..s)
            .fold(0, |acc, i| {
                let k = (s - i - 1) * s;
                (acc << s) | (0..s)
                    .fold(0, |acc, j| (acc << 1) | bit(x, k + j))
            });
        Square{
            value: result,
            size: self.size,
        }
    }

    fn rotate90(self) -> Square {
        //  #..  .#.
        //  #.#  ..#
        //  ##.  ###
        let x = self.value;
        let s = self.size as u16;

        let result = if self.size == 2 {
            (0..s*s).fold(0, |acc, i| {
                let j = match i {
                    0 => 2,
                    1 => 0,
                    2 => 3,
                    _ => 1,
                };
                acc | (bit(x, i) << j)
            })
        } else {
            assert_eq!(self.size, 3);
            (0..s*s).fold(0, |acc, i| {
                let j = match i {
                    0 => 6,
                    1 => 3,
                    2 => 0,
                    3 => 7,
                    4 => 4,
                    5 => 1,
                    6 => 8,
                    7 => 5,
                    _ => 2,
                };
                acc | (bit(x, i) << j)
            })
        };

        Square{
            value: result,
            size: self.size,
        }
    }
}

fn parse_rules(input: &String) -> HashMap<Square, Square> {
    let mut result : HashMap<Square, Square> = HashMap::new();

    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let mut tokens = line
                .split(" => ")
                .map(|token| Square::from(token));
            let square = tokens.next().unwrap();
            let value = tokens.next().unwrap();

            let square90 = square.rotate90();
            let square180 = square90.rotate90();
            let square270 = square180.rotate90();
            result.insert(square, value);
            result.insert(square90, value);
            result.insert(square180, value);
            result.insert(square270, value);

            let flipped = square.flip();
            let flipped90 = flipped.rotate90();
            let flipped180 = flipped90.rotate90();
            let flipped270 = flipped180.rotate90();
            result.insert(flipped, value);
            result.insert(flipped90, value);
            result.insert(flipped180, value);
            result.insert(flipped270, value);
        });

    return result;
}

fn solution1(input: &String, iterations: usize) -> usize {
    let rules = parse_rules(input);

    let max_size = 4096;
    let mut b = vec![BitVec::from_elem(max_size, false); max_size];

    // .#.
    // ..#
    // ###
    b[0].set(0, false);
    b[0].set(1, true);
    b[0].set(2, false);

    b[1].set(0, false);
    b[1].set(1, false);
    b[1].set(2, true);

    b[2].set(0, true);
    b[2].set(1, true);
    b[2].set(2, true);

    let mut size = 3usize;

    for _ in 0..iterations {
        let sz = if size % 2 == 0 {2} else {3};
        let d = size / sz;

        for y in (0..d).rev() {
            let offset_y = y * sz;
            for x in (0..d).rev() {
                let offset_x = x * sz;
                let value = (0..sz)
                    .fold(0, |acc, j| {
                        (acc << sz) | (0..sz)
                            .fold(0, |acc, k| (acc << 1) | b[offset_y + j][offset_x + k] as u16)
                    });

                let rule = rules[&Square{value: value, size: sz as u8}];

                (0..sz + 1).for_each(|j| {
                    (0..sz + 1).for_each(|k| {
                        let nbit = ((sz - j) * (sz + 1) + (sz - k)) as u16;
                        b[offset_y + y + j].set(offset_x + x + k, bit(rule.value, nbit) == 1);
                    });
                });
            }
        }

        size += d;
    }

    return b
        .iter()
        .map(|x| x
             .iter()
             .filter(|&x| x)
             .count())
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_from() {
        assert_eq!(Square::from("../.."),  Square{size: 2, value: 0});
        assert_eq!(Square::from(".../.../..."), Square{size: 3, value: 0});

        assert_eq!(Square::from("../.#"),  Square{size: 2, value: 1});
        assert_eq!(Square::from("../##"),  Square{size: 2, value: 3});
        assert_eq!(Square::from("#./.."),  Square{size: 2, value: 8});
        assert_eq!(Square::from("##/.."),  Square{size: 2, value: 12});
        assert_eq!(Square::from("##/#."),  Square{size: 2, value: 14});
        assert_eq!(Square::from("##/##"),  Square{size: 2, value: 15});

        assert_eq!(Square::from(".../.../.#."), Square{size: 3, value: 2});
        assert_eq!(Square::from(".../.../###"), Square{size: 3, value: 7});
    }

    #[test]
    fn test_square_flip() {
        assert_eq!(Square::from("../..").flip(), Square::from("../.."));
        assert_eq!(Square::from("../.#").flip(), Square::from("../#."));
        assert_eq!(Square::from("../#.").flip(), Square::from("../.#"));
        assert_eq!(Square::from(".#/..").flip(), Square::from("#./.."));
        assert_eq!(Square::from("#./..").flip(), Square::from(".#/.."));

        assert_eq!(Square::from(".../.../...").flip(), Square::from(".../.../..."));
        assert_eq!(Square::from(".../.#./...").flip(), Square::from(".../.#./..."));
        assert_eq!(Square::from(".../.##/...").flip(), Square::from(".../##./..."));
        assert_eq!(Square::from("#../.##/...").flip(), Square::from("..#/##./..."));
        assert_eq!(Square::from("#../.##/##.").flip(), Square::from("..#/##./.##"));
    }

    #[test]
    fn test_square_rotate() {
        assert_eq!(Square::from("../..").rotate90(), Square::from("../.."));
        assert_eq!(Square::from("../.#").rotate90(), Square::from(".#/.."));
        assert_eq!(Square::from("../#.").rotate90(), Square::from("../.#"));

        assert_eq!(Square::from(".../.../...").rotate90(), Square::from(".../.../..."));

        assert_eq!(Square::from(".../.../..#").rotate90(), Square::from("..#/.../..."));
        assert_eq!(Square::from(".../.../.#.").rotate90(), Square::from(".../..#/..."));
        assert_eq!(Square::from(".../.../#..").rotate90(), Square::from(".../.../..#"));

        assert_eq!(Square::from(".../..#/...").rotate90(), Square::from(".#./.../..."));
        assert_eq!(Square::from(".../.#./...").rotate90(), Square::from(".../.#./..."));
        assert_eq!(Square::from(".../#../...").rotate90(), Square::from(".../.../.#."));

        assert_eq!(Square::from("..#/.../...").rotate90(), Square::from("#../.../..."));
        assert_eq!(Square::from(".#./.../...").rotate90(), Square::from(".../#../..."));
        assert_eq!(Square::from("#../.../...").rotate90(), Square::from(".../.../#.."));
    }

    #[test]
    fn test_solution1() {
        let input = String::from("\
            ../.# => ##./#../...\n\
            .#./..#/### => #..#/..../..../#..#\n\
            ");
        assert_eq!(solution1(&input, 2), 12);
    }
}

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    let s1 = solution1(&input, 5);
    println!("solution1: {}", s1);

    let s2 = solution1(&input, 18);
    println!("solution2: {}", s2);
}
