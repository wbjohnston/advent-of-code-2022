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

    let len = input.len();
    let mut visibility_scores: Vec<Vec<_>> = vec![vec![0; len]; len];

    for i in 1..len - 1 {
        for j in 1..len - 1 {
            let height = input[i][j];

            let current_score = &mut visibility_scores[i][j];
            // right
            let mut right_score = 0;
            for j in (j + 1)..len {
                let curr = input[i][j];

                right_score += 1;

                if curr >= height {
                    break;
                }
            }

            // left
            let mut left_score = 0;
            for j in (0..j).rev() {
                let curr = input[i][j];
                left_score += 1;

                if curr >= height {
                    break;
                }
            }

            // down
            let mut down_score = 0;
            for i in (i + 1)..len {
                let curr = input[i][j];

                down_score += 1;

                if curr >= height {
                    break;
                }
            }

            // up
            let mut up_score = 0;
            for i in (0..i).rev() {
                let curr = input[i][j];

                up_score += 1;

                if curr >= height {
                    break;
                }
            }

            let score = up_score * down_score * right_score * left_score;
            println!(
                "({}, {}) -> U {} D {} L {} R {} {}",
                i, j, up_score, down_score, right_score, left_score, score
            );

            *current_score = up_score * down_score * right_score * left_score;
        }
    }

    println!(
        "{}",
        visibility_scores
            .into_iter()
            .map(|x| x.into_iter().max().unwrap())
            .max()
            .unwrap()
    );
}
