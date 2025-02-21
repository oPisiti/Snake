use std::{
    collections::VecDeque,
    io::{stdin, stdout, Write},
    sync::{Arc, Mutex, MutexGuard},
};
use std::{thread, time};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

//*** Global constants ***
//Board
const BOARD_HEIGHT: u8 = 15;
const BOARD_WIDTH: u8 = 15;
const BOARD_CHAR: &str = "#";

// Snake
const SNAKE_CHAR: &str = "*";
const SNAKE_HEAD_CHAR: &str = "@";

// Game
const TICKS_PER_SEC: f32 = 1.0;
const GAME_SLEEP: time::Duration = time::Duration::from_millis((1000.0 / TICKS_PER_SEC) as u64);
const LISTENS_PER_SEC: f32 = 5.0;
const LISTENER_SLEEP: time::Duration =
    time::Duration::from_millis((1000.0 / LISTENS_PER_SEC) as u64);

#[derive(Clone)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

fn main() {
    // Movement
    let mut direction: Arc<Mutex<Direction>> = Arc::new(Mutex::new(Direction::UP));

    // Create a keyboard listener
    let mut listener_mutex = Arc::clone(&direction);
    let listener = thread::spawn(move || {
        let stdin = stdin();

        // Detecting keydown events
        for k in stdin.keys() {
            match k.unwrap() {
                Key::Char('s') => {
                    *listener_mutex.lock().unwrap() = Direction::UP;
                }
                Key::Char('w') => {
                    *listener_mutex.lock().unwrap() = Direction::DOWN;
                }
                Key::Char('a') => {
                    *listener_mutex.lock().unwrap() = Direction::LEFT;
                }
                Key::Char('d') => {
                    *listener_mutex.lock().unwrap() = Direction::RIGHT;
                }
                Key::Ctrl('c') => break,
                Key::Alt('t') => println!("termion is cool"),
                _ => (),
            }
        }

        thread::sleep(LISTENER_SLEEP);
    });

    let board = [BOARD_CHAR; (BOARD_WIDTH * BOARD_HEIGHT) as usize];
    let mut snake: VecDeque<[u8; 2]> =
        VecDeque::from([[2, 3], [2, 2], [1, 2], [0, 2], [0, 1], [0, 0]]);

    loop {
        println!(
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        );

        // Copy the board for editing
        let mut curr_board = board;

        // Move the snake
        move_snake(&mut snake, Arc::clone(&direction));

        // Make changes to the board
        put_snake_to_board(&snake, &mut curr_board);

        // Final rendering
        print_board(&curr_board);

        thread::sleep(GAME_SLEEP);
    }

    // listener.join();
}

fn move_snake(snake: &mut VecDeque<[u8; 2]>, direction: Arc<Mutex<Direction>>) {
    // Get the direction from mutex
    let dir;
    if let Ok(inner_dir_data) = direction.lock() {
        dir = (*inner_dir_data).clone();
    } else {
        dir = Direction::DOWN;
    }

    let new_pos = match dir{
        Direction::DOWN =>{
            [snake[0][0] - 1, snake[0][1]]
        }
        Direction::UP => {
            [snake[0][0] + 1, snake[0][1]]
        },
        Direction::RIGHT => {
            [snake[0][0], snake[0][1] + 1]
        },
        Direction::LEFT => {
            [snake[0][0], snake[0][1] - 1]
        }
    };

    snake.push_front(new_pos);
}

fn print_board(board: &[&str]) {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let top_boundary = "╭".to_string() + &"─".repeat(BOARD_WIDTH as usize) + "╮";
    let bot_boundary = "╰".to_string() + &"─".repeat(BOARD_WIDTH as usize) + "╯";

    println!("{top_boundary}\r");
    for r in 0..BOARD_HEIGHT {
        let start_index = r * BOARD_WIDTH;
        // let row = board[start_index..(start_index + BOARD_WIDTH)];
        let mut row = String::new();
        for col in board
            .iter()
            .skip(start_index.into())
            .take(BOARD_WIDTH.into())
        {
            row += col;
        }

        println!("│{row}│\r");
    }
    println!("{bot_boundary}");
    stdout.flush().unwrap();
}

fn put_snake_to_board(snake: &VecDeque<[u8; 2]>, board: &mut [&str]) {
    // Body
    for piece in snake.iter().take(snake.len()) {
        let index = (piece[0] * BOARD_WIDTH + piece[1]) as usize;
        board[index] = SNAKE_CHAR;
    }

    // Head
    let index: usize = (snake[0][0] as usize) * (BOARD_WIDTH as usize) + (snake[0][1] as usize);
    board[index] = SNAKE_HEAD_CHAR;
}
