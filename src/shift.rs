use crate::{board::Board, cell::Cell, color::Color, kind::Kind, position::Position};

#[derive(Clone)]
pub struct Shift {
    possible_moves: Vec<Position>,
    possible_checks: Vec<Position>,
}

impl Shift {
    pub fn new() -> Shift {
        Shift {
            possible_moves: Vec::new(),
            possible_checks: Vec::new(),
        }
    }

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

    pub fn set_possible_checks(&mut self, board: Board, color: Color) -> () {
        self.clear();
        let size: usize = board.get_size();
        for row in 0..size {
            for col in 0..size {
                let current_position: Position = Position { row, col };
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

    pub fn get_possible_moves(&self) -> Vec<Position> {
        self.possible_moves.clone()
    }

    pub fn get_possible_checks(&self) -> Vec<Position> {
        self.possible_checks.clone()
    }

    fn clear(&mut self) -> () {
        self.possible_moves.clear();
        self.possible_checks.clear();
    }

    fn is_piece_there(&mut self, board: &Board, position: Position, color: Color) -> bool {
        board.get_cell(position).get_piece().is_some()
            && board.get_cell(position).get_piece_color() == color
    }

    fn is_illegal_move(&mut self, board: &Board, position: Position) -> bool {
        if self.possible_checks.is_empty() {
            self.set_possible_checks(board.clone(), Color::White);
        }
        self.get_possible_checks().contains(&Position {
            row: position.row,
            col: position.col,
        })
    }

    fn get_pawn_possible_attacks(&mut self, board: &Board, cell: Cell) -> Vec<Position> {
        let Position { row, col } = cell.get_coord();
        let color: Color = cell.get_piece_color();
        let size: usize = board.get_size();
        let mut possible_moves: Vec<Position> = Vec::new();

        match color {
            Color::White => {
                if col > 0
                    && self.is_piece_there(
                        board,
                        Position {
                            row: row - 1,
                            col: col - 1,
                        },
                        Color::Black,
                    )
                {
                    possible_moves.push(Position {
                        row: row - 1,
                        col: col - 1,
                    });
                }
                if col < size - 1
                    && self.is_piece_there(
                        board,
                        Position {
                            row: row - 1,
                            col: col + 1,
                        },
                        Color::Black,
                    )
                {
                    possible_moves.push(Position {
                        row: row - 1,
                        col: col + 1,
                    });
                }
            }
            Color::Black => {
                if col > 0
                    && self.is_piece_there(
                        board,
                        Position {
                            row: row + 1,
                            col: col - 1,
                        },
                        Color::White,
                    )
                {
                    possible_moves.push(Position {
                        row: row + 1,
                        col: col - 1,
                    });
                }
                if col < size - 1
                    && self.is_piece_there(
                        board,
                        Position {
                            row: row + 1,
                            col: col + 1,
                        },
                        Color::White,
                    )
                {
                    possible_moves.push(Position {
                        row: row + 1,
                        col: col + 1,
                    });
                }
            }
        }

        possible_moves
    }

    // TODO: add en passant
    fn get_pawn_possible_moves(&mut self, board: &Board, cell: Cell) -> Vec<Position> {
        let Position { row, col } = cell.get_coord();
        let color: Color = cell.get_piece_color();
        let size: usize = board.get_size();
        let mut possible_moves: Vec<Position> = Vec::new();

        match color {
            Color::White => {
                // first move
                if row == 6
                    && col < size
                    && !self.is_piece_there(board, Position { row: row - 1, col }, color)
                    && !self.is_piece_there(board, Position { row: row - 2, col }, color)
                    && !self.is_piece_there(board, Position { row: row - 1, col }, !color)
                    && !self.is_piece_there(board, Position { row: row - 2, col }, !color)
                {
                    possible_moves.push(Position { row: row - 2, col });
                }
                // forward
                if row > 0
                    && !self.is_piece_there(board, Position { row: row - 1, col }, color)
                    && !self.is_piece_there(board, Position { row: row - 1, col }, !color)
                {
                    possible_moves.push(Position { row: row - 1, col });
                }
                // attack left
                if row > 0
                    && col > 0
                    && !self.is_piece_there(
                        board,
                        Position {
                            row: row - 1,
                            col: col - 1,
                        },
                        color,
                    )
                    && board
                        .get_cell(Position {
                            row: row - 1,
                            col: col - 1,
                        })
                        .get_piece()
                        .is_some()
                    && board
                        .get_cell(Position {
                            row: row - 1,
                            col: col - 1,
                        })
                        .get_piece_color()
                        == !color
                {
                    possible_moves.push(Position {
                        row: row - 1,
                        col: col - 1,
                    });
                }
                // attack right
                if row > 0
                    && col + 1 < size
                    && !self.is_piece_there(
                        board,
                        Position {
                            row: row - 1,
                            col: col + 1,
                        },
                        color,
                    )
                    && board
                        .get_cell(Position {
                            row: row - 1,
                            col: col + 1,
                        })
                        .get_piece()
                        .is_some()
                    && board
                        .get_cell(Position {
                            row: row - 1,
                            col: col + 1,
                        })
                        .get_piece_color()
                        == !color
                {
                    possible_moves.push(Position {
                        row: row - 1,
                        col: col + 1,
                    });
                }
                possible_moves
            }
            Color::Black => {
                // first move
                if row == 1
                    && !self.is_piece_there(board, Position { row: row + 1, col }, color)
                    && !self.is_piece_there(board, Position { row: row + 1, col }, !color)
                    && !self.is_piece_there(board, Position { row: row + 2, col }, color)
                    && !self.is_piece_there(board, Position { row: row + 2, col }, !color)
                {
                    possible_moves.push(Position { row: row + 2, col });
                }
                // forward
                if row + 1 < size
                    && !self.is_piece_there(board, Position { row: row + 1, col }, color)
                    && !self.is_piece_there(board, Position { row: row + 1, col }, !color)
                {
                    possible_moves.push(Position { row: row + 1, col });
                }
                // attack left
                if row > 0
                    && col > 0
                    && !self.is_piece_there(
                        board,
                        Position {
                            row: row + 1,
                            col: col - 1,
                        },
                        color,
                    )
                    && board
                        .get_cell(Position {
                            row: row + 1,
                            col: col - 1,
                        })
                        .get_piece()
                        .is_some()
                    && board
                        .get_cell(Position {
                            row: row + 1,
                            col: col - 1,
                        })
                        .get_piece_color()
                        == !color
                {
                    possible_moves.push(Position {
                        row: row + 1,
                        col: col - 1,
                    });
                }
                // attack right
                if row > 0
                    && col + 1 < size
                    && !self.is_piece_there(
                        board,
                        Position {
                            row: row + 1,
                            col: col + 1,
                        },
                        color,
                    )
                    && board
                        .get_cell(Position {
                            row: row + 1,
                            col: col + 1,
                        })
                        .get_piece()
                        .is_some()
                    && board
                        .get_cell(Position {
                            row: row + 1,
                            col: col + 1,
                        })
                        .get_piece_color()
                        == !color
                {
                    possible_moves.push(Position {
                        row: row + 1,
                        col: col + 1,
                    });
                }
                possible_moves
            }
        }
    }
    fn get_knight_possible_moves(&mut self, board: &Board, cell: Cell) -> Vec<Position> {
        let Position { row, col } = cell.get_coord();
        let color: Color = cell.get_piece_color();
        let size: usize = board.get_size();
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
            if new_row >= 0 && new_row < size as isize && new_col >= 0 && new_col < size as isize {
                let position: Position = Position {
                    row: new_row as usize,
                    col: new_col as usize,
                };
                if !self.is_piece_there(board, position, color) {
                    possible_moves.push(position);
                }
            }
        }
        possible_moves
    }

    fn get_bishop_possible_moves(&mut self, board: &Board, cell: Cell) -> Vec<Position> {
        let Position { row, col } = cell.get_coord();
        let color: Color = cell.get_piece_color();
        let size: usize = board.get_size();
        let mut possible_moves: Vec<Position> = Vec::new();

        // top-left
        for i in 1..=row.min(col) {
            let position: Position = Position {
                row: row - i,
                col: col - i,
            };
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
        for i in 1..=row.min(size - 1 - col) {
            let position = Position {
                row: row - i,
                col: col + i,
            };
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
        for i in 1..=(size - 1 - row).min(col) {
            let pos = Position {
                row: row + i,
                col: col - i,
            };
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
        for i in 1..=(size - 1 - row).min(size - 1 - col) {
            let pos = Position {
                row: row + i,
                col: col + i,
            };
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

    fn get_rook_possible_moves(&mut self, board: &Board, cell: Cell) -> Vec<Position> {
        let Position { row, col } = cell.get_coord();
        let color: Color = cell.get_piece_color();
        let size: usize = board.get_size();
        let mut possible_moves: Vec<Position> = Vec::new();

        // up
        for r in (0..row).rev() {
            let position: Position = Position { row: r, col };
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
        for r in row + 1..size {
            let position: Position = Position { row: r, col };
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
            let position: Position = Position { row, col: c };
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
        for c in col + 1..size {
            let position: Position = Position { row, col: c };
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

    fn get_queen_possible_moves(&mut self, board: &Board, cell: Cell) -> Vec<Position> {
        let mut possible_moves: Vec<Position> = Vec::new();
        possible_moves.extend(self.get_bishop_possible_moves(board, cell));
        possible_moves.extend(self.get_rook_possible_moves(board, cell));
        possible_moves
    }

    fn get_king_possible_moves(&mut self, board: &Board, cell: Cell) -> Vec<Position> {
        let Position { row, col } = cell.get_coord();
        let size: usize = board.get_size();
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
                && new_row < size as isize
                && new_col >= 0
                && new_col < size as isize
                && !self.is_piece_there(
                    board,
                    Position {
                        row: new_row as usize,
                        col: new_col as usize,
                    },
                    color,
                )
                && !self.is_illegal_move(
                    board,
                    Position {
                        row: new_row as usize,
                        col: new_col as usize,
                    },
                )
            {
                possible_moves.push(Position {
                    row: new_row as usize,
                    col: new_col as usize,
                });
            }
        }
        possible_moves
    }
}
