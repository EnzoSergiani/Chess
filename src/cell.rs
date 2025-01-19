use crate::{color::Color, kind::Kind, piece::Piece, position::Position};

/// Represents a cell on the chess board.
#[derive(Clone, Copy)]
pub struct Cell {
    /// The color of the cell.
    color: Color,
    /// The piece currently on the cell.
    piece: Piece,
    /// Indicates whether the cell is selected.
    is_selected: bool,
    /// Indicates whether the cell is in check.
    is_check: bool,
    /// The position of the cell on the board.
    position: Position,
}

impl Cell {
    /// Creates an empty cell with the given color and position.
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the cell.
    /// * `position` - The position of the cell on the board.
    ///
    /// # Returns
    ///
    /// A new `Cell` instance with no piece.
    pub fn empty(color: Color, position: Position) -> Self {
        Cell {
            piece: Piece::none(),
            color,
            is_selected: false,
            is_check: false,
            position,
        }
    }

    /// Creates a cell with the given piece, color, and position.
    ///
    /// # Arguments
    ///
    /// * `piece` - The piece to place in the cell.
    /// * `color` - The color of the cell.
    /// * `position` - The position of the cell on the board.
    ///
    /// # Returns
    ///
    /// A new `Cell` instance with the specified piece.
    pub fn create(piece: Piece, color: Color, position: Position) -> Self {
        Cell {
            piece,
            color,
            is_selected: false,
            is_check: false,
            position,
        }
    }

    /// Sets the piece on the cell.
    ///
    /// # Arguments
    ///
    /// * `piece` - The piece to place on the cell.
    pub fn set_piece(&mut self, piece: Piece) -> () {
        self.piece = piece;
    }

    /// Returns an `Option` containing the `Piece` on the cell if there is one.
    ///
    /// # Returns
    ///
    /// - `Some(Piece)` if the cell contains a piece.
    /// - `None` if the cell is empty.
    pub fn get_piece(&self) -> Option<Piece> {
        if self.piece.get_kind() != Kind::None {
            Some(self.piece.clone())
        } else {
            None
        }
    }
    /// Returns the `Position` of the cell.
    ///
    /// # Returns
    ///
    /// A `Position` struct containing the row and column of the cell.
    pub fn get_position(&self) -> (usize, usize) {
        self.position.get_position()
    }

    /// Sets the check status of the cell.
    ///
    /// # Arguments
    ///
    /// * `is_check` - A boolean indicating whether the cell is in check.
    pub fn set_is_check(&mut self, is_check: bool) -> () {
        self.is_check = is_check;
    }

    /// Gets the check status of the cell.
    ///
    /// # Returns
    ///
    /// `true` if the cell is in check, `false` otherwise.
    pub fn get_is_check(&self) -> bool {
        self.is_check
    }

    /// Sets the selection status of the cell.
    ///
    /// # Arguments
    ///
    /// * `is_selected` - A boolean indicating whether the cell is selected.
    pub fn set_is_selected(&mut self, is_selected: bool) -> () {
        self.is_selected = is_selected;
    }

    /// Gets the selection status of the cell.
    ///
    /// # Returns
    ///
    /// `true` if the cell is selected, `false` otherwise.
    pub fn get_is_selected(&self) -> bool {
        self.is_selected
    }

    /// Gets the color of the cell.
    ///
    /// # Returns
    ///
    /// The color of the cell.
    pub fn get_color(&self) -> Color {
        self.color
    }

    /// Returns the color of the piece on the cell.
    ///
    /// # Panics
    ///
    /// This function will panic if the cell is empty.
    ///
    /// # Returns
    ///
    /// A `Color` enum representing the color of the piece on the cell.
    pub fn get_piece_color(&self) -> Color {
        if self.get_piece().is_some() {
            self.get_piece().unwrap().get_color()
        } else {
            panic!("Cell is empty")
        }
    }

    /// Returns the kind of the piece on the cell.
    ///
    /// # Panics
    ///
    /// This function will panic if the cell is empty.
    ///
    /// # Returns
    ///
    /// A `Kind` enum representing the kind of the piece on the cell.
    pub fn get_piece_kind(&self) -> Kind {
        if self.get_piece().is_some() {
            self.get_piece().unwrap().get_kind()
        } else {
            panic!("Cell is empty")
        }
    }
}
