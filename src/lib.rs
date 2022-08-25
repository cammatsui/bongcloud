/// A BitBoard is a 64-bit unsigned integer which gives piece occupancy. See chessprogrammingwiki
/// page for more details.
type BitBoard = u64;

/// See Little-Endian Rank-File Mapping on www.chessprogramming.org/Square_Mapping_Considerations.
mod idx_utils {
    const LSB3_BITMASK: u64 = 7;

    /// Get the index of a square from its rank and file indices.
    pub fn square_idx(rank_idx: u64, file_idx: u64) -> u64 {
        (rank_idx<<3) + file_idx
    }

    /// Get the file index of a square.
    pub fn file_idx(square_idx: u64) -> u64 {
        square_idx & LSB3_BITMASK
    }

    /// Get the rank index of a square.
    pub fn rank_idx(square_idx: u64) -> u64 {
        square_idx >> 3
    }
}



/// Piece type index into GameState's bitboards arrays. To be used in conjuction with Color enum
/// for black.
///
/// e.g. 
/// let white_rooks = gamestate.bbs[PieceIndex::ROOK];
/// let black_queens = gamestate.bbs[Color::BLACK+PieceIndex::QUEEN];
enum PieceIndex {
    PAWN = 0,
    BISHOP = 1,
    KNIGHT = 2,
    ROOK = 3,
    QUEEN = 4,
    KING = 5,
}

enum Color {
    BLACK = 6,
}


/// Represents the state of the board as well as game metadata (en passant square, castle rights, 
/// and player to move). 
/// See FEN (Forsyth-Edwards) Notation wiki page for more info.
pub struct GameState {
    bbs: [BitBoard;12],
    white_to_move: bool,
    ep_square: Option<u64>, // BitBoard with only en passant square set.
    halfmove_clock: u8,
    white_castlerights: [bool;2], // Kingside and queenside.
    black_castlerights: [bool;2],
    square_masks: [BitBoard;64],
}

// Public functions for GameState.
impl GameState {
    /// Make a GameMove on the board.
    pub fn make_move(&mut self, game_move: GameMove) {
        // TODO: For now we just remove the piece from the from-square and remove a piece from the
        // to-square if it exists. Of course we need to add logic to handle special moves.
        // Also, we want to increase speed to find which piece (index into self.bbs) we are moving 
        // and which we are capturing.
        let moving_bb_idx = self.occupying_bb_idx(game_move.fromsquare())
            .expect(&format!("No piece on index {}", game_move.fromsquare()));
        let captured_bb_idx = self.occupying_bb_idx(game_move.tosquare());
        if let Some(bb_idx) = captured_bb_idx {
            self.unset_bit(bb_idx, game_move.tosquare());
        }
        self.unset_bit(moving_bb_idx, game_move.fromsquare());
        self.set_bit(moving_bb_idx, game_move.tosquare());
    }

    /// Determine whether the bit at `sq_idx` on the bitboard specified by `bb_idx` is set.
    pub fn bit_set(&self, bb_idx: usize, sq_idx: usize) -> bool {
        self.bbs[bb_idx] & self.square_masks[sq_idx] != 0
    }
}

// Utility functions for GameState.
impl GameState {
    fn occupying_bb_idx(&self, sq_idx: usize) -> Option<usize> {
        for i in 0..self.bbs.len() {
            if self.bbs[i] & self.square_masks[sq_idx] != 0 {
                return Some(i);
            }
        }
        None
    }

    fn set_bit(&mut self, bb_idx: usize, sq_idx: usize) {
        self.bbs[bb_idx] |= self.square_masks[sq_idx]
    }

    fn unset_bit(&mut self, bb_idx: usize, sq_idx: usize) {
        self.bbs[bb_idx] &= !self.square_masks[sq_idx]
    }

    /// Make an array `masks` such that `masks[i]` gives `u64` with only the `i`th bit set.
    fn make_square_masks() -> [BitBoard;64] {
        let mut square_masks = [0;64];
        for i in 0..64 {
            square_masks[i] = 1 << (63-i);
        }
        square_masks
    }
}

/// Represents a move with from-to square indices as first 12 (6 for each) bits, and last 4 bits as
/// metadata. 
/// See www.chessprogramming.org/Encoding_Moves for more info.
pub struct GameMove {
    repr: u16,
}

const LSB6_BITMASK: u16 = 63;
impl GameMove {
    pub fn from_val(val: u16) -> Self {
        GameMove { repr: val }
    }

    pub fn fromsquare(&self) -> usize {
        (self.repr >> 10).into()
    }

    pub fn tosquare(&self) -> usize {
        (self.repr >> 4 & LSB6_BITMASK).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_make_move() {
        let mut game_state = GameState{
            bbs: [0;12],
            white_to_move: true,
            ep_square: None, // BitBoard with only en passant square set.
            halfmove_clock: 0,
            white_castlerights: [true;2],
            black_castlerights: [true;2],
            square_masks: GameState::make_square_masks(),
        };
        game_state.set_bit(0, 0);

        let game_move = GameMove::from_val(0b000000_111111_0000u16);
        assert_eq!(game_move.fromsquare(), 0);
        assert_eq!(game_move.tosquare(), 63);

        assert_eq!(game_state.occupying_bb_idx(0), Some(0));
        assert_eq!(game_state.occupying_bb_idx(63), None);
        game_state.make_move(game_move);
        assert_eq!(game_state.occupying_bb_idx(0), None);
        game_state.set_bit(0, 63);
        println!("0 bitboard is {}", game_state.bbs[0]);
        assert_eq!(game_state.occupying_bb_idx(63), Some(0));
    }

    #[test]
    fn test_set_unset() {
        let mut game_state = GameState{
            bbs: [0;12],
            white_to_move: true,
            ep_square: None, // BitBoard with only en passant square set.
            halfmove_clock: 0,
            white_castlerights: [true;2],
            black_castlerights: [true;2],
            square_masks: GameState::make_square_masks(),
        };

        game_state.set_bit(0, 30);
        assert!(game_state.bit_set(0, 30));
        game_state.unset_bit(0, 30);
        assert!(!game_state.bit_set(0, 30));

    }
}
