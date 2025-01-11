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
                    piece: Piece::from_symbol(char),
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

    pub fn handle_click(&mut self, cell: Cell) -> () {
        let position = cell.get_coord();

        if let Some(selected_pos) = self.selected_piece {
            if self.is_valid_move(selected_pos, position) {
                self.move_piece(selected_pos, position);
                self.next_turn();
            } else {
                self.handle_selection(cell);
            }
        } else {
            self.handle_selection(cell);
        }
    }

    fn clear(&mut self) -> () {
        for r in 0..self.size {
            for c in 0..self.size {
                self.board[r][c].is_selected = false;
                self.board[r][c].is_check = false;
            }
        }
    }

    fn to_chess_notation(&self, position: Position) -> (char, usize) {
        let char_index: char = match position.col {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => '?',
        };
        let usize_index: usize = match position.row {
            1 => 7,
            2 => 6,
            3 => 5,
            4 => 4,
            5 => 3,
            6 => 2,
            7 => 1,
            _ => 0,
        };

        (char_index, usize_index)
    }

    fn print_notation(&self, from: Position, to: Position) -> () {
        let index_position: (char, usize) = self.to_chess_notation(to);

        let is_attack: bool = self.get_cell(to).get_piece().is_some();
        // let is_check: bool = {
        //     let mut temp_board: Board = self.clone();
        //     temp_board.move_piece(from, to);
        //     let king_position: Option<Position> = temp_board.get_position_king(!self.color_turn);
        //     if let Some(king_pos) = king_position {
        //         temp_board.is_king_in_check(king_pos)
        //     } else {
        //         false
        //     }
        // };
        let piece_symbol: char = self.get_cell(from).get_piece().unwrap().get_symbol();

        let move_str: String = if is_attack {
            format!("{}x{}{}", piece_symbol, index_position.0, index_position.1)
        }
        //  else if is_check {
        //     format!("{}{}{}+", piece_symbol, index_position.0, index_position.1)
        // }
        else {
            format!("{}{}{}", piece_symbol, index_position.0, index_position.1)
        };
        web_sys::console::log_1(&move_str.into());
    }

    fn move_piece(&mut self, from: Position, to: Position) -> () {
        let Position {
            row: old_row,
            col: old_col,
        } = from;
        let Position {
            row: new_row,
            col: new_col,
        } = to;

        self.print_notation(from, to);
        self.board[new_row][new_col].piece = self.board[old_row][old_col].piece.clone();
        self.board[old_row][old_col].piece = Piece::none();
        self.selected_piece = None;

        self.clear();
        self.check_king_status();

        if self.board[new_row][new_col].piece.get_kind() == Kind::Pawn {
            let color: Color = self.board[new_row][new_col].get_piece_color();
            if new_row == 0 && color == Color::White {
                self.promote(to);
            } else if new_row == self.size - 1 && color == Color::Black {
                self.promote(to);
            }
        }
    }

    fn select_new_piece(&mut self, cell: Cell) -> () {
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
        self.color_turn = !self.color_turn;
        web_sys::console::log_1(&format!("Next turn").into());
    }

    fn promote(&mut self, position: Position) -> () {
        self.board[position.row][position.col].piece = Piece::create(Kind::Queen, self.color_turn);
    }

    fn is_valid_move(&self, from: Position, to: Position) -> bool {
        self.get_cell(from).get_piece().is_some()
            && self.get_cell(from).get_piece_kind() != Kind::None
            && self.shift.get_possible_moves().contains(&to)
    }

    fn handle_selection(&mut self, cell: Cell) -> () {
        let position = cell.get_coord();

        if let Some(piece) = self.get_cell(position).get_piece() {
            if piece.get_kind() != Kind::None && piece.get_color() == self.color_turn {
                self.select_new_piece(cell);
            } else {
                self.selected_piece = None;
            }
        } else {
            self.selected_piece = None;
        }
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

    fn check_king_status(&mut self) -> () {
        let opposant_color: Color = !self.color_turn;
        self.shift.set_possible_checks(self.clone(), opposant_color);
        let position_king: Option<Position> = self.get_position_king(opposant_color);

        if let Some(position_king) = position_king {
            if self.is_king_in_check(position_king) {
                self.display_king_in_check(position_king);
                web_sys::console::log_1(&"King is in check".into());
            }
        }
    }
}
