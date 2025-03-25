use rand::seq::IndexedRandom;
use std::cmp::{max, min};
use std::fmt;
use std::io::stdin;

const ROWS: usize = 6;
const COLS: usize = 7;

const BOT: i8 = -1;
const EMPTY: i8 = 0;
const PLAYER: i8 = 1;

struct ConnectFour {
    board: [[i8; COLS]; ROWS],
}

impl ConnectFour {
    fn new() -> Self {
        Self {
            board: [[EMPTY; COLS]; ROWS],
        }
    }

    fn get_valid_moves(&self) -> Vec<usize> {
        (0..COLS)
            .filter(|&col| self.board[0][col] == EMPTY)
            .collect()
    }

    fn check_win(&self, player: i8) -> bool {
        for row in 0..ROWS {
            for col in 0..(COLS - 3) {
                if (0..4).all(|i| self.board[row][col + i] == player) {
                    return true;
                }
            }
        }
        for row in 0..(ROWS - 3) {
            for col in 0..COLS {
                if (0..4).all(|i| self.board[row + i][col] == player) {
                    return true;
                }
            }
        }
        for row in 0..(ROWS - 3) {
            for col in 0..(COLS - 3) {
                if (0..4).all(|i| self.board[row + i][col + i] == player) {
                    return true;
                }
                if (0..4).all(|i| self.board[row + 3 - i][col + i] == player) {
                    return true;
                }
            }
        }
        false
    }

    fn drop_piece(&mut self, col: usize, piece: i8) -> bool {
        for row in (0..ROWS).rev() {
            if self.board[row][col] == EMPTY {
                self.board[row][col] = piece;
                return true;
            }
        }
        false
    }

    fn clone(&self) -> Self {
        Self {
            board: self.board.clone(),
        }
    }
}

impl fmt::Display for ConnectFour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = |val: i8| match val {
            BOT => "x",
            EMPTY => ".",
            PLAYER => "o",
            _ => " ",
        };
        for row in &self.board {
            for &cell in row {
                write!(f, " {}", symbol(cell))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct BotPlayer {
    max_depth: i32,
    reward: i32,
}

impl BotPlayer {
    fn new(max_depth: i32) -> Self {
        Self {
            max_depth,
            reward: 100,
        }
    }

    fn minimax(
        &mut self,
        game: &mut ConnectFour,
        depth: i32,
        alpha: i32,
        beta: i32,
        is_maximizing: bool,
    ) -> i32 {
        let valid_moves = game.get_valid_moves();
        if valid_moves.is_empty() || depth == 0 {
            return 0;
        }

        let mut alpha = alpha;
        let mut beta = beta;

        if is_maximizing {
            let mut max_score = i32::MIN;

            for col in valid_moves {
                if let Some(row) = (0..ROWS).rev().find(|&row| game.board[row][col] == EMPTY) {
                    game.board[row][col] = BOT;

                    let score = if game.check_win(BOT) {
                        self.reward
                    } else {
                        self.minimax(game, depth - 1, alpha, beta, false)
                    };
                    game.board[row][col] = EMPTY;
                    max_score = max(max_score, score);
                    alpha = max(alpha, score);
                    if beta <= alpha {
                        break;
                    }
                }
            }
            max_score
        } else {
            let mut min_score = i32::MAX;

            for col in valid_moves {
                if let Some(row) = (0..ROWS).rev().find(|&row| game.board[row][col] == EMPTY) {
                    game.board[row][col] = PLAYER;

                    let score = if game.check_win(PLAYER) {
                        -self.reward
                    } else {
                        self.minimax(game, depth - 1, alpha, beta, true)
                    };
                    game.board[row][col] = EMPTY;
                    min_score = min(min_score, score);
                    beta = min(beta, score);
                    if beta <= alpha {
                        break;
                    }
                }
            }
            min_score
        }
    }

    fn get_best_move(&mut self, game: &ConnectFour) -> Option<usize> {
        let mut game_clone = game.clone();
        let mut best_score = i32::MIN;
        let mut best_moves: Vec<usize> = Vec::new();

        for col in game.get_valid_moves() {
            if let Some(row) = (0..ROWS)
                .rev()
                .find(|&row| game_clone.board[row][col] == EMPTY)
            {
                game_clone.board[row][col] = BOT;

                let score =
                    self.minimax(&mut game_clone, self.max_depth, i32::MIN, i32::MAX, false);
                game_clone.board[row][col] = EMPTY;

                if score > best_score {
                    best_score = score;
                    best_moves.clear();
                    best_moves.push(col);
                } else if score == best_score {
                    best_moves.push(col);
                }
            }
        }
        best_moves.choose(&mut rand::rng()).cloned()
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}

fn main() {
    let mut game = ConnectFour::new();
    let mut bot = BotPlayer::new(4);
    let mut current_player = PLAYER;

    loop {
        clear_screen();
        println!("{}", game);

        if game.check_win(BOT) || game.check_win(PLAYER) {
            println!("Game over!");
            break;
        }
        if game.get_valid_moves().is_empty() {
            println!("Draw!");
            break;
        }

        if current_player == BOT {
            if let Some(col) = bot.get_best_move(&game) {
                game.drop_piece(col, BOT);
            }
            current_player = PLAYER;
            continue;
        }

        println!("Enter column number (1-7):");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if let Ok(col) = input.trim().parse::<usize>() {
            let col = col - 1;
            if game.get_valid_moves().contains(&col) {
                game.drop_piece(col, PLAYER);
                current_player = BOT;
            }
        }
    }
}
