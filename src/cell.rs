use crate::{color::Color, kind::Kind, piece::Piece, position::Position};

#[derive(Clone, Copy)]
pub struct Cell {
    pub color: Color,
    pub piece: Piece,
    pub is_selected: bool,
    pub is_check: bool,
    pub position: Position,
}

impl Cell {
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
    pub fn get_coord(&self) -> Position {
        Position {
            row: self.position.row,
            col: self.position.col,
        }
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
}
