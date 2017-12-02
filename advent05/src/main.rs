use std::fs::File;
use std::io::prelude::*;

fn solution1(input: String, f: fn(i32) -> i32) -> u32 {
    let mut js = input.split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let length = js.len() as i32;
    let mut steps = 0u32;
    let mut i = 0i32;

    while 0 <= i && i < length {
        steps += 1;

        let j = js[i as usize];
        js[i as usize] = f(j);
        //println!("{:>6}. {:>4}{:<+4}", steps, i, j);

        i += j;
    }
    //println!("{:?}", js);

    return steps;
}

fn f1(x: i32) -> i32 { x + 1 }
fn f2(x: i32) -> i32 { if x < 3 {x + 1} else {x - 1} }

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    assert_eq!(solution1(String::from("0\n3\n0\n1\n-3\n"), f1), 5);
    assert_eq!(solution1(String::from("0\n3\n0\n1\n-3\n"), f2), 10);

    let s1 = solution1(input.clone(), f1);
    println!("solution 1: {}", s1);

    let s2 = solution1(input, f2);
    println!("solution 2: {}", s2);
}
