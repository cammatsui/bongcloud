///! This file contains structs and types related to the state of the game board.
use crate::bits::masks;
use crate::game_move::{ GameMove, MoveType };


/// A BitBoard is a 64-bit unsigned integer which gives piece occupancy. See chessprogrammingwiki
/// page for more details.
pub type BitBoard = u64;


/// Index for a square.
pub type Square = u8;


/// Piece type index into GameState's bitboards arrays.
///
/// e.g.
/// let white_rooks = gamestate.bbs[Piece::WhiteRook];
/// let black_queens = gamestate.bbs[Piece::BlackQueen];
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    WhitePawn = 0,
    WhiteBishop = 1,
    WhiteKnight = 2,
    WhiteRook = 3,
    WhiteQueen = 4,
    WhiteKing = 5,
    BlackPawn = 6,
    BlackBishop = 7,
    BlackKnight = 8,
    BlackRook = 9,
    BlackQueen = 10,
    BlackKing = 11,
    Null = 12,
}


/// Represents a Game; a wrapper around a StateStack of GameStates.
pub struct Game {
    stack: StateStack,
    depth_from_start: u8,
}

impl Game {
    /// Create a new game, initializing the StateStack with the given starting GameState.
    pub fn new(starting_state: GameState) -> Self {
        let mut game = Game {
            stack: StateStack::new(),
            depth_from_start: 0,
        };
        game.stack.push(starting_state);
        game
    }

    /// Apply the given GameMove to the current state and push the new state to the stack.
    pub fn make(&mut self, game_move: GameMove) {
        let cur_state = self.stack.peek().unwrap();
        let next_state = cur_state.make(game_move);
        self.stack.push(next_state);
        self.depth_from_start += 1;
    }

    /// Revert this Game to the state before the previous move.
    pub fn unmake(&mut self) {
        self.stack.pop();
        self.depth_from_start -= 1;
    }

    /// Get the current GameState.
    pub fn current_state(&self) -> GameState {
        *self.stack.peek().unwrap()
    }
}


/// Represents the state of the board as well as game metadata (en passant square, castle rights,
/// and player to move).
///
/// See FEN (Forsyth-Edwards) Notation wiki page for more info.
#[derive(Clone, Copy)]
pub struct GameState {
    pub bbs: [BitBoard; 12],
    pub white_to_move: bool,
    pub ep_square: Option<Square>, // BitBoard with only en passant square set.
    pub halfmove_clock: u8,
    pub fullmove_clock: u32,
    pub castlerights: [bool; 4], // White/black, kingside and queenside.
    occupancy: PieceBitBoards,
}

// Public functions for GameState.
impl GameState {
    /// Returns a new gamestate with empty bbs, white to move, no ep square, 0 halfmove clock, full
    /// castle rights.
    pub fn new_empty() -> Self {
        GameState {
            bbs: [0; 12],
            white_to_move: true,
            ep_square: None, // BitBoard with only en passant square set.
            halfmove_clock: 0,
            fullmove_clock: 1,
            castlerights: [true; 4],
            occupancy: PieceBitBoards::new(),
        }
    }

    /// Make a new GameState.
    pub fn new(
        bbs: [BitBoard; 12],
        white_to_move: bool,
        ep_square: Option<Square>,
        halfmove_clock: u8,
        fullmove_clock: u32,
        castlerights: [bool; 4],
    ) -> Self {
        GameState {
            bbs,
            white_to_move,
            ep_square,
            halfmove_clock,
            fullmove_clock,
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

    /// Remove the piece at the given square, if one exists. Returns piece which was removed if
    /// there was a piece on the given square, None o.w.
    pub fn remove_piece(&mut self, sq: Square) -> Option<Piece> {
        match self.occupancy.get(sq) {
            None => None,
            Some(piece) => {
                let res = self.occupancy.remove(sq);
                self.bbs[piece as usize] &= !masks::SQUARES[sq as usize];
                res
            }
        }
    }

    /// Promote the piece at the given square to new_piece.
    pub fn promote_piece(&mut self, sq: Square, new_piece: Piece) {
        self.remove_piece(sq);
        self.add_piece(new_piece, sq);
    }

    /// Apply the given move to this GameState, and return the GameState after the move is applied.
    // TODO: Could do this in-place by instead not keeping position in the StateStack.
    //
    // left TODO: Castling, correct e.p. square, double check everything.
    pub fn make(&self, game_move: GameMove) -> Self {
        let mut new_state = self.clone();
        let mut reset_halfmove_clock = false;

        let move_type = game_move.move_type();
        let castle_color_flag = if self.white_to_move { 0 } else { 2 };

        // Handles castling.
        if (move_type == MoveType::QueenCastle) || (move_type == MoveType::KingCastle) {
            // Move the king.
            let (king_from_sq, king) = 
                if self.white_to_move { (4, Piece::WhiteKing) } else { (60, Piece::BlackKing) };
            let king_to_sq = match (move_type, self.white_to_move) {
                (MoveType::QueenCastle, true) => 2,
                (MoveType::KingCastle, true) => 6,
                (MoveType::QueenCastle, false) => 58,
                (MoveType::KingCastle, false) => 62,
                _ => 100,
            };
            new_state.remove_piece(king_from_sq);
            new_state.add_piece(king, king_to_sq);

            // Move the rook.
            let rook = if self.white_to_move { Piece::WhiteRook } else { Piece::BlackRook };
            let rook_from_sq = match (move_type, self.white_to_move) {
                (MoveType::QueenCastle, true) => 0,
                (MoveType::KingCastle, true) => 7,
                (MoveType::QueenCastle, false) => 56,
                (MoveType::KingCastle, false) => 63,
                _ => 100,
            };
            let rook_to_sq = match (move_type, self.white_to_move) {
                (MoveType::QueenCastle, true) => 3,
                (MoveType::KingCastle, true) => 5,
                (MoveType::QueenCastle, false) => 59,
                (MoveType::KingCastle, false) => 61,
                _ => 100,
            };
            new_state.remove_piece(rook_from_sq);
            new_state.add_piece(rook, rook_to_sq);

            // Clear castle rights.
            new_state.castlerights[castle_color_flag+0] = false;
            new_state.castlerights[castle_color_flag+1] = false;

            // Update ep square, side to move, clocks.
            new_state.white_to_move = !self.white_to_move;
            if !self.white_to_move { new_state.fullmove_clock += 1 }
            new_state.halfmove_clock = self.halfmove_clock + 1;

            return new_state;
        }

        let fromsquare = game_move.fromsquare();
        let tosquare = game_move.tosquare();
        let moving = self.occupying_piece(fromsquare)
            .expect("Illegal move; no piece on fromsquare");

        // If capture, find the capturing square (either tosquare or e.p. square), and remove the
        // existing piece there.
        if game_move.is_capture() {
            let mut cap_sq = tosquare;
            if move_type == MoveType::EpCapture {
                cap_sq = match self.white_to_move {
                    true => self.ep_square.unwrap() + 8,
                    false => self.ep_square.unwrap() - 8,
                }
            }
            new_state.remove_piece(cap_sq);
            reset_halfmove_clock = true;
        }

        // Move the actual piece.
        new_state.remove_piece(fromsquare);
        new_state.add_piece(moving, tosquare);

        // Reset halfmove clock if pawn was moved.
        reset_halfmove_clock =
            reset_halfmove_clock || (moving == Piece::WhitePawn || moving == Piece::BlackPawn);

        // Promote the moved piece, if necessary.
        if game_move.is_promo() {
            let promo_piece = game_move.promo_piece(self.white_to_move)
                .expect("Invalid move.");
            new_state.promote_piece(tosquare, promo_piece);
        }

        // Update ep square, side to move, clocks.
        new_state.ep_square = if move_type != MoveType::DoublePawnPush { None } else {
            if self.white_to_move { Some(tosquare - 8) } else { Some(tosquare + 8) }
        };
        new_state.white_to_move = !self.white_to_move;
        if !self.white_to_move { new_state.fullmove_clock += 1 }
        new_state.halfmove_clock = if reset_halfmove_clock {0} else { self.halfmove_clock + 1 };

        // Update castle rights if necessary.
        let has_castlerights = 
            self.castlerights[castle_color_flag+1] || self.castlerights[castle_color_flag+1];
        if has_castlerights && (moving == Piece::WhiteRook || moving == Piece::BlackRook) {
            match (moving, fromsquare) {
                (Piece::WhiteRook,  0) => new_state.castlerights[0] = false,
                (Piece::WhiteRook,  7) => new_state.castlerights[1] = false,
                (Piece::BlackRook, 63) => new_state.castlerights[3] = false,
                (Piece::BlackRook, 56) => new_state.castlerights[3] = false,
                _ => ()
            }
        }

        new_state
    }
}


/// Data structure to map from square number -> occupying piece.
#[derive(Copy, Clone)]
struct PieceBitBoards {
    map: [Piece; 64],
}

/// Map from squre to occupying piece.
impl PieceBitBoards {
    pub fn new() -> Self {
        PieceBitBoards {
            map: [Piece::Null; 64],
        }
    }

    /// Get Some(<piece_at_sq>) or None if there is no such piece.
    pub fn get(&self, sq: Square) -> Option<Piece> {
        match self.map[sq as usize] {
            Piece::Null => None,
            pi => Some(pi),
        }
    }

    /// Remove the piece from the map at the given square and return the piece. If there is no such
    /// piece, returns None.
    pub fn remove(&mut self, sq: Square) -> Option<Piece> {
        let removed = self.get(sq);
        self.map[sq as usize] = Piece::Null;
        removed
    }

    /// Put the piece at the given square in the map.
    pub fn put(&mut self, sq: Square, pi: Piece) {
        self.map[sq as usize] = pi;
    }
}


/// We store the state stack in the stack for fast access. Thus we need a max size.
pub const MAX_MOVESTACK_DEPTH: usize = 100;

/// Represents a stack of game states that have occured from the initial position.
struct StateStack {
    backing: [Option<GameState>; MAX_MOVESTACK_DEPTH],
    size: usize,
}

impl StateStack {
    /// Create a new empty StateStack.
    pub fn new() -> Self {
        StateStack {
            backing: [None; MAX_MOVESTACK_DEPTH],
            size: 0,
        }
    }

    /// Add a GameState to the top of the StateStack.
    pub fn push(&mut self, elt: GameState) {
        self.backing[self.size] = Some(elt);
        self.size += 1;
    }

    /// Get an immutable reference to the top element on the StateStack, or None if the stack is
    /// empty.
    pub fn peek(&self) -> Option<&GameState> {
        if self.size <= 0 {
            return None;
        }
        self.backing[self.size - 1].as_ref()
    }

    /// Remove and return the top GameState on the StateStack, or None if the stack is empty
    pub fn pop(&mut self) -> Option<GameState> {
        if self.size <= 0 {
            return None;
        }
        let mut res = None;
        std::mem::swap(&mut self.backing[self.size], &mut res);
        self.size -= 1;
        res
    }
}
