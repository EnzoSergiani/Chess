use crate::{color::Color, kind::Kind};

/// Represents a chess piece.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Piece {
    /// The kind of the piece.
    kind: Kind,
    /// The color of the piece.
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

    /// Gets the symbol representing the chess piece.
    ///
    /// # Returns
    ///
    /// A `char` representing the piece. Uppercase for white pieces and lowercase for black pieces.
    pub fn get_symbol(&self) -> char {
        match self.color {
            Color::White => match self.kind {
                Kind::None => ' ',
                Kind::Pawn => ' ',
                Kind::Knight => 'N',
                Kind::Bishop => 'B',
                Kind::Rook => 'R',
                Kind::Queen => 'Q',
                Kind::King => 'K',
            },
            Color::Black => match self.kind {
                Kind::None => ' ',
                Kind::Pawn => ' ',
                Kind::Knight => 'n',
                Kind::Bishop => 'b',
                Kind::Rook => 'r',
                Kind::Queen => 'q',
                Kind::King => 'k',
            },
        }
    }

    /// Creates a `Piece` from a given symbol.
    ///
    /// # Arguments
    ///
    /// * `symbol` - A `char` representing the piece. Uppercase for white pieces and lowercase for black pieces.
    ///
    /// # Returns
    ///
    /// A `Piece` instance corresponding to the given symbol.
    pub fn from_symbol(symbol: char) -> Piece {
        let (kind, color) = match symbol {
            'P' => (Kind::Pawn, Color::White),
            'p' => (Kind::Pawn, Color::Black),
            'N' => (Kind::Knight, Color::White),
            'n' => (Kind::Knight, Color::Black),
            'B' => (Kind::Bishop, Color::White),
            'b' => (Kind::Bishop, Color::Black),
            'R' => (Kind::Rook, Color::White),
            'r' => (Kind::Rook, Color::Black),
            'Q' => (Kind::Queen, Color::White),
            'q' => (Kind::Queen, Color::Black),
            'K' => (Kind::King, Color::White),
            'k' => (Kind::King, Color::Black),
            _ => (Kind::None, Color::White),
        };

        Piece { kind, color }
    }
}
