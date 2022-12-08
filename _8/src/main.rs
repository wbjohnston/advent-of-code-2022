mod input;

fn num_visible(trees: impl DoubleEndedIterator<Item = u32> + Clone) -> usize {
    fn num_visibile_from_left(trees: impl Iterator<Item = u32>) -> usize {
        let mut min_height = 0;
        let mut visible = 0;
        for t in trees {
            println!("{}", t);
            if t > min_height {
                min_height = t;
                visible += 1;
            }
        }

        visible
        // tress.red
    }

    num_visibile_from_left(trees.clone()) + num_visibile_from_left(trees.rev()) - 1
}

fn main() {
    let input: Vec<Vec<u32>> = input::INPUT
        .split("\n")
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let mut visibility_map: Vec<Vec<_>> = input
        .iter()
        .map(|row| row.iter().map(|_| false).collect())
        .collect();

    let len = visibility_map.len();

    for i in 0..len {
        // top row
        visibility_map[0][i] = true;

        // bottom row
        visibility_map[len - 1][i] = true;

        // left column
        visibility_map[i][0] = true;

        // right column
        visibility_map[i][len - 1] = true;
    }

    for i in 0..input.len() {
        // left to right
        {
            let mut min_height = 0;
            for j in 0..len {
                let current = input[i][j];
                visibility_map[i][j] = current > min_height || visibility_map[i][j];
                min_height = min_height.max(current);
            }
        }
        // right to left
        {
            let mut min_height = 0;
            for j in (0..len).rev() {
                let current = input[i][j];
                visibility_map[i][j] = current > min_height || visibility_map[i][j];
                min_height = min_height.max(current);
            }
        }

        // top to bottom
        {
            let mut min_height = 0;
            for j in 0..len {
                let current = input[j][i];
                visibility_map[j][i] = current > min_height || visibility_map[j][i];
                min_height = min_height.max(current);
                println!("{} {}", i, j);
                // for row in visibility_map.iter() {
                //     for &t in row {
                //         print!("{}", if t { "T" } else { "." });
                //     }
                //     println!("");
                // }
                // println!("-----");
            }
        }

        // bottom to top
        {
            let mut min_height = 0;
            for j in (0..len).rev() {
                let current = input[j][i];
                visibility_map[j][i] = current > min_height || visibility_map[j][i];
                min_height = min_height.max(current);
            }
        }

        // for row in visibility_map.iter() {
        //     for &t in row {
        //         print!("{}", if t { "T" } else { "." });
        //     }
        //     println!("");
        // }
        // println!("=====");
    }

    let visible: usize = visibility_map
        .iter()
        .map(|r| r.iter().filter(|&&x| x).count())
        .sum();

    for row in visibility_map.iter() {
        for &t in row {
            print!("{}", if t { "T" } else { "." });
        }
        println!("");
    }

    println!("{}", visible);
}
