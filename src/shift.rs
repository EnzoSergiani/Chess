use crate::{board::Board, cell::Cell, color::Color, kind::Kind, position::Position};

/// Represents the possible moves and checks for a piece on the board.
#[derive(Clone)]
pub struct Shift {
    /// A vector of positions representing the possible moves for a piece.
    possible_moves: Vec<Position>,
    /// A vector of positions representing the possible checks for a piece.
    possible_checks: Vec<Position>,
}

impl Shift {
    /// Creates a new `Shift` instance with empty possible moves and checks.
    ///
    /// # Returns
    ///
    /// A new `Shift` instance.
    pub fn new() -> Shift {
        Shift {
            possible_moves: Vec::new(),
            possible_checks: Vec::new(),
        }
    }

    /// Sets the possible moves for a given piece on the board.
    ///
    /// # Arguments
    ///
    /// * `board` - A reference to the game board.
    /// * `cell` - The cell containing the piece for which to set possible moves.
    pub fn set_possible_moves(&mut self, board: Board, cell: Cell) -> () {
        self.clear();

        if let Some(piece) = cell.get_piece() {
            let moves: Vec<Position> = match piece.get_kind() {
                Kind::Pawn => self.get_pawn_possible_moves(&board, cell),
                Kind::Knight => self.get_knight_possible_moves(&board, cell),
                Kind::Bishop => self.get_bishop_possible_moves(&board, cell),
                Kind::Rook => self.get_rook_possible_moves(&board, cell),
                Kind::Queen => self.get_queen_possible_moves(&board, cell),
                Kind::King => self.get_king_possible_moves(&board, cell),
                Kind::None => Vec::new(),
            };
            self.possible_moves.extend(moves);
        }
    }

    /// Sets the possible checks for a given color on the board.
    ///
    /// # Arguments
    ///
    /// * `board` - A reference to the game board.
    /// * `color` - The color for which to set possible checks.
    pub fn set_possible_checks(&mut self, board: Board, color: Color) -> () {
        self.clear();
        for row in 0..board.get_size() {
            for col in 0..board.get_size() {
                let current_position: Position = Position::new(row, col);
                if let Some(piece) = board.get_cell(current_position).get_piece() {
                    if piece.get_color() != color {
                        let cell: Cell = board.get_cell(current_position).clone();
                        let kind_enemy: Kind = cell.get_piece_kind();
                        let moves: Vec<Position> = match kind_enemy {
                            Kind::Pawn => self.get_pawn_possible_attacks(&board, cell.clone()),
                            Kind::Knight => self.get_knight_possible_moves(&board, cell.clone()),
                            Kind::Bishop => self.get_bishop_possible_moves(&board, cell.clone()),
                            Kind::Rook => self.get_rook_possible_moves(&board, cell.clone()),
                            Kind::Queen => self.get_queen_possible_moves(&board, cell.clone()),
                            Kind::King => self.get_king_possible_moves(&board, cell.clone()),
                            Kind::None => Vec::new(),
                        };
                        self.possible_checks.extend(moves);
                    }
                }
            }
        }
    }

    /// Returns the possible moves.
    ///
    /// # Returns
    ///
    /// A vector of positions representing the possible moves.
    pub fn get_possible_moves(&self) -> Vec<Position> {
        self.possible_moves.clone()
    }

    /// Returns the possible checks.
    ///
    /// # Returns
    ///
    /// A vector of positions representing the possible checks.
    pub fn get_possible_checks(&self) -> Vec<Position> {
        self.possible_checks.clone()
    }

    /// Clears the possible moves and checks.
    fn clear(&mut self) -> () {
        self.possible_moves.clear();
        self.possible_checks.clear();
    }

    /// Checks if there is a piece at the given position and if it matches the given color.
    ///
    /// # Arguments
    ///
    /// * `board` - A reference to the game board.
    /// * `position` - The position to check.
    /// * `color` - The color to match.
    ///
    /// # Returns
    ///
    /// `true` if there is a piece at the given position and it matches the given color, `false` otherwise.
    fn is_piece_there(&mut self, board: &Board, position: Position, color: Color) -> bool {
        board.get_cell(position).get_piece().is_some()
            && board.get_cell(position).get_piece_color() == color
    }

    /// Checks if the move to the given position is illegal.
    ///
    /// # Arguments
    ///
    /// * `board` - A reference to the game board.
    /// * `position` - The position to check.
    ///
    /// # Returns
    ///
    /// `true` if the move is illegal, `false` otherwise.
    fn is_illegal_move(&mut self, board: &Board, position: Position) -> bool {
        self.set_possible_checks(board.clone(), board.get_cell(position).get_piece_color());
        self.get_possible_checks().contains(&position)
    }

    /// Returns the possible attacks for a pawn.
    ///s
    /// # Arguments
    ///
    /// * `board` - A reference to the game board.
    /// * `cell` - The cell containing the pawn.
    ///
    /// # Returns
    ///
    /// A vector of positions representing the possible attacks of the pawn.
    fn get_pawn_possible_attacks(&mut self, board: &Board, cell: Cell) -> Vec<Position> {
        let (row, col) = cell.get_position();
        let color: Color = cell.get_piece_color();
        let mut possible_moves: Vec<Position> = Vec::new();

        match color {
            Color::White => {
                if col > 0 && self.is_piece_there(board, Position::new(row - 1, col), !color) {
                    possible_moves.push(Position::new(row - 1, col - 1));
                }
                if col < board.get_size() - 1
                    && self.is_piece_there(board, Position::new(row - 1, col + 1), !color)
                {
                    possible_moves.push(Position::new(row - 1, col + 1));
                }
            }
            Color::Black => {
                if col > 0 && self.is_piece_there(board, Position::new(row + 1, col - 1), !color) {
                    possible_moves.push(Position::new(row + 1, col - 1));
                }
                if col < board.get_size() - 1
                    && self.is_piece_there(board, Position::new(row + 1, col + 1), !color)
                {
                    possible_moves.push(Position::new(row + 1, col + 1))
                }
            }
        }

        possible_moves
    }

    /// Returns the possible moves for a pawn.
    ///
    /// # Arguments
    ///
    /// * `board` - A reference to the game board.
    /// * `cell` - The cell containing the pawn.
    ///
    /// # Returns
    ///
    /// A vector of positions representing the possible moves of the pawn.
    fn get_pawn_possible_moves(&mut self, board: &Board, cell: Cell) -> Vec<Position> {
        // TODO: add en passant
        let (row, col) = cell.get_position();
        let color: Color = cell.get_piece_color();
        let mut possible_moves: Vec<Position> = Vec::new();

        match color {
            Color::White => {
                // first move
                if row == 6
                    && col < board.get_size()
                    && !self.is_piece_there(board, Position::new(row - 1, col), color)
                    && !self.is_piece_there(board, Position::new(row - 2, col), color)
                    && !self.is_piece_there(board, Position::new(row - 1, col), !color)
                    && !self.is_piece_there(board, Position::new(row - 2, col), !color)
                {
                    possible_moves.push(Position::new(row - 2, col));
                }
                // forward
                if row > 0
                    && !self.is_piece_there(board, Position::new(row - 1, col), color)
                    && !self.is_piece_there(board, Position::new(row - 1, col), !color)
                {
                    possible_moves.push(Position::new(row - 1, col));
                }
                // attack left
                if row > 0
                    && col > 0
                    && !self.is_piece_there(board, Position::new(row - 1, col - 1), color)
                    && board
                        .get_cell(Position::new(row - 1, col - 1))
                        .get_piece()
                        .is_some()
                    && board
                        .get_cell(Position::new(row - 1, col - 1))
                        .get_piece_color()
                        == !color
                {
                    possible_moves.push(Position::new(row - 1, col - 1));
                }
                // attack right
                if row > 0
                    && col + 1 < board.get_size()
                    && !self.is_piece_there(board, Position::new(row - 1, col + 1), color)
                    && board
                        .get_cell(Position::new(row - 1, col + 1))
                        .get_piece()
                        .is_some()
                    && board
                        .get_cell(Position::new(row - 1, col + 1))
                        .get_piece_color()
                        == !color
                {
                    possible_moves.push(Position::new(row - 1, col + 1));
                }
                possible_moves
            }
            Color::Black => {
                // first move
                if row == 1
                    && !self.is_piece_there(board, Position::new(row + 1, col), color)
                    && !self.is_piece_there(board, Position::new(row + 1, col), !color)
                    && !self.is_piece_there(board, Position::new(row + 2, col), color)
                    && !self.is_piece_there(board, Position::new(row + 2, col), !color)
                {
                    possible_moves.push(Position::new(row + 2, col));
                }
                // forward
                if row + 1 < board.get_size()
                    && !self.is_piece_there(board, Position::new(row + 1, col), color)
                    && !self.is_piece_there(board, Position::new(row + 1, col), !color)
                {
                    possible_moves.push(Position::new(row + 1, col));
                }
                // attack left
                if row > 0
                    && col > 0
                    && !self.is_piece_there(board, Position::new(row + 1, col - 1), color)
                    && board
                        .get_cell(Position::new(row + 1, col - 1))
                        .get_piece()
                        .is_some()
                    && board
                        .get_cell(Position::new(row + 1, col - 1))
                        .get_piece_color()
                        == !color
                {
                    possible_moves.push(Position::new(row + 1, col - 1));
                }
                // attack right
                if row > 0
                    && col + 1 < board.get_size()
                    && !self.is_piece_there(board, Position::new(row + 1, col + 1), color)
                    && board
                        .get_cell(Position::new(row + 1, col + 1))
                        .get_piece()
                        .is_some()
                    && board
                        .get_cell(Position::new(row + 1, col + 1))
                        .get_piece_color()
                        == !color
                {
                    possible_moves.push(Position::new(row + 1, col + 1));
                }
                possible_moves
            }
        }
    }

    /// Returns the possible moves for a knight.
    ///
    /// # Arguments
    ///
    /// * `board` - A reference to the game board.
    /// * `cell` - The cell containing the knight.
    ///
    /// # Returns
    ///
    /// A vector of positions representing the possible moves of the knight.
    fn get_knight_possible_moves(&mut self, board: &Board, cell: Cell) -> Vec<Position> {
        let (row, col) = cell.get_position();
        let color: Color = cell.get_piece_color();
        let mut possible_moves: Vec<Position> = Vec::new();

        let knight_moves: [(isize, isize); 8] = [
            (-1, -2),
            (1, -2),
            (-1, 2),
            (1, 2),
            (-2, -1),
            (2, -1),
            (-2, 1),
            (2, 1),
        ];

        for (r, c) in knight_moves.iter() {
            let new_row: isize = row as isize + r;
            let new_col: isize = col as isize + c;
            if new_row >= 0
                && new_row < board.get_size() as isize
                && new_col >= 0
                && new_col < board.get_size() as isize
            {
                let position: Position = Position::new(new_row as usize, new_col as usize);
                if !self.is_piece_there(board, position, color) {
                    possible_moves.push(position);
                }
            }
        }
        possible_moves
    }

    /// Returns the possible moves for a bishop.
    ///
    /// # Arguments
    ///
    /// * `board` - A reference to the game board.
    /// * `cell` - The cell containing the bishop.
    ///
    /// # Returns
    ///
    /// A vector of positions representing the possible moves of the bishop.
    fn get_bishop_possible_moves(&mut self, board: &Board, cell: Cell) -> Vec<Position> {
        let (row, col) = cell.get_position();
        let color: Color = cell.get_piece_color();
        let mut possible_moves: Vec<Position> = Vec::new();

        // top-left
        for i in 1..=row.min(col) {
            let position: Position = Position::new(row - i, col - i);
            if self.is_piece_there(board, position, color) {
                break;
            } else {
                possible_moves.push(position);
                if board.get_cell(position).get_piece().is_some() {
                    break;
                }
            }
        }

        // top-right
        for i in 1..=row.min(board.get_size() - 1 - col) {
            let position: Position = Position::new(row - i, col + i);
            if self.is_piece_there(board, position, color) {
                break;
            } else {
                possible_moves.push(position);
                if board.get_cell(position).get_piece().is_some() {
                    break;
                }
            }
        }

        // bottom-left
        for i in 1..=(board.get_size() - 1 - row).min(col) {
            let pos = Position::new(row + i, col - i);
            if self.is_piece_there(board, pos, color) {
                break;
            } else {
                possible_moves.push(pos);
                if board.get_cell(pos).get_piece().is_some() {
                    break;
                }
            }
        }

        // bottom-right
        for i in 1..=(board.get_size() - 1 - row).min(board.get_size() - 1 - col) {
            let pos = Position::new(row + i, col + i);
            if self.is_piece_there(board, pos, color) {
                break;
            } else {
                possible_moves.push(pos);
                if board.get_cell(pos).get_piece().is_some() {
                    break;
                }
            }
        }
        possible_moves
    }

    /// Returns the possible moves for a rook.
    ///
    /// # Arguments
    ///
    /// * `board` - A reference to the game board.
    /// * `cell` - The cell containing the rook.
    ///
    /// # Returns
    ///
    /// A vector of positions representing the possible moves of the rook.
    fn get_rook_possible_moves(&mut self, board: &Board, cell: Cell) -> Vec<Position> {
        let (row, col) = cell.get_position();
        let color: Color = cell.get_piece_color();
        let mut possible_moves: Vec<Position> = Vec::new();

        // up
        for r in (0..row).rev() {
            let position: Position = Position::new(r, col);
            if self.is_piece_there(board, position, color) {
                break;
            } else {
                possible_moves.push(position);
                if board.get_cell(position).get_piece().is_some() {
                    break;
                }
            }
        }

        // down
        for r in row + 1..board.get_size() {
            let position: Position = Position::new(r, col);
            if self.is_piece_there(board, position, color) {
                break;
            } else {
                possible_moves.push(position);
                if board.get_cell(position).get_piece().is_some() {
                    break;
                }
            }
        }

        // left
        for c in (0..col).rev() {
            let position: Position = Position::new(row, c);
            if self.is_piece_there(board, position, color) {
                break;
            } else {
                possible_moves.push(position);
                if board.get_cell(position).get_piece().is_some() {
                    break;
                }
            }
        }

        // right
        for c in col + 1..board.get_size() {
            let position: Position = Position::new(row, c);
            if self.is_piece_there(board, position, color) {
                break;
            } else {
                possible_moves.push(position);
                if board.get_cell(position).get_piece().is_some() {
                    break;
                }
            }
        }

        possible_moves
    }

    /// Returns the possible moves for a queen.
    ///
    /// # Arguments
    ///
    /// * `board` - A reference to the game board.
    /// * `cell` - The cell containing the queen.
    ///
    /// # Returns
    ///
    /// A vector of positions representing the possible moves of the queen.
    fn get_queen_possible_moves(&mut self, board: &Board, cell: Cell) -> Vec<Position> {
        let mut possible_moves: Vec<Position> = Vec::new();
        possible_moves.extend(self.get_bishop_possible_moves(board, cell));
        possible_moves.extend(self.get_rook_possible_moves(board, cell));
        possible_moves
    }

    /// Returns the possible moves for a king.
    ///
    /// # Arguments
    ///
    /// * `board` - A reference to the game board.
    /// * `cell` - The cell containing the king.
    ///
    /// # Returns
    ///
    /// A vector of positions representing the possible moves of the king.
    fn get_king_possible_moves(&mut self, board: &Board, cell: Cell) -> Vec<Position> {
        let (row, col) = cell.get_position();
        let color: Color = cell.get_piece_color();
        let mut possible_moves: Vec<Position> = Vec::new();

        let king_moves: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (r, c) in king_moves.iter() {
            let new_row: isize = row as isize + r;
            let new_col: isize = col as isize + c;

            if new_row >= 0
                && new_row < board.get_size() as isize
                && new_col >= 0
                && new_col < board.get_size() as isize
                && !self.is_piece_there(
                    board,
                    Position::new(new_row as usize, new_col as usize),
                    color,
                )
                && !self.is_illegal_move(board, Position::new(new_row as usize, new_col as usize))
            {
                possible_moves.push(Position::new(new_row as usize, new_col as usize));
            }
        }
        possible_moves
    }
}
