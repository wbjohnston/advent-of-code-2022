mod input;
use input::input;

fn main() {
    let elf_from_line = |l: &'_ str| {
        let lines = l.split_ascii_whitespace();

        Elf {
            meals: lines.filter_map(|x| x.parse().ok()).collect(),
        }
    };
    let elves: Vec<Elf> = input.split("\n\n").map(elf_from_line).collect();

    let calorieSums = elves.iter().map(|e| e.meals.iter().sum::<u64>());

    println!("{:?}", calorieSums.max());
}

#[derive(Debug, Clone)]
struct Elf {
    pub meals: Vec<u64>,
}
