use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Program {
    name: String,
    weight: u32,
    disk: Vec<String>,
}

impl Program {
    pub fn new(name: &str) -> Program {
        Program{
            name: String::from(name),
            weight: 0,
            disk: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
struct Tower {
    program: Program,
    support: Option<String>,
    towers: HashMap<String, Tower>,
}

fn parse_tower(line: &str, towers: &mut HashMap<String, Tower>) {
    let tokens : Vec<_> = line.split(" -> ").collect();
    let program_spec : Vec<_> = tokens[0].split(" ").collect();

    let program_name = program_spec[0];
    let weight_text = program_spec[1];
    let weight = weight_text[1..weight_text.len()-1].parse::<u32>().unwrap();
    assert!(weight > 0);

    let mut disk = Vec::new();

    if tokens.len() == 2 {
        let program_names = tokens[1].split(", ");
        for program_name2 in program_names {
            let ref mut tower = towers.entry(String::from(program_name2))
                .or_insert_with(|| Tower::new(program_name2));
            tower.support = Some(String::from(program_name));
            disk.push(String::from(program_name2));
        }
    }

    let ref mut tower = towers.entry(String::from(program_name))
        .or_insert_with(|| Tower::new(program_name));
    tower.program.weight = weight;
    tower.program.disk.extend(disk.iter().cloned());
}

fn calculate_weights(program: &Program, towers: &HashMap<String, Tower>, weights: &mut HashMap<String, u32>) {
    let mut weight = 0u32;

    for name in &program.disk {
        let tower = towers.get(name).unwrap();
        calculate_weights(&tower.program, &towers, weights);
        weight += weights[&tower.program.name];
    }

    assert!(!weights.contains_key(&program.name));
    weights.insert(program.name.clone(), program.weight + weight);
}

impl Tower {
    pub fn new(program_name: &str) -> Tower {
        Tower{
            program: Program::new(program_name),
            support: None,
            towers: HashMap::new()
        }
    }

    pub fn from_input(input: String) -> Tower {
        let lines = input.split('\n')
            .filter(|x| !x.is_empty());

        let mut towers = HashMap::new();
        lines.for_each(|line| parse_tower(line, &mut towers));
        assert!(towers.iter().all(|(_,v)| v.program.weight > 0));

        let mut bottom = towers.iter().find(|&(_, tower)| tower.support.is_none()).unwrap().1.clone();
        bottom.towers = towers;
        return bottom;
    }

    fn unbalanced_helper(&self, towers: &HashMap<String, Tower>, weights: &HashMap<String, u32>) -> Option<(String, u32)> {
        if self.program.disk.len() < 3 {
            return None;
        }

        for i in 0..self.program.disk.len() {
            let tower = towers.get(&self.program.disk[i]).unwrap();
            let u = tower.unbalanced_helper(towers, weights);
            if u.is_some() {
                return u;
            }
        }

        let w : Vec<_> = weights.iter().filter(|&(k, _)| self.program.disk.contains(k)).collect();
        println!("{:?}", w);

        let w0 = *w[0].1;
        let w1 = *w[1].1;
        let w2 = *w[2].1;
        let expected_weight = if w0 == w1 { w0 } else { w2 };

        match w.iter().find(|&&(_, v)| *v != expected_weight) {
            Some(&(k, v)) => return Some((k.clone(), towers[k].program.weight + expected_weight - v)),
            None => return None,
        }
    }

    pub fn unbalanced(&self) -> Option<(String, u32)> {
        let mut weights: HashMap<String, u32> = HashMap::new();
        calculate_weights(&self.program, &self.towers, &mut weights);
        return self.unbalanced_helper(&self.towers, &weights);
    }
}

fn solution1(input: String) -> (String, u32) {
    let tower = Tower::from_input(input);
    let bottom_program_name = tower.program.name.clone();
    let (_, w) = tower.unbalanced().unwrap();
    return (bottom_program_name, w);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
            pbga (66)\n\
            xhth (57)\n\
            ebii (61)\n\
            havc (66)\n\
            ktlj (57)\n\
            fwft (72) -> ktlj, cntj, xhth\n\
            qoyq (66)\n\
            padx (45) -> pbga, havc, qoyq\n\
            tknk (41) -> ugml, padx, fwft\n\
            jptl (61)\n\
            ugml (68) -> gyxo, ebii, jptl\n\
            gyxo (61)\n\
            cntj (57)\n";

        let tower = Tower::from_input(String::from(input));
        assert_eq!(tower.program.name, "tknk");
        assert_eq!(tower.unbalanced(), Some((String::from("ugml"), 60)));
    }
}

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    let (s, w) = solution1(input);
    println!("solution: {} {}", s, w);
}
