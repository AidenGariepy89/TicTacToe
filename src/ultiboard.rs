use colored::*;
use std::fmt;

use crate::utils::Piece;

// Constants

// Starting at index 0
pub const BOARD_LEN: usize = 8;
pub const ROW_LEN: usize = 2;

const WIN_STATES: [u16; 8] = [
    0b1_1100_0000,
    0b0_0011_1000,
    0b0_0000_0111,
    0b1_0010_0100,
    0b0_1001_0010,
    0b0_0100_1001,
    0b1_0001_0001,
    0b0_0101_0100,
];

// Type Definitions

pub struct UltimateBoard {
    boards: [Board; 9],
    active_board: BoardSelection,
    turn: Piece,
}

struct Board {
    spaces: [Piece; 9],
    active: bool,
    state: BoardState,
}

#[derive(Clone, Copy)]
pub enum BoardState {
    InPlay,
    Winner(Piece),
    CatsGame,
}

pub enum BoardSelection {
    Unselected,
    Selected(usize),
}

#[derive(Debug)]
pub enum UltiError {
    OutOfBoundsError,
    SpaceTakenError,
}

pub type UltiResult<T> = Result<T, UltiError>;

// Type Implementations

impl fmt::Display for UltiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UltiError::SpaceTakenError => write!(f, "There is already a piece there!"),
            UltiError::OutOfBoundsError => write!(f, "That space does not exist!"),
        }
    }
}

impl Board {
    fn new() -> Self {
        Self {
            active: false,
            state: BoardState::InPlay,
            spaces: [ Piece::Empty; 9 ],
        }
    }

    fn play(&mut self, space: usize, piece: Piece) -> UltiResult<()> {
        if !self.active { panic!(); }

        match self.spaces[space] {
            Piece::Empty => self.spaces[space] = piece,
            _ => return Err(UltiError::SpaceTakenError),
        }

        return Ok(());
    }

    fn get_state(&self) -> BoardState { self.state }

    fn win_check(&mut self) -> BoardState {
        let mut xs: u16 = 0b0_0000_0000;
        let mut os: u16 = 0b0_0000_0000;

        for i in 0..self.spaces.len() {
            xs <<= 1;
            os <<= 1;
            match self.spaces[i] {
                Piece::X => { xs ^= 0b0_0000_0001; },
                Piece::O => { os ^= 0b0_0000_0001; },
                Piece::Empty => { },
            }

            //println!("X: 0b{:09b}", xs);
            //println!("O: 0b{:09b}", os);
        }

        for state in WIN_STATES {
            if xs & state == state {
                self.state = BoardState::Winner(Piece::X);

                self.spaces = [Piece::X; 9];

                return self.state;
            }
            if os & state == state {
                self.state = BoardState::Winner(Piece::O);

                self.spaces = [Piece::O; 9];

                return self.state;
            }
        }

        if xs | os == 0b1_1111_1111 {
            self.state = BoardState::CatsGame;
            return self.state;
        }

        return BoardState::InPlay;
    }
}

impl UltimateBoard {
    pub fn new() -> Self {
        Self {
            active_board: BoardSelection::Unselected,
            turn: Piece::X,
            boards: [
                Board::new(),
                Board::new(),
                Board::new(),
                Board::new(),
                Board::new(),
                Board::new(),
                Board::new(),
                Board::new(),
                Board::new(),
            ],
        }
    }

    pub fn focus(&mut self, board_index: BoardSelection) -> UltiResult<()> {
        if let BoardSelection::Selected(s) = self.active_board {
            self.boards[s].active = false;
        }

        if let BoardSelection::Selected(s) = board_index {
            if s > BOARD_LEN { return Err(UltiError::OutOfBoundsError); }

            match self.boards[s].state {
                BoardState::InPlay => {
                    self.boards[s].active = true;
                },
                _ => {
                    self.active_board = BoardSelection::Unselected;
                    return Ok(());
                },
            }

        }

        self.active_board = board_index;

        return Ok(());
    }

    pub fn get_focus(&self) -> &BoardSelection { return &self.active_board; }

    pub fn play(&mut self, space: usize) -> UltiResult<()> {
        if space > BOARD_LEN { return Err(UltiError::OutOfBoundsError); }

        let index;

        match self.active_board {
            BoardSelection::Unselected => { panic!("No board is active!"); },
            BoardSelection::Selected(s) => {
                index = s;
            },
        }

        return self.boards[index].play(space, self.turn);
    }

    pub fn get_turn(&self) -> Piece { self.turn }

    pub fn next_turn(&mut self) {
        self.turn = match self.turn {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
            _ => Piece::X,
        };
    }

    pub fn get_board_state(&self, index: usize) -> BoardState {
        return self.boards[index].get_state();
    }

    pub fn win_check(&mut self) -> BoardState {
        //let mut count = 0;
        for board in &mut self.boards {
            if let BoardState::InPlay = board.state {
                //println!("checking board {}", count);
                board.win_check();
            }
            //count += 1;
        }

        if let BoardSelection::Selected(index) = self.active_board {
            match self.boards[index].state {
                BoardState::InPlay => { },
                _ => {
                    self.focus(BoardSelection::Unselected).unwrap();
                },
            }
        }

        let mut xg: u16 = 0b0_0000_0000;
        let mut og: u16 = 0b0_0000_0000;

        for i in 0..self.boards.len() {
            xg <<= 1;
            og <<= 1;
            if let BoardState::Winner(piece) = self.boards[i].state {
                match piece {
                    Piece::X => { xg ^= 0b0_0000_0001; },
                    Piece::O => { og ^= 0b0_0000_0001; },
                    Piece::Empty => { },
                }
            }
        }

        for state in WIN_STATES {
            if xg & state == state { return BoardState::Winner(Piece::X); }
            if og & state == state { return BoardState::Winner(Piece::O); }
        }

        if xg | og == 0b1_1111_1111 { return BoardState::CatsGame; }

        return BoardState::InPlay;
    }

    pub fn print(&self) {
        println!(
            "
             ___________1___________ ___________2___________ ___________3___________ 
            |   _____ _____ _____   |   _____ _____ _____   |   _____ _____ _____   |
            |  |     |     |     |  |  |     |     |     |  |  |     |     |     |  |
            |  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |
            |  |_____|_____|_____|  |  |_____|_____|_____|  |  |_____|_____|_____|  |
            |  |     |     |     |  |  |     |     |     |  |  |     |     |     |  |
            A  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |
            |  |_____|_____|_____|  |  |_____|_____|_____|  |  |_____|_____|_____|  |
            |  |     |     |     |  |  |     |     |     |  |  |     |     |     |  |
            |  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |
            |  |_____|_____|_____|  |  |_____|_____|_____|  |  |_____|_____|_____|  |
            |_______________________|_______________________|_______________________|
            |   _____ _____ _____   |   _____ _____ _____   |   _____ _____ _____   |
            |  |     |     |     |  |  |     |     |     |  |  |     |     |     |  |
            |  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |
            |  |_____|_____|_____|  |  |_____|_____|_____|  |  |_____|_____|_____|  |
            |  |     |     |     |  |  |     |     |     |  |  |     |     |     |  |
            B  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |
            |  |_____|_____|_____|  |  |_____|_____|_____|  |  |_____|_____|_____|  |
            |  |     |     |     |  |  |     |     |     |  |  |     |     |     |  |
            |  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |
            |  |_____|_____|_____|  |  |_____|_____|_____|  |  |_____|_____|_____|  |
            |_______________________|_______________________|_______________________|
            |   _____ _____ _____   |   _____ _____ _____   |   _____ _____ _____   |
            |  |     |     |     |  |  |     |     |     |  |  |     |     |     |  |
            |  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |
            |  |_____|_____|_____|  |  |_____|_____|_____|  |  |_____|_____|_____|  |
            |  |     |     |     |  |  |     |     |     |  |  |     |     |     |  |
            C  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |
            |  |_____|_____|_____|  |  |_____|_____|_____|  |  |_____|_____|_____|  |
            |  |     |     |     |  |  |     |     |     |  |  |     |     |     |  |
            |  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |  |  {}  |  {}  |  {}  |  |
            |  |_____|_____|_____|  |  |_____|_____|_____|  |  |_____|_____|_____|  |
            |_______________________|_______________________|_______________________|
            ",
            self.boards[0].spaces[0].to_colored_string(),
            self.boards[0].spaces[1].to_colored_string(),
            self.boards[0].spaces[2].to_colored_string(),
            self.boards[1].spaces[0].to_colored_string(),
            self.boards[1].spaces[1].to_colored_string(),
            self.boards[1].spaces[2].to_colored_string(),
            self.boards[2].spaces[0].to_colored_string(),
            self.boards[2].spaces[1].to_colored_string(),
            self.boards[2].spaces[2].to_colored_string(),
            self.boards[0].spaces[3].to_colored_string(),
            self.boards[0].spaces[4].to_colored_string(),
            self.boards[0].spaces[5].to_colored_string(),
            self.boards[1].spaces[3].to_colored_string(),
            self.boards[1].spaces[4].to_colored_string(),
            self.boards[1].spaces[5].to_colored_string(),
            self.boards[2].spaces[3].to_colored_string(),
            self.boards[2].spaces[4].to_colored_string(),
            self.boards[2].spaces[5].to_colored_string(),
            self.boards[0].spaces[6].to_colored_string(),
            self.boards[0].spaces[7].to_colored_string(),
            self.boards[0].spaces[8].to_colored_string(),
            self.boards[1].spaces[6].to_colored_string(),
            self.boards[1].spaces[7].to_colored_string(),
            self.boards[1].spaces[8].to_colored_string(),
            self.boards[2].spaces[6].to_colored_string(),
            self.boards[2].spaces[7].to_colored_string(),
            self.boards[2].spaces[8].to_colored_string(),
            self.boards[3].spaces[0].to_colored_string(),
            self.boards[3].spaces[1].to_colored_string(),
            self.boards[3].spaces[2].to_colored_string(),
            self.boards[4].spaces[0].to_colored_string(),
            self.boards[4].spaces[1].to_colored_string(),
            self.boards[4].spaces[2].to_colored_string(),
            self.boards[5].spaces[0].to_colored_string(),
            self.boards[5].spaces[1].to_colored_string(),
            self.boards[5].spaces[2].to_colored_string(),
            self.boards[3].spaces[3].to_colored_string(),
            self.boards[3].spaces[4].to_colored_string(),
            self.boards[3].spaces[5].to_colored_string(),
            self.boards[4].spaces[3].to_colored_string(),
            self.boards[4].spaces[4].to_colored_string(),
            self.boards[4].spaces[5].to_colored_string(),
            self.boards[5].spaces[3].to_colored_string(),
            self.boards[5].spaces[4].to_colored_string(),
            self.boards[5].spaces[5].to_colored_string(),
            self.boards[3].spaces[6].to_colored_string(),
            self.boards[3].spaces[7].to_colored_string(),
            self.boards[3].spaces[8].to_colored_string(),
            self.boards[4].spaces[6].to_colored_string(),
            self.boards[4].spaces[7].to_colored_string(),
            self.boards[4].spaces[8].to_colored_string(),
            self.boards[5].spaces[6].to_colored_string(),
            self.boards[5].spaces[7].to_colored_string(),
            self.boards[5].spaces[8].to_colored_string(),
            self.boards[6].spaces[0].to_colored_string(),
            self.boards[6].spaces[1].to_colored_string(),
            self.boards[6].spaces[2].to_colored_string(),
            self.boards[7].spaces[0].to_colored_string(),
            self.boards[7].spaces[1].to_colored_string(),
            self.boards[7].spaces[2].to_colored_string(),
            self.boards[8].spaces[0].to_colored_string(),
            self.boards[8].spaces[1].to_colored_string(),
            self.boards[8].spaces[2].to_colored_string(),
            self.boards[6].spaces[3].to_colored_string(),
            self.boards[6].spaces[4].to_colored_string(),
            self.boards[6].spaces[5].to_colored_string(),
            self.boards[7].spaces[3].to_colored_string(),
            self.boards[7].spaces[4].to_colored_string(),
            self.boards[7].spaces[5].to_colored_string(),
            self.boards[8].spaces[3].to_colored_string(),
            self.boards[8].spaces[4].to_colored_string(),
            self.boards[8].spaces[5].to_colored_string(),
            self.boards[6].spaces[6].to_colored_string(),
            self.boards[6].spaces[7].to_colored_string(),
            self.boards[6].spaces[8].to_colored_string(),
            self.boards[7].spaces[6].to_colored_string(),
            self.boards[7].spaces[7].to_colored_string(),
            self.boards[7].spaces[8].to_colored_string(),
            self.boards[8].spaces[6].to_colored_string(),
            self.boards[8].spaces[7].to_colored_string(),
            self.boards[8].spaces[8].to_colored_string(),
            );
    }
}

//            ______________________ ______________________ ______________________ 
//           |   _____ _____ _____  |   _____ _____ _____  |   _____ _____ _____  |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |_____|_____|_____| |  |_____|_____|_____| |  |_____|_____|_____| |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |_____|_____|_____| |  |_____|_____|_____| |  |_____|_____|_____| |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |_____|_____|_____| |  |_____|_____|_____| |  |_____|_____|_____| |
//           |______________________|______________________|______________________|
//           |   _____ _____ _____  |   _____ _____ _____  |   _____ _____ _____  |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |_____|_____|_____| |  |_____|_____|_____| |  |_____|_____|_____| |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |_____|_____|_____| |  |_____|_____|_____| |  |_____|_____|_____| |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |_____|_____|_____| |  |_____|_____|_____| |  |_____|_____|_____| |
//           |______________________|______________________|______________________|
//           |   _____ _____ _____  |   _____ _____ _____  |   _____ _____ _____  |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |_____|_____|_____| |  |_____|_____|_____| |  |_____|_____|_____| |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |_____|_____|_____| |  |_____|_____|_____| |  |_____|_____|_____| |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |     |     |     | |  |     |     |     | |  |     |     |     | |
//           |  |_____|_____|_____| |  |_____|_____|_____| |  |_____|_____|_____| |
//           |______________________|______________________|______________________|
