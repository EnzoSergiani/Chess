/// Represents the different kinds of chess pieces.
///
/// # Variants
///
/// - `None`: Represents no piece.
/// - `Pawn`: Represents a pawn piece.
/// - `Knight`: Represents a knight piece.
/// - `Bishop`: Represents a bishop piece.
/// - `Rook`: Represents a rook piece.
/// - `Queen`: Represents a queen piece.
/// - `King`: Represents a king piece.

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Kind {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
