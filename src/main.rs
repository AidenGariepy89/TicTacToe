use tictactoe::{
    basic_game,
    board::Board,
    ultigame,
    ultiboard::UltimateBoard, input::get_input, utils::LoopState, cubegame::{cubeboard::CubeBoard, self},
};

enum Game {
    TicTacToe,
    Ultimate,
    Cube,
    NoGame,
}

fn main() {
    loop {
        let mut game = Game::NoGame;

        println!("Would you like to play 'n'ormal TicTacToe, 'u'ltimate TicTacToe, or '3'D TicTacToe?");
        let input = get_input().to_lowercase();
        let input = input.trim();

        match input {
            "n" => { game = Game::TicTacToe; },
            "u" => { game = Game::Ultimate; },
            "3" => { game = Game::Cube; },
            _ => { },
        }

        match game {
            Game::TicTacToe => { tictactoe(); },
            Game::Ultimate => { ultimate(); },
            Game::Cube => { cube(); },
            Game::NoGame => { break; }
        }
    }
}

fn ultimate() {
    let mut board = UltimateBoard::new();
    let mut result = LoopState::Continue;

    while let LoopState::Continue = result {
        result = ultigame::run(&mut board);
    }
}

fn tictactoe() {
    let mut board = Board::new();
    let mut result = LoopState::Continue;

    while let LoopState::Continue = result {
        result = basic_game::run(&mut board);
    }
}

fn cube() {
    let mut board = CubeBoard::new();
    let mut result = LoopState::Continue;

    while let LoopState::Continue = result {
        result = cubegame::run(&mut board);
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

