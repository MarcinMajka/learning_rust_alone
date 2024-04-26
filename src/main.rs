use rand::{self, Rng};
use std::io;

#[derive(PartialEq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
    Invalid,
}
// Have to use &mut String, because it's for user input, which is modifiable by definition
fn match_move(user_input: &mut String) -> Move {
    // trim() returns &str, so it matches the type
    match user_input.trim() {
        "w" => Move::Up,
        "s" => Move::Down,
        "a" => Move::Left,
        "d" => Move::Right,
        _ => Move::Invalid,
    }
}

fn make_move(mov: Move, board: &mut Board) {
    match mov {
        Move::Up => {
            if board.player_position.row > 1 {
                board.player_position.row -= 1;
                println!("You moved up!");
            }
        }
        Move::Down => {
            if board.player_position.row < 6 {
                board.player_position.row += 1;
                println!("You moved down!");
            }
        }
        Move::Left => {
            if board.player_position.col > 1 {
                board.player_position.col -= 1;
                println!("You moved left!");
            }
        }
        Move::Right => {
            if board.player_position.col < 6 {
                board.player_position.col += 1;
                println!("You moved right!");
            }
        }
        Move::Invalid => println!("Invalid move!"),
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Field {
    Empty,
    Apple,
    Wall,
    Player,
}

impl Field {
    fn match_field(&self) -> String {
        match self {
            Field::Empty => ".",
            Field::Apple => "*",
            Field::Wall => "#",
            Field::Player => "@",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Loc {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
struct Board {
    field: Vec<Vec<Field>>,
    apples_positions: Vec<Loc>,
    player_position: Loc,
}

fn initialize_board() -> Board {
    let board = Board {
        field: vec![vec![Field::Empty; 8]; 8],
        apples_positions: vec![],
        player_position: Loc { row: 0, col: 0 },
    };
    board
}

fn generate_apples(board: &mut Board, rng: &mut rand::rngs::ThreadRng) {
    while board.apples_positions.len() < 7 {
        let row: usize = rng.gen_range(1..6);
        let col: usize = rng.gen_range(1..6);
        let current_apple_position = Loc { row, col };
        if !board.apples_positions.contains(&current_apple_position) {
            board.apples_positions.push(current_apple_position);
        }
    }
}

fn generate_player(board: &mut Board, rng: &mut rand::rngs::ThreadRng) {
    loop {
        let row: usize = rng.gen_range(1..6);
        let col: usize = rng.gen_range(1..6);

        if !board.apples_positions.contains(&Loc { row, col }) {
            board.player_position = Loc { row, col };
            break;
        }
    }
}

fn update_board(board: &mut Board) {
    for row in 0..8 {
        for col in 0..8 {
            let current_loc = Loc { row, col };
            let should_be_wall = row == 0 || row == 7 || col == 0 || col == 7;

            if current_loc == board.player_position {
                board.field[row][col] = Field::Player;
            } else if board.apples_positions.contains(&current_loc) {
                board.field[row][col] = Field::Apple;
            } else if should_be_wall {
                board.field[row][col] = Field::Wall;
            } else {
                board.field[row][col] = Field::Empty;
            }
        }
    }
}

fn print_board(board: &Board) {
    for row in &board.field {
        for cell in row {
            print!("{} ", cell.match_field());
        }
        println!();
    }
}

fn update_player_position(board: &mut Board) {
    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let matched_move = match_move(&mut user_input);
        if matched_move != Move::Invalid {
            make_move(matched_move, board);
            break;
        }
    }
}

fn remove_apples(board: &mut Board) {
    if board.apples_positions.contains(&board.player_position) {
        // Idea taken from https://stackoverflow.com/questions/26243025/how-to-remove-an-element-from-a-vector-given-the-element
        // Basically the same as chatGPT said:
        // 1. Create an iterator of apples positions
        // 2. Get the index of the apple the player is currently standing on
        // 3. Remove the value from the index
        let apple_to_be_eaten_index = board
            .apples_positions
            .iter()
            .position(|x| *x == board.player_position)
            .unwrap();
        board.apples_positions.remove(apple_to_be_eaten_index);
        println!("Apple eaten!");
    }
}

fn no_apples_left(board: &Board) -> bool {
    board.apples_positions.len() == 0
}

fn print_congrats_and_restart() {
    println!("\nCongrats! Y O U  W O N!!!\n");
    main();
}

fn main() {
    let mut board = initialize_board();
    let mut rng = rand::thread_rng();

    generate_apples(&mut board, &mut rng);
    generate_player(&mut board, &mut rng);

    loop {
        update_board(&mut board);
        print_board(&board);
        update_player_position(&mut board);
        remove_apples(&mut board);
        if no_apples_left(&board) {
            print_congrats_and_restart();
        }
    }
}
