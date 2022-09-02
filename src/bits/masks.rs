///! This file contains various useful bitmasks for bitboards.
use crate::game_state::{ BitBoard, Square };


// Bit representations for ranks.
pub const RANK_1: BitBoard = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;
pub const RANK_2: BitBoard = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
pub const RANK_3: BitBoard = 0b00000000_00000000_00000000_00000000_00000000_11111111_00000000_00000000;
pub const RANK_4: BitBoard = 0b00000000_00000000_00000000_00000000_11111111_00000000_00000000_00000000;
pub const RANK_5: BitBoard = 0b00000000_00000000_00000000_11111111_00000000_00000000_00000000_00000000;
pub const RANK_6: BitBoard = 0b00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000;
pub const RANK_7: BitBoard = 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
pub const RANK_8: BitBoard = 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000;

// Bit representations for files.
pub const FILE_A: BitBoard = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
pub const FILE_B: BitBoard = 0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010;
pub const FILE_C: BitBoard = 0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100;
pub const FILE_D: BitBoard = 0b00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000;
pub const FILE_E: BitBoard = 0b00010000_00010000_00010000_00010000_00010000_00010000_00010000_00010000;
pub const FILE_F: BitBoard = 0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000;
pub const FILE_G: BitBoard = 0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000;
pub const FILE_H: BitBoard = 0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;

// Bit representations for squares.
pub const SQUARES: [BitBoard; 64] = make_square_masks();

// Knight move masks.
pub const KNIGHT_MOVES: [BitBoard; 64] = make_knight_move_masks();


// Make masks with bit set for each square.
const fn make_square_masks() -> [BitBoard; 64] {
    let mut squares = [0; 64];
    let mut i = 0;
    while i < 64 {
        squares[i] = 1 << i;
        i += 1;
    }
    squares
}

/// Make masks for knight moves.
const fn make_knight_move_masks() -> [BitBoard; 64] {
    let mut masks = [0; 64];
    let mut i: Square = 0;
    while i < 64 {
        masks[i as usize] = make_knight_mask(i);
        i += 1;
    }
    masks
}

/// Make mask for knight move from a square.
const fn make_knight_mask(sq: Square) -> BitBoard {
    let mut mask = 0;
    let knight_sq = SQUARES[sq as usize];

    mask |= knight_sq << 17 & !FILE_A;
    mask |= knight_sq << 10 & !(FILE_A | FILE_B);
    mask |= knight_sq >>  6 & !(FILE_A | FILE_B);
    mask |= knight_sq >> 15 & !FILE_A;
    mask |= knight_sq << 15 & !FILE_H;
    mask |= knight_sq <<  6 & !(FILE_H | FILE_G);
    mask |= knight_sq >> 10 & !(FILE_H | FILE_G);
    mask |= knight_sq >> 17 & !FILE_H;

    mask
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_square_masks() {
        assert!(SQUARES[0]  == RANK_1 & FILE_A);
        assert!(SQUARES[7]  == RANK_1 & FILE_H);
        assert!(SQUARES[56] == RANK_8 & FILE_A);
        assert!(SQUARES[63] == RANK_8 & FILE_H);
    }

}
