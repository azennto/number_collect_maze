use rand::Rng;
use std::cmp;
use std::collections::BinaryHeap;

type ScoreType = isize;

const H: usize = 5;
const W: usize = 5;
const END_TURN: usize = 5;
const CHARACTER_N: usize = 3;
const INF: ScoreType = 1e9 as isize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    characters_: Vec<Coord>,
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
        let characters_ = vec![Coord { y_: 0, x_: 0 }; CHARACTER_N];
        AutoMoveMazeState {
            points_: points_,
            turn_: 0,
            characters_: characters_,
            game_score_: 0,
            evaluated_score_: 0,
        }
    }

    fn setCharacter(&mut self, characters_id: usize, y: isize, x: isize) {
        self.characters_[characters_id].y_ = y;
        self.characters_[characters_id].x_ = x;
    }

    fn isDone(&self) -> bool {
        self.turn_ == END_TURN
    }

    fn toString(&self) {
        println!("turn: {}", self.turn_);
        println!("score: {}", self.game_score_);
        for y in 0..H {
            for x in 0..W {
                let mut character_cell = false;
                for characters_id in 0..CHARACTER_N {
                    if self.characters_[characters_id].y_ == y as isize
                        && self.characters_[characters_id].x_ == x as isize
                    {
                        character_cell = true;
                        print!("@");
                        break;
                    }
                }
                if character_cell {
                    continue;
                }
                if self.points_[y][x] > 0 {
                    print!("{}", self.points_[y][x]);
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    const dx: [isize; 4] = [1, -1, 0, 0];
    const dy: [isize; 4] = [0, 0, 1, -1];

    fn movePlayer(&mut self, character_id: usize) {
        let mut best_point = 0;
        let mut best_action_index = 0;
        for action in 0..4 {
            let ty = self.characters_[character_id].y_ + Self::dy[action];
            let tx = self.characters_[character_id].x_ + Self::dx[action];

            if ty >= 0 && ty < H as isize && tx >= 0 && tx < W as isize {
                if self.points_[ty as usize][tx as usize] > best_point {
                    best_point = self.points_[ty as usize][tx as usize];
                    best_action_index = action;
                }
            }
        }

        self.characters_[character_id].y_ += Self::dy[best_action_index];
        self.characters_[character_id].x_ += Self::dx[best_action_index];
    }

    fn advance(&mut self) {
        for character_id in 0..CHARACTER_N {
            self.movePlayer(character_id);
        }
        for character in self.characters_.iter() {
            self.game_score_ += self.points_[character.y_ as usize][character.x_ as usize];
            self.points_[character.y_ as usize][character.x_ as usize] = 0;
        }
        self.turn_ += 1;
    }

    fn getScore(&mut self, is_print: bool) -> usize {
        for character in self.characters_.iter() {
            self.points_[character.y_ as usize][character.x_ as usize] = 0
        }
        while !self.isDone() {
            self.advance();
            if is_print {
                self.toString();
            }
        }
        return self.game_score_;
    }
}

fn randomAction(state: &AutoMoveMazeState) -> AutoMoveMazeState {
    let mut now_state = state.clone();
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(0);
    for character_id in 0..CHARACTER_N {
        let y = rng.gen_range(0..H) as isize;
        let x = rng.gen_range(0..W) as isize;
        now_state.setCharacter(character_id, y, x);
    }
    return now_state;
}

fn playGame(seed: Option<u64>) {
    let mut state = AutoMoveMazeState::new(seed);
    state = randomAction(&state);
    state.toString();
    let score = state.getScore(true);
    println!("Score :{}", score);
}

fn main() {
    playGame(Some(4));
}
