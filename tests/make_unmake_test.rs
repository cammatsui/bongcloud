///! Tests the make/unmake move API offered by the GameState, Game, and GameMove structs.
///! Put test cases in the `test_cases` function.
use bongcloud::game_state:: Game;
use bongcloud::game_move::{ GameMove, MoveType };
use bongcloud::fen::{ parse_fen, to_fen };


/// This function generates test cases for this integration test. Add new test cases in the Vec
/// returned here.
fn test_cases() -> Vec<TestCase> {
    vec![
        // White Kingside Castle
        TestCase { 
            game_move:  GameMove::new(4, 6, MoveType::KingCastle),
            fen:        String::from(
                "rnbqkbnr/ppp1pppp/8/3p4/8/4PN2/PPPP2PP/RNBQK2R w KQkq - 3 10"
            ),
            expect_fen: String::from(
                "rnbqkbnr/ppp1pppp/8/3p4/8/4PN2/PPPP2PP/RNBQ1RK1 b kq - 3 11"
            ),
        },
        // White Queenside Castle
        TestCase { 
            game_move:  GameMove::new(4, 2, MoveType::QueenCastle),
            fen:        String::from(
                "r3k2r/pppqpppp/2n1b1bn/8/2B1PB2/2NP1P1N/PPP1Q1PP/R3K2R w KQkq - 14 8"
            ),
            expect_fen: String::from(
                "r3k2r/pppqpppp/2n1b1bn/8/2B1PB2/2NP1P1N/PPP1Q1PP/2KR3R b kq - 14 9"
            ),
        }, 
        // Black Kingside Castle.
        TestCase {
            game_move:  GameMove::new(60, 62, MoveType::KingCastle),
            fen:        String::from(
                "r3k2r/pppqpppp/2n1b1bn/8/2B1PB2/2NP1P1N/PPP1Q1PP/R3K2R b KQkq - 14 8"
            ),
            expect_fen: String::from(
                "r4rk1/pppqpppp/2n1b1bn/8/2B1PB2/2NP1P1N/PPP1Q1PP/R3K2R w KQ - 15 9"
            ),
        },
        // Black Queenside Castle.
        TestCase { 
            game_move:  GameMove::new(60, 58, MoveType::QueenCastle),
            fen:        String::from(
                "r3k2r/pppqpppp/2n1b1bn/8/2B1PB2/2NP1P1N/PPP1Q1PP/R3K2R b KQkq - 14 8"
            ),
            expect_fen: String::from(
                "2kr3r/pppqpppp/2n1b1bn/8/2B1PB2/2NP1P1N/PPP1Q1PP/R3K2R w KQ - 15 9"
            ),
        },
        // White Quiet Move.
        TestCase {
            game_move:  GameMove::new(3, 39, MoveType::Quiet),
            fen:        String::from(
                "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 30 5",
            ),
            expect_fen: String::from(
                "rnbqkbnr/pppp1ppp/8/4p2Q/4P3/8/PPPP1PPP/RNB1KBNR b KQkq - 30 6",
            )
        },
        // Black Quiet Move.
        TestCase {
            game_move:  GameMove::new(59, 31, MoveType::Quiet),
            fen:        String::from(
                "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 30 5",
            ),
            expect_fen: String::from(
                "rnb1kbnr/pppp1ppp/8/4p3/4P2q/8/PPPP1PPP/RNBQKBNR w KQkq - 31 6",
            )
        },
        // White Double Pawn Push.
        TestCase {
            game_move:  GameMove::new(12, 28, MoveType::DoublePawnPush),
            fen:        String::from(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 10",
            ),
            expect_fen: String::from(
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 0",
            ),
        },
        // Black Double Pawn Push.
        TestCase {
            game_move:  GameMove::new(52, 36, MoveType::DoublePawnPush),
            fen:        String::from(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 11",
            ),
            expect_fen: String::from(
                "rnbqkbnr/pppp1ppp/8/4p3/8/8/PPPPPPPP/RNBQKBNR w KQkq e6 1 0",
                )
        },
        // White Capture.
        TestCase {
            game_move:  GameMove::new(21, 36, MoveType::Capture),
            fen:        String::from(
                "rnbqkbnr/pppp1ppp/8/4p3/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 1 10"
            ),
            expect_fen: String::from(
                "rnbqkbnr/pppp1ppp/8/4N3/8/8/PPPPPPPP/RNBQKB1R b KQkq - 1 0"
            ),
        },
        // Black Capture.
        TestCase {
            game_move:  GameMove::new(34, 36, MoveType::Capture),
            fen:        String::from(
                "rnb1kbnr/pppp1ppp/8/2q1N3/8/8/PPPPPPPP/RNBQKB1R b KQkq - 1 11"
            ),
            expect_fen: String::from(
                "rnb1kbnr/pppp1ppp/8/4q3/8/8/PPPPPPPP/RNBQKB1R w KQkq - 2 0"
            ),
        },
        // White EP Capture.
        TestCase {
            game_move:  GameMove::new(36, 43, MoveType::EpCapture),
            fen:        String::from(
                "rnbqkbnr/ppp2ppp/8/3pP3/8/8/PPPP1PPP/RNBQKB1R w KQkq d6 2 10",
            ),
            expect_fen: String::from(
                "rnbqkbnr/ppp2ppp/3P4/8/8/8/PPPP1PPP/RNBQKB1R b KQkq - 2 0",
            )
        },
        // Black EP Capture.
        TestCase {
            game_move:  GameMove::new(29, 20, MoveType::EpCapture),
            fen:        String::from(
                "rnbqkbnr/ppp3pp/8/8/4Pp2/8/PPPP1PPP/RNBQKB1R b KQkq e3 2 10",
            ),
            expect_fen: String::from(
                "rnbqkbnr/ppp3pp/8/8/8/4p3/PPPP1PPP/RNBQKB1R w KQkq - 3 0",
            )
        },
        // White Knight Promo.
        TestCase {
            game_move:  GameMove::new(53, 61, MoveType::KnightPromo),
            fen:        String::from(
                "rnbq4/ppp2P2/k7/8/8/K7/PPPP1p2/RNBQ4 w KQkq - 3 0",
            ),
            expect_fen: String::from(
                "rnbq1N2/ppp5/k7/8/8/K7/PPPP1p2/RNBQ4 b KQkq - 3 0",
            ),
        },
        // Black Knight Promo.
        TestCase {
            game_move:  GameMove::new(13, 5, MoveType::KnightPromo),
            fen:        String::from(
                "rnbq4/ppp2P2/k7/8/8/K7/PPPP1p2/RNBQ4 b KQkq - 3 0"
            ),
            expect_fen: String::from(
                "rnbq4/ppp2P2/k7/8/8/K7/PPPP4/RNBQ1n2 w KQkq - 4 0"
            ),
        },
        // White Bishop Promo.
        TestCase {
            game_move:  GameMove::new(53, 61, MoveType::BishopPromo),
            fen:        String::from(
                "rnbq4/ppp2P2/k7/8/8/K7/PPPP1p2/RNBQ4 w KQkq - 3 0",
            ),
            expect_fen: String::from(
                "rnbq1B2/ppp5/k7/8/8/K7/PPPP1p2/RNBQ4 b KQkq - 3 0",
            ),
        },
        // Black Bishop Promo.
        TestCase {
            game_move:  GameMove::new(13, 5, MoveType::BishopPromo),
            fen:        String::from(
                "rnbq4/ppp2P2/k7/8/8/K7/PPPP1p2/RNBQ4 b KQkq - 3 0"
            ),
            expect_fen: String::from(
                "rnbq4/ppp2P2/k7/8/8/K7/PPPP4/RNBQ1b2 w KQkq - 4 0"
            ),
        },
        // White Rook Promo.
        TestCase {
            game_move:  GameMove::new(53, 61, MoveType::RookPromo),
            fen:        String::from(
                "rnbq4/ppp2P2/k7/8/8/K7/PPPP1p2/RNBQ4 w KQkq - 3 0",
            ),
            expect_fen: String::from(
                "rnbq1R2/ppp5/k7/8/8/K7/PPPP1p2/RNBQ4 b KQkq - 3 0",
            ),
        },
        // Black Rook Promo.
        TestCase {
            game_move:  GameMove::new(13, 5, MoveType::RookPromo),
            fen:        String::from(
                "rnbq4/ppp2P2/k7/8/8/K7/PPPP1p2/RNBQ4 b KQkq - 3 0"
            ),
            expect_fen: String::from(
                "rnbq4/ppp2P2/k7/8/8/K7/PPPP4/RNBQ1r2 w KQkq - 4 0"
            ),
        },
        // White Queen Promo.
        TestCase {
            game_move:  GameMove::new(53, 61, MoveType::QueenPromo),
            fen:        String::from(
                "rnbq4/ppp2P2/k7/8/8/K7/PPPP1p2/RNBQ4 w KQkq - 3 0",
            ),
            expect_fen: String::from(
                "rnbq1Q2/ppp5/k7/8/8/K7/PPPP1p2/RNBQ4 b KQkq - 3 0",
            ),
        },
        // Black Queen Promo.
        TestCase {
            game_move:  GameMove::new(13, 5, MoveType::QueenPromo),
            fen:        String::from(
                "rnbq4/ppp2P2/k7/8/8/K7/PPPP1p2/RNBQ4 b KQkq - 3 0"
            ),
            expect_fen: String::from(
                "rnbq4/ppp2P2/k7/8/8/K7/PPPP4/RNBQ1q2 w KQkq - 4 0"
            ),
        },
        // White Knight Promo-Capture.
        TestCase {
            game_move:  GameMove::new(53, 60, MoveType::KnightPromoCapture),
            fen:        String::from(
                "rnb1q3/ppp2P2/k7/8/8/K7/PPPP1p2/RNB1Q3 w KQkq - 3 0",
            ),
            expect_fen: String::from(
                "rnb1N3/ppp5/k7/8/8/K7/PPPP1p2/RNB1Q3 b KQkq - 3 0",
            ),
        },
        // Black Knight Promo-Capture.
        TestCase {
            game_move:  GameMove::new(13, 4, MoveType::KnightPromoCapture),
            fen:        String::from(
                "rnb1q3/ppp2P2/k7/8/8/K7/PPPP1p2/RNB1Q3 b KQkq - 3 0",
            ),
            expect_fen: String::from(
                "rnb1q3/ppp2P2/k7/8/8/K7/PPPP4/RNB1n3 w KQkq - 4 0",
            ),
        },
        // White Bishop Promo-Capture.
        TestCase {
            game_move:  GameMove::new(53, 60, MoveType::BishopPromoCapture),
            fen:        String::from(
                "rnb1q3/ppp2P2/k7/8/8/K7/PPPP1p2/RNB1Q3 w KQkq - 3 0",
            ),
            expect_fen: String::from(
                "rnb1B3/ppp5/k7/8/8/K7/PPPP1p2/RNB1Q3 b KQkq - 3 0",
            ),
        },
        // Black Bishop Promo-Capture.
        TestCase {
            game_move:  GameMove::new(13, 4, MoveType::BishopPromoCapture),
            fen:        String::from(
                "rnb1q3/ppp2P2/k7/8/8/K7/PPPP1p2/RNB1Q3 b KQkq - 3 0",
            ),
            expect_fen: String::from(
                "rnb1q3/ppp2P2/k7/8/8/K7/PPPP4/RNB1b3 w KQkq - 4 0",
            ),
        },
        // White Rook Promo-Capture.
        TestCase {
            game_move:  GameMove::new(53, 60, MoveType::RookPromoCapture),
            fen:        String::from(
                "rnb1q3/ppp2P2/k7/8/8/K7/PPPP1p2/RNB1Q3 w KQkq - 3 0",
            ),
            expect_fen: String::from(
                "rnb1R3/ppp5/k7/8/8/K7/PPPP1p2/RNB1Q3 b KQkq - 3 0",
            ),
        },
        // Black Rook Promo-Capture.
        TestCase {
            game_move:  GameMove::new(13, 4, MoveType::RookPromoCapture),
            fen:        String::from(
                "rnb1q3/ppp2P2/k7/8/8/K7/PPPP1p2/RNB1Q3 b KQkq - 3 0",
            ),
            expect_fen: String::from(
                "rnb1q3/ppp2P2/k7/8/8/K7/PPPP4/RNB1r3 w KQkq - 4 0",
            ),
        },
        // White Queen Promo-Capture.
        TestCase {
            game_move:  GameMove::new(53, 60, MoveType::QueenPromoCapture),
            fen:        String::from(
                "rnb1q3/ppp2P2/k7/8/8/K7/PPPP1p2/RNB1Q3 w KQkq - 3 0",
            ),
            expect_fen: String::from(
                "rnb1Q3/ppp5/k7/8/8/K7/PPPP1p2/RNB1Q3 b KQkq - 3 0",
            ),
        },
        // Black Queen Promo-Capture.
        TestCase {
            game_move:  GameMove::new(13, 4, MoveType::QueenPromoCapture),
            fen:        String::from(
                "rnb1q3/ppp2P2/k7/8/8/K7/PPPP1p2/RNB1Q3 b KQkq - 3 0",
            ),
            expect_fen: String::from(
                "rnb1q3/ppp2P2/k7/8/8/K7/PPPP4/RNB1q3 w KQkq - 4 0",
            ),
        },
    ]
}


/// This struct represents a test case for this integration test.
struct TestCase {
    fen: String,
    game_move: GameMove,
    expect_fen: String,
}


/// Run the test cases from `test_cases()`
#[test]
pub fn run_test_cases() {
    for test_case in test_cases() {
        let mut game = Game::new(parse_fen(&test_case.fen));
        assert_eq!(&to_fen(&game.current_state()), &test_case.fen);

        game.make(test_case.game_move);
        assert_eq!(&to_fen(&game.current_state()), &test_case.expect_fen);
        game.unmake();
        assert_eq!(&to_fen(&game.current_state()), &test_case.fen);
    }
}
