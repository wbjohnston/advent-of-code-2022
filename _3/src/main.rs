use std::collections::{HashMap, HashSet};

mod input;

fn main() {
    let input = input::input;
    let lines = input.split("\n");
    let priorities = priority_map();
    let x: u64 = lines
        .map(|l| {
            let len = l.len();
            let midpoint = l.len() / 2;
            let first_compartment = l.chars().take(midpoint).collect();
            let second_compartment = l.chars().skip(midpoint).collect();
            Rucksack {
                compartments: (first_compartment, second_compartment),
            }
        })
        .map(|x| x.get_common_item())
        .map(|x| priorities.get(&x).unwrap())
        .sum::<u64>();

    println!("{}", x);

    // println!("{:?}", priorities);

    // println!("{:?}", rucksacks);
}

fn priority_map() -> HashMap<char, u64> {
    let mut map = HashMap::new();
    // upper case letters
    for i in 65u8..=90 {
        map.insert(i as char, (i - 65 + 27) as u64);
    }

    // lower case
    for i in 97u8..=122 {
        map.insert(i as char, (i - 96) as u64);
    }

    map
}

#[derive(Debug, Clone)]
struct Rucksack {
    compartments: (HashSet<char>, HashSet<char>),
}

impl Rucksack {
    fn get_common_item(&self) -> char {
        let intersection: Vec<_> = self
            .compartments
            .0
            .intersection(&self.compartments.1)
            .collect();

        *intersection[0]
    }
}
