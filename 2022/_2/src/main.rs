use std::str::FromStr;
mod input;

enum GameResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl From<GameResult> for u64 {
    fn from(val: GameResult) -> Self {
        use GameResult::*;
        match val {
            Loss => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<Shape> for u64 {
    fn from(val: Shape) -> Self {
        use Shape::*;
        match val {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
            _ => unimplemented!(),
        }
    }
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            c => panic!("{} not implementated", c),
        }
    }
}

struct Game {
    pub you: Shape,
    pub them: Shape,
}

impl Game {
    pub fn score(&self) -> GameResult {
        use GameResult::*;
        use Shape::*;
        match (&self.you, &self.them) {
            (Rock, Scissors) => Win,
            (Rock, Paper) => Loss,
            (Paper, Rock) => Win,
            (Paper, Scissors) => Loss,
            (Scissors, Paper) => Win,
            (Scissors, Rock) => Loss,
            _ => Draw,
        }
    }
}

fn main() {
    //     let input = "A Y
    // B X
    // C Z";
    let input = input::input;
    // let game_from_line = |l| {
    //     let mut shapes = l.split(" ").map(|x| Shape::from_str(x).unwrap());

    //     Game {
    //         you: shapes.next(),
    //         them: shapes.next(),
    //     }
    // };

    let games = input.split("\n").map(|l| {
        let mut shapes = l.split(" ").map(|x| Shape::from_str(x).unwrap());

        Game {
            them: shapes.next().unwrap(),
            you: shapes.next().unwrap(),
        }
    });

    let scores: Vec<u64> = games
        .map(|x| u64::from(x.score()) + u64::from(x.you))
        .collect();
    println!("{:?}", scores);
    let total: u64 = scores.iter().sum();

    println!("{:?}", total);
}
