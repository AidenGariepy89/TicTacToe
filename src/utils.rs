use colored::*;

pub enum LoopState {
    Continue,
    Exit,
}

#[derive(Clone, Copy)]
pub enum Piece {
    X,
    O,
    Empty,
}

impl Piece {
    pub fn to_colored_string(&self) -> ColoredString {
        match self {
            Piece::X => "X".bright_red(),
            Piece::O => "O".green(),
            Piece::Empty => " ".normal(),
        }
    }
}

