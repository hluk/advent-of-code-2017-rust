use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

extern crate time;
use time::precise_time_ns;

#[derive(Debug)]
enum Dance {
    Spin(u8),
    Exchange(u8),
    Partner(u8),
}

fn dance(dance: &str) -> Option<Dance> {
    match dance.chars().nth(0) {
        Some('s') => {
            let s = dance[1..].parse::<u8>().unwrap();
            Some(Dance::Spin(s))
        },

        Some('x') => {
            let mut s = dance[1..].split('/').map(|x| x.parse::<u8>().unwrap());
            let i = s.next().unwrap();
            let j = s.next().unwrap();
            Some(Dance::Exchange(i | (j << 4)))
        },

        Some('p') => {
            let a = dance.chars().nth(1).unwrap();
            let b = dance.chars().nth(3).unwrap();
            let ax = a as u8 - 'a' as u8;
            let bx = b as u8 - 'a' as u8;
            Some(Dance::Partner(ax | (bx << 4)))
        },

        _ => None
    }
}

fn exchange(p: u64, i: u8, j: u8) -> u64 {
    let a = ((p >> (i * 4)) & 0b1111) << (j * 4);
    let b = ((p >> (j * 4)) & 0b1111) << (i * 4);
    let a0 = 0b1111 << (i * 4);
    let b0 = 0b1111 << (j * 4);
    return (p & !(a0 | b0)) | a | b;
}

fn perf_now() -> u64 {
    precise_time_ns()
}

fn perf(start: u64, text: &str) {
    let end = perf_now();
    println!("TIME {}: {}", text, (end - start) / 1000);
}

fn solution1(programs: &str, input: &str, times: u32) -> String {
    let n = programs.len() as u64;
    assert!(n <= 16);

    macro_rules! ind {
        ($e:expr) => ((0..n).map(|i| ($e >> (i * 4)) & 0b1111));
    }

    let mut p : u64 = (0..n)
        .map(|i| i << i * 4).sum();
    let mask : u64 = (0..n)
        .map(|i| 0b1111 << i * 4).sum();

    let dance_moves = input
        .split(',')
        .filter(|x| !x.is_empty())
        .map(|x| dance(x.trim()))
        .filter_map(|x| x)
        .collect::<Vec<_>>();

    let mut map : HashMap<u64, u64> = HashMap::new();

    (0..times).for_each(|i| {
        if i % 10000000 == 0 { println!("i:{}", i) }
        let p0 = p;
        match map.get(&p0) {
            Some(&found) => p = found,
            None => {
                //let start = perf_now();
                dance_moves.iter().for_each(|dance| {
                    match *dance {
                        Dance::Spin(s) => {
                            let mid = n - s as u64;
                            let a = p >> (mid * 4);
                            let b = p << (s * 4);
                            //println!("s{}: {:?} -> {:?}", s, ind(p, n), ind(a | b, n));
                            p = (a | b) & mask;
                        },

                        Dance::Exchange(ij) => {
                            let i = ij & 0b1111;
                            let j = ij >> 4;
                            //println!("x{}/{}: {:?} -> {:?}", i, j, ind(p, n), ind(exchange(p, i, j), n));
                            p = exchange(p, i, j);
                        },

                        Dance::Partner(ab) => {
                            let a = ab & 0b1111;
                            let b = ab >> 4;
                            //println!("p{}/{}: {:?}", a, b, ind(p, n));
                            let i = ind!(p).position(|i| i as u8 == a).unwrap() as u8;
                            let j = ind!(p).position(|i| i as u8 == b).unwrap() as u8;
                            //println!("p{}/{} -> x{}/{}: {:?} -> {:?}", a, b, i, j, ind(p, n), ind(exchange(p, i, j), n));
                            p = exchange(p, i, j);
                        },
                    };

                    // Check if no indexes are lost.
                    //assert_eq!({let mut x = ind(p, n); x.sort(); x}, (0..n).collect::<Vec<_>>());
                });
                //perf(start, "_");
                map.insert(p0, p);
            }
        }
        //println!("{} {:?}", i, ind!(p).collect::<Vec<_>>());
    });

    let bytes = ind!(p)
        .map(|i| programs.as_bytes()[i as usize])
        .collect::<Vec<_>>();

    return String::from_utf8(bytes).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        assert_eq!(solution1("abcde", "s1", 1), "eabcd");
        assert_eq!(solution1("abcde", "s1,x3/4", 1), "eabdc");
        assert_eq!(solution1("abcde", "s1,x3/4,pe/b", 1), "baedc");

        assert_eq!(solution1("abcde", "s1,x3/4,pe/b", 1), "baedc");
        assert_eq!(solution1("abcde", "s1,x3/4,pe/b", 2), "ceadb");

        assert_eq!(solution1("abcde", "pa/b", 1), "bacde");
        assert_eq!(solution1("abcde", "pa/b", 2), "abcde");
        assert_eq!(solution1("abcde", "pa/b,pa/c", 1), "bcade");
        assert_eq!(solution1("abcde", "pa/b,pa/c", 2), "cabde");
        assert_eq!(solution1("abcde", "pa/b,pa/c", 3), "abcde");
    }
}

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    let s1 = solution1("abcdefghijklmnop", &input, 1);
    println!("solution 1: {}", s1);

    let s2 = solution1("abcdefghijklmnop", &input, 1_000_000_000);
    println!("solution 2: {}", s2);
}
