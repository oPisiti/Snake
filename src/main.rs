use std::{
    io::{stdin, stdout, Write},
    sync::{Arc, Mutex},
};
use std::{thread, time};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

//*** Global constants ***
//Board
const BOARD_HEIGHT: usize = 5;
const BOARD_WIDTH: usize = 7;
const BOARD_CHAR: &str = "#";

// Snake
const SNAKE_CHAR: &str = "*";
const SNAKE_HEAD_CHAR: &str = "@";

// Game
const TICKS_PER_SEC: f32 = 1.0;
const GAME_SLEEP: time::Duration = time::Duration::from_millis((1000.0 / TICKS_PER_SEC) as u64);
const LISTENS_PER_SEC: f32 = 55555.0;
const LISTENER_SLEEP: time::Duration =
    time::Duration::from_millis((1000.0 / LISTENS_PER_SEC) as u64);

fn main() {
    // Movement
    let mut direction = Arc::new(Mutex::new([1, 0]));

    // Create a keyboard listener
    let mut listener_mutex = Arc::clone(&direction);
    let listener = thread::spawn(move || {
        let stdin = stdin();

        // Detecting keydown events
        for k in stdin.keys() {
            match k.unwrap() {
                Key::Char('s') => {
                    *listener_mutex.lock().unwrap() = [-1, 0];
                }
                Key::Char('w') => {
                    *listener_mutex.lock().unwrap() = [1, 0];
                }
                Key::Char('a') => {
                    *listener_mutex.lock().unwrap() = [0, -1];
                }
                Key::Char('d') => {
                    *listener_mutex.lock().unwrap() = [0, 1];
                }
                Key::Ctrl('c') => break,
                Key::Alt('t') => println!("termion is cool"),
                _ => (),
            }
        }

        thread::sleep(LISTENER_SLEEP);
    });

    let board = [BOARD_CHAR; BOARD_WIDTH * BOARD_HEIGHT];
    let mut snake: Vec<[usize; 2]> = vec![[3, 2], [0, 2], [0, 0]];

    loop {
        println!(
            "{}Dir: {:?}{}",
            termion::clear::All,
            direction.lock().unwrap(),
            termion::cursor::Goto(1, 1)
        );

        // Copy the board for editing
        let mut curr_board = board;

        // Make changes to the board
        put_snake_to_board(&snake, &mut curr_board);

        // Final rendering
        print_board(&curr_board);

        thread::sleep(GAME_SLEEP);
    }

    // listener.join();
}

fn print_board(board: &[&str]) {
    let mut stdout = stdout().into_raw_mode().unwrap();

    for r in 0..BOARD_HEIGHT {
        let start_index = r * BOARD_WIDTH;
        // let row = board[start_index..(start_index + BOARD_WIDTH)];
        let mut row = String::new();
        for col in board.iter().skip(start_index).take(BOARD_WIDTH) {
            row += col;
        }

        println!("{row}\r");
    }
     stdout.flush().unwrap();
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
