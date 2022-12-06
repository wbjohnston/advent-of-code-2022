use std::str::FromStr;

mod input;

fn main() {
    /*
            [M]     [B]             [N]
    [T]     [H]     [V] [Q]         [H]
    [Q]     [N]     [H] [W] [T]     [Q]
    [V]     [P] [F] [Q] [P] [C]     [R]
    [C]     [D] [T] [N] [N] [L] [S] [J]
    [D] [V] [W] [R] [M] [G] [R] [N] [D]
    [S] [F] [Q] [Q] [F] [F] [F] [Z] [S]
    [N] [M] [F] [D] [R] [C] [W] [T] [M]
    1   2   3   4   5   6   7   8   9
     */
    let mut input = vec![
        vec!['N', 'S', 'D', 'C', 'V', 'Q', 'T'],
        vec!['M', 'F', 'V'],
        vec!['F', 'Q', 'W', 'D', 'P', 'N', 'H', 'M'],
        vec!['D', 'Q', 'R', 'T', 'F'],
        vec!['R', 'F', 'M', 'N', 'Q', 'H', 'V', 'B'],
        vec!['C', 'F', 'G', 'N', 'P', 'W', 'Q'],
        vec!['W', 'F', 'R', 'L', 'C', 'T'],
        vec!['T', 'Z', 'N', 'S'],
        vec!['M', 'S', 'D', 'J', 'R', 'Q', 'H', 'N'],
    ];

    // reverse because I put it in backwards
    // for mut c in &mut input {
    //     c.reverse();
    // }

    let instructions: Vec<_> = input::INPUT
        .split("\n")
        .map(|l| Instruction::from_str(l).unwrap())
        .collect();
    let mut move_buffer = vec![];

    for i in instructions {
        let Instruction { amount, from, to } = i;

        for _ in 0..amount {
            let x = input[from - 1].pop().unwrap();
            move_buffer.push(x);
        }

        // move_buffer.reverse();

        while let Some(x) = move_buffer.pop() {
            input[to - 1].push(x);
        }
    }

    for col in input.iter() {
        print!("{}", col.last().unwrap());
    }

    println!("");

    // dbg!(input);
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" ").filter_map(|x| x.parse().ok());
        let i = Instruction {
            amount: iter.next().unwrap(),
            from: iter.next().unwrap(),
            to: iter.next().unwrap(),
        };

        Ok(i)
    }
}
