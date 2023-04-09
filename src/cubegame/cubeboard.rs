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

pub struct CubeBoard {
    layers: [Board; 3],
    turn: Piece,
}

struct Board {
    spaces: [Piece; 9],
}

pub type CubeResult<T> = Result<T, CubeError>;

#[derive(Debug)]
pub enum CubeError {
    SpaceTakenError,
}

// Type Implementations
  
impl fmt::Display for CubeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CubeError::SpaceTakenError => write!(f, "Space already taken!"),
        }
    }
}

impl Board {
    fn new() -> Self {
        Self {
            spaces: [Piece::Empty; 9],
        }
    }

    fn play(&mut self, index: usize, piece: Piece) -> CubeResult<()> {
        assert!(index < 9);

        match self.spaces[index] {
            Piece::Empty => { self.spaces[index] = piece; },
            _ => { return Err(CubeError::SpaceTakenError); },
        }

        return Ok(());
    }
}

impl CubeBoard {
    pub fn new() -> Self {
        Self {
            layers: [Board::new(), Board::new(), Board::new()],
            turn: Piece::X,
        }
    }

    pub fn play(&mut self, layer: usize, index: usize) -> CubeResult<()> {
        assert!(layer < 3);

        self.layers[layer].play(index, self.turn)?;

        return Ok(());
    }

    pub fn get_turn(&self) -> Piece { self.turn }

    pub fn next_turn(&mut self) {
        match self.turn {
            Piece::X => { self.turn = Piece::O; },
            Piece::O => { self.turn = Piece::X; },
            Piece::Empty => { },
        }
    }

    pub fn win_check(&self) -> Piece {
        let mut cs_x: [u16; 9] = [0b0_0000_0000; 9];
        let mut cs_o: [u16; 9] = [0b0_0000_0000; 9];
        let mut index = 0;

        for layer in 0..3 {
            for space in 0..9 {
                cs_x[index] <<= 1;
                cs_o[index] <<= 1;

                match self.layers[layer].spaces[space] {
                    Piece::X => { cs_x[index] ^= 0b0_0000_0001; },
                    Piece::O => { cs_o[index] ^= 0b0_0000_0001; },
                    Piece::Empty => { },
                }
            }
            index += 1;
        }

        for row in 0..3 {
            for layer in 0..3 {
                for col in 0..3 {
                    cs_x[index] <<= 1;
                    cs_o[index] <<= 1;

                    match self.layers[layer].spaces[(row * 3) + col] {
                        Piece::X => { cs_x[index] ^= 0b0_0000_0001; },
                        Piece::O => { cs_o[index] ^= 0b0_0000_0001; },
                        Piece::Empty => { },
                    }
                }
            }
            index += 1;
        }

        for col in 0..3 {
            for layer in 0..3 {
                for row in 0..3 {
                    cs_x[index] <<= 1;
                    cs_o[index] <<= 1;

                    match self.layers[layer].spaces[(row * 3) + col] {
                        Piece::X => { cs_x[index] ^= 0b0_0000_0001; },
                        Piece::O => { cs_o[index] ^= 0b0_0000_0001; },
                        Piece::Empty => { },
                    }
                }
            }
            index += 1;
        }

        for state in WIN_STATES {
            for cs in cs_x {
                if cs & state == state { return Piece::X; }
            }
            for cs in cs_o {
                if cs & state == state { return Piece::O; }
            }
        }

        if cs_x[0] & 0b1_0000_0000 == 0b1_0000_0000 &&
           cs_x[1] & 0b0_0001_0000 == 0b0_0001_0000 &&
           cs_x[2] & 0b0_0000_0001 == 0b0_0000_0001 { return Piece::X; }
        if cs_x[0] & 0b0_0000_0001 == 0b0_0000_0001 &&
           cs_x[1] & 0b0_0001_0000 == 0b0_0001_0000 &&
           cs_x[2] & 0b1_0000_0000 == 0b1_0000_0000 { return Piece::X; }
        if cs_x[0] & 0b0_0100_0000 == 0b0_0100_0000 &&
           cs_x[1] & 0b0_0001_0000 == 0b0_0001_0000 &&
           cs_x[2] & 0b0_0000_0100 == 0b0_0000_0100 { return Piece::X; }
        if cs_x[0] & 0b0_0000_0100 == 0b0_0000_0100 &&
           cs_x[1] & 0b0_0001_0000 == 0b0_0001_0000 &&
           cs_x[2] & 0b0_0100_0000 == 0b0_0100_0000 { return Piece::X; }
        if cs_o[0] & 0b1_0000_0000 == 0b1_0000_0000 &&
           cs_o[1] & 0b0_0001_0000 == 0b0_0001_0000 &&
           cs_o[2] & 0b0_0000_0001 == 0b0_0000_0001 { return Piece::O; }
        if cs_o[0] & 0b0_0000_0001 == 0b0_0000_0001 &&
           cs_o[1] & 0b0_0001_0000 == 0b0_0001_0000 &&
           cs_o[2] & 0b1_0000_0000 == 0b1_0000_0000 { return Piece::O; }
        if cs_o[0] & 0b0_0100_0000 == 0b0_0100_0000 &&
           cs_o[1] & 0b0_0001_0000 == 0b0_0001_0000 &&
           cs_o[2] & 0b0_0000_0100 == 0b0_0000_0100 { return Piece::O; }
        if cs_o[0] & 0b0_0000_0100 == 0b0_0000_0100 &&
           cs_o[1] & 0b0_0001_0000 == 0b0_0001_0000 &&
           cs_o[2] & 0b0_0100_0000 == 0b0_0100_0000 { return Piece::O; }

        return Piece::Empty;
    }

    pub fn print(&self) {
        println!(
            "
                     ____1_______2_______3___
                     \\       \\       \\       \\
                     |A   {}   \\   {}   \\   {}   \\
                     | \\_______\\_______\\_______\\
                     |  \\       \\       \\       \\
                     |   B   {}   \\   {}   \\   {}   \\ X 
                     |    \\_______\\_______\\_______\\ 
                     |     \\       \\       \\       \\
                     |      C   {}   \\   {}   \\   {}   \\
                     |       \\_______\\_______\\_______\\
                     |___1___|___2_______3__|        |
                     \\       |       \\       \\       |
                     |A   {}  |\\   {}   \\   {}   \\      |
                     | \\_____|_\\_______\\_______\\     |
                     |  \\    |  \\       \\       \\    |
                     |   B   {}   \\   {}   \\   {}   \\ Y |
                     |    \\__|____\\_______\\_______\\  |
                     |     \\ |     \\       \\       \\ |
                     |      C|  {}   \\   {}   \\   {}   \\|
                     |       |_______\\_______\\_______|
                     |___1___|___2_______3__|        | 
                     \\       |       \\       \\       | 
                      A   {}  |\\   {}   \\   {}   \\      | 
                       \\_____|_\\_______\\_______\\     | 
                        \\    |  \\       \\       \\    |
                         B   {}   \\   {}   \\   {}   \\ Z |
                          \\__|____\\_______\\_______\\  |
                           \\ |     \\       \\       \\ |
                            C|  {}   \\   {}   \\   {}   \\|
                             |_______\\_______\\_______|
         ",
         self.layers[0].spaces[0].to_colored_string(),
         self.layers[0].spaces[1].to_colored_string(),
         self.layers[0].spaces[2].to_colored_string(),
         self.layers[0].spaces[3].to_colored_string(),
         self.layers[0].spaces[4].to_colored_string(),
         self.layers[0].spaces[5].to_colored_string(),
         self.layers[0].spaces[6].to_colored_string(),
         self.layers[0].spaces[7].to_colored_string(),
         self.layers[0].spaces[8].to_colored_string(),
         self.layers[1].spaces[0].to_colored_string(),
         self.layers[1].spaces[1].to_colored_string(),
         self.layers[1].spaces[2].to_colored_string(),
         self.layers[1].spaces[3].to_colored_string(),
         self.layers[1].spaces[4].to_colored_string(),
         self.layers[1].spaces[5].to_colored_string(),
         self.layers[1].spaces[6].to_colored_string(),
         self.layers[1].spaces[7].to_colored_string(),
         self.layers[1].spaces[8].to_colored_string(),
         self.layers[2].spaces[0].to_colored_string(),
         self.layers[2].spaces[1].to_colored_string(),
         self.layers[2].spaces[2].to_colored_string(),
         self.layers[2].spaces[3].to_colored_string(),
         self.layers[2].spaces[4].to_colored_string(),
         self.layers[2].spaces[5].to_colored_string(),
         self.layers[2].spaces[6].to_colored_string(),
         self.layers[2].spaces[7].to_colored_string(),
         self.layers[2].spaces[8].to_colored_string(),
        );
    }
}

//
//    ________________________
//    \       \       \       \
//    |\   X   \   X   \   X   \
//    | \_______\_______\_______\
//    |  \       \       \       \
//    |   \   X   \   X   \   X   \
//    |    \_______\_______\_______\
//    |     \       \       \       \
//    |      \   X   \   X   \   X   \
//    |       \_______\_______\_______\
//    |_______|______________|        |
//    \       |       \       \       |
//    |\   X  |\   X   \   X   \      |
//    | \_____|_\_______\_______\     |
//    |  \    |  \       \       \    |
//    |   \   X   \   X   \   X   \   |
//    |    \__|____\_______\_______\  |
//    |     \ |     \       \       \ |
//    |      \|  X   \   X   \   X   \|
//    |       |_______\_______\_______|
//    |_______|______________|        |
//    \       |       \       \       |
//     \   X  |\   X   \   X   \      |
//      \_____|_\_______\_______\     |
//       \    |  \       \       \    |
//        \   X   \   X   \   X   \   |
//         \__|____\_______\_______\  |
//          \ |     \       \       \ |
//           \|  X   \   X   \   X   \|
//            |_______\_______\_______|
//
//
//
