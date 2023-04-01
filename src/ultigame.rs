use crate::ultiboard::{UltimateBoard, Piece, BoardSelection, self};

pub enum LoopState {
    Continue,
    Exit,
}

type GameResult<T> = Result<T, GameError>;

enum GameError {
    InvalidInput,
}

pub fn run(board: &mut UltimateBoard, turn: Piece) -> LoopState {
    clearscr!();
    println!("Welcome to Ultimate TicTacToe! Please input to make your move! 'q' to quit");

    board.print();

    if let BoardSelection::Unselected = board.get_focus() {
    
    }

    return LoopState::Exit;
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

