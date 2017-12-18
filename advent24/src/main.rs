use std::fs::File;
use std::io::prelude::*;

use std::cmp;

fn strongest_bridge(a: u8, bridges: &Vec<(u8, u8)>, indexes: u64) -> usize {
    let mut strength = 0usize;

    for j in 0..bridges.len() {
        if indexes & (1 << j) != 0 {
            continue;
        }

        let (c, d) = bridges[j];
        let part_strength = (c + d) as usize;
        if a == c {
            strength = cmp::max(
                strength, part_strength + strongest_bridge(d, &bridges, indexes | (1 << j)));
        }

        if c != d && a == d {
            strength = cmp::max(
                strength, part_strength + strongest_bridge(c, &bridges, indexes | (1 << j)));
        }
    }

    return strength;
}

fn strongest_bridge_from(i: usize, bridges: &Vec<(u8, u8)>) -> usize {
    let (a, b) = bridges[i];
    let indexes = 1 << i;
    if a == 0 {
        b as usize + strongest_bridge(b, &bridges, indexes)
    } else if b == 0 {
        a as usize + strongest_bridge(a, &bridges, indexes)
    } else {
        0
    }
}

fn solution1(input: &String) -> usize {
    let bridges : Vec<(u8, u8)> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut pair = line
                .split('/')
                .map(|x| x.parse::<u8>().unwrap());
            (pair.next().unwrap(), pair.next().unwrap())
        })
        .collect();

    return (0..bridges.len())
        .map(|i| strongest_bridge_from(i, &bridges))
        .max()
        .unwrap();
}

fn longest_bridge(a: u8, bridges: &Vec<(u8, u8)>, indexes: u64, size: u8) -> (u8, usize) {
    let mut z = (size, 0usize);

    for j in 0..bridges.len() {
        if indexes & (1 << j) != 0 {
            continue;
        }

        let (c, d) = bridges[j];
        let part_strength = (c + d) as usize;

        if a == c {
            let mut x = longest_bridge(d, &bridges, indexes | (1 << j), size + 1);
            x.1 += part_strength;
            z = cmp::max(z, x);
        }

        if c != d && a == d {
            let mut x = longest_bridge(c, &bridges, indexes | (1 << j), size + 1);
            x.1 += part_strength;
            z = cmp::max(z, x);
        }
    }

    return z;
}

fn longest_bridge_from(i: usize, bridges: &Vec<(u8, u8)>) -> (u8, usize) {
    let (a, b) = bridges[i];
    let indexes = 1 << i;
    if a == 0 {
        let x = longest_bridge(b, &bridges, indexes, 1);
        (x.0, x.1 + b as usize)
    } else if b == 0 {
        let x = longest_bridge(a, &bridges, indexes, 1);
        (x.0, x.1 + a as usize)
    } else {
        (0, 0)
    }
}

fn solution2(input: &String) -> usize {
    let bridges : Vec<(u8, u8)> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut pair = line
                .split('/')
                .map(|x| x.parse::<u8>().unwrap());
            (pair.next().unwrap(), pair.next().unwrap())
        })
        .collect();

    return (0..bridges.len())
        .map(|i| longest_bridge_from(i, &bridges))
        .max()
        .unwrap().1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let input = String::from("\
            0/2\n\
            2/2\n\
            2/3\n\
            3/4\n\
            3/5\n\
            0/1\n\
            10/1\n\
            9/10\n\
            ");

        assert_eq!(solution1(&input), 31);
    }

    #[test]
    fn test_solution2() {
        let input = String::from("\
            0/2\n\
            2/2\n\
            2/3\n\
            3/4\n\
            3/5\n\
            0/1\n\
            10/1\n\
            9/10\n\
            ");

        assert_eq!(solution2(&input), 19);
    }
}

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
