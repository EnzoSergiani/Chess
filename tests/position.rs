use chess::position::Position;

#[test]
fn test_position_getters() {
    let position: Position = Position::new(3, 4);
    assert_eq!(position.get_row(), 3);
    assert_eq!(position.get_col(), 4);
}
