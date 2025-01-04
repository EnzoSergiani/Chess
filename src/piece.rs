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

#[derive(Copy, Clone, PartialEq)]
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

    pub fn none() -> Self {
        Piece::create(Kind::None, Color::White)
    }

    pub fn get_piece(&self) -> Self {
        *self
    }

    pub fn get_kind(&self) -> Kind {
        self.kind
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

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
