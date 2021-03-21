// Debug
const GAME_DEBUG: bool = false;
const AI_DEBUG: bool = true;

// Ai Settings
const AI_DEPTH: u64 = 500;

/*
    type board = [[i8; 3]; 3];
    [
        [-1, -1, -1],
        [-1, -1, -1],
        [-1, -1, -1]
    ];
*/
type Board = [[i8; 3]; 3];
type Winning = [[[usize; 2]; 3]; 8];
struct Move {
    row: usize,
    column: usize,
}
type PlayerCallback = fn(&mut TicTacToe) -> Move;

// Helpers
fn console_clear() {
    std::process::Command::new("clear").status().unwrap();
}
fn input(message: &str) -> usize {
    let mut value = String::new();
    println!("{}", message);

    std::io::stdin()
        .read_line(&mut value)
        .expect("Something went wrong!");
    return value
        .trim()
        .replace("\n", "")
        .to_string()
        .parse::<usize>()
        .expect("Type a number!");
}

#[derive(Clone)]
struct TicTacToe {
    player1: PlayerCallback,
    player2: PlayerCallback,
    board: Board,
    turn: i8,
    status: i8,
    message: String,
    debug: bool,
    ai_debug: bool,
}
impl TicTacToe {
    fn new(
        player1: PlayerCallback,
        player2: PlayerCallback,
        board: Option<Board>,
        turn: Option<i8>,
        debug: Option<bool>,
        ai_debug: Option<bool>,
    ) -> TicTacToe {
        let default_board = [[-1, -1, -1], [-1, -1, -1], [-1, -1, -1]];
        let mut game = TicTacToe {
            player1,
            player2,
            board: board
                .or_else(|| Some(default_board))
                .expect("Some err, tic_tac_toe new!"),
            debug: debug
                .or_else(|| Some(false))
                .expect("Some err, tic_tac_toe new!"),
            ai_debug: ai_debug
                .or_else(|| Some(false))
                .expect("Some err, tic_tac_toe new!"),
            turn: turn
                .or_else(|| Some(0))
                .expect("Some err, tic_tac_toe new!"),
            status: 2,
            message: String::new(),
        };
        game.status = game.is_game_over(None);
        return game;
    }
    fn swap_turn(&mut self) {
        if self.status != 1 {
            if self.turn == 0 {
                self.turn = 1;
            } else {
                self.turn = 0;
            }
        }
    }
    fn get_formated_turn(&self) -> char {
        let mut formated_turn = 'X';
        if self.turn == 1 {
            formated_turn = 'O';
        }
        return formated_turn;
    }
    fn is_game_over(&self, relative_to: Option<i8>) -> i8 {
        let relative_to = relative_to
            .or_else(|| Some(self.turn))
            .expect("Some err, tic_tac_toe new!");
        let winning_positions: Winning = [
            [[0, 0], [0, 1], [0, 2]],
            [[1, 0], [1, 1], [1, 2]],
            [[2, 0], [2, 1], [2, 2]],
            [[0, 0], [1, 0], [2, 0]],
            [[0, 1], [1, 1], [2, 1]],
            [[0, 2], [1, 2], [2, 2]],
            [[0, 0], [1, 1], [2, 2]],
            [[0, 2], [1, 1], [2, 0]],
        ];
        let mut game_status: i8 = 2;
        for (_sequences_i, &winnin_sequences) in winning_positions.iter().enumerate() {
            let mut values: [i8; 3] = [-1; 3];
            for (position_i, &winning_position) in winnin_sequences.iter().enumerate() {
                values[position_i] = self.board[winning_position[0]][winning_position[1]];
            }
            if values[0] == relative_to && values[1] == relative_to && values[2] == relative_to {
                game_status = 1;
                break;
            } else if values[0] != -1
                && values[1] != -1
                && values[2] != -1
                && values[0] != relative_to
                && values[1] != relative_to
                && values[2] != relative_to
            {
                game_status = -1;
                break;
            }
        }
        if game_status == 1 || game_status == -1 {
            return game_status;
        }
        let mut did_tie = true;
        for (_i, &row) in self.board.iter().enumerate() {
            for (_i, &column) in row.iter().enumerate() {
                if column == -1 {
                    did_tie = false;
                    break;
                }
            }
            if did_tie == false {
                break;
            }
        }
        if did_tie == true {
            return 0;
        }
        return game_status;
    }
    fn make_move(&mut self, row: &usize, column: &usize) -> bool {
        if *row >= self.board.len() {
            return false;
        }
        if *column >= self.board[*row].len() {
            return false;
        }
        if self.board[*row][*column] == -1 {
            self.board[*row][*column] = self.turn;
            return true;
        }
        return false;
    }
    fn draw(&mut self) {
        console_clear();
        println!("{}", self.message);
        self.message = String::new();
        println!("\nColumn:   | 1 | | 2 | | 3 | ");
        println!("          _________________");
        for (row_i, &row) in self.board.iter().enumerate() {
            print!("Row: {}    ", row_i + 1);
            for (_column_i, &column_v) in row.iter().enumerate() {
                let mut formated_value = ' ';
                if column_v == 0 {
                    formated_value = 'X';
                } else if column_v == 1 {
                    formated_value = 'O';
                }
                if self.debug == true {
                    print!("| {} | ", column_v);
                } else {
                    print!("| {} | ", formated_value);
                }
            }
            print!("\n");
        }
        println!("          -----------------");
        print!("\n");
        println!("Turn: {}", self.get_formated_turn());
    }
    fn start(&mut self) {
        let pos_already_taked_msg =
            "You might have typed the wrong row or column, or the position that you typed is not available.";
        while self.status == 2 {
            self.draw();
            let player_move: Move;
            if self.turn == 0 {
                player_move = (self.player1)(self);
            } else {
                player_move = (self.player2)(self);
            }
            let did_move = self.make_move(&player_move.row, &player_move.column);
            if !did_move {
                self.message = String::from(pos_already_taked_msg);
                continue;
            }
            self.status = self.is_game_over(None);
            self.swap_turn();
        }
        if self.status == 1 {
            println!("Player {} won!", self.get_formated_turn());
        } else if self.status == 0 {
            println!("Tie!");
        }
    }
}

// Minimax
#[derive(Copy, Clone, Debug)]
struct Score {
    row: usize,
    column: usize,
    score: i8,
    depth: u64,
    comparitions: u64,
}

fn minimax_min(best_score: &Score, next_game_score: &Score) -> bool {
    if best_score.score > next_game_score.score {
        return true;
    }

    return false;
}
fn minimax_max(best_score: &Score, next_game_score: &Score) -> bool {
    if best_score.score < next_game_score.score {
        return true;
    }

    return false;
}
/*
    alpha is the best score
    beta is the worst score,


    Usually you pass -infinity as alpha and +infinity as beta
*/
fn minimax(
    game: TicTacToe,
    depth: u64,
    alpha: i8,
    beta: i8,
    is_maximizing: bool,
    player: i8,
    comparitions: u64,
) -> Score {
    let mut comparitions = comparitions;
    let mut alpha = alpha;
    let mut beta = beta;
    let current_game_status = game.is_game_over(Some(player));
    if current_game_status < 2 || depth == 0 {
        return Score {
            row: 0,
            column: 0,
            score: current_game_status,
            depth,
            comparitions,
        };
    }

    let mut best_score: Score;

    if is_maximizing {
        best_score = Score {
            row: 0,
            column: 0,
            score: -2,
            depth: 200,
            comparitions: 0,
        };
    } else {
        best_score = Score {
            row: 0,
            column: 0,
            score: 5,
            depth: 200,
            comparitions: 0,
        };
    }

    for (row_i, &row) in game.board.clone().iter().enumerate() {
        let mut found: bool = false;
        for (column_i, &column) in row.iter().enumerate() {
            if column == -1 {
                comparitions += 1;
                let mut next_game: TicTacToe = game.clone();
                next_game.make_move(&row_i, &column_i);
                next_game.swap_turn();

                let next_game_score = minimax(
                    next_game,
                    depth - 1,
                    alpha,
                    beta,
                    !is_maximizing,
                    player,
                    comparitions,
                );

                let new_best_move: bool;

                if is_maximizing {
                    new_best_move = minimax_max(&mut best_score, &next_game_score);
                    if alpha < next_game_score.score {
                        alpha = next_game_score.score;
                    }
                } else {
                    new_best_move = minimax_min(&mut best_score, &next_game_score);
                    if beta > next_game_score.score {
                        beta = next_game_score.score;
                    }
                }

                if new_best_move == true {
                    best_score = Score {
                        row: row_i,
                        column: column_i,
                        ..next_game_score
                    };
                }

                if beta <= alpha {
                    found = true;
                    break;
                }
            }
        }

        if found {
            break;
        }
    }
    return best_score;
}

// Players
fn real_player(_game: &mut TicTacToe) -> Move {
    let row = input("What is the row?") - 1;
    let column = input("What is the column?") - 1;

    return Move { row, column };
}
fn minimax_player(game: &mut TicTacToe) -> Move {
    let best_move = minimax(game.clone(), AI_DEPTH, -5, 5, true, game.turn, 0);
    if game.ai_debug {
        game.message.push_str(&format!(
            "Minimax(AI): \nDEPTH: {}\nComparitions: {}\n------------------------------------------------------",
            AI_DEPTH - best_move.depth,
            best_move.comparitions,
        ));
    }

    return Move {
        row: best_move.row,
        column: best_move.column,
    };
}

fn main() {
    console_clear();
    let game_mode = input(
        "Choose a game mode:

[1] Play against computer.

[2] Play against other player.",
    );

    let mut game: TicTacToe;

    if game_mode == 2 {
        game = TicTacToe::new(
            real_player,
            real_player,
            None,
            None,
            Some(GAME_DEBUG),
            Some(AI_DEBUG),
        );
        game.start();
    } else {
        let game_turn = input(
            "Do you want to be the first or second to move?
[1] First.
[2] Second.",
        );
        if game_turn == 1 {
            game = TicTacToe::new(
                real_player,
                minimax_player,
                None,
                None,
                Some(GAME_DEBUG),
                Some(AI_DEBUG),
            );
            game.start();
        } else {
            game = TicTacToe::new(
                minimax_player,
                real_player,
                None,
                None,
                Some(GAME_DEBUG),
                Some(AI_DEBUG),
            );
            game.start();
        }
    }
}
