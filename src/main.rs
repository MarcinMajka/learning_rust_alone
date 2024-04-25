use rand::{self, Rng};

#[allow(dead_code)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, PartialEq)]
enum Field {
    Empty,
    Apple,
    Wall,
    Player,
}

impl Field {
    fn matct_field(&self) -> String {
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
            }
        }
    }
}

fn print_board(board: &Board) {
    for row in &board.field {
        for cell in row {
            print!("{} ", cell.matct_field());
        }
        println!();
    }
}

fn main() {
    let mut board = initialize_board();
    let mut rng = rand::thread_rng();
    generate_apples(&mut board, &mut rng);
    generate_player(&mut board, &mut rng);
    println!("Apples:");
    for apple in &board.apples_positions {
        println!("{:?}", apple);
    }
    println!("Player:");
    println!("{:?}", &board.player_position);
    update_board(&mut board);
    println!("Board:");
    print_board(&board);
}
