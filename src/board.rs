use web_sys;

use yew::prelude::*;

use crate::{cell::Cell, color::Color, kind::Kind, piece::Piece, position::Position, shift::Shift};

#[derive(Clone)]
pub struct Board {
    board: Vec<Vec<Cell>>,
    size: usize,
    selected_piece: Option<Position>,
    shift: Shift,
}

impl Board {
    pub fn new() -> Board {
        let mut board: Vec<Vec<Cell>> = Vec::new();
        let size: usize = 8;
        let shift: Shift = Shift::new();

        for row_idx in 0..size {
            let mut row: Vec<Cell> = Vec::new();

            for col_idx in 0..size {
                let color: Color = if (row_idx + col_idx) % 2 == 0 {
                    Color::White
                } else {
                    Color::Black
                };

                row.push(Cell {
                    color,
                    piece: Piece::none(),
                    is_selected: false,
                    is_check: false,
                    position: Position {
                        row: row_idx,
                        col: col_idx,
                    },
                });
            }

            board.push(row);
        }

        Board {
            board,
            size: 8,
            selected_piece: None,
            shift,
        }
    }

    pub fn load_from_fen(&mut self, fen: &str) -> () {
        let mut index_row: usize = 0;
        let mut index_col: usize = 0;

        for char in fen.chars() {
            if char.is_digit(10) {
                index_col += char.to_digit(10).unwrap() as usize;
            }
            if char.is_alphabetic() {
                self.board[index_row][index_col] = Cell {
                    color: self.board[index_row][index_col].color,
                    piece: match char {
                        'P' => Piece::create(Kind::Pawn, Color::White),
                        'N' => Piece::create(Kind::Knight, Color::White),
                        'B' => Piece::create(Kind::Bishop, Color::White),
                        'R' => Piece::create(Kind::Rook, Color::White),
                        'Q' => Piece::create(Kind::Queen, Color::White),
                        'K' => Piece::create(Kind::King, Color::White),
                        'p' => Piece::create(Kind::Pawn, Color::Black),
                        'n' => Piece::create(Kind::Knight, Color::Black),
                        'b' => Piece::create(Kind::Bishop, Color::Black),
                        'r' => Piece::create(Kind::Rook, Color::Black),
                        'q' => Piece::create(Kind::Queen, Color::Black),
                        'k' => Piece::create(Kind::King, Color::Black),
                        _ => Piece::none(),
                    },
                    is_selected: false,
                    is_threat: false,
                    position: Position {
                        row: index_row,
                        col: index_col,
                    },
                };
                index_col += 1;
            }
            if char == '/' {
                index_col = 0;
                index_row += 1;
            }
        }
    }

    pub fn initialize(mut self) -> Self {
        let fen_init: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        self.load_from_fen(fen_init);

        // for row in &self.board {
        //     for cell in row {
        //         self.mv.check(&self, cell);
        //     }
        // }

        self
    }

    pub fn render(&self, on_click: Callback<Position>) -> Html {
        html! {
            <div class={classes!("board-border")}>
                <div class={classes!("board")}>
                    {for self.board.iter().enumerate().map(|(row_idx, row)| {
                        html! {
                            <div class="row">
                                {for row.iter().enumerate().map(|(col_idx, cell)| {
                                    let on_click = {
                                        let on_click = on_click.clone();
                                        Callback::from(move |_| on_click.emit(Position { row: row_idx, col: col_idx }))
                                    };
                                    html! {
                                        <div class={
                                            classes!(
                                                if cell.is_selected { "cell_move" } else { "" },
                                                if cell.is_check { "cell_check" } else { "" },
                                                if cell.color == Color::White { "cell cell_white" } else { "cell cell_black" }
                                            )
                                        } onclick={on_click}>
                                            <img src={cell.piece.get_svg()} height="60px" />
                                        </div>
                                    }
                                })}
                            </div>
                        }
                    })}
                </div>
            </div>
        }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> &Cell {
        &self.board[row][col]
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    fn clear(&mut self) -> () {
        for r in 0..self.size {
            for c in 0..self.size {
                self.board[r][c].is_selected = false;
                self.board[r][c].is_threat = false;
            }
        }
    }

    fn move_piece(&mut self, old_position: Position, new_position: Position) -> () {
        web_sys::console::log_1(&format!("piece moved",).into());
        let Position {
            row: old_row,
            col: old_col,
        } = old_position;
        let Position {
            row: new_row,
            col: new_col,
        } = new_position;
        self.board[new_row][new_col].piece = self.board[old_row][old_col].piece.clone();
        self.board[old_row][old_col].piece = Piece::none();
        self.selected_piece = None;
        self.clear();
    }

    fn select_new_piece(&mut self, cell: Cell) {
        let position = cell.get_coord();
        self.selected_piece = Some(position);
        self.shift.check(self.clone(), cell);
        let possible_moves: Vec<Position> = self.shift.get_possible_moves();
        // let possible_checks: Vec<Position> = self.shift.get_possible_checks();

        self.clear();
        self.display_possible_moves(possible_moves.clone());
        // self.display_possible_checks(possible_checks.clone());
    }

    fn display_possible_moves(&mut self, possible_moves: Vec<Position>) -> () {
        for pos in possible_moves.iter() {
            self.board[pos.row][pos.col].is_selected = true;
        }
    }

    fn display_possible_checks(&mut self, possible_checks: Vec<Position>) -> () {
        for pos in possible_checks.iter() {
            self.board[pos.row][pos.col].is_threat = true;
        }
    }

    pub fn handle_click(&mut self, cell: Cell) {
        let Position { row, col } = cell.get_coord();

        if let Some(selected_pos) = self.selected_piece {
            self.shift.check(
                self.clone(),
                *self.get_cell(selected_pos.row, selected_pos.col),
            );
            let possible_moves: Vec<Position> = self.shift.get_possible_moves();

            if possible_moves.contains(&cell.get_coord()) {
                self.move_piece(selected_pos, cell.get_coord());
                self.selected_piece = None;
            } else {
                if let Some(piece) = self.get_cell(row, col).get_piece() {
                    if piece.get_kind() != Kind::None {
                        self.select_new_piece(cell);
                    } else {
                        self.selected_piece = None;
                    }
                } else {
                    self.selected_piece = None;
                }
            }
        } else {
            if self.get_cell(row, col).get_piece().is_some() {
                self.select_new_piece(cell);
            }
        }
    }
}
