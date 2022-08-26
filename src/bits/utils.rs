use crate::game_state::{ BitBoard, Square };


const LSB3_BITMASK: u64 = 7;

/// Get index of lsb on the given bitboard.
pub fn bitscan(bb: BitBoard) -> u32 {
    bb.trailing_zeros()
}

/// Get least significant bit in the given bitboard.
pub fn lsb_mask(bb: BitBoard) -> BitBoard {
    bb & 0u64.wrapping_sub(bb)
}

/// Get the index of a square from its rank and file indices.
pub fn square_idx(rank_idx: u8, file_idx: u8) -> Square {
    (rank_idx << 3) + file_idx
}

/// Get the file index of a square.
/// See Little-Endian Rank-File Mapping on www.chessprogramming.org/Square_Mapping_Considerations.
pub fn file_idx(square_idx: u8) -> u8 {
    square_idx & (LSB3_BITMASK as u8)
}

/// Get the rank index of a square.
/// See Little-Endian Rank-File Mapping on www.chessprogramming.org/Square_Mapping_Considerations.
pub fn rank_idx(square_idx: u8) -> u8 {
    square_idx >> 3
}
