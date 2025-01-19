use std::ops::Not;

/// Represents the two colors possible in a chess.
///
/// # Variants
///
/// - `Black`: Represents black color.
/// - `White`: Represents white color.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    Black,
    White,
}

/// Returns the opposite color.
impl Not for Color {
    type Output = Color;

    fn not(self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}
