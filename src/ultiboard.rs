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

struct UltimateBoard {
    boards: [Board; 9],
    state: BoardState,
    active_board: i32,
}

struct Board {
    spaces: [Piece; 9],
    active: bool,
    state: BoardState,
}

#[derive(Clone, Copy)]
enum Piece {
    X,
    O,
    Empty,
}

enum BoardState {
    InPlay,
    Winner(Piece),
    CatsGame,
}

enum UltiError {
    OutOfBoundsError,
    SpaceTakenError,
}

type UltiResult<T> = Result<T, UltiError>;

// Type Implementations

impl UltimateBoard {
    fn new() -> Self {
        Self {
            active_board: -1,
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

    fn play(&mut self, space: usize, piece: Piece) -> UltiResult<()> {
        if space > BOARD_LEN { return Err(UltiError::OutOfBoundsError); }
        if self.active_board < 0 { panic!("No board is active!"); }
        if self.active_board as usize > BOARD_LEN { panic!("Board does not exist!"); }

        return self.boards[self.active_board as usize].play(space, piece);
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

