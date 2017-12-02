use std::fs::File;
use std::io::prelude::*;

fn is_valid_passphrase(line: &str, f: fn(lhs: &str, rhs: &str) -> bool) -> bool {
    let words = line.split(' ').collect::<Vec<_>>();
    //println!("{}:", line);
    return words.iter()
        .enumerate()
        //.inspect(|x| println!("- {:?}", x))
        .all(|(index, word)| {
            words.iter()
                .skip(index + 1)
                //.inspect(|x| println!("-- {}", x))
                .all(|word2| f(word, word2))
        });
}

fn solution1(input: String, f: fn(lhs: &str, rhs: &str) -> bool) -> u32 {
    let lines = input.split('\n')
        .filter(|x| !x.is_empty());
    let count = lines.fold(0, |acc, line| if is_valid_passphrase(line, f) {acc + 1} else {acc});
    return count;
}

fn not_same(lhs: &str, rhs: &str) -> bool {
    return lhs != rhs;
}

fn anagram<'a>(word: &'a str) -> Vec<char> {
    let mut chars = word.chars().collect::<Vec<_>>();
    chars.sort();
    return chars;
}

fn not_anagram(lhs: &str, rhs: &str) -> bool {
    //println!("{:?} {:?}", anagram(lhs), anagram(rhs));
    return anagram(lhs) != anagram(rhs);
}

fn main() {
    assert_eq!(solution1(String::from("aa bb cc dd ee"), not_same), 1);
    assert_eq!(solution1(String::from("aa bb cc dd aa"), not_same), 0);
    assert_eq!(solution1(String::from("aa bb cc dd aaa"), not_same), 1);

    let mut input_file = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    let s1 = solution1(input.clone(), not_same);
    println!("solution: {}", s1);

    assert_eq!(solution1(String::from("abcde fghij"), not_anagram), 1);
    assert_eq!(solution1(String::from("abcde xyz ecdab"), not_anagram), 0);
    assert_eq!(solution1(String::from("a ab abc abd abf abj"), not_anagram), 1);
    assert_eq!(solution1(String::from("iiii oiii ooii oooi oooo"), not_anagram), 1);
    assert_eq!(solution1(String::from("oiii ioii iioi iiio"), not_anagram), 0);

    let s1 = solution1(input, not_anagram);
    println!("solution: {}", s1);
}
