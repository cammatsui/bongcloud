use crate::game_state::{ GameState, Square, PieceIndex };

// Do not change the order.
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
pub struct GameMove {
    data: u16,
}

const PROMO_TYPE_MASK: u16 = 0b1000;
const CAPTURE_TYPE_MASK: u16 = 0b0100;
const LSB6_BITMASK: u16 = 63;
const LSB4_BITMASK: u16 = 15;
impl GameMove {
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

    /// Get the MoveType of this move. Could it be optimized to not cast to usize?
    pub fn movetype(&self) -> MoveType {
        METADATA_TO_MOVETYPE[(LSB4_BITMASK & self.data) as usize]
    }

    /// Returns whether this move is a capture.
    pub fn is_capture(&self) -> bool {
        CAPTURE_TYPE_MASK & self.data != 0
    }

    /// Returns whether this move is a promotion.
    pub fn is_promo(&self) -> bool {
        PROMO_TYPE_MASK & self.data != 0
    }
}

/// Represents the types of moves that can occur. See GameMove docs.
#[derive(Copy, Clone)]
pub enum MoveType {
    NullMove,
    Quiet,
    DoublePawnPush,
    KingCastle,
    QueenCastle,
    Capture,
    EpCapture,
    KnightPromo,
    BishopPromo,
    RookPromo,
    QueenPromo,
    KnightPromoCapture,
    BishopPromoCapture,
    RookPromoCapture,
    QueenPromoCapture,
}



