mod input;
use input::input;

fn main() {
    // let input = "
    // 1

    // 2

    // 3

    // 4

    // 5";
    let elf_from_line = |l: &'_ str| {
        let lines = l.split_ascii_whitespace();

        Elf {
            meals: lines.filter_map(|x| x.parse().ok()).collect(),
        }
    };
    let elves: Vec<Elf> = input.split("\n\n").map(elf_from_line).collect();

    let calorieSums = elves.iter().map(|e| e.meals.iter().sum::<u64>());

    let mut top3 = [0, 0, 0];

    for s in calorieSums {
        for i in (0..top3.len()).rev() {
            if top3[i] < s {
                // shift other values down
                // println!("{:?}", top3);
                for j in 0..i {
                    // println!("{} = {}", top3[j], top3[j + 1]);
                    top3[j] = top3[j + 1];
                }
                // 4
                // 1 2 3
                // 2 3 4

                top3[i] = s;
                break;
            }
        }
    }

    println!("{:?}", top3.iter().sum::<u64>());
}

#[derive(Debug, Clone)]
struct Elf {
    pub meals: Vec<u64>,
}
