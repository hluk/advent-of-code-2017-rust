use std::fs::File;
use std::io::prelude::*;

fn solution(lengths: Vec<u16>, number_of_items: u16, number_of_rounds: u16) -> Vec<u16> {
    let mut items : Vec<u16> = (0..number_of_items).collect();
    println!("*** {:?} {:?}", items, lengths);

    let mut i = 0u16;
    let mut skip_size = 0u16;

    for _ in 0..number_of_rounds {
        for length in &lengths {
            let start = i as usize;
            let end = ((i + length) % number_of_items) as usize;
            if i == 0 && *length == number_of_items {
                items.reverse();
            } else if start <= end {
                items[start..end].reverse();
            } else {
                let len = items.len();
                items = items.iter().cycle().map(|&x| x).skip(start).take(len).collect();
                items[0..len - start + end].reverse();
                items = items.iter().cycle().map(|&x| x).skip(len - start).take(len).collect();
            }

            i = (i + length + skip_size) % number_of_items;
            skip_size += 1;
        }
    }

    return items;
}

fn knot_hash(items: Vec<u16>) -> String {
    assert_eq!(items.len(), 256);
    let result = (0..16)
        .map(|i| items[i*16..i*16+16]
             .iter()
             .inspect(|&&x| assert!(x < 256))
             .fold(0u16, |acc, &x| acc ^ x))
             .inspect(|&x| assert!(x < 256))
             .inspect(|&x| println!("{}", x))
             .map(|x| format!("{:02x}", x))
             .fold(String::new(), |acc, x| acc + &x);
    return result;
}

fn solution1(input: String, number_of_items: u16) -> u16 {
    let lengths = input
        .split(',')
        .map(|x| x.trim().parse::<u16>().unwrap())
        .collect::<Vec<_>>();
    let items = solution(lengths, number_of_items, 1);
    return items[0] * items[1];
}

fn solution2(input: String) -> String {
    let lengths = input
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| c as u16)
        .chain([17u16, 31u16, 73u16, 47u16, 23u16].iter().map(|&x| x))
        .collect::<Vec<_>>();
    let items = solution(lengths, 256, 64);
    return knot_hash(items);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(String::from("5"), 5), 12);
        assert_eq!(solution1(String::from("0"), 5), 0);

        assert_eq!(solution1(String::from("3,4,1,5"), 5), 12);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution2(String::from("")), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(solution2(String::from("\n")), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(solution2(String::from("AoC 2017")), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(solution2(String::from("AoC 2017\n")), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(solution2(String::from("1,2,3")), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(solution2(String::from("1,2,4")), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    let s1 = solution1(input.clone(), 256);
    println!("solution 1: {}", s1);

    let s2 = solution2(input);
    println!("solution 2: {}", s2);
}
