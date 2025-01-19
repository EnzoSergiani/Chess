use chess::board::Board;

#[test]
fn test_initialize_board() {
    let board: Board = Board::new().initialize();
    let fen_init: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    let mut expected_board: Board = Board::new();
    expected_board.load_from_fen(fen_init);
    assert_eq!(board.get_board(), expected_board.get_board());
}
