use rand::Rng;
use std::cmp;
use std::collections::BinaryHeap;

type ScoreType = isize;

const H: usize = 5;
const W: usize = 5;
const END_TURN: usize = 5;
const CHARACTER_N: usize = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Coord {
    y_: isize,
    x_: isize,
}

impl Coord {
    fn new() -> Self {
        Coord { y_: 0, x_: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AutoMoveMazeState {
    points_: Vec<Vec<usize>>,
    turn_: usize,
    character_: Vec<Coord>,
    game_score_: usize,
    evaluated_score_: ScoreType,
}

impl AutoMoveMazeState {
    fn new(seed: Option<u64>) -> Self {
        let mut rng: rand::rngs::StdRng =
            rand::SeedableRng::seed_from_u64(rand::thread_rng().gen());
        if let Some(s) = seed {
            rng = rand::SeedableRng::seed_from_u64(s);
        }
        let mut points_ = vec![vec![0usize; W]; H];
        for y in 0..H {
            for x in 0..W {
                points_[y][x] = rng.gen_range(1..=9);
            }
        }
        let character_ = vec![Coord { y_: 0, x_: 0 }; CHARACTER_N];
        AutoMoveMazeState {
            points_: points_,
            turn_: 0,
            character_: character_,
            game_score_: 0,
            evaluated_score_: 0,
        }
    }

    fn setCharacter(&mut self, character_id: usize, y: isize, x: isize) {
        self.character_[character_id].y_ = y;
        self.character_[character_id].x_ = x;
    }
}

fn main() {}
