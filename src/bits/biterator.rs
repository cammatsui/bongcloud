use crate::game_state::BitBoard;
use crate::bits::utils::{ lsb_mask, bitscan };


/// Get a BitErator for a bitboard.
pub fn biterator(bb: BitBoard) -> BitErator {
    BitErator { bb }
}

/// Get a BitBoardErator for a bitboard.
pub fn bitboarderator(bb: BitBoard) -> BitBoardErator {
    BitBoardErator { bb }
}


/// An iterator over bits, returning bit indices of set bits from lsb to msb.
pub struct BitErator {
    bb: BitBoard,
}

impl Iterator for BitErator {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.bb == 0 {
            return None;
        }

        let lsb_idx = bitscan(self.bb);
        self.bb ^= lsb_mask(self.bb);
        Some(lsb_idx)
    }
}


/// An iterator over bits returning bitboards with only next set significant bit in original 
/// bitboard set.
pub struct BitBoardErator {
    bb: BitBoard,
}

impl Iterator for BitBoardErator {
    type Item = BitBoard;

    fn next(&mut self) -> Option<BitBoard> {
        if self.bb == 0 {
            return None;
        }

        let mask = lsb_mask(self.bb);
        self.bb ^= mask;
        Some(mask)
    }
}
