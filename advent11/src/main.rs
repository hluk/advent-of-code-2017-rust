use std::fs::File;
use std::io::prelude::*;

fn hex_coordinated(origin: (i32,i32,i32), direction: &str) -> (i32,i32,i32) {
    let (x, y, z) = match direction {
        "n"  => ( 0, 1,-1),
        "ne" => ( 1, 0,-1),
        "se" => ( 1,-1, 0),
        "s"  => ( 0,-1, 1),
        "sw" => (-1, 0, 1),
        "nw" => (-1, 1, 0),
        _ => (0, 0, 0),
    };
    (origin.0 + x, origin.1 + y, origin.2 + z)
}

fn hex_distance(position: (i32,i32,i32)) -> u32 {
    return ((position.0.abs() + position.1.abs() + position.2.abs()) / 2) as u32;
}

fn solution1(input: String) -> u32 {
    let position = input.trim().split(',').fold((0i32, 0i32, 0i32), hex_coordinated);
    return hex_distance(position);
}

fn solution2(input: String) -> u32 {
    let mut position = (0i32, 0i32, 0i32);
    return input.trim()
        .split(',')
        .map(|d| { position = hex_coordinated(position, d); hex_distance(position) } )
        .max()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(String::from("ne,ne,ne")), 3);
        assert_eq!(solution1(String::from("ne,ne,ne\n")), 3);
        assert_eq!(solution1(String::from("ne,ne,sw,sw")), 0);
        assert_eq!(solution1(String::from("ne,ne,s,s")), 2);
        assert_eq!(solution1(String::from("se,sw,se,sw,sw")), 3);
    }
}

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    let s1 = solution1(input.clone());
    println!("solution: {}", s1);

    let s2 = solution2(input);
    println!("solution: {}", s2);
}
