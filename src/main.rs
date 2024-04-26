use rand::Rng;
use std::io;

#[derive(PartialEq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    // Have to use &mut String, because it's for user input, which is modifiable by definition
    fn match_move(user_input: String) -> Option<Move> {
        // trim() returns &str, so it matches the type
        match user_input.trim() {
            "w" => Some(Move::Up),
            "s" => Some(Move::Down),
            "a" => Some(Move::Left),
            "d" => Some(Move::Right),
            _ => None,
        }
    }

    fn make_move(mov: Move, board: &mut Board) {
        match mov {
            Move::Up if board.player_position.row > 1 => {
                board.player_position.row -= 1;
                println!("You moved up!");
            }
            Move::Down if board.player_position.row < 6 => {
                board.player_position.row += 1;
                println!("You moved down!");
            }
            Move::Left if board.player_position.col > 1 => {
                board.player_position.col -= 1;
                println!("You moved left!");
            }
            Move::Right if board.player_position.col < 6 => {
                board.player_position.col += 1;
                println!("You moved right!");
            }
            _ => {}
        }
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

impl Board {
    fn initialize_board() -> Board {
        let board = Board {
            field: vec![vec![Field::Empty; 8]; 8],
            apples_positions: vec![],
            player_position: Loc { row: 0, col: 0 },
        };
        board
    }

    fn generate_apples(&mut self, rng: &mut rand::rngs::ThreadRng) {
        while self.apples_positions.len() < 7 {
            let row: usize = rng.gen_range(1..6);
            let col: usize = rng.gen_range(1..6);
            let current_apple_position = Loc { row, col };
            if !self.apples_positions.contains(&current_apple_position) {
                self.apples_positions.push(current_apple_position);
            }
        }
    }

    fn generate_player(&mut self, rng: &mut rand::rngs::ThreadRng) {
        loop {
            let row: usize = rng.gen_range(1..6);
            let col: usize = rng.gen_range(1..6);

            if !self.apples_positions.contains(&Loc { row, col }) {
                self.player_position = Loc { row, col };
                break;
            }
        }
    }

    fn update_board(&mut self) {
        for row in 0..8 {
            for col in 0..8 {
                let current_loc = Loc { row, col };
                let should_be_wall = row == 0 || row == 7 || col == 0 || col == 7;

                let matched_field = if current_loc == self.player_position {
                    Field::Player
                } else if self.apples_positions.contains(&current_loc) {
                    Field::Apple
                } else if should_be_wall {
                    Field::Wall
                } else {
                    Field::Empty
                };

                self.field[row][col] = matched_field;
            }
        }
    }

    fn print_board(&self) {
        for row in &self.field {
            for cell in row {
                print!("{} ", cell.match_field());
            }
            println!();
        }
    }

    fn update_player_position(&mut self) {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        if let Some(matched_move) = Move::match_move(user_input) {
            Move::make_move(matched_move, self);
        }
    }
    // Idea taken from https://stackoverflow.com/questions/26243025/how-to-remove-an-element-from-a-vector-given-the-element
    // 1. Create an iterator of apples positions
    // 2. Get the index of the apple the player is currently standing on
    // 3. Remove the value from the index
    // if let works like this:
    // if let Some(variable) = function_returning_something_or_nothing {code block that executes if the function returned something}
    fn remove_apples(&mut self) {
        if let Some(apple_to_be_eaten_index) = self
            .apples_positions
            .iter()
            .position(|x| *x == self.player_position)
        {
            self.apples_positions.remove(apple_to_be_eaten_index);
            println!("Apple eaten!");
        }
    }

    fn no_apples_left(&self) -> bool {
        self.apples_positions.is_empty()
    }
}

fn print_congrats_and_restart() {
    println!("\nCongrats! Y O U  W O N!!!\n");
    main();
}

fn main() {
    let mut board = Board::initialize_board();
    let mut rng = rand::thread_rng();

    Board::generate_apples(&mut board, &mut rng);
    Board::generate_player(&mut board, &mut rng);

    loop {
        Board::update_board(&mut board);
        Board::print_board(&mut board);
        Board::update_player_position(&mut board);
        Board::remove_apples(&mut board);
        if Board::no_apples_left(&board) {
            print_congrats_and_restart();
        }
    }
}
