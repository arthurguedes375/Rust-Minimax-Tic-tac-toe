/*
    type board = [[i8; 3]; 3];
    [
        [-1, -1, -1],
        [-1, -1, '-1],
        [-1, -1, -1]
    ];
*/
type Board = [[i8; 3]; 3];
type Winning = [[[usize; 2]; 3]; 8];

fn input(message: &str) -> usize {
    let mut value = String::new();
    println!("{}", message);

    std::io::stdin()
        .read_line(&mut value)
        .expect("Something went wrong!");
    return value
        .replace("\n", "")
        .to_string()
        .parse::<usize>()
        .expect("Type a number!");
}

struct TicTacToe {
    board: Board,
    turn: i8,
    status: i8,
    message: String,
    debug: bool,
}
impl TicTacToe {
    fn new(board: Option<Board>, turn: Option<i8>, debug: Option<bool>) -> TicTacToe {
        let default_board = [[-1, -1, -1], [-1, -1, -1], [-1, -1, -1]];
        let mut game = TicTacToe {
            board: board
                .or_else(|| Some(default_board))
                .expect("Some err, tic_tac_toe new!"),
            debug: debug
                .or_else(|| Some(false))
                .expect("Some err, tic_tac_toe new!"),
            turn: turn
                .or_else(|| Some(0))
                .expect("Some err, tic_tac_toe new!"),
            status: 2,
            message: String::new(),
        };
        game.status = game.is_game_over();
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
    fn is_game_over(&self) -> i8 {
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
            if values[0] == self.turn && values[1] == self.turn && values[2] == self.turn {
                game_status = 1;
                break;
            }
        }
        if game_status == 1 {
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
        if self.board[*row][*column] == -1 {
            self.board[*row][*column] = self.turn;
            return true;
        }
        return false;
    }
    fn draw(&mut self) {
        std::process::Command::new("clear").status().unwrap();
        println!("{}", self.message);
        self.message = String::new();
        println!("\nColumn:   | 0 | | 1 | | 2 | ");
        println!("          _________________");
        for (row_i, &row) in self.board.iter().enumerate() {
            print!("Row: {}    ", row_i);
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
        while self.status == 2 {
            self.draw();
            let row = input("What is the row?");
            let column = input("What is the column?");
            let did_move = self.make_move(&row, &column);
            if !did_move {
                self.message =
                    String::from("You can not put a value in a position that already has a value!");
                continue;
            }
            self.status = self.is_game_over();
            self.swap_turn();
        }
        println!("Player {} won!", self.get_formated_turn());
    }
}

fn main() {
    let mut game = TicTacToe::new(None, None, None);
    game.start();
}
