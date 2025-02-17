// Global constants
const BOARD_HEIGHT: usize = 5;
const BOARD_WIDTH: usize = 5;

fn main() {
    // Constants
    const BOARD_CHAR: &str = "#";
    const SNAKE_CHAR: &str = "*";
    const SNAKE_HEAD_CHAR: &str = "@";

    let board = [BOARD_CHAR; BOARD_WIDTH * BOARD_HEIGHT];

    let curr_board = board;
    print_board(&curr_board);
}

fn print_board(board: &[&str]) {
    for r in 0..BOARD_HEIGHT {
        let start_index = r * BOARD_WIDTH;
        // let row = board[start_index..(start_index + BOARD_WIDTH)];
        let mut row = String::new();
        for col in board.iter().skip(start_index).take(BOARD_WIDTH) {
            row += col;
        }

        println!("{row}");
    }
}

// Print board
// Update snake's vertices
// Update board
