// Global constantS
const BOARD_HEIGHT: usize = 5;
const BOARD_WIDTH: usize = 7;
const BOARD_CHAR: &str = " ";
const SNAKE_CHAR: &str = "*";
const SNAKE_HEAD_CHAR: &str = "@";

fn main() {
    let board = [BOARD_CHAR; BOARD_WIDTH * BOARD_HEIGHT];
    let mut snake: Vec<[usize; 2]> = vec![[3, 2], [0, 2], [0, 0]];

    // Copy the board for editing
    let mut curr_board = board;

    // Make changes to the board
    put_snake_to_board(&snake, &mut curr_board);

    // Final rendering
    print_board(&curr_board);
}

fn print_board(board: &[&str]) {
    let top_boundary = "╭".to_string() + &"─".repeat(BOARD_WIDTH) + "╮";
    let bot_boundary = "╰".to_string() + &"─".repeat(BOARD_WIDTH) + "╯";

    println!("{top_boundary}");
    for r in 0..BOARD_HEIGHT {
        let start_index = r * BOARD_WIDTH;
        let end_index = start_index + BOARD_WIDTH;
        let row = board[start_index..end_index].concat();

        println!("│{row}│");
    }
    println!("{bot_boundary}");

}

fn put_snake_to_board(snake: &[[usize; 2]], board: &mut [&str]) {
    let mut last_vertex = &snake[0];
    let mut row_boundaries;
    let mut col_boundaries;

    // Body
    for vertex in snake.iter().skip(1).take(snake.len()) {
        let mut delta: [usize; 2] = [0, 0];

        // Set deltas
        if vertex[0] == last_vertex[0] {
            delta[1] = 1;
        } else {
            delta[0] = 1;
        }

        // Set starting and ending vertices
        row_boundaries = sorted(last_vertex[0], vertex[0]);
        col_boundaries = sorted(last_vertex[1], vertex[1]);

        // Put to board
        let mut curr_r = row_boundaries.0;
        let mut curr_c = col_boundaries.0;

        while curr_r < row_boundaries.1 || curr_c < col_boundaries.1 {
            let index = curr_r * BOARD_WIDTH + curr_c;
            board[index] = SNAKE_CHAR;

            curr_r += delta[0];
            curr_c += delta[1];
        }

        last_vertex = vertex;
    }

    // Head
    board[snake[0][0] * BOARD_WIDTH + snake[0][1]] = SNAKE_HEAD_CHAR;
}

#[inline]
fn sorted<T: std::cmp::PartialOrd>(a: T, b: T) -> (T, T) {
    if a <= b {
        return (a, b);
    }
    (b, a)
}
