#[derive(Copy, Clone)]
pub enum Kind {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone)]
pub enum Color {
    Black,
    White,
}

#[derive(Copy, Clone)]
pub struct Piece {
    kind: Kind,
    color: Color,
}

impl Piece {
    pub fn create(kind: Kind, color: Color) -> Piece {
        return Piece { kind, color };
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            Kind::None => write!(f, " "),
            Kind::Pawn => write!(f, "P"),
            Kind::Knight => write!(f, "N"),
            Kind::Bishop => write!(f, "B"),
            Kind::Rook => write!(f, "R"),
            Kind::Queen => write!(f, "Q"),
            Kind::King => write!(f, "K"),
        }
    }
}
