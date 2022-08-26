use crate::game_state::{ GameState, Square, Piece };
use crate::bits::utils;


/// Make a GameState from the given FEN string.
pub fn parse_fen(fen: String) -> GameState {
    let fields: Vec<&str> = fen.split(" ").collect();
    if fields.len() != 6 { panic!("Invalid FEN string.") }
    let pos_str = fields[0];

    let to_move_str = fields[1];
    let castle_str = fields[2];
    let ep_str = fields[3];
    let halfmove: u8 = fields[4].trim().parse().expect("Halfmove is not a string.");
    let fullmove_str: u8 = fields[5].trim().parse().expect("Fullmove is not a string.");

    let mut game_state = GameState::new(
        [0;12],
        white_to_move_from(to_move_str).expect("Could not parse player to move."),
        ep_square_from(ep_str),
        halfmove,
        castlerights_from(castle_str),
    );

    add_pieces(pos_str, &mut game_state);
    game_state
}

/// Get the white_to_move bool from the FEN side to move string.
fn white_to_move_from(to_move_str: &str) -> Result<bool, String> {
    if to_move_str == "w" {
        Ok(true)
    } else if to_move_str == "b" {
        Ok(false)
    } else {
        Err("Invalid to move field.".to_string())
    }
}

/// Get the castlerights bool array from the FEN castle string.
fn castlerights_from(castle_str: &str) -> [bool;4] {
    [
        castle_str.contains("K"),
        castle_str.contains("Q"),
        castle_str.contains("k"),
        castle_str.contains("q"),
    ]
}

/// Add the pieces to the board from the position string.
fn add_pieces(pos_str: &str, game_state: &mut GameState) {
    let ranks: Vec<&str> = pos_str.split("/").collect();
    if ranks.len() != 8 { panic!("Invalid number of ranks.") }

    for i in 0..8 {
        let mut j = 0;
        let rank = ranks[i];
        while j < rank.len() {
            let rank_char = format!("{}", ranks[i].chars().nth(j).unwrap());
            match rank_char.parse::<usize>() {
                Ok(num) => j += num,
                _ => {
                    let piece = piece_from_char(rank_char.chars().nth(0).unwrap()).unwrap();
                    game_state.add_piece(piece, utils::square_idx(i as u8, j as u8));
                    println!("Placing {:?} on {}", piece, utils::square_idx(i as u8, j as u8));
                    j += 1;
                },
            }
        }
    }

}

/// Get the ep square from the ep string.
fn ep_square_from(ep_str: &str) -> Option<Square> {
    if ep_str == "-" { return None }
    let file_idx = file_idx(ep_str.chars().nth(0).unwrap()).unwrap();
    let rank_idx = rank_idx(ep_str.chars().nth(1).unwrap()).unwrap();
    Some(utils::square_idx(rank_idx, file_idx))
}

/// Map character to piece type for position string parsing.
fn piece_from_char(piece_char: char) -> Option<Piece> {
    match piece_char {
        'p' => Some(Piece::WhitePawn),
        'b' => Some(Piece::WhiteBishop),
        'n' => Some(Piece::WhiteKnight),
        'r' => Some(Piece::WhiteRook),
        'q' => Some(Piece::WhiteQueen),
        'k' => Some(Piece::WhiteKing),
        'P' => Some(Piece::BlackPawn),
        'B' => Some(Piece::BlackBishop),
        'N' => Some(Piece::BlackKnight),
        'R' => Some(Piece::BlackRook),
        'Q' => Some(Piece::BlackQueen),
        'K' => Some(Piece::BlackKing),
        _ => None,
    }
}

/// Map character to file index to get ep square.
fn file_idx(file_char: char) -> Option<u8> {
    match file_char {
        '1' => Some(0),
        '2' => Some(1),
        '3' => Some(2),
        '4' => Some(3),
        '5' => Some(4),
        '6' => Some(5),
        '7' => Some(6),
        '8' => Some(7),
        _ => None,
    }
}

/// Map character to rank index to get ep square.
fn rank_idx(file_char: char) -> Option<u8> {
    match file_char {
        'a' => Some(0),
        'b' => Some(1),
        'c' => Some(2),
        'd' => Some(3),
        'e' => Some(4),
        'f' => Some(5),
        'g' => Some(6),
        'h' => Some(7),
        _ => None,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    #[test]
    /// Test that the starting FEN is parsed correctly.
    fn test_parse_starting_position() {
        let game_state = parse_fen(STARTING_FEN.to_string());
        let white_pawns: u64 = game_state.bbs[0];
        println!("Game state: {white_pawns:#b}");
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
    }
}
