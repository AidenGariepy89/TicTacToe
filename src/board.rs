use std::fmt;

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

#[derive(Debug, Clone)]
pub enum BoardError {
    SpaceTakenError,
    OutOfBoundsError,
}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoardError::SpaceTakenError => write!(f, "This space is already occupied!"),
            BoardError::OutOfBoundsError => write!(f, "That space does not exist!"),
        }
    }
}

#[derive(Clone)]
pub enum Piece {
    X,
    O,
    Empty,
}

impl Piece {
    fn to_char(&self) -> char {
        return match self {
            Piece::X => 'X',
            Piece::O => 'O',
            Piece::Empty => ' ',
        };
    }
}

pub enum EndGame {
    Winner(Piece),
    CatsGame,
    NotDone,
}

pub struct Board {
    spaces: [Piece; 9],
}

impl Board {
    pub fn new() -> Board {
        return Board {
            spaces: [
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
            ],
        };
    }

    pub fn play(&mut self, space: usize, piece: Piece) -> Result<(), BoardError> {
        if space > BOARD_LEN {
            return Err(BoardError::OutOfBoundsError);
        };

        match &self.spaces[space] {
            Piece::Empty => {
                self.spaces[space] = piece;
            }
            _ => {
                return Err(BoardError::SpaceTakenError);
            }
        }

        return Ok(());
    }

    pub fn win_check(&self) -> EndGame {
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
        }

        for state in WIN_STATES {
            if xs & state == state { return EndGame::Winner(Piece::X); }
            if os & state == state { return EndGame::Winner(Piece::O); }
        }

        if xs | os == 0b1_1111_1111 { return EndGame::CatsGame; }

        return EndGame::NotDone;
    }

    pub fn print(&self) {
        let board_display = String::from(format!(
            //"      1     2     3\n    _____ _____ _____\n   |     |     |     |\n a |  {}  |  {}  |  {}  |\n   |_____|_____|_____|\n   |     |     |     |\n b |  {}  |  {}  |  {}  |\n   |_____|_____|_____|\n   |     |     |     |\n c |  {}  |  {}  |  {}  |\n   |_____|_____|_____|\n",
            "      1     2     3\n                     \n         |     |     \n a    {}  |  {}  |  {}  \n    _____|_____|_____\n         |     |     \n b    {}  |  {}  |  {}  \n    _____|_____|_____\n         |     |     \n c    {}  |  {}  |  {}  \n         |     |     \n",
            self.spaces[0].to_char(),
            self.spaces[1].to_char(),
            self.spaces[2].to_char(),
            self.spaces[3].to_char(),
            self.spaces[4].to_char(),
            self.spaces[5].to_char(),
            self.spaces[6].to_char(),
            self.spaces[7].to_char(),
            self.spaces[8].to_char()
        ));

        println!("{board_display}");
    }
}

//"    1     2     3\n
//        |     |     \n
// a      |     |     \n
//   _____|_____|_____\n
//        |     |     \n
// b      |     |     \n
//   _____|_____|_____\n
//        |     |     \n
// c      |     |     \n
//        |     |     \n"
