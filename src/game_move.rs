///! This file contains structs/types related to moves on a GameState.
use crate::game_state::{ Square, Piece };


// Do not change the order!
const METADATA_TO_MOVETYPE: [MoveType;16] = [
    MoveType::Quiet,                // code 0
    MoveType::DoublePawnPush,       // code 1
    MoveType::KingCastle,           // code 2
    MoveType::QueenCastle,          // code 3
    MoveType::Capture,              // code 4
    MoveType::EpCapture,            // code 5
    MoveType::NullMove,             // code 6
    MoveType::NullMove,             // code 7
    MoveType::KnightPromo,          // code 8
    MoveType::BishopPromo,          // code 9
    MoveType::RookPromo,            // code 10
    MoveType::QueenPromo,           // code 11
    MoveType::KnightPromoCapture,   // code 12
    MoveType::BishopPromoCapture,   // code 13
    MoveType::RookPromoCapture,     // code 14
    MoveType::QueenPromoCapture,    // code 15
];


/// Bitmasks for getting info from move.
const IS_PROMO_MASK: u16 = 1u16 << 3;
const IS_CAPTURE_MASK: u16 = 1u16 << 2;
const KNIGHT_PROMO_MASK: u16 = 0u16;
const BISHOP_PROMO_MASK: u16 = 1u16;
const ROOK_PROMO_MASK: u16 = 2u16;
const QUEEN_PROMO_MASK: u16 =  3u16;
const LSB6_BITMASK: u16 = 63;
const LSB4_BITMASK: u16 = 15;


/// Represents a move with from-to square indices as first 12 (6 for each) bits, and last 4 bits as
/// metadata. 
/// See www.chessprogramming.org/Encoding_Moves for more info.
///
/// Metadata:
///  | code | promotion |  capture | special 1 | special 2 | enum val           |
///  |------|-----------|----------|-----------|-----------|--------------------|
///  | 0    | 0         | 0        | 0         | 0         | Quiet              |
///  | 1    | 0         | 0        | 0         | 1         | DoublePawnPush     |
///  | 2    | 0         | 0        | 1         | 0         | KingCastle         |
///  | 3    | 0         | 0        | 1         | 1         | QueenCastle        |
///  | 4    | 0         | 1        | 0         | 0         | Capture            |
///  | 5    | 0         | 1        | 0         | 1         | EpCapture          |
///  | 8    | 1         | 0        | 0         | 0         | KnightPromo        |
///  | 9    | 1         | 0        | 0         | 1         | BishopPromo        |
///  | 10   | 1         | 0        | 1         | 0         | RookPromo          |
///  | 11   | 1         | 0        | 1         | 1         | QueenPromo         |
///  | 12   | 1         | 1        | 0         | 0         | KnightPromoCapture |
///  | 13   | 1         | 1        | 0         | 1         | BishopPromoCapture |
///  | 14   | 1         | 1        | 1         | 0         | RookPromoCapture   |
///  | 15   | 1         | 1        | 1         | 1         | QueenPromoCapture  |
///
/// Note: castling fromsquare is king's square, tosquare is castle side's rook square.
#[derive(Copy, Clone)]
pub struct GameMove {
    pub data: u16,
}

impl GameMove {
    /// Create a gamemove from the given fromsquare, tosquare, and move type.
    pub fn new(fromsquare: Square, tosquare: Square, move_type: MoveType) -> Self {
        let val = ((((0u16 | fromsquare as u16) << 6) | tosquare as u16) << 4) | move_type as u16;
        GameMove { data: val }
    }

    /// Create a GameMove struct from a u16.
    pub fn from_val(val: u16) -> Self {
        GameMove { data: val }
    }

    /// Get the from-square index of this move.
    pub fn fromsquare(&self) -> Square {
        (self.data >> 10) as u8
    }

    /// Get the to-square index of this move.
    pub fn tosquare(&self) -> Square {
        (self.data >> 4 & LSB6_BITMASK) as u8
    }

    /// Get the MoveType of this move.
    pub fn move_type(&self) -> MoveType {
        METADATA_TO_MOVETYPE[(LSB4_BITMASK & self.data) as usize]
    }

    /// Returns whether this move is a capture.
    pub fn is_capture(&self) -> bool {
        IS_CAPTURE_MASK & self.data != 0
    }

    /// Returns whether this move is a promotion.
    pub fn is_promo(&self) -> bool {
        IS_PROMO_MASK & self.data != 0
    }

    /// Get the piece which is to be promoted.
    pub fn promo_piece(&self, white_to_move: bool) -> Option<Piece> {
        if !self.is_promo() {
            return None;
        }

        return match self.data & 3u16 {
            KNIGHT_PROMO_MASK => 
                Some(if white_to_move { Piece::WhiteKnight } else { Piece::BlackKnight }),
            BISHOP_PROMO_MASK =>
                Some(if white_to_move { Piece::WhiteBishop } else { Piece::BlackBishop }),
            ROOK_PROMO_MASK =>
                Some(if white_to_move { Piece::WhiteRook } else { Piece::BlackRook }),
            QUEEN_PROMO_MASK =>
                Some(if white_to_move { Piece::WhiteQueen } else { Piece::BlackQueen }),
            _ => None,
        }
    }
}


/// Represents the types of moves that can occur. See GameMove docs.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum MoveType {
    NullMove            = 0b0110,
    Quiet               = 0b0000,
    DoublePawnPush      = 0b0001,
    KingCastle          = 0b0010,
    QueenCastle         = 0b0011,
    Capture             = 0b0100,
    EpCapture           = 0b0101,
    KnightPromo         = 0b1000,
    BishopPromo         = 0b1001,
    RookPromo           = 0b1010,
    QueenPromo          = 0b1011,
    KnightPromoCapture  = 0b1100,
    BishopPromoCapture  = 0b1101,
    RookPromoCapture    = 0b1110,
    QueenPromoCapture   = 0b1111,
}
