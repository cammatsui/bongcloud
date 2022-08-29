use bongcloud::game_state:: Game;
use bongcloud::game_move::{ GameMove, MoveType };
use bongcloud::fen::{ parse_fen, to_fen };


/// This function generates test cases for this integration test. Add new test cases in the Vec
/// returned here.
fn test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            fen:        String::from("rnbqkbnr/ppp1pppp/8/3p4/8/4PN2/PPPP2PP/RNBQK2R w KQkq - 3 10"),
            game_move:  GameMove::new(4, 6, MoveType::KingCastle),
            expect_fen: String::from("rnbqkbnr/ppp1pppp/8/3p4/8/4PN2/PPPP2PP/RNBQ1RK1 b kq - 3 11"),
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
