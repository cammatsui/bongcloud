use bongcloud::game_state::bb_utils::debug_print;
use bongcloud::bits::masks::KNIGHT_MOVES;

pub fn main() {
    let bb = KNIGHT_MOVES[29];
    debug_print(bb);
}
