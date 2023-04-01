use tictactoe::{
    basic_game,
    board,
    board::Board,
    ultigame,
    ultiboard,
    ultiboard::UltimateBoard,
};

enum Game {
    TicTacToe,
    Ultimate,
}

fn main() {
    let game = Game::Ultimate;

    match game {
        Game::TicTacToe => { tictactoe(); },
        Game::Ultimate => { ultimate(); },
    }
}

fn ultimate() {
    use ultiboard::Piece;
    use ultigame::LoopState;

    let mut board = UltimateBoard::new();
    let mut result = ultigame::LoopState::Continue;
    let mut turn = Piece::X;

    while let LoopState::Continue = result {
        result = ultigame::run(&mut board, turn.clone());
        turn = match turn {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
            _ => Piece::X,
        };
    }
}

fn tictactoe() {
    use board::Piece;
    use basic_game::LoopState;

    let mut board = Board::new();
    let mut result = LoopState::Continue;
    let mut turn = Piece::X;

    while let LoopState::Continue = result {
        result = basic_game::run(&mut board, turn.clone());
        turn = match turn {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
            _ => Piece::X,
        };
    }
}

#[test]
fn test() {
    use tictactoe::ultiboard::{UltimateBoard, BoardSelection};

    let mut ultiboard = UltimateBoard::new();

    println!("{}", ultiboard.print());

    let selected = ultiboard.get_focus();
    match selected {
        BoardSelection::Unselected => println!("No selection!"),
        BoardSelection::Selected(s) => println!("Selected board {}", s),
    }

    ultiboard.focus(BoardSelection::Selected(1)).unwrap();
    ultiboard.play(5, tictactoe::ultiboard::Piece::X).unwrap();

    let selected = ultiboard.get_focus();
    match selected {
        BoardSelection::Unselected => println!("No selection!"),
        BoardSelection::Selected(s) => println!("Selected board {}", s),
    }
}

