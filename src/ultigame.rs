use crate::{ultiboard::{UltimateBoard, Piece, BoardSelection, self}, input::get_input};
use std::fmt;
use colored::*;

pub enum LoopState {
    Continue,
    Exit,
}

type GameResult<T> = Result<T, GameError>;

#[derive(Debug)]
enum GameError {
    InvalidInput,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::InvalidInput => write!(f, "Invalid input!"),
        }
    }
}

pub fn run(board: &mut UltimateBoard, turn: Piece) -> LoopState {
    clearscr!();
    println!("Welcome to {} Please input to make your move! {}", "Ultimate TicTacToe!".green().bold(), "'q' to quit".red());

    board.print();

    if let BoardSelection::Unselected = board.get_focus() {
        println!("{}\n", "Select a board to play in.".magenta());
        let input = get_input();

        match notation_to_usize(&input) {
            Ok(index) => {
                board.focus(BoardSelection::Selected(index)).unwrap();
            },
            Err(error) => {
                println!("{}", error);

                #[allow(unused_variables)]
                let input = get_input();

                return LoopState::Continue;
            }
        }
    }

    return LoopState::Continue;
}

fn notation_to_usize(input: &str) -> GameResult<usize> {
    let mut it = input.chars();
    let row = it.next().unwrap_or_else(|| 'z') as usize;
    let col = it.next().unwrap_or_else(|| '9') as usize;

    if row < 'a' as usize || col < '1' as usize { return Err(GameError::InvalidInput); }

    let row = row - 'a' as usize;
    let col = col - '1' as usize;

    if row > ultiboard::ROW_LEN || col > ultiboard::ROW_LEN { return Err(GameError::InvalidInput); }

    let index = (row * 3) + col;
    return Ok(index);
}

fn usize_to_notation(input: usize) -> String {
    return String::from("Isn't finished yet");
}

