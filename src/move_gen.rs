///! This file contains functions for generating legal moves (stored in a MoveList) from a given 
///! GameState.
use crate::game_state::{ GameState, BitBoard, Square };
use crate::bits::biterator::{ biterator, bitboarderator };
use crate::bits::masks;
use crate::game_move::{ GameMove, MoveType };
use crate::move_list::MoveList;


pub fn gen_legal_moves(state: GameState) -> MoveList {
    MoveList::new()   
}

mod move_gen_utils {
    use super::*;

    /// Append knight moves to the given MoveList.
    pub fn append_knight_moves(
        move_list: &mut MoveList,
        movable_knights: BitBoard,
        stm_occupancy: BitBoard,
        opp_occupancy: BitBoard,
    ) {
        for knight_sq in biterator(movable_knights) {
            let all_moves_bb = masks::KNIGHT_MOVES[knight_sq as usize];

            let moves_bb = all_moves_bb & !(stm_occupancy | opp_occupancy);
            for move_sq in biterator(moves_bb) {
                move_list.push(GameMove::new(knight_sq as u8, move_sq as u8, MoveType::Quiet));
            }

            let captures_bb = all_moves_bb & opp_occupancy;
            for cap_sq in biterator(captures_bb) {
                move_list.push(GameMove::new(knight_sq as u8, cap_sq as u8, MoveType::Capture));
            }
        }
    }

    /// Append single pawn pushes to the given MoveList.
    pub fn append_pawn_single_pushes(
        move_list: &mut MoveList,
        pushable_pawns: BitBoard,
        total_occupancy: BitBoard,
        white_to_move: bool,
    ) {
        let push_mask = if white_to_move { pushable_pawns << 8 } else { pushable_pawns >> 8 };
        let pushes = push_mask & !total_occupancy;
        for move_sq in biterator(pushes) {
            let push_move = if white_to_move {
                GameMove::new(move_sq-8, move_sq, MoveType::Quiet)
            } else {
                GameMove::new(move_sq+8, move_sq, MoveType::Quiet)
            };
            move_list.push(push_move);
        }
    }

    /// Append double pawn pushes to the given MoveList.
    pub fn append_pawn_double_pushes(
        move_list: &mut MoveList,
        pushable_pawns: BitBoard,
        total_occupancy: BitBoard,
        white_to_move: bool,
    ) {
        let started_pawns = if white_to_move { pushable_pawns & masks::RANK_2 } 
            else { pushable_pawns & masks::RANK_7 };
        let not_blocked = if white_to_move {
            started_pawns & ! (total_occupancy << 8 | total_occupancy << 16)
        } else {
            started_pawns & ! (total_occupancy >> 8 | total_occupancy >> 16)
        };
        let push_mask = if white_to_move { not_blocked << 16 } else { not_blocked >> 16 };
        for move_sq in biterator(push_mask) {
            let push_move = if white_to_move {
                GameMove::new(move_sq-16, move_sq, MoveType::DoublePawnPush)
            } else {
                GameMove::new(move_sq+16, move_sq, MoveType::DoublePawnPush)
            };
            move_list.push(push_move)
        }
    }
}
