#[derive(Copy, Clone)]
struct Generator {
    value: u64,
    factor: u64,
}

impl Generator {
    fn next(&mut self) -> u64 {
        const DIV : u64 = 2147483647;
        self.value = (self.value * self.factor) % DIV;
        return self.value;
    }

    fn next_m(&mut self, m: u64) -> u64 {
        self.next();
        while self.value % m != 0 {
            self.next();
        }
        return self.value;
    }
}

fn low16_match(lhs: u64, rhs: u64) -> bool {
    const MASK: u64 = 0b1111_1111_1111_1111;
    lhs & MASK == rhs & MASK
    //lhs << (64-16) == rhs << (64-16)
}

fn solution1(a0: Generator, b0: Generator) -> usize {
    let mut a = a0;
    let mut b = b0;

    return (0..40_000_000)
        //.map(|_| (a.next(), b.next()))
        //.filter(|&(a, b)| low16_match(a, b) )
        .filter(|_| low16_match(a.next(), b.next()) )
        .count();
}

fn solution2(a0: Generator, b0: Generator) -> usize {
    let mut a = a0;
    let mut b = b0;

    return (0..5_000_000)
        .filter(|_| low16_match(a.next_m(4), b.next_m(8)) )
        .count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_a() {
        let mut a = Generator{value: 65, factor: 16807};
        assert_eq!(a.next(), 1092455);
        assert_eq!(a.next(), 1181022009);
        assert_eq!(a.next(), 245556042);
        assert_eq!(a.next(), 1744312007);
        assert_eq!(a.next(), 1352636452);
    }

    #[test]
    fn test_generator_b() {
        let mut b = Generator{value: 8921, factor: 48271};
        assert_eq!(b.next(), 430625591);
        assert_eq!(b.next(), 1233683848);
        assert_eq!(b.next(), 1431495498);
        assert_eq!(b.next(), 137874439);
        assert_eq!(b.next(), 285222916);
    }

    #[test]
    fn test_generator_a2() {
        let mut a = Generator{value: 65, factor: 16807};
        assert_eq!(a.next_m(4), 1352636452);
        assert_eq!(a.next_m(4), 1992081072);
        assert_eq!(a.next_m(4), 530830436);
        assert_eq!(a.next_m(4), 1980017072);
        assert_eq!(a.next_m(4), 740335192);
    }

    #[test]
    fn test_generator_b2() {
        let mut b = Generator{value: 8921, factor: 48271};
        assert_eq!(b.next_m(8), 1233683848);
        assert_eq!(b.next_m(8), 862516352);
        assert_eq!(b.next_m(8), 1159784568);
        assert_eq!(b.next_m(8), 1616057672);
        assert_eq!(b.next_m(8), 412269392);
    }

    #[test]
    fn test_low16_match() {
        assert_eq!(low16_match(0, 0), true);
        assert_eq!(low16_match(1, 1), true);
        assert_eq!(low16_match(1, 0), false);
        assert_eq!(low16_match(0, 1), false);
        assert_eq!(low16_match(245556042, 1431495498), true);
        assert_eq!(low16_match(0b1111_1111_1111_1111, 0b1111_1111_1111_1111), true);
        assert_eq!(low16_match(0b0111_1111_1111_1111, 0b1111_1111_1111_1111), false);
        assert_eq!(low16_match(0b1111_1111_1111_1110, 0b1111_1111_1111_1111), false);
        assert_eq!(low16_match(0b1111_1111_1111_1111_1111, 0b0000_1111_1111_1111_1111), true);
        assert_eq!(low16_match(0b1111_1111_1111_1111_1111, 0b1111_1111_1111_1111_1111), true);
    }

    #[test]
    fn test_solution1() {
        let a = Generator{value: 65, factor: 16807};
        let b = Generator{value: 8921, factor: 48271};
        assert_eq!(solution1(a, b), 588);
    }

    #[test]
    fn test_solution2() {
        let a = Generator{value: 65, factor: 16807};
        let b = Generator{value: 8921, factor: 48271};
        assert_eq!(solution2(a, b), 309);
    }
}

fn main() {
    let a = Generator{value: 699, factor: 16807};
    let b = Generator{value: 124, factor: 48271};

    let s1 = solution1(a, b);
    println!("solution 1: {}", s1);

    let s2 = solution2(a, b);
    println!("solution 2: {}", s2);
}
