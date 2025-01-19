/// Represents a position on a chessboard with a specific row and column.
///
/// # Fields
///
/// * `row` -
/// * `col` -
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
    /// The row index of the position.
    row: usize,
    /// The column index of the position.
    col: usize,
}

impl Position {
    /// Creates a new `Position` instance with the given row and column.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index of the position.
    /// * `col` - The column index of the position.
    ///
    /// # Returns
    ///
    /// A new `Position` instance.
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    /// Gets the row index of the position.
    ///
    /// # Returns
    ///
    /// The row index of the position.
    pub fn get_row(&self) -> usize {
        self.row
    }

    /// Gets the column index of the position.
    ///
    /// # Returns
    ///
    /// The column index of the position.
    pub fn get_col(&self) -> usize {
        self.col
    }

    /// Gets the position as a tuple of row and column indices.
    ///
    /// # Returns
    ///
    /// A tuple containing the row and column indices of the position.
    pub fn get_position(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}
