use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

fn parse_scanners(input: &String) -> HashMap<u32, u32> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let tokens : Vec<_> = line
                .split(": ")
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            (tokens[0], tokens[1])
        })
        .collect()
}

fn is_caught(depth: u32, range: u32, delay: u32) -> bool {
   return (depth + delay) % (range * 2 - 2) == 0;
}

fn is_caught_by_any(scanners: &HashMap<u32, u32>, delay: u32) -> bool {
    return scanners
        .iter()
        .filter(|&(depth, range)| is_caught(*depth, *range, delay))
        .next().is_some();
}

fn solution1(input: &String) -> u32 {
    return parse_scanners(input)
        .iter()
        .filter(|&(depth, range)| is_caught(*depth, *range, 0))
        .inspect(|&(depth, range)| println!("caught at depth {} (range {})", depth, range))
        .map(|(depth, range)| depth * range)
        .sum();
}

fn solution2(input: &String) -> u32 {
    let mut delay = 0;
    let scanners = parse_scanners(input);
    // TODO: Find mathematical solution.
    while is_caught_by_any(&scanners, delay) {
        delay += 1;
    }
    return delay;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let input = String::from("\
            0: 3\n\
            1: 2\n\
            4: 4\n\
            6: 4\n\
            ");
        assert_eq!(solution1(&input), 24);
        assert_eq!(solution2(&input), 10);
    }
}

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    let s1 = solution1(&input);
    println!("solution: {}", s1);

    let s2 = solution2(&input);
    println!("solution: {}", s2);
}
