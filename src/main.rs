use tictactoe::{
    LoopState,
    board::Board,
    board::Piece,
};

fn main() {
    let mut board = Board::new();
    let mut result = LoopState::Continue;
    let mut turn = Piece::X;
    while let LoopState::Continue = result {
        result = tictactoe::run(&mut board, turn.clone());
        turn = match turn {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
            _ => Piece::X,
        };
    }
}

