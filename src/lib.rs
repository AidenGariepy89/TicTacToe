#[macro_use]
mod input;
pub mod board;

use board::{Board, Piece};
use std::fmt;

#[derive(Debug)]
enum GameError {
    InvalidInputError,
    InputFailedError
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            GameError::InvalidInputError => write!(f, "Invalid input!"),
            GameError::InputFailedError => write!(f, "Input failed!"),
        };
    }
}

type GameResult<T> = Result<T, GameError>;

pub enum LoopState {
    Continue,
    Exit,
}

pub fn run(board: &mut Board, turn: Piece) -> LoopState {
    clearscr!();
    println!("Welcome to TicTacToe! Please input to make your move! 'q' to quit\n");

    board.print();

    match board.win_check() {
        board::EndGame::Winner(winner) => {
            match winner {
                Piece::X => {
                    println!("X wins!");
                    return LoopState::Exit;
                },
                Piece::O => {
                    println!("O wins!");
                    return LoopState::Exit;
                },
                _ => { },
            }
        },
        board::EndGame::CatsGame => {
            println!("Cat's game!");
            return LoopState::Exit;
        },
        _ => { },
    }

    let mut input = String::new();
    get_input(&mut input).unwrap();

    if input.trim() == "q" { return LoopState::Exit; }

    match notation_to_index(&input) {
        Ok(index) => {
            board.play(index, turn).unwrap_or_else(|error| { println!("{}", error); });
        },
        Err(error) => {
            println!("{}", error);
            get_input(&mut input).unwrap();
            return LoopState::Continue;
        },
    }

    return LoopState::Continue;
}

/// # Panics
/// Subtraction that results in negative number no work yet
fn notation_to_index(input: &str) -> GameResult<usize> {
    let mut it = input.chars();
    let row = it.next().unwrap_or_else(|| 'z') as usize - 'a' as usize;
    let col = it.next().unwrap_or_else(|| '9') as usize - '1' as usize;

    if row > board::ROW_LEN || col > board::ROW_LEN { return Err(GameError::InvalidInputError); }

    let index = (row * 3) + col;
    return Ok(index);
}

fn get_input(input: &mut String) -> GameResult<()> {
    if let Err(_) = std::io::stdin().read_line(input) {
        return Err(GameError::InputFailedError);
    }
    return Ok(());
}

#[test]
fn test_board() {
    let mut board = Board::new();

    board.print();

    board.play(1, Piece::X).unwrap_or_else(|error| { println!("{}", error); });
    board.play(5, Piece::O).unwrap_or_else(|error| { println!("{}", error); });

    board.print();

    notation_to_index("a1").unwrap();
    notation_to_index("c3").unwrap();
    notation_to_index("b1").unwrap();

}

