///! Structs and types related to the state of the game board.
use crate::bits::masks;


/// A BitBoard is a 64-bit unsigned integer which gives piece occupancy. See chessprogrammingwiki
/// page for more details.
pub type BitBoard = u64;


/// Index for a square.
pub type Square = u8;


/// Piece type index into GameState's bitboards arrays. To be used in conjuction with Color enum
/// for black.
///
/// e.g. 
/// let white_rooks = gamestate.bbs[PieceIndex::ROOK];
/// let black_queens = gamestate.bbs[Color::BLACK+PieceIndex::QUEEN];
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    WhitePawn   = 0,
    WhiteBishop = 1,
    WhiteKnight = 2,
    WhiteRook   = 3,
    WhiteQueen  = 4,
    WhiteKing   = 5,
    BlackPawn   = 6,
    BlackBishop = 7,
    BlackKnight = 8,
    BlackRook   = 9,
    BlackQueen  = 10,
    BlackKing   = 11,
    Null        = 12,
}


/// Represents the state of the board as well as game metadata (en passant square, castle rights, 
/// and player to move). 
/// See FEN (Forsyth-Edwards) Notation wiki page for more info.
#[derive(Clone, Copy)]
pub struct GameState {
    pub bbs: [BitBoard;12],
    white_to_move: bool,
    ep_square: Option<Square>, // BitBoard with only en passant square set.
    halfmove_clock: u8,
    castlerights: [bool;4], // White/black, kingside and queenside.
    occupancy: PieceBitBoards,
}

// Public functions for GameState.
impl GameState {
    /// Returns a new gamestate with empty bbs, white to move, no ep square, 0 halfmove clock, full
    /// castle rights.
    pub fn new_empty() -> Self {
        GameState {
            bbs: [0;12],
            white_to_move: true,
            ep_square: None, // BitBoard with only en passant square set.
            halfmove_clock: 0,
            castlerights: [true;4],
            occupancy: PieceBitBoards::new(),
        }
    }

    /// Make a new GameState.
    pub fn new(
        bbs: [BitBoard;12],
        white_to_move: bool,
        ep_square: Option<Square>,
        halfmove_clock: u8,
        castlerights: [bool;4],
    ) -> Self {
        GameState {
            bbs,
            white_to_move,
            ep_square,
            halfmove_clock,
            castlerights,
            occupancy: PieceBitBoards::new(),
        }

    }

    /// Find the bitboard index of the piece occupying the square given by sq_idx. If no such
    /// bitboard exists, return None.
    pub fn occupying_piece(&self, sq: Square) -> Option<Piece> {
        self.occupancy.get(sq)
    }

    /// Set the bit at the given sq_idx on the given bitboard.
    pub fn add_piece(&mut self, piece: Piece, sq: Square) {
        self.occupancy.put(sq, piece);
        self.bbs[piece as usize] |= masks::SQUARES[sq as usize]
    }

    /// Unset the bit at the given sq_idx on the given bitboard.
    pub fn remove_piece(&mut self, sq: Square) {
        let bb = self.occupancy.get(sq);
        match self.occupancy.get(sq) {
            None => return,
            Some(pi) => {
                self.occupancy.remove(sq);
                self.bbs[pi as usize] &= !masks::SQUARES[sq as usize]
            }
        }
    }
}


/// Data structure to map from square number -> occupying piece bitboard idx.
#[derive(Copy, Clone)]
struct PieceBitBoards {
    map: [Piece;64],
}

/// Map from squre to occupying piece.
impl PieceBitBoards {
    pub fn new() -> Self {
        PieceBitBoards { map: [Piece::Null;64] }
    }

    /// Get Some(<piece_at_sq>) or None if there is no such piece.
    pub fn get(&self, sq: Square) -> Option<Piece> {
        match self.map[sq as usize] {
            Piece::Null => None,
            pi => Some(pi),
        }
    }

    /// Remove the piece from the map at the given square.
    pub fn remove(&mut self, sq: Square) {
        self.map[sq as usize] = Piece::Null;
    }

    /// Put the piece at the given square in the map.
    pub fn put(&mut self, sq: Square, pi: Piece) {
        self.map[sq as usize] = pi;
    }
}


/// We store the state stack in the stack for fast access. Thus we need a max size.
pub const MAX_MOVESTACK_DEPTH: usize = 100;

/// Represents a stack of game states that have occured from the initial position.
struct GameStateStack {
    backing: [Option<GameState>;MAX_MOVESTACK_DEPTH],
    size: usize,
}

impl GameStateStack {
    pub fn new() -> Self {
        GameStateStack { backing: [None;MAX_MOVESTACK_DEPTH], size: 0 }
    }

    pub fn push(&mut self, elt: GameState) {
        self.backing[self.size] = Some(elt);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<GameState> {
        if self.size <= 0 {
            return None;
        }
        let mut res = None;
        std::mem::swap(&mut self.backing[self.size], &mut res);
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test setting/unsetting each bit.
    pub fn test_set_unset() {
        let mut game_state = GameState::new_empty();
        game_state.add_piece(Piece::WhitePawn, 30);
        assert_eq!(game_state.occupying_piece(30), Some(Piece::WhitePawn));
        game_state.remove_piece(30);
        assert_eq!(game_state.occupying_piece(30), None);
    }
}
