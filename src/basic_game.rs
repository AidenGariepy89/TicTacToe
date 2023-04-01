use std::fmt;
use crate::board::{Board, Piece, EndGame};
use crate::input::get_input;

#[derive(Debug)]
enum GameError {
    InvalidInputError,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            GameError::InvalidInputError => write!(f, "Invalid input!"),
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
        EndGame::Winner(winner) => {
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
        EndGame::CatsGame => {
            println!("Cat's game!");
            return LoopState::Exit;
        },
        _ => { },
    }

    let input = get_input();

    if input.trim() == "q" { return LoopState::Exit; }

    match notation_to_index(input.trim()) {
        Ok(index) => {
            board.play(index, turn).unwrap_or_else(|error| { println!("{}", error); });
        },
        Err(error) => {
            println!("{}", error);

            #[allow(unused_variables)]
            let input = get_input();

            return LoopState::Continue;
        },
    }

    return LoopState::Continue;
}

fn notation_to_index(input: &str) -> GameResult<usize> {
    let mut it = input.chars();
    let row = it.next().unwrap_or_else(|| 'z') as usize;
    let col = it.next().unwrap_or_else(|| '9') as usize;
    
    if row < 'a' as usize || col < '1' as usize { return Err(GameError::InvalidInputError); }

    let row = row - 'a' as usize;
    let col = col - '1' as usize;

    if row > crate::board::ROW_LEN || col > crate::board::ROW_LEN { return Err(GameError::InvalidInputError); }

    let index = (row * 3) + col;
    return Ok(index);
}

//fn get_input(input: &mut String) -> GameResult<()> {
//    if let Err(_) = std::io::stdin().read_line(input) {
//        return Err(GameError::InputFailedError);
//    }
//    return Ok(());
//}

