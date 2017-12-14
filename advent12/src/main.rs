use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;
use std::collections::HashSet;

fn add_connections(line: &str, map: &mut HashMap<u32, Vec<u32>>) {
    let tokens = line.split(" <-> ").collect::<Vec<_>>();
    let value = tokens[0]
        .parse::<u32>()
        .unwrap();
    let values = tokens[1]
        .trim()
        .split(", ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    map.insert(value, values);
}

fn connections(input: &String) -> HashMap<u32, Vec<u32>> {
    let mut map : HashMap<u32, Vec<u32>> = HashMap::new();
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .for_each(|line| add_connections(line, &mut map));
    return map;
}

fn visit_one(map: &mut HashMap<u32, Vec<u32>>, visit: &mut HashSet<u32>) -> bool {
    let x = match visit.iter().nth(0) {
        Some(&x) => x,
        None => return false,
    };

    match map.get(&x) {
        Some(xs) => xs.iter()
            .filter(|y| map.contains_key(y))
            .for_each(|&x| { visit.insert(x); }),
        None => (),
    }
    visit.remove(&x);
    map.remove(&x);

    return true;
}

fn solution1(input: &String) -> u32 {
    let mut map = connections(input);
    let mut count = 0u32;
    let mut visit : HashSet<u32> = [0].iter().cloned().collect();

    while visit_one(&mut map, &mut visit) {
        count += 1;
    }

    return count;
}

fn solution2(input: &String) -> u32 {
    let mut map = connections(input);
    let mut count = 0u32;

    loop {
        let x = match map.iter().nth(0) {
            Some((&x, _)) => x,
            None => break,
        };

        let mut visit : HashSet<u32> = map.get(&x).unwrap().iter().cloned().collect();
        map.remove(&x);
        count += 1;

        while visit_one(&mut map, &mut visit) {}
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let input = String::from("\
            0 <-> 2\n\
            1 <-> 1\n\
            2 <-> 0, 3, 4\n\
            3 <-> 2, 4\n\
            4 <-> 2, 3, 6\n\
            5 <-> 6\n\
            6 <-> 4, 5\n\
            ");
        assert_eq!(solution1(&input), 6);
        assert_eq!(solution2(&input), 2);
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
