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
    state: BoardState,
    active_board: BoardSelection,
}

struct Board {
    spaces: [Piece; 9],
    active: bool,
    state: BoardState,
}

#[derive(Clone, Copy)]
pub enum Piece {
    X,
    O,
    Empty,
}

enum BoardState {
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

impl Piece {
    fn to_char(&self) -> char {
        match self {
            Piece::X => 'X',
            Piece::O => 'O',
            Piece::Empty => ' ',
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
}

impl UltimateBoard {
    pub fn new() -> Self {
        Self {
            active_board: BoardSelection::Unselected,
            state: BoardState::InPlay,
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
            self.boards[s].active = true;
        }

        self.active_board = board_index;

        return Ok(());
    }

    pub fn get_focus(&self) -> &BoardSelection { return &self.active_board; }

    pub fn play(&mut self, space: usize, piece: Piece) -> UltiResult<()> {
        if space > BOARD_LEN { return Err(UltiError::OutOfBoundsError); }

        let mut index = 0;

        match self.active_board {
            BoardSelection::Unselected => { panic!("No board is active!"); },
            BoardSelection::Selected(s) => {
                index = s;
            },
        }

        return self.boards[index].play(space, piece);
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
            self.boards[0].spaces[0].to_char(),
            self.boards[0].spaces[1].to_char(),
            self.boards[0].spaces[2].to_char(),
            self.boards[1].spaces[0].to_char(),
            self.boards[1].spaces[1].to_char(),
            self.boards[1].spaces[2].to_char(),
            self.boards[2].spaces[0].to_char(),
            self.boards[2].spaces[1].to_char(),
            self.boards[2].spaces[2].to_char(),
            self.boards[0].spaces[3].to_char(),
            self.boards[0].spaces[4].to_char(),
            self.boards[0].spaces[5].to_char(),
            self.boards[1].spaces[3].to_char(),
            self.boards[1].spaces[4].to_char(),
            self.boards[1].spaces[5].to_char(),
            self.boards[2].spaces[3].to_char(),
            self.boards[2].spaces[4].to_char(),
            self.boards[2].spaces[5].to_char(),
            self.boards[0].spaces[6].to_char(),
            self.boards[0].spaces[7].to_char(),
            self.boards[0].spaces[8].to_char(),
            self.boards[1].spaces[6].to_char(),
            self.boards[1].spaces[7].to_char(),
            self.boards[1].spaces[8].to_char(),
            self.boards[2].spaces[6].to_char(),
            self.boards[2].spaces[7].to_char(),
            self.boards[2].spaces[8].to_char(),
            self.boards[3].spaces[0].to_char(),
            self.boards[3].spaces[1].to_char(),
            self.boards[3].spaces[2].to_char(),
            self.boards[4].spaces[0].to_char(),
            self.boards[4].spaces[1].to_char(),
            self.boards[4].spaces[2].to_char(),
            self.boards[5].spaces[0].to_char(),
            self.boards[5].spaces[1].to_char(),
            self.boards[5].spaces[2].to_char(),
            self.boards[3].spaces[3].to_char(),
            self.boards[3].spaces[4].to_char(),
            self.boards[3].spaces[5].to_char(),
            self.boards[4].spaces[3].to_char(),
            self.boards[4].spaces[4].to_char(),
            self.boards[4].spaces[5].to_char(),
            self.boards[5].spaces[3].to_char(),
            self.boards[5].spaces[4].to_char(),
            self.boards[5].spaces[5].to_char(),
            self.boards[3].spaces[6].to_char(),
            self.boards[3].spaces[7].to_char(),
            self.boards[3].spaces[8].to_char(),
            self.boards[4].spaces[6].to_char(),
            self.boards[4].spaces[7].to_char(),
            self.boards[4].spaces[8].to_char(),
            self.boards[5].spaces[6].to_char(),
            self.boards[5].spaces[7].to_char(),
            self.boards[5].spaces[8].to_char(),
            self.boards[6].spaces[0].to_char(),
            self.boards[6].spaces[1].to_char(),
            self.boards[6].spaces[2].to_char(),
            self.boards[7].spaces[0].to_char(),
            self.boards[7].spaces[1].to_char(),
            self.boards[7].spaces[2].to_char(),
            self.boards[8].spaces[0].to_char(),
            self.boards[8].spaces[1].to_char(),
            self.boards[8].spaces[2].to_char(),
            self.boards[6].spaces[3].to_char(),
            self.boards[6].spaces[4].to_char(),
            self.boards[6].spaces[5].to_char(),
            self.boards[7].spaces[3].to_char(),
            self.boards[7].spaces[4].to_char(),
            self.boards[7].spaces[5].to_char(),
            self.boards[8].spaces[3].to_char(),
            self.boards[8].spaces[4].to_char(),
            self.boards[8].spaces[5].to_char(),
            self.boards[6].spaces[6].to_char(),
            self.boards[6].spaces[7].to_char(),
            self.boards[6].spaces[8].to_char(),
            self.boards[7].spaces[6].to_char(),
            self.boards[7].spaces[7].to_char(),
            self.boards[7].spaces[8].to_char(),
            self.boards[8].spaces[6].to_char(),
            self.boards[8].spaces[7].to_char(),
            self.boards[8].spaces[8].to_char(),
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
