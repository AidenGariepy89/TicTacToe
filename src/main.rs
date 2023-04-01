use tictactoe::{
    basic_game,
    board,
    board::Board,
    ultigame,
    ultiboard::UltimateBoard, input::get_input,
};

enum Game {
    TicTacToe,
    Ultimate,
    NoGame,
}

fn main() {
    loop {
        let mut game = Game::NoGame;

        println!("Would you like to play 'n'ormal TicTacToe, or 'u'ltimate TicTacToe?");
        let input = get_input().to_lowercase();
        let input = input.trim();

        match input {
            "n" => { game = Game::TicTacToe; },
            "u" => { game = Game::Ultimate; },
            _ => { },
        }

        match game {
            Game::TicTacToe => { tictactoe(); },
            Game::Ultimate => { ultimate(); },
            Game::NoGame => { break; }
        }
    }
}

fn ultimate() {
    use ultigame::LoopState;

    let mut board = UltimateBoard::new();
    let mut result = ultigame::LoopState::Continue;

    while let LoopState::Continue = result {
        result = ultigame::run(&mut board);
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

    let selected = ultiboard.get_focus();
    match selected {
        BoardSelection::Unselected => println!("No selection!"),
        BoardSelection::Selected(s) => println!("Selected board {}", s),
    }

    ultiboard.focus(BoardSelection::Selected(1)).unwrap();
    ultiboard.play(5).unwrap();

    let selected = ultiboard.get_focus();
    match selected {
        BoardSelection::Unselected => println!("No selection!"),
        BoardSelection::Selected(s) => println!("Selected board {}", s),
    }
}

