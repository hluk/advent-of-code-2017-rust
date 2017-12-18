use std::fs::File;
use std::io::prelude::*;

use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct XYZ {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Copy, Clone, Debug)]
struct Particle {
    p: XYZ,
    v: XYZ,
    a: XYZ,
}

impl XYZ {
    fn from_string(input: &str) -> XYZ {
        let mut triplet = input
            .split(',')
            .map(|x| x.parse::<i64>().unwrap());

        return XYZ {
            x: triplet.next().unwrap(),
            y: triplet.next().unwrap(),
            z: triplet.next().unwrap(),
        };
    }

    fn manhattan(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Particle {
    fn from_string(input: &str) -> Particle {
        let mut triplets = input
            .split(">")
            .map(|token| XYZ::from_string(token.split('<').nth(1).unwrap()) );

        return Particle {
            p: triplets.next().unwrap(),
            v: triplets.next().unwrap(),
            a: triplets.next().unwrap(),
        };
    }

    fn position_after_time(&self, t: i64) -> XYZ {
        XYZ{
            x: self.p.x + self.v.x * t + t * t * self.a.x / 2,
            y: self.p.y + self.v.y * t + t * t * self.a.y / 2,
            z: self.p.z + self.v.z * t + t * t * self.a.z / 2,
        }
    }
}

fn solution1(input: &String) -> usize {
    let t = 1024;
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| Particle::from_string(line))
        .map(|particle| {
            let p = particle.p;
            let v = particle.v;
            let a = particle.a;
            let pt = particle.position_after_time(t);
            let delta_d = (p.manhattan() - pt.manhattan()).abs();
            (a.manhattan(), v.manhattan(), delta_d)
        })
        .enumerate()
        .min_by_key(|&(_, x)| x)
        .unwrap().0
}

fn solution2(input: &String) -> usize {
    let mut particles : Vec<Particle> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| Particle::from_string(line))
        .collect();

    let mut positions : HashSet<XYZ> = HashSet::new();
    let mut colliding_positions : HashSet<XYZ> = HashSet::new();

    for _ in 0..100_000 {
        positions.clear();
        colliding_positions.clear();

        for x in &mut particles {
            x.v.x += x.a.x;
            x.v.y += x.a.y;
            x.v.z += x.a.z;

            x.p.x += x.v.x;
            x.p.y += x.v.y;
            x.p.z += x.v.z;

            if !positions.insert(x.p) {
                colliding_positions.insert(x.p);
            }
        }
        particles.retain(|x| !colliding_positions.contains(&x.p));
        if particles.len() < 2 {
            break;
        }
    }

    return particles.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1_1() {
        let input = String::new()
            + "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>\n"
            + "p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>\n";

        assert_eq!(solution1(&input), 0);
    }

    #[test]
    fn test_solution1_position() {
        let input = String::new()
            + "p=<0,0,0>, v=<0,0,0>, a=<1,0,0>\n"
            + "p=<-10000,0,0>, v=<0,0,0>, a=<1,0,0>\n";

        assert_eq!(solution1(&input), 1);
    }

    #[test]
    fn test_solution1_position2() {
        let input = String::new()
            + "p=<0,0,0>, v=<0,0,0>, a=<1,0,0>\n"
            + "p=<10000,0,0>, v=<0,0,0>, a=<-1,0,0>\n";

        assert_eq!(solution1(&input), 1);
    }

    #[test]
    fn test_solution1_velocity() {
        let input = String::new()
            + "p=<0,0,0>, v=<0,0,2>, a=<0,0,0>\n"
            + "p=<0,0,0>, v=<1,0,0>, a=<0,0,0>\n";

        assert_eq!(solution1(&input), 1);
    }

    #[test]
    fn test_solution1_acceleration() {
        let input = String::new()
            + "p=<0,0,0>, v=<0,0,0>, a=<1,0,1>\n"
            + "p=<0,0,0>, v=<0,0,0>, a=<0,0,1>\n";

        assert_eq!(solution1(&input), 1);
    }

    #[test]
    fn test_solution2_parallel() {
        let input = String::new()
            + "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>\n"
            + "p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>\n"
            + "p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>\n"
            + "p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>\n";

        assert_eq!(solution2(&input), 1);
    }

    #[test]
    fn test_solution2_facing() {
        let input = String::new()
            + "p=<1,0,0>, v=<-1,0,0>, a=<0,0,0>\n"
            + "p=<-1,0,0>, v=<1,0,0>, a=<0,0,0>\n";

        assert_eq!(solution2(&input), 0);
    }
}

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    let s1 = solution1(&input);
    println!("solution1: {}", s1);

    let s2 = solution2(&input);
    println!("solution2: {}", s2);
}
