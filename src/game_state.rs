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
pub enum PieceIndex {
    Pawn = 0,
    Bishop = 1,
    Knight = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
    Null = 6,
}

pub enum Color {
    Black = 6,
    White = 0,
}


/// Represents the state of the board as well as game metadata (en passant square, castle rights, 
/// and player to move). 
/// See FEN (Forsyth-Edwards) Notation wiki page for more info.
pub struct GameState {
    pub bbs: [BitBoard;12],
    white_to_move: bool,
    ep_square: Option<u64>, // BitBoard with only en passant square set.
    halfmove_clock: u8,
    white_castlerights: [bool;2], // Kingside and queenside.
    black_castlerights: [bool;2],
    occupancy: PieceBitBoards,
}

// Public functions for GameState.
impl GameState {

    /// Returns a new gamestate with empty bbs, white to move, no ep square, 0 halfmove clock, full
    /// castle rights.
    pub fn new_empty() -> Self {
        GameState{
            bbs: [0;12],
            white_to_move: true,
            ep_square: None, // BitBoard with only en passant square set.
            halfmove_clock: 0,
            white_castlerights: [true;2],
            black_castlerights: [true;2],
            occupancy: PieceBitBoards::new(),
        }
    }

    /// Find the bitboard index of the piece occupying the square given by sq_idx. If no such
    /// bitboard exists, return None.
    pub fn occupying_piece(&self, sq: Square) -> Option<PieceIndex> {
        self.occupancy.get(sq)
    }

    /// Set the bit at the given sq_idx on the given bitboard.
    pub fn add_piece(&mut self, bb: PieceIndex, sq: Square) {
        self.occupancy.put(sq, bb);
        self.bbs[bb as usize] |= masks::SQUARES[sq as usize]
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
struct PieceBitBoards {
    map: [PieceIndex;64],
}

/// Map from squre to occupying piece.
impl PieceBitBoards {
    pub fn new() -> Self {
        PieceBitBoards { map: [PieceIndex::Null;64] }
    }

    /// Get Some(<piece_at_sq>) or None if there is no such piece.
    pub fn get(&self, sq: Square) -> Option<PieceIndex> {
        match self.map[sq as usize] {
            PieceIndex::Null => None,
            pi => Some(pi),
        }
    }

    /// Remove the piece from the map at the given square.
    pub fn remove(&mut self, sq: Square) {
        self.map[sq as usize] = PieceIndex::Null;
    }

    /// Put the piece at the given square in the map.
    pub fn put(&mut self, sq: Square, pi: PieceIndex) {
        self.map[sq as usize] = pi;
    }
}


/// We store this in the stack for fast access. Thus we need a max size.
pub const MAX_MOVESTACK_DEPTH: usize = 100;
struct GameStateStack {
    backing: [Option<StackElt>;MAX_MOVESTACK_DEPTH],
    size: usize,
}

impl GameStateStack {
    pub fn new() -> Self {
        GameStateStack { backing: [None;MAX_MOVESTACK_DEPTH], size: 0 }
    }

    pub fn push(&mut self, elt: StackElt) {
        self.backing[self.size] = Some(elt);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<StackElt> {
        if self.size <= 0 {
            return None;
        }
        let mut res = None;
        std::mem::swap(&mut self.backing[self.size], &mut res);
        res
    }
}

#[derive(Copy, Clone)]
struct StackElt {
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test setting/unsetting each bit.
    pub fn test_set_unset() {
        let mut game_state = GameState::new_empty();
        game_state.add_piece(PieceIndex::Pawn, 30);
        assert_eq!(game_state.occupying_piece(30), Some(PieceIndex::Pawn));
        game_state.remove_piece(30);
        assert_eq!(game_state.occupying_piece(30), None);
    }

}
