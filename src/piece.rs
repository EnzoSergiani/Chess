use crate::{color::Color, kind::Kind};

#[derive(Copy, Clone)]
pub struct Piece {
    kind: Kind,
    color: Color,
}

/// Represents a chess piece with a specific kind and color.
impl Piece {
    /// Creates a new `Piece` with the given kind and color.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of the chess piece (e.g., Pawn, Knight, Bishop, etc.).
    /// * `color` - The color of the chess piece (White or Black).
    ///
    /// # Returns
    ///
    /// A new `Piece` instance with the specified kind and color.
    pub fn create(kind: Kind, color: Color) -> Piece {
        return Piece { kind, color };
    }

    /// Creates a `Piece` representing no piece (an empty square).
    ///
    /// # Returns
    ///
    /// A `Piece` instance with `Kind::None` and `Color::White`.
    pub fn none() -> Self {
        Piece::create(Kind::None, Color::White)
    }

    /// Gets the kind of the chess piece.
    ///
    /// # Returns
    ///
    /// The kind of the chess piece.
    pub fn get_kind(&self) -> Kind {
        self.kind
    }

    /// Gets the color of the chess piece.
    ///
    /// # Returns
    ///
    /// The color of the chess piece (Black or White).
    pub fn get_color(&self) -> Color {
        self.color
    }

    /// Gets the value of the chess piece in terms of its relative strength.
    ///
    /// # Returns
    ///
    /// An `Option<u8>` representing the value of the piece. Returns `None` for `Kind::None` and `Kind::King`.
    pub fn get_value(&self) -> Option<u8> {
        match self.kind {
            Kind::None => None,
            Kind::Pawn => Some(1),
            Kind::Knight => Some(3),
            Kind::Bishop => Some(3),
            Kind::Rook => Some(5),
            Kind::Queen => Some(9),
            Kind::King => None,
        }
    }

    /// Gets the SVG file path for the chess piece's icon.
    ///
    /// # Returns
    ///
    /// A `String` containing the file path to the SVG icon representing the piece.
    pub fn get_svg(&self) -> String {
        match self.color {
            Color::White => match self.kind {
                Kind::Pawn => "../assets/svg/icon_pawn_white.svg".to_string(),
                Kind::Knight => "../assets/svg/icon_knight_white.svg".to_string(),
                Kind::Bishop => "../assets/svg/icon_bishop_white.svg".to_string(),
                Kind::Rook => "../assets/svg/icon_rook_white.svg".to_string(),
                Kind::Queen => "../assets/svg/icon_queen_white.svg".to_string(),
                Kind::King => "../assets/svg/icon_king_white.svg".to_string(),
                Kind::None => "".to_string(),
            },
            Color::Black => match self.kind {
                Kind::Pawn => "../assets/svg/icon_pawn_black.svg".to_string(),
                Kind::Knight => "../assets/svg/icon_knight_black.svg".to_string(),
                Kind::Bishop => "../assets/svg/icon_bishop_black.svg".to_string(),
                Kind::Rook => "../assets/svg/icon_rook_black.svg".to_string(),
                Kind::Queen => "../assets/svg/icon_queen_black.svg".to_string(),
                Kind::King => "../assets/svg/icon_king_black.svg".to_string(),
                Kind::None => "".to_string(),
            },
        }
    }
}
