extern crate time;
use time::PreciseTime;

/*
Memory layout:
17  16  15  14  13 ..
18   5   4   3  12 29
19   6   1   2  11 28
20   7   8   9  10 27
21  22  23  24  25 26

location of X (x,y) from center, distance abs(x) + abs(y)
location of 1 (0,0) from center, distance 0
location of 2 (1,0) from center, distance 1
location of 3 (1,1) from center, distance 2
location of 4 (0,1) from center, distance 1
location of 5 (-1,1) from center, distance 2
location of 6 (-1,0) from center, distance 1
location of 7 (-1,-1) from center, distance 2
location of 8 (0,-1) from center, distance 1
location of 9 (1,-1) from center, distance 2
location of 10 (2,-1) from center, distance 3

distance of 25 is 4
distance of 24 is 3
distance of 23 is 2
distance of 22 is 3
distance of 21 is 4
distance of 20 is 3
distance of 19 is 2
distance of 18 is 3

which square? 1x1, 3x3, 5x5 ...
*/

fn solution1(input: &str) -> u32 {
    let n : u32 = input.parse().unwrap();
    let side = ((n as f32).sqrt().ceil() as u32) | 1;
    let end = side.pow(2);
    let half_side = (side + 1) / 2;
    let rem = (n - 1) % (side - 1);
    let distance = if rem < half_side {side - 1 - rem} else {rem};
    println!("n={} side={} end={} rem={} d={}", n, side, end, rem, distance);
    return distance;
}

/*
147  142  133  122   59
304    5    4    2   57
330   10    1    1   54
351   11   23   25   26
362  747  806--->   ...
*/

fn solution2(input: &str) -> u32 {
    let n : u32 = input.parse().unwrap();
    let side = ((n as f32).sqrt().ceil() as u32) | 1;
    let end = side.pow(2);
    let half_side = (side + 1) / 2;
    let rem = (n - 1) % (side - 1);
    let distance = if rem < half_side {side - 1 - rem} else {rem};
    println!("n={} side={} end={} rem={} d={}", n, side, end, rem, distance);
    return distance;
}

fn main() {
    let input = "325489";

    //assert_eq!(solution1("1"), 0);
    assert_eq!(solution1("9"), 2);
    assert_eq!(solution1("8"), 1);
    assert_eq!(solution1("7"), 2);
    assert_eq!(solution1("6"), 1);
    assert_eq!(solution1("5"), 2);
    assert_eq!(solution1("4"), 1);
    assert_eq!(solution1("3"), 2);
    assert_eq!(solution1("2"), 1);

    assert_eq!(solution1("25"), 4);
    assert_eq!(solution1("24"), 3);
    assert_eq!(solution1("23"), 2);
    assert_eq!(solution1("22"), 3);
    assert_eq!(solution1("21"), 4);
    assert_eq!(solution1("20"), 3);
    assert_eq!(solution1("19"), 2);
    assert_eq!(solution1("18"), 3);
    assert_eq!(solution1("17"), 4);

    assert_eq!(solution1("10"), 3);
    assert_eq!(solution1("11"), 2);

    {
    let start = PreciseTime::now();
    let s = solution1(input);
    let end = PreciseTime::now();
    println!("solution: {} ({:?})", s, start.to(end));
    }

    assert_eq!(solution2("2"), 4);
    assert_eq!(solution2("4"), 5);
    assert_eq!(solution2("5"), 10);
    assert_eq!(solution2("10"), 11);
    assert_eq!(solution2("23"), 25);
    assert_eq!(solution2("25"), 26);
    assert_eq!(solution2("26"), 54);

    {
    let start = PreciseTime::now();
    let s = solution1(input);
    let end = PreciseTime::now();
    println!("solution: {} ({:?})", s, start.to(end));
    }
}
