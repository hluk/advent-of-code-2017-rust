use std::collections::HashSet;

fn solution1(input: &str) -> (i32, usize) {
    let mut banks = input.split('\t')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let bank_count = banks.len();
    let mut seen_banks_set = HashSet::new();
    let mut seen_banks = Vec::new();
    let mut iterations = 0i32;
    let sum : u32 = banks.iter().sum();

    while !seen_banks_set.contains(&banks) {
        iterations += 1;
        seen_banks_set.insert(banks.clone());
        seen_banks.push(banks.clone());
        let (max_index, &max_value) = banks.iter().enumerate().max_by_key(|&(index, x)| (x, bank_count - index)).unwrap();

        let mut rem = max_value;
        let value = (max_value as f32 / bank_count as f32).ceil() as u32;
        print!("{:>2}. {}:{} {} {:?}", iterations, max_index, max_value, value, banks);

        banks[max_index] = 0;
        let mut index = max_index;
        while rem > 0 {
            index = (index + 1) % bank_count;
            banks[index] += 1;
            rem -= 1;
        }
        println!(" -> {:?}", banks);

        assert_eq!(sum, banks.iter().sum());
    }

    let loop_size = seen_banks.len() - seen_banks.iter().position(|x| *x == banks).unwrap();
    return (iterations, loop_size);
}

fn main() {
    let input = "10	3	15	10	5	15	5	15	9	2	5	8	5	2	3	6";

    assert_eq!(solution1("0\t2\t7\t0"), (5, 4));

    let (iterations, loop_size) = solution1(input);
    println!("solution: {} {}", iterations, loop_size);
}
