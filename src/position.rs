/// Represents a position on a chessboard with a row and column.
///
/// # Fields
///
/// * `row` - The row index of the position (0-based).
/// * `col` - The column index of the position (0-based).
#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}
