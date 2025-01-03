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
