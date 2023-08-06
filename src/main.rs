use rand::Rng;

const H: usize = 3;
const W: usize = 4;
const END_TURN: usize = 4;

#[derive(Debug, Clone)]
struct Coord {
    y_: isize,
    x_: isize,
}

impl Coord {
    fn new() -> Self {
        Coord { y_: 0, x_: 0 }
    }
}

#[derive(Debug, Clone)]
struct MazeState {
    points_: Vec<Vec<usize>>,
    turn_: usize,
    character_: Coord,
    game_score_: usize,
}

impl MazeState {
    const dx: [isize; 4] = [1, -1, 0, 0];
    const dy: [isize; 4] = [0, 0, 1, -1];
    fn new(seed: Option<u64>) -> Self {
        let mut rng: rand::rngs::StdRng =
            rand::SeedableRng::seed_from_u64(rand::thread_rng().gen());
        if let Some(s) = seed {
            rng = rand::SeedableRng::seed_from_u64(s);
        }
        let mut character_ = Coord::new();
        character_.y_ = rng.gen_range(0..H) as isize;
        character_.x_ = rng.gen_range(0..W) as isize;

        let mut points_ = vec![vec![0usize; W]; H];
        for y in 0..H {
            for x in 0..W {
                if y as isize == character_.y_ && x as isize == character_.x_ {
                    continue;
                }
                points_[y][x] = rng.gen_range(0..10)
            }
        }
        MazeState {
            points_: points_,
            turn_: 0,
            character_: character_,
            game_score_: 0,
        }
    }
    fn isDone(&self) -> bool {
        self.turn_ == END_TURN
    }
    fn advance(&mut self, action: usize) {
        self.character_.x_ += Self::dx[action];
        self.character_.y_ += Self::dy[action];
        unsafe {
            let point = &mut self.points_[self.character_.y_ as usize][self.character_.x_ as usize]
                as *mut usize;
            if *point > 0 {
                self.game_score_ += *point;
                *point = 0;
            }
        }
        self.turn_ += 1;
    }
    fn legalActions(&self) -> Vec<usize> {
        let mut actions = vec![];
        for action in 0..4 {
            let ty = self.character_.y_ + Self::dy[action];
            let tx = self.character_.x_ + Self::dx[action];
            if ty >= 0 && ty < H as isize && tx >= 0 && tx < W as isize {
                actions.push(action);
            }
        }
        actions
    }
    fn toString(&self) {
        println!("turn: {}", self.turn_);
        println!("score: {}", self.game_score_);
        for y in 0..H {
            for x in 0..W {
                if self.character_.y_ == y as isize && self.character_.x_ == x as isize {
                    print!("@");
                } else if self.points_[y][x] > 0 {
                    print!("{}", self.points_[y][x]);
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

fn randomAction(state: &MazeState) -> usize {
    let legal_actions = state.legalActions();
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(0);
    legal_actions[rng.gen_range(0..legal_actions.len())]
}

fn playGame(seed: Option<u64>) -> usize {
    let mut state = MazeState::new(seed);
    state.toString();
    while !state.isDone() {
        println!("");
        state.advance(randomAction(&state));
        state.toString();
    }
    state.game_score_
}

fn main() {
    playGame(Some(121321));
}
