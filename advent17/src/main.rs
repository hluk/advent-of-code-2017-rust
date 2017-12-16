fn solution1(input: usize, times: usize) -> usize {
    let mut b : Vec<usize> = Vec::with_capacity(times+1);
    b.push(0);
    let mut i = 0usize;
    let mut v = 0;

    (0..times).for_each(|j| {
        if j % 10000 == 0 {
            println!("j:{}", j)
        }
        i = ((i + input) % b.len()) + 1;
        v += 1;
        b.insert(i, v);
    });

    return b[i + 1];
}

fn solution2(input: usize, times: usize) -> usize {
    let mut len = 1;
    let mut i = 0usize;
    let mut v = 0;
    let mut v1 = 0;

    (0..times).for_each(|j| {
        if j % 1000000 == 0 {
            println!("j:{}", j)
        }

        i = ((i + input) % len) + 1;
        v += 1;
        if i == 1 {
            v1 = v;
        }
        len += 1;
    });

    return v1;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(3, 2017), 638);
    }
}

fn main() {
    let input = 303;

    let s1 = solution1(input, 2017);
    println!("solution 1: {}", s1);

    let s2 = solution2(input, 50000000);
    println!("solution 2: {}", s2);
}
