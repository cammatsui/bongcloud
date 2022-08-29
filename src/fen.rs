use crate::game_state::{ GameState, Square, Piece };
use crate::bits::utils;


/// Used to conver between FEN and GameState reprs.
const FEN_PIECES: [(char, Piece);12] = [
    ('P', Piece::WhitePawn),
    ('B', Piece::WhiteBishop),
    ('N', Piece::WhiteKnight),
    ('R', Piece::WhiteRook),
    ('Q', Piece::WhiteQueen),
    ('K', Piece::WhiteKing),
    ('p', Piece::BlackPawn),
    ('b', Piece::BlackBishop),
    ('n', Piece::BlackKnight),
    ('r', Piece::BlackRook),
    ('q', Piece::BlackQueen),
    ('k', Piece::BlackKing),
];
const FEN_RANKS: [(char, u8);8] = [
    ('a', 0),
    ('b', 1),
    ('c', 2),
    ('d', 3),
    ('e', 4),
    ('f', 5),
    ('g', 6),
    ('h', 7),
];
const FEN_FILES: [(char, u8);8] = [
    ('1', 0),
    ('2', 1),
    ('3', 2),
    ('4', 3),
    ('5', 4),
    ('6', 5),
    ('7', 6),
    ('8', 7),
];


/// Make a GameState from the given FEN string.
pub fn parse_fen(fen: &String) -> GameState {
    let fields: Vec<&str> = fen.split(" ").collect();
    if fields.len() != 6 { panic!("Invalid FEN string.") }
    let pos_str = fields[0];

    let to_move_str = fields[1];
    let castle_str = fields[2];
    let ep_str = fields[3];
    let fullmove: u32 = fields[4].trim().parse().expect("Fullmove is not a string.");
    let halfmove: u8 = fields[5].trim().parse().expect("Halfmove is not a string.");

    let mut game_state = GameState::new(
        [0;12],
        parse_utils::white_to_move_from(to_move_str).expect("Could not parse player to move."),
        parse_utils::ep_square_from(ep_str),
        halfmove,
        fullmove,
        parse_utils::castlerights_from(castle_str),
    );

    parse_utils::add_pieces(pos_str, &mut game_state);
    game_state
}


/// Make a FEN string from the given GameState.
pub fn to_fen(game_state: &GameState) -> String {
    vec![
        serialize_utils::ser_bbs(game_state),
        serialize_utils::ser_side_to_move(game_state),
        serialize_utils::ser_castle_rights(game_state),
        serialize_utils::ser_ep_square(game_state),
        serialize_utils::ser_fullmove_clock(game_state),
        serialize_utils::ser_halfmove_clock(game_state),
    ].join(" ")
}


/// Utility functions for serializing GameState to a FEN string.
mod serialize_utils {
    use super::*;

    /// Create the FEN field for castle rights from the GameState.
    pub fn ser_castle_rights(game_state: &GameState) -> String {
        let mut result = String::new();
        if game_state.castlerights[0] { result.push_str("K") }
        if game_state.castlerights[1] { result.push_str("Q") }
        if game_state.castlerights[2] { result.push_str("k") }
        if game_state.castlerights[3] { result.push_str("q") }
        result
    }

    /// Create the FEN field for side-to-move from the GameState.
    pub fn ser_side_to_move(game_state: &GameState) -> String {
        match game_state.white_to_move {
            true => String::from("w"),
            false => String::from("b"),
        }
    }

    /// Create the FEN field for the en-passant square from the GameState.
    pub fn ser_ep_square(game_state: &GameState) -> String {
        if let None = game_state.ep_square { return String::from("-") }

        let ep_square = game_state.ep_square.unwrap();
        let file_idx: u8 = utils::file_idx(ep_square);
        let rank_idx: u8 = utils::file_idx(ep_square);
        format!("{}{}", char_from_file(file_idx).unwrap(), char_from_rank(rank_idx).unwrap())
    }

    /// Create the FEN field for the halfmove clock from the GameState.
    pub fn ser_halfmove_clock(game_state: &GameState) -> String {
        format!("{}", game_state.halfmove_clock)
    }

    /// Create the FEN field for the fullmove clock from the GameState.
    pub fn ser_fullmove_clock(game_state: &GameState) -> String {
        format!("{}", game_state.fullmove_clock)
    }

    /// Create the FEN field for the board position from the GameState('s bitboards).
    pub fn ser_bbs(game_state: &GameState) -> String {
        let mut result = String::new();
        for rank_idx in (0..8).rev() {
            let mut cur_empty_count = 0;
            for file_idx in 0..8 {
                let sq_idx = utils::square_idx(rank_idx, file_idx);
                match game_state.occupying_piece(sq_idx) {
                    Some(piece) => {
                        let piece_char = char_from_piece(piece).unwrap();
                        if cur_empty_count > 0 {
                            result.push_str(&format!("{}", cur_empty_count));
                            cur_empty_count = 0;
                        }
                        result.push(piece_char);
                    },
                    None => cur_empty_count += 1,
                }
                if file_idx == 7 && cur_empty_count > 0 {
                    result.push_str(&format!("{}", cur_empty_count));
                }
            }
            if rank_idx != 0 { result.push('/') }
        }
        result
    }


    fn char_from_piece(p: Piece) -> Option<char> {
        for i in 0..FEN_PIECES.len() {
            if FEN_PIECES[i].1 == p { return Some(FEN_PIECES[i].0) }
        }
        None
    }

    fn char_from_file(file: u8) -> Option<char> {
        for i in 0..FEN_FILES.len() {
            if FEN_FILES[i].1 == file { return Some(FEN_FILES[i].0) }
        }
        None
    }

    fn char_from_rank(rank: u8) -> Option<char> {
        for i in 0..FEN_FILES.len() {
            if FEN_RANKS[i].1 == rank { return Some(FEN_RANKS[i].0) }
        }
        None
    }

}


/// Utility functions for parsing a FEN string to a GameState.
mod parse_utils {
    use super::*;

    /// Get the white_to_move bool from the FEN side to move string.
    pub fn white_to_move_from(to_move_str: &str) -> Result<bool, String> {
        if to_move_str == "w" {
            Ok(true)
        } else if to_move_str == "b" {
            Ok(false)
        } else {
            Err("Invalid to move field.".to_string())
        }
    }

    /// Get the castlerights bool array from the FEN castle string.
    pub fn castlerights_from(castle_str: &str) -> [bool;4] {
        [
            castle_str.contains("K"),
            castle_str.contains("Q"),
            castle_str.contains("k"),
            castle_str.contains("q"),
        ]
    }

    /// Add the pieces to the board from the position string.
    pub fn add_pieces(pos_str: &str, game_state: &mut GameState) {
        let mut ranks: Vec<&str> = pos_str.split("/").collect();
        for i in 0..4 { ranks.swap(i, 7-i) }
        if ranks.len() != 8 { panic!("Invalid number of ranks.") }

        for i in 0..8 {
            let mut j = 0;
            let rank = ranks[i];
            let mut file_idx = 0;
            while j < rank.len() {
                let rank_char = format!("{}", ranks[i].chars().nth(j).unwrap());
                match rank_char.parse::<usize>() {
                    Ok(num) => {
                        file_idx += num;
                        j += 1;
                    }
                    _ => {
                        let piece = piece_from_char(rank_char.chars().nth(0).unwrap()).unwrap();
                        game_state.add_piece(piece, utils::square_idx(i as u8, file_idx as u8));
                        file_idx += 1;
                        j += 1;
                    },
                }
            }
        }

    }

    /// Get the ep square from the ep string.
    pub fn ep_square_from(ep_str: &str) -> Option<Square> {
        if ep_str == "-" { return None }
        let file_idx = file_from_char(ep_str.chars().nth(0).unwrap()).unwrap();
        let rank_idx = rank_from_char(ep_str.chars().nth(1).unwrap()).unwrap();
        Some(utils::square_idx(rank_idx, file_idx))
    }


    fn piece_from_char(c: char) -> Option<Piece> {
        for i in 0..FEN_PIECES.len() {
            if FEN_PIECES[i].0 == c { return Some(FEN_PIECES[i].1) }
        }
        None
    }

    fn file_from_char(c: char) -> Option<u8> {
        for i in 0..FEN_FILES.len() {
            if FEN_FILES[i].0 == c { return Some(FEN_FILES[i].1) }
        }
        None
    }

    fn rank_from_char(c: char) -> Option<u8> {
        for i in 0..FEN_FILES.len() {
            if FEN_RANKS[i].0 == c { return Some(FEN_RANKS[i].1) }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    #[test]
    /// Test that the starting FEN is parsed correctly.
    fn test_parse_starting_position() {
        let game_state = parse_fen(&STARTING_FEN.to_string());
        assert_eq!(game_state.occupying_piece(0  as u8).unwrap(), Piece::WhiteRook);
        assert_eq!(game_state.occupying_piece(1  as u8).unwrap(), Piece::WhiteKnight);
        assert_eq!(game_state.occupying_piece(2  as u8).unwrap(), Piece::WhiteBishop);
        assert_eq!(game_state.occupying_piece(3  as u8).unwrap(), Piece::WhiteQueen);
        assert_eq!(game_state.occupying_piece(4  as u8).unwrap(), Piece::WhiteKing);
        assert_eq!(game_state.occupying_piece(5  as u8).unwrap(), Piece::WhiteBishop);
        assert_eq!(game_state.occupying_piece(6  as u8).unwrap(), Piece::WhiteKnight);
        assert_eq!(game_state.occupying_piece(7  as u8).unwrap(), Piece::WhiteRook);

        assert_eq!(game_state.occupying_piece(8  as u8).unwrap(), Piece::WhitePawn);
        assert_eq!(game_state.occupying_piece(9  as u8).unwrap(), Piece::WhitePawn);
        assert_eq!(game_state.occupying_piece(10 as u8).unwrap(), Piece::WhitePawn);
        assert_eq!(game_state.occupying_piece(11 as u8).unwrap(), Piece::WhitePawn);
        assert_eq!(game_state.occupying_piece(12 as u8).unwrap(), Piece::WhitePawn);
        assert_eq!(game_state.occupying_piece(13 as u8).unwrap(), Piece::WhitePawn);
        assert_eq!(game_state.occupying_piece(14 as u8).unwrap(), Piece::WhitePawn);
        assert_eq!(game_state.occupying_piece(15 as u8).unwrap(), Piece::WhitePawn);

        for i in 16..48 {
            assert_eq!(game_state.occupying_piece(i as u8), None);
        }

        assert_eq!(game_state.occupying_piece(48 as u8).unwrap(), Piece::BlackPawn);
        assert_eq!(game_state.occupying_piece(49 as u8).unwrap(), Piece::BlackPawn);
        assert_eq!(game_state.occupying_piece(50 as u8).unwrap(), Piece::BlackPawn);
        assert_eq!(game_state.occupying_piece(51 as u8).unwrap(), Piece::BlackPawn);
        assert_eq!(game_state.occupying_piece(52 as u8).unwrap(), Piece::BlackPawn);
        assert_eq!(game_state.occupying_piece(53 as u8).unwrap(), Piece::BlackPawn);
        assert_eq!(game_state.occupying_piece(54 as u8).unwrap(), Piece::BlackPawn);
        assert_eq!(game_state.occupying_piece(55 as u8).unwrap(), Piece::BlackPawn);
        
        assert_eq!(game_state.occupying_piece(56 as u8).unwrap(), Piece::BlackRook);
        assert_eq!(game_state.occupying_piece(57 as u8).unwrap(), Piece::BlackKnight);
        assert_eq!(game_state.occupying_piece(58 as u8).unwrap(), Piece::BlackBishop);
        assert_eq!(game_state.occupying_piece(59 as u8).unwrap(), Piece::BlackQueen);
        assert_eq!(game_state.occupying_piece(60 as u8).unwrap(), Piece::BlackKing);
        assert_eq!(game_state.occupying_piece(61 as u8).unwrap(), Piece::BlackBishop);
        assert_eq!(game_state.occupying_piece(62 as u8).unwrap(), Piece::BlackKnight);
        assert_eq!(game_state.occupying_piece(63 as u8).unwrap(), Piece::BlackRook);

        let serialized = to_fen(&game_state);
        assert_eq!(STARTING_FEN, serialized);
    }
}
