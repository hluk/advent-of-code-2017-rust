fn knot_hash_helper1(lengths: Vec<u16>, number_of_items: u16, number_of_rounds: u16) -> Vec<u16> {
    let mut items : Vec<u16> = (0..number_of_items).collect();

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

fn knot_hash_helper2(items: Vec<u16>) -> Vec<u8> {
    assert_eq!(items.len(), 256);
    return (0..16)
        .map(|i| items[i*16..i*16+16]
             .iter()
             .inspect(|&&x| assert!(x < 256))
             .fold(0u8, |acc, &x| acc ^ (x as u8)))
        .collect();
}

fn knot_hash(input: &String) -> Vec<u8> {
    let lengths = input
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| c as u16)
        .chain([17u16, 31u16, 73u16, 47u16, 23u16].iter().map(|&x| x))
        .collect::<Vec<_>>();
    let items = knot_hash_helper1(lengths, 256, 64);
    return knot_hash_helper2(items);
}

fn bit(x: u8, b: u8) -> bool {
    return ((x >> b) & 1) == 1;
}

fn solution1(input: &String) -> u32 {
    let mut count = 0u32;
    (0..128).for_each(|i| {
        let text = format!("{}-{}", input, i.to_string());
        knot_hash(&text)
            .iter()
            .for_each(|&x| {
                count += (0..8).map(|b| bit(x, b) as u32).sum::<u32>();
                //println!("{:03} {:08b} {}", x, x, count);
            });
    });
    return count;
}

fn p(x: bool) -> char {
    if x {'#'} else {'.'}
}

fn solution2(input: &String) -> u32 {
    let mut count = 0u32;
    let mut q = [false; 128];
    let mut a = [false; 128];
    (0..128).for_each(|i| {
        let text = format!("{}-{}", input, i.to_string());
        let r = knot_hash(&text)
            .iter()
            .flat_map(|&x| {
                (0..8).map(|b| bit(x, b)).collect::<Vec<bool>>()
            })
            .collect::<Vec<bool>>();
        count += (0..128)
            .fold((false, false, 0u8), |(x0, y0, count), i| {
                let x = r[i];
                let y = q[i];
                let ref mut a = a[i];
                let result = match (x, x0, y, y0) {
                    //_.
                    //.#
                    (true, false, false, _) => { *a = true; (true, false, count + 1) },
                    //..
                    //##
                    (true, true, false, false) => (true, false, count),
                    //.#
                    //##
                    (true, true, true, false) => (true, true, count - 1),
                    //#.
                    //##
                    (true, true, false, true) => (true, true, count),
                    //##
                    //##
                    (true, true, true, true) => if *a {(true, true, count - 1)} else {(true, true, count)},
                    //?#
                    //.#
                    (true, false, true, _) => (true, true, count),
                    //??
                    //?.
                    (false, _, _, _) => (false, false, count),
                };
                //println!("{}{}", p(y0), p(y));
                //println!("{}{}", p(x0), p(x));
                //println!("---- {}", result.2);
                result
            }).2 as u32;
        q.clone_from_slice(r.as_slice());
    });
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knot_hash() {
        assert_eq!(knot_hash(&String::from("")), vec![0xa2, 0x58, 0x2a, 0x3a, 0x0e, 0x66, 0xe6, 0xe8, 0x6e, 0x38, 0x12, 0xdc, 0xb6, 0x72, 0xa2, 0x72]);
        assert_eq!(knot_hash(&String::from("\n")), vec![0xa2, 0x58, 0x2a, 0x3a, 0x0e, 0x66, 0xe6, 0xe8, 0x6e, 0x38, 0x12, 0xdc, 0xb6, 0x72, 0xa2, 0x72]);
        assert_eq!(knot_hash(&String::from("AoC 2017")), vec![0x33, 0xef, 0xeb, 0x34, 0xea, 0x91, 0x90, 0x2b, 0xb2, 0xf5, 0x9c, 0x99, 0x20, 0xca, 0xa6, 0xcd]);
        assert_eq!(knot_hash(&String::from("AoC 2017\n")), vec![0x33, 0xef, 0xeb, 0x34, 0xea, 0x91, 0x90, 0x2b, 0xb2, 0xf5, 0x9c, 0x99, 0x20, 0xca, 0xa6, 0xcd]);
        assert_eq!(knot_hash(&String::from("1,2,3")), vec![0x3e, 0xfb, 0xe7, 0x8a, 0x8d, 0x82, 0xf2, 0x99, 0x79, 0x03, 0x1a, 0x4a, 0xa0, 0xb1, 0x6a, 0x9d]);
        assert_eq!(knot_hash(&String::from("1,2,4")), vec![0x63, 0x96, 0x08, 0x35, 0xbc, 0xdc, 0x13, 0x0f, 0x0b, 0x66, 0xd7, 0xff, 0x4f, 0x6a, 0x5a, 0x8e]);
    }

    #[test]
    fn test_solution1() {
        //assert_eq!(solution1(&String::from("flqrgnkx")), 8108);
        assert_eq!(solution2(&String::from("flqrgnkx")), 1242);
    }
}

fn main() {
    let input = String::from("hfdlxzhv");

    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    //let s2 = solution2(&input);
    //println!("solution 2: {}", s1);
}
