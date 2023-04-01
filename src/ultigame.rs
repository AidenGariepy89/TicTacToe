use crate::{ultiboard::{UltimateBoard, BoardSelection, self, BoardState}, input::get_input};
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

pub fn run(board: &mut UltimateBoard) -> LoopState {
    clearscr!();
    println!("Welcome to {} Please input to make your move! {}", "Ultimate TicTacToe!".green().bold(), "'q' to quit".red());

    board.win_check();

    board.print();

    match board.get_focus() {
        BoardSelection::Selected(index) => {
            println!("{} {}", "Current board:".green(), usize_to_notation(index.clone()));
        },
        BoardSelection::Unselected => {
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
                },
            }
            return LoopState::Continue;
        },
    }

    println!("Make your move, {}!", board.get_turn().to_char());
    let input = get_input();

    if input.trim() == "q" { return LoopState::Exit; }

    match notation_to_usize(&input) {
        Ok(index) => {
            board.play(index).unwrap();

            if let BoardState::InPlay = board.get_board_state(index) {
                board.focus(BoardSelection::Selected(index)).unwrap();
            } else {
                board.focus(BoardSelection::Unselected).unwrap();
            }
        }
        Err(error) => {
            println!("{}", error);

            #[allow(unused_variables)]
            let input = get_input();

            return LoopState::Continue;
        }
    }

    board.next_turn();

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
    // bad solution
    
    match input {
        0 => String::from("A1"),
        1 => String::from("A2"),
        2 => String::from("A3"),
        3 => String::from("B1"),
        4 => String::from("B2"),
        5 => String::from("B3"),
        6 => String::from("C1"),
        7 => String::from("C2"),
        8 => String::from("C3"),
        _ => String::from("uh oh"),
    }
}

