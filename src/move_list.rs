///! This file contains the MoveList struct.
use crate::game_move::GameMove;


/// A 256-length array to hold a list of GameMoves. Fixed length array (not Vec) to store in the 
/// stack for faster access.
pub struct MoveList {
    backing: [GameMove;256],
    size: usize,
}

impl MoveList {
    /// Create a new, empty MoveList.
    pub fn new() -> Self {
        MoveList { backing: [GameMove { data: u16::MAX };256], size: 0 }
    }

    /// Add the given GameMove to the end of the MoveList.
    pub fn push(&mut self, elt: GameMove) {
        self.backing[self.size as usize] = elt;
        self.size += 1;
    }

    pub fn peek(&self) -> Option<GameMove> {
        if self.size <= 0 {
            return None;
        }
        Some(self.backing[self.size-1])
    }

    pub fn pop(&mut self) -> Option<GameMove> {
        if self.size <= 0 {
            return None;
        }
        let val = Some(self.backing[self.size-1]);
        self.size -= 1;
        val
    }
}
