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

fn swap_turn(turn: &mut i8) {
    if *turn == 0 {
        *turn = 1;
    } else {
        *turn = 0;
    }
}

fn format_turn(turn: &i8) -> char {
    let mut formated_turn = 'X';
    if *turn == 1 {
        formated_turn = 'O';
    }
    return formated_turn;
}

fn is_game_over(board: &Board, turn: &i8) -> i8 {
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
            values[position_i] = board[winning_position[0]][winning_position[1]];
        }
        if values[0] == *turn && values[1] == *turn && values[2] == *turn {
            game_status = 1;
            break;
        }
    }

    if game_status == 1 {
        return game_status;
    }

    let mut did_tie = true;

    for (_i, &row) in board.iter().enumerate() {
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

fn make_move(turn: &mut i8, board: &mut Board, row: &usize, column: &usize) -> bool {
    if board[*row][*column] == -1 {
        board[*row][*column] = *turn;
        return true;
    }
    return false;
}

fn draw(board: &Board, turn: &i8, message: &mut String, debug: Option<bool>) {
    std::process::Command::new("clear").status().unwrap();

    println!("{}", message);
    *message = String::new();

    println!("\nColumn:   | 0 | | 1 | | 2 | ");
    println!("          _________________");
    for (row_i, &row) in board.iter().enumerate() {
        print!("Row: {}    ", row_i);
        for (_column_i, &column_v) in row.iter().enumerate() {
            let mut formated_value = ' ';

            if column_v == 0 {
                formated_value = 'X';
            } else if column_v == 1 {
                formated_value = 'O';
            }
            if debug == Some(true) {
                print!("| {} | ", column_v);
            } else {
                print!("| {} | ", formated_value);
            }
        }
        print!("\n");
    }
    println!("          -----------------");
    print!("\n");
    println!("Turn: {}", format_turn(turn));
}

fn main() {
    let mut board = [[-1, -1, -1], [-1, -1, -1], [-1, -1, -1]];
    let mut turn: i8 = 0;
    let mut message: String = String::new();
    let mut game_status: i8 = is_game_over(&board, &turn);
    while game_status == 2 {
        draw(&board, &turn, &mut message, Some(false));

        let row = input("What is the row?");
        let column = input("What is the column?");

        let did_move = make_move(&mut turn, &mut board, &row, &column);

        if !did_move {
            message =
                String::from("You can not put a value in a position that already has a value!");
        }
        game_status = is_game_over(&board, &turn);
        if game_status != 1 {
            swap_turn(&mut turn);
        }
    }

    println!("Player {} won!", format_turn(&turn))
}
