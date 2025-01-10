use crate::{board::Board, cell::Cell, color::Color, kind::Kind, piece::Piece, position::Position};

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

    pub fn check(&mut self, board: Board, cell: Cell) {
        self.clear();

        let piece: Option<Piece> = cell.get_piece();
        if piece.is_some() {
            let kind: Kind = piece.unwrap().get_kind();

            match kind {
                Kind::Pawn => self.check_pawn_position(&board, cell),
                Kind::Knight => self.check_knight_position(&board, cell),
                Kind::Bishop => self.check_bishop_position(&board, cell),
                Kind::Rook => self.check_rook_position(&board, cell),
                Kind::Queen => self.check_queen_position(&board, cell),
                Kind::King => self.check_king_position(&board, cell),
                Kind::None => (),
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

    // TODO: add en passant
    fn check_pawn_position(&mut self, board: &Board, cell: Cell) -> () {
        let Position { row, col } = cell.get_coord();
        let color: Color = cell.get_piece_color();
        let size: usize = board.get_size();

        match color {
            Color::White => {
                // first move
                if row == 6
                    && col < size
                    && !Self::is_piece_there(
                        self,
                        board,
                        Position { row: row - 1, col },
                        Color::White,
                    )
                    && !Self::is_piece_there(
                        self,
                        board,
                        Position { row: row - 2, col },
                        Color::White,
                    )
                    && !Self::is_piece_there(
                        self,
                        board,
                        Position { row: row - 1, col },
                        Color::Black,
                    )
                    && !Self::is_piece_there(
                        self,
                        board,
                        Position { row: row - 2, col },
                        Color::Black,
                    )
                {
                    self.possible_moves.push(Position { row: row - 2, col });
                }
                // forward
                if row > 0
                    && !Self::is_piece_there(
                        self,
                        board,
                        Position { row: row - 1, col },
                        Color::White,
                    )
                    && !Self::is_piece_there(
                        self,
                        board,
                        Position { row: row - 1, col },
                        Color::Black,
                    )
                {
                    self.possible_moves.push(Position { row: row - 1, col });
                }
                // attack left
                if row > 0
                    && col > 0
                    && !Self::is_piece_there(
                        self,
                        board,
                        Position {
                            row: row - 1,
                            col: col - 1,
                        },
                        Color::White,
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
                        == Color::Black
                {
                    self.possible_moves.push(Position {
                        row: row - 1,
                        col: col - 1,
                    });
                }
                // attack right
                if row > 0
                    && col + 1 < size
                    && !Self::is_piece_there(
                        self,
                        board,
                        Position {
                            row: row - 1,
                            col: col + 1,
                        },
                        Color::White,
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
                        == Color::Black
                {
                    self.possible_moves.push(Position {
                        row: row - 1,
                        col: col + 1,
                    });
                }
            }
            Color::Black => {
                // first move
                if row == 1
                    && !Self::is_piece_there(
                        self,
                        board,
                        Position { row: row + 1, col },
                        Color::White,
                    )
                    && !Self::is_piece_there(
                        self,
                        board,
                        Position { row: row + 1, col },
                        Color::Black,
                    )
                    && !Self::is_piece_there(
                        self,
                        board,
                        Position { row: row + 2, col },
                        Color::White,
                    )
                    && !Self::is_piece_there(
                        self,
                        board,
                        Position { row: row + 2, col },
                        Color::Black,
                    )
                {
                    self.possible_moves.push(Position {
                        row: row + 2,
                        col: col,
                    });
                }
                // forward
                if row + 1 < size
                    && !Self::is_piece_there(
                        self,
                        board,
                        Position { row: row + 1, col },
                        Color::White,
                    )
                    && !Self::is_piece_there(
                        self,
                        board,
                        Position { row: row + 1, col },
                        Color::Black,
                    )
                {
                    self.possible_moves.push(Position { row: row + 1, col });
                }
                // attack left
                if row > 0
                    && col > 0
                    && !Self::is_piece_there(
                        self,
                        board,
                        Position {
                            row: row + 1,
                            col: col - 1,
                        },
                        Color::Black,
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
                        == Color::White
                {
                    self.possible_moves.push(Position {
                        row: row + 1,
                        col: col - 1,
                    });
                }
                // attack right
                if row > 0
                    && col + 1 < size
                    && !Self::is_piece_there(
                        self,
                        board,
                        Position {
                            row: row + 1,
                            col: col + 1,
                        },
                        Color::Black,
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
                        == Color::White
                {
                    self.possible_moves.push(Position {
                        row: row + 1,
                        col: col + 1,
                    });
                }
            }
        }
    }
    fn check_knight_position(&mut self, board: &Board, cell: Cell) -> () {
        let Position { row, col } = cell.get_coord();
        let color: Color = cell.get_piece_color();
        let size: usize = board.get_size();

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
            let new_row: isize = row as isize + *r;
            let new_col: isize = col as isize + *c;
            if new_row >= 0 && new_row < size as isize && new_col >= 0 && new_col < size as isize {
                let new_row_usize = new_row as usize;
                let new_col_usize = new_col as usize;
                if !Self::is_piece_there(
                    self,
                    board,
                    Position {
                        row: new_row_usize,
                        col: new_col_usize,
                    },
                    color,
                ) {
                    self.possible_moves.push(Position {
                        row: new_row_usize,
                        col: new_col_usize,
                    });
                }
            }
        }
    }

    fn check_bishop_position(&mut self, board: &Board, cell: Cell) -> () {
        let Position { row, col } = cell.get_coord();
        let color: Color = cell.get_piece_color();
        let size: usize = board.get_size();

        // top-left
        for i in 1..=row.min(col) {
            if Self::is_piece_there(
                self,
                board,
                Position {
                    row: row - i,
                    col: col - i,
                },
                color,
            ) {
                break;
            } else {
                self.possible_moves.push(Position {
                    row: row - i,
                    col: col - i,
                });
                if board
                    .get_cell(Position {
                        row: row - i,
                        col: col - i,
                    })
                    .get_piece()
                    .is_some()
                {
                    break;
                }
            }
        }

        // top-right
        for i in 1..=row.min(size - 1 - col) {
            if Self::is_piece_there(
                self,
                board,
                Position {
                    row: row - i,
                    col: col + i,
                },
                color,
            ) {
                break;
            } else {
                self.possible_moves.push(Position {
                    row: row - i,
                    col: col + i,
                });
                if board
                    .get_cell(Position {
                        row: row - i,
                        col: col + i,
                    })
                    .get_piece()
                    .is_some()
                {
                    break;
                }
            }
        }

        // bottom-left
        for i in 1..=(size - 1 - row).min(col) {
            if Self::is_piece_there(
                self,
                board,
                Position {
                    row: row + i,
                    col: col - i,
                },
                color,
            ) {
                break;
            } else {
                self.possible_moves.push(Position {
                    row: row + i,
                    col: col - i,
                });
                if board
                    .get_cell(Position {
                        row: row + i,
                        col: col - i,
                    })
                    .get_piece()
                    .is_some()
                {
                    break;
                }
            }
        }

        // bottom-right
        for i in 1..=(size - 1 - row).min(size - 1 - col) {
            if Self::is_piece_there(
                self,
                board,
                Position {
                    row: row + i,
                    col: col + i,
                },
                color,
            ) {
                break;
            } else {
                self.possible_moves.push(Position {
                    row: row + i,
                    col: col + i,
                });
                if board
                    .get_cell(Position {
                        row: row + i,
                        col: col + i,
                    })
                    .get_piece()
                    .is_some()
                {
                    break;
                }
            }
        }
    }

    fn check_rook_position(&mut self, board: &Board, cell: Cell) -> () {
        let Position { row, col } = cell.get_coord();
        let color: Color = cell.get_piece_color();
        let size: usize = board.get_size();

        // up
        for r in (0..row).rev() {
            if Self::is_piece_there(self, board, Position { row: r, col }, color) {
                break;
            } else {
                self.possible_moves.push(Position { row: r, col });
                if board
                    .get_cell(Position { row: r, col })
                    .get_piece()
                    .is_some()
                {
                    break;
                }
            }
        }

        // down
        for r in row + 1..size {
            if Self::is_piece_there(self, board, Position { row: r, col }, color) {
                break;
            } else {
                self.possible_moves.push(Position { row: r, col: col });
                if board
                    .get_cell(Position { row: r, col })
                    .get_piece()
                    .is_some()
                {
                    break;
                }
            }
        }

        // left
        for c in (0..col).rev() {
            if Self::is_piece_there(self, board, Position { row, col: c }, color) {
                break;
            } else {
                self.possible_moves.push(Position { row: row, col: c });
                if board
                    .get_cell(Position { row, col: c })
                    .get_piece()
                    .is_some()
                {
                    break;
                }
            }
        }

        // right
        for c in col + 1..size {
            if Self::is_piece_there(self, board, Position { row, col: c }, color) {
                break;
            } else {
                self.possible_moves.push(Position { row: row, col: c });
                if board
                    .get_cell(Position { row, col: c })
                    .get_piece()
                    .is_some()
                {
                    break;
                }
            }
        }
    }

    fn check_queen_position(&mut self, board: &Board, cell: Cell) -> () {
        self.check_bishop_position(board, cell);
        self.check_rook_position(board, cell);
    }

    fn check_king_position(&mut self, board: &Board, cell: Cell) -> () {
        let Position { row, col } = cell.get_coord();
        let size: usize = board.get_size();
        let color: Color = cell.get_piece_color();

        for (r, c) in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        {
            let new_row: isize = row as isize + *r;
            let new_col: isize = col as isize + *c;

            if new_row >= 0
                && new_row < size as isize
                && new_col >= 0
                && new_col < size as isize
                && !Self::is_piece_there(
                    self,
                    board,
                    Position {
                        row: new_row as usize,
                        col: new_col as usize,
                    },
                    color,
                )
                && !self.possible_checks.iter().any(|check_position| {
                    check_position.row == new_row as usize && check_position.col == new_col as usize
                })
            {
                self.possible_moves.push(Position {
                    row: new_row as usize,
                    col: new_col as usize,
                });
            }
        }
    }

    fn is_piece_there(&mut self, board: &Board, position: Position, color: Color) -> bool {
        board.get_cell(position).get_piece().is_some()
            && board.get_cell(position).get_piece_color() == color
    }
}
