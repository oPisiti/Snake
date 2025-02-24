use std::{
    collections::VecDeque, io::{stdin, stdout, Write}, sync::{Arc, Mutex}
};
use std::{thread, time};
use rand::{rng, Rng};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

//*** Global constants ***
// Game
const TICKS_PER_SEC: f32 = 5.0;
const GAME_SLEEP: time::Duration = time::Duration::from_millis((1000.0 / TICKS_PER_SEC) as u64);
const LISTENS_PER_SEC: f32 = 5.0;
const LISTENER_SLEEP: time::Duration =
    time::Duration::from_millis((1000.0 / LISTENS_PER_SEC) as u64);
const FOOD_CHAR: &str = "O";
const FOOD_PLACING_MAX_TRIES: u16 = 100;

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
enum GameError {
    Collision,
    FoodPosition,
    OutOfBounds,
}

struct Board<'a>{
    width: u8,
    height: u8,
    background: &'a str
}

struct Snake<'a>{
    body_char: &'a str,
    head_char: &'a str,
    body: VecDeque<[usize; 2]>
}

fn main() {
    // Config
    const BOARD_CONFIG: Board = Board{
        width: 17,
        height: 17,
        background: " "
    };

    // Movement
    let direction: Arc<Mutex<Direction>> = Arc::new(Mutex::new(Direction::Down));

    // Create a keyboard listener
    let listener_mutex = Arc::clone(&direction);
    thread::spawn(move || {
        let stdin = stdin();

        // Detecting keydown events
        for k in stdin.keys() {
            match k.unwrap() {
                Key::Char('s') | Key::Char('j') | Key::Down => {
                    *listener_mutex.lock().unwrap() = Direction::Down;
                }
                Key::Char('w') | Key::Char('k') | Key::Up => {
                    *listener_mutex.lock().unwrap() = Direction::Up;
                }
                Key::Char('a') | Key::Char('h') | Key::Left => {
                    *listener_mutex.lock().unwrap() = Direction::Left;
                }
                Key::Char('d') | Key::Char('l') | Key::Right => {
                    *listener_mutex.lock().unwrap() = Direction::Right;
                }
                Key::Ctrl('c') => break,
                Key::Alt('t') => println!("termion is cool"),
                _ => (),
            }
        }

        thread::sleep(LISTENER_SLEEP);
    });

    let board = [BOARD_CONFIG.background; (BOARD_CONFIG.width as usize) * (BOARD_CONFIG.height as usize)];
    let mut snake = Snake{
        body_char: "*",
        head_char: "@",
        body: VecDeque::from([[2, 3], [2, 2], [1, 2], [0, 2], [0, 1], [0, 0]]),
    };

    let mut food_pos = set_food(&snake, &BOARD_CONFIG).unwrap();

    // Main game loop
    loop {
        println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

        // Copy the board for editing
        let mut curr_board = board;

        // Move the snake
        let attempt_move = move_snake(&mut snake, Arc::clone(&direction), &BOARD_CONFIG);
        if let Err(game_err) = attempt_move {
            match game_err {
                GameError::OutOfBounds => println!("YOU LOSE hehe"),
                GameError::Collision => println!("You ate yourself, mate :("),
                GameError::FoodPosition => println!("Could not place new food. So I guess you win?"),
            }
            break;
        }

        // Maybe eat food
        if snake.body[0] == food_pos{
            let placing_result = set_food(&snake, &BOARD_CONFIG);
            if placing_result.is_err(){
                println!("Could not place new food. That's a bad programmer :(");
                break;
            }
            food_pos = placing_result.unwrap();
            
            // Try to create a new body instance at the rear
            if let Some(last_element ) = snake.body.back(){
                snake.body.push_back(*last_element);
            }
            else {
                println!("Could not handle eating food :(");
                break;
            }
        }

        // Make changes to the board
        put_snake_to_board(&snake, &mut curr_board, &BOARD_CONFIG);

        // Final rendering
        print_board(&mut curr_board, &food_pos, &BOARD_CONFIG);

        thread::sleep(GAME_SLEEP);
    }
}

fn set_food(snake: &Snake, board_config: &Board) -> Result<[usize; 2], GameError> {
    let mut food_pos = [0, 0];
    let mut rng = rng();
    for _ in 0..FOOD_PLACING_MAX_TRIES{
        food_pos[0] = rng.random_range(0..board_config.height) as usize; 
        food_pos[1] = rng.random_range(0..board_config.width) as usize; 

        // Check for collision 
        if !snake.body.contains(&food_pos){
            return Ok(food_pos);
        }
    }

    Err(GameError::FoodPosition)
}

fn move_snake(
    snake: &mut Snake,
    direction: Arc<Mutex<Direction>>,
    board_config: &Board
) -> Result<(), GameError> {
    // Get the direction from mutex
    let dir;
    if let Ok(inner_dir_data) = direction.lock() {
        dir = (*inner_dir_data).clone();
    } else {
        dir = Direction::Down;
    }

    let [row, col] = snake.body[0];
    let new_pos = match dir {
        Direction::Down => {
            if row >= (board_config.height - 1).into(){
                return Err(GameError::OutOfBounds);
            }
            [row + 1, col]
        }
        Direction::Up => {
            if row == 0 {
                return Err(GameError::OutOfBounds);
            }
            [row - 1, col]
        }
        Direction::Right => {
            if col >= board_config.width.into() {
                return Err(GameError::OutOfBounds);
            }
            [row, col + 1]
        }
        Direction::Left => {
            if col == 0 {
                return Err(GameError::OutOfBounds);
            }
            [row, col - 1]
        }
    };

    // Collision with itself
    if snake.body.contains(&new_pos) {
        return Err(GameError::Collision);
    }

    // Remove tail and append Head
    snake.body.pop_back();
    snake.body.push_front(new_pos);

    Ok(())
}

fn print_board(board: &mut [&str], food_pos: &[usize; 2], board_config: &Board) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    board[food_pos[0] * board_config.width as usize + food_pos[1]] = FOOD_CHAR;

    let top_boundary = "╭".to_string() + &"─".repeat(board_config.width as usize) + "╮";
    let bot_boundary = "╰".to_string() + &"─".repeat(board_config.width as usize) + "╯";

    println!("{top_boundary}\r");
    for r in 0..(board_config.height as usize) {
        let start_index = r * (board_config.width as usize);

        let mut row = String::new();
        for col in board
            .iter()
            .skip(start_index)
            .take(board_config.width.into())
        {
            row += col;
        }

        println!("│{row}│\r");
    }
    println!("{bot_boundary}");
    stdout.flush().unwrap();
}

fn put_snake_to_board<'a>(snake: &'a Snake, board: &mut [&'a str], board_config: &Board) {
    // Body
    for piece in snake.body.iter().take(snake.body.len()) {
        let index = piece[0] * board_config.width as usize + piece[1];
        board[index] = snake.body_char;
    }

    // Head
    let index: usize = snake.body[0][0] * (board_config.width as usize) + snake.body[0][1];
    board[index] = snake.head_char;
}
