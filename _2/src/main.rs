use std::str::FromStr;
mod input;

#[derive(Debug, Clone, Copy)]
enum GameResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GameResult::*;
        match s {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => unimplemented!(),
        }
    }
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

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    pub fn loses_to(self) -> Shape {
        use Shape::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    pub fn wins_to(self) -> Shape {
        use Shape::*;
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
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
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            c => panic!("{} not implementated", c),
        }
    }
}

struct Game {
    pub them: Shape,
    pub result: GameResult,
}

impl Game {
    pub fn shape_needed(&self) -> Shape {
        use GameResult::*;
        use Shape::*;
        match self.result {
            Win => self.them.loses_to(),
            Draw => self.them,
            Loss => self.them.wins_to(),
        }
    }

    // pub fn score(&self) -> GameResult {
    //     use GameResult::*;
    //     use Shape::*;
    //     match (&self.you, &self.them) {
    //         (Rock, Scissors) => Win,
    //         (Rock, Paper) => Loss,
    //         (Paper, Rock) => Win,
    //         (Paper, Scissors) => Loss,
    //         (Scissors, Paper) => Win,
    //         (Scissors, Rock) => Loss,
    //         _ => Draw,
    //     }
    // }
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
        let mut shapes = l.split(" ");

        Game {
            them: shapes.next().map(|x| Shape::from_str(x).unwrap()).unwrap(),
            result: shapes
                .next()
                .map(|x| GameResult::from_str(x).unwrap())
                .unwrap(),
        }
    });

    let scores: Vec<u64> = games
        .map(|x| u64::from(x.result) + u64::from(x.shape_needed()))
        .collect();
    let total: u64 = scores.iter().sum();

    println!("{:?}", total);
}
