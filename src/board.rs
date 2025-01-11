use web_sys;

use yew::prelude::*;

use crate::{cell::Cell, color::Color, kind::Kind, piece::Piece, position::Position, shift::Shift};

#[derive(Clone)]
pub struct Board {
    board: Vec<Vec<Cell>>,
    size: usize,
    selected_piece: Option<Position>,
    shift: Shift,
    color_turn: Color,
}

impl Board {
    pub fn new() -> Board {
        let mut board: Vec<Vec<Cell>> = Vec::new();
        let size: usize = 8;

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
            size,
            selected_piece: None,
            shift: Shift::new(),
            color_turn: Color::White,
        }
    }

    pub fn load_from_fen(&mut self, fen: &str) -> () {
        let mut row: usize = 0;
        let mut col: usize = 0;

        for char in fen.chars() {
            if char.is_digit(10) {
                col += char.to_digit(10).unwrap() as usize;
            }
            if char.is_alphabetic() {
                self.board[row][col] = Cell {
                    color: self.board[row][col].color,
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
                    is_check: false,
                    position: Position { row, col },
                };
                col += 1;
            }
            if char == '/' {
                row += 1;
                col = 0;
            }
        }
    }

    pub fn initialize(mut self) -> Self {
        let fen_init: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        self.load_from_fen(fen_init);

        self.print_board();
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

    pub fn get_cell(&self, position: Position) -> &Cell {
        &self.board[position.row][position.col]
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    fn clear(&mut self) -> () {
        for r in 0..self.size {
            for c in 0..self.size {
                self.board[r][c].is_selected = false;
                self.board[r][c].is_check = false;
            }
        }
    }

    fn move_piece(&mut self, old_position: Position, new_position: Position) -> () {
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
        self.check_king_status();

        if self.board[new_row][new_col].piece.get_kind() == Kind::Pawn {
            let color: Color = self.board[new_row][new_col].get_piece_color();
            if new_row == 0 && color == Color::White {
                self.promote(new_position);
            } else if new_row == self.size - 1 && color == Color::Black {
                self.promote(new_position);
            }
        }
    }

    fn check_king_status(&mut self) -> () {
        let opposant_color: Color = !self.color_turn;
        self.shift.set_possible_checks(self.clone(), opposant_color);
        let position_king: Option<Position> = self.get_position_king(opposant_color);

        if let Some(position_king) = position_king {
            if self.is_king_in_check(position_king) {
                self.display_king_in_check(position_king);
            }
        }
    }

    fn select_new_piece(&mut self, cell: Cell) {
        let position = cell.get_coord();
        self.selected_piece = Some(position);
        self.shift.set_possible_moves(self.clone(), cell);

        self.clear();
        self.display_possible_moves();
    }

    fn display_possible_moves(&mut self) -> () {
        for pos in self.shift.get_possible_moves().iter() {
            self.board[pos.row][pos.col].is_selected = true;
        }
    }

    fn next_turn(&mut self) -> () {
        self.color_turn = !self.color_turn
    }

    pub fn handle_click(&mut self, cell: Cell) -> () {
        let Position { row, col } = cell.get_coord();
        if let Some(selected_pos) = self.selected_piece {
            if self.get_cell(selected_pos).get_piece().is_some()
                && self.get_cell(selected_pos).get_piece_kind() != Kind::None
                && self.shift.get_possible_moves().is_empty()
            {
                self.shift.set_possible_moves(
                    self.clone(),
                    *self.get_cell(Position {
                        row: selected_pos.row,
                        col: selected_pos.col,
                    }),
                );
            }

            let possible_moves: Vec<Position> = self.shift.get_possible_moves();

            if possible_moves.contains(&cell.get_coord()) {
                self.move_piece(selected_pos, cell.get_coord());
                self.selected_piece = None;
                self.print_board();
                self.next_turn();
            } else {
                if let Some(piece) = self.get_cell(Position { row, col }).get_piece() {
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
            if let Some(piece) = self.get_cell(Position { row, col }).get_piece() {
                if piece.get_kind() != Kind::None && piece.get_color() == self.color_turn {
                    self.select_new_piece(cell);
                }
            } else {
                self.select_new_piece(cell);
            }
        }
    }

    fn promote(&mut self, position: Position) -> () {
        self.board[position.row][position.col].piece = Piece::create(Kind::Queen, self.color_turn);
    }

    pub fn print_board(&self) {
        let mut board_string: String = String::new();
        for row in &self.board {
            for cell in row {
                let piece_char = match cell.piece.get_kind() {
                    Kind::Pawn => {
                        if cell.piece.get_color() == Color::White {
                            'P'
                        } else {
                            'p'
                        }
                    }
                    Kind::Knight => {
                        if cell.piece.get_color() == Color::White {
                            'N'
                        } else {
                            'n'
                        }
                    }
                    Kind::Bishop => {
                        if cell.piece.get_color() == Color::White {
                            'B'
                        } else {
                            'b'
                        }
                    }
                    Kind::Rook => {
                        if cell.piece.get_color() == Color::White {
                            'R'
                        } else {
                            'r'
                        }
                    }
                    Kind::Queen => {
                        if cell.piece.get_color() == Color::White {
                            'Q'
                        } else {
                            'q'
                        }
                    }
                    Kind::King => {
                        if cell.piece.get_color() == Color::White {
                            'K'
                        } else {
                            'k'
                        }
                    }
                    Kind::None => '.',
                };
                board_string.push(piece_char);
                board_string.push(' ');
            }
            board_string.push('\n');
        }

        web_sys::console::log_1(&board_string.into());
    }

    fn display_king_in_check(&mut self, position: Position) -> () {
        self.board[position.row][position.col].is_check = true;
    }

    fn get_position_king(&self, color: Color) -> Option<Position> {
        let size: usize = self.get_size();
        for row in 0..size {
            for col in 0..size {
                if let Some(piece) = self.board[row][col].get_piece() {
                    if piece.get_kind() == Kind::King && piece.get_color() == color {
                        return Some(Position { row, col });
                    }
                }
            }
        }
        None
    }

    fn is_king_in_check(&self, position: Position) -> bool {
        self.shift.get_possible_checks().contains(&position)
    }
}
