use crate::{utils::LoopState, input::get_input};
use self::cubeboard::CubeBoard;
use std::fmt;

pub mod cubeboard;

#[derive(Debug)]
enum GameError {
    InvalidInputError,
}

type GameResult<T> = Result<T, GameError>;

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::InvalidInputError => write!(f, "Invalid input!"),
        }
    }
}

pub fn run(board: &mut CubeBoard) -> LoopState {
    clearscr!();

    board.print();

    println!("({}) Make your move! (Example move: xa1 - moves to layer x, row a, and column 1)", board.get_turn().to_colored_string());

    let input = get_input().to_lowercase();
    let input = input.trim();

    if input == "q" { return LoopState::Exit; }

    match notation_to_index(input) {
        Ok((layer, index)) => {
            if let Err(error) = board.play(layer, index) {
                println!("{} Press 'Enter' to continue.", error);

                #[allow(unused_variables)]
                let input = get_input();

                return LoopState::Exit;
            }
        },
        Err(error) => {
            println!("{} Press 'Enter' to continue.", error);

            #[allow(unused_variables)]
            let input = get_input();

            return LoopState::Continue;
        },
    }

    board.next_turn();
    return LoopState::Continue;
}

fn notation_to_index(input: &str) -> GameResult<(usize, usize)> {
    let mut it = input.chars();
    let layer = it.next().unwrap_or_else(|| '{') as usize;
    let row = it.next().unwrap_or_else(|| 'z') as usize;
    let column = it.next().unwrap_or_else(|| '9') as usize;

    if layer < 'x' as usize || row < 'a' as usize || column < '1' as usize { return Err(GameError::InvalidInputError); }

    let layer = layer - 'x' as usize;
    let row = row - 'a' as usize;
    let column = column - '1' as usize;

    if layer > 2 || row > 2 || column > 2 { return Err(GameError::InvalidInputError); }

    let index = (row * 3) + column;
    return Ok((layer, index));
}

