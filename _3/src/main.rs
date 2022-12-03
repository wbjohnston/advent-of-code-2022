use itertools::Itertools;
use std::collections::{HashMap, HashSet};

mod input;

fn main() {
    let input = input::input;
    let lines = input.split("\n");
    let priorities = priority_map();
    let x: u64 = lines
        .map(|l| HashSet::from_iter(l.chars()))
        .chunks(3)
        .into_iter()
        .map(|mut c| {
            let (first, second, third) = c.next_tuple().unwrap();

            let first_inter: HashSet<char> = first
                .clone()
                .intersection(&second)
                .into_iter()
                .map(|x| *x)
                .collect();
            let second_inter: Vec<_> = first_inter.intersection(&third).collect();

            second_inter[0].clone()
        })
        .map(|i| priorities.get(&i).unwrap())
        .sum();

    println!("here");
    println!("{:?}", x);
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
