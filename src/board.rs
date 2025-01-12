use web_sys;

use yew::prelude::*;

use crate::{cell::Cell, color::Color, kind::Kind, piece::Piece, position::Position, shift::Shift};

/// Represents the game board.
#[derive(Clone)]
pub struct Board {
    /// A 2D vector of cells representing the board.
    board: Vec<Vec<Cell>>,
    /// The size of the board (typically 8 for an 8x8 board).
    size: usize,
    /// The currently selected piece, if any.
    selected_piece: Option<Position>,
    /// An instance of `Shift` to manage possible moves and checks.
    shift: Shift,
    /// The color of the player whose turn it is.
    color_turn: Color,
}
impl Board {
    /// Creates a new `Board` instance with an 8x8 grid of cells.
    ///
    /// # Returns
    ///
    /// A new `Board` instance with an initialized 8x8 grid of cells.
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

                row.push(Cell::empty(color, Position::new(row_idx, col_idx)));
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

    /// Initializes the board with the standard chess starting position.
    ///
    /// # Returns
    ///
    /// The `Board` instance initialized with the standard chess starting position.
    pub fn initialize(mut self) -> Self {
        let fen_init: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        self.load_from_fen(fen_init);

        self
    }

    /// Returns a reference to the cell at the given position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the cell to retrieve.
    ///
    /// # Returns
    ///
    /// A reference to the cell at the given position.
    pub fn get_cell(&self, position: Position) -> &Cell {
        &self.board[position.get_row()][position.get_col()]
    }

    /// Returns the size of the board.
    ///
    /// # Returns
    ///
    /// The size of the board.
    pub fn get_size(&self) -> usize {
        self.size
    }

    /// Gets the position of the king of the given color.
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the king to find.
    ///
    /// # Returns
    ///
    /// An `Option<Position>` containing the position of the king if found, or `None` if not found.
    fn get_position_king(&self, color: Color) -> Option<Position> {
        for row in 0..self.size {
            for col in 0..self.size {
                if let Some(piece) = self.board[row][col].get_piece() {
                    if piece.get_kind() == Kind::King && piece.get_color() == color {
                        return Some(Position::new(row, col));
                    }
                }
            }
        }
        None
    }

    /// Loads the board state from a FEN (Forsyth-Edwards Notation) string.
    ///
    /// # Arguments
    ///
    /// * `fen` - A string slice representing the board state in FEN format.
    fn load_from_fen(&mut self, fen: &str) -> () {
        let mut row: usize = 0;
        let mut col: usize = 0;

        for char in fen.chars() {
            if char.is_digit(10) {
                col += char.to_digit(10).unwrap() as usize;
            }
            if char.is_alphabetic() {
                self.board[row][col] = Cell::create(
                    Piece::from_symbol(char),
                    self.board[row][col].get_color(),
                    Position::new(row, col),
                );
                col += 1;
            }
            if char == '/' {
                row += 1;
                col = 0;
            }
        }
    }

    /// Handles a click event on a cell.
    ///
    /// # Arguments
    ///
    /// * `cell` - The cell that was clicked.
    pub fn handle_click(&mut self, cell: Cell) -> () {
        if let Some(selected_pos) = self.selected_piece {
            let (new_position_row, new_position_col): (usize, usize) = cell.get_position();
            let new_position: Position = Position::new(new_position_row, new_position_col);
            if self.is_valid_move(selected_pos, new_position) {
                self.move_piece(selected_pos, new_position);
                self.next_turn();
            } else {
                self.handle_selection(cell);
            }
        } else {
            self.handle_selection(cell);
        }
    }

    /// Advances to the next turn.
    fn next_turn(&mut self) -> () {
        self.color_turn = !self.color_turn;
        web_sys::console::log_1(&format!("Next turn").into());
    }

    /// Handles the selection of a cell.
    ///
    /// # Arguments
    ///
    /// * `cell` - The cell that was selected.
    fn handle_selection(&mut self, cell: Cell) -> () {
        let (row, col) = cell.get_position();
        let position: Position = Position::new(row, col);
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

    /// Selects a new piece on the board.
    ///
    /// # Arguments
    ///
    /// * `cell` - The cell containing the piece to select.
    fn select_new_piece(&mut self, cell: Cell) -> () {
        let (row, col) = cell.get_position();
        let position: Position = Position::new(row, col);
        self.selected_piece = Some(position);
        self.shift.set_possible_moves(self.clone(), cell);

        self.clear();
        self.display_possible_moves();
    }

    /// Moves a piece from one position to another on the board.
    ///
    /// # Arguments
    ///
    /// * `from` - The starting position of the piece.
    /// * `to` - The ending position of the piece.
    fn move_piece(&mut self, from: Position, to: Position) -> () {
        self.print_notation(from, to);
        let piece: Piece = self.board[from.get_row()][from.get_col()]
            .get_piece()
            .unwrap()
            .clone();

        self.board[to.get_row()][to.get_col()].set_piece(piece);
        self.board[from.get_row()][from.get_col()].set_piece(Piece::none());
        self.selected_piece = None;

        self.clear();
        self.check_king_status();
        self.check_promote(to);
    }

    /// Checks if a move from one position to another is valid.
    ///
    /// # Arguments
    ///
    /// * `from` - The starting position of the piece.
    /// * `to` - The ending position of the piece.
    ///
    /// # Returns
    ///
    /// `true` if the move is valid, `false` otherwise.
    fn is_valid_move(&self, from: Position, to: Position) -> bool {
        self.get_cell(from).get_piece().is_some()
            && self.get_cell(from).get_piece_kind() != Kind::None
            && self.shift.get_possible_moves().contains(&to)
    }

    /// Checks if the king is in check at the given position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the king to check.
    ///
    /// # Returns
    ///
    /// `true` if the king is in check, `false` otherwise.
    fn is_king_in_check(&self, position: Position) -> bool {
        self.shift.get_possible_checks().contains(&position)
    }

    /// Checks the status of the king and updates the board if the king is in check.
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

    /// Checks if a pawn should be promoted and promotes it if necessary.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the pawn to check for promotion.
    fn check_promote(&mut self, position: Position) {
        if self.board[position.get_row()][position.get_col()].get_piece_kind() == Kind::Pawn {
            let color: Color = self.board[position.get_row()][position.get_col()].get_piece_color();
            if position.get_row() == 0 && color == Color::White {
                self.promote(position);
            } else if position.get_row() == self.size - 1 && color == Color::Black {
                self.promote(position);
            }
        }
    }

    /// Promotes a pawn to a queen at the given position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the pawn to promote.
    fn promote(&mut self, position: Position) -> () {
        self.board[position.get_row()][position.get_col()]
            .set_piece(Piece::create(Kind::Queen, self.color_turn));
    }

    /// Clears the selection and check status of all cells on the board.
    fn clear(&mut self) -> () {
        for r in 0..self.size {
            for c in 0..self.size {
                self.board[r][c].set_is_selected(false);
                self.board[r][c].set_is_check(false);
            }
        }
    }

    /// Displays the possible moves for the selected piece.
    fn display_possible_moves(&mut self) -> () {
        for pos in self.shift.get_possible_moves().iter() {
            self.board[pos.get_row()][pos.get_col()].set_is_selected(true);
        }
    }

    /// Displays the king in check by setting the `is_check` flag on the king's cell.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the king in check.
    fn display_king_in_check(&mut self, position: Position) -> () {
        self.board[position.get_row()][position.get_col()].set_is_check(true);
    }

    /// Converts a board position to a chess notation index.
    ///
    /// # Arguments
    ///
    /// * `position` - The position on the board to convert.
    ///
    /// # Returns
    ///
    /// A tuple containing the column as a character ('a' to 'h') and the row as a usize (1 to 8).
    fn to_chess_notation(&self, position: Position) -> (char, usize) {
        let char_index: char = match position.get_col() {
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
        let usize_index: usize = match position.get_row() {
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

    /// Prints the chess notation for a move from one position to another.
    ///
    /// # Arguments
    ///
    /// * `from` - The starting position of the piece.
    /// * `to` - The ending position of the piece.
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

    /// Renders the board as HTML.
    ///
    /// # Arguments
    ///
    /// * `on_click` - A callback function to handle click events on the board cells.
    ///
    /// # Returns
    ///
    /// An `Html` representation of the board.
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
                                        Callback::from(move |_| on_click.emit(Position::new(row_idx, col_idx)))
                                    };
                                    html! {
                                        <div class={
                                            classes!(
                                                if cell.get_is_selected() { "cell_move" } else { "" },
                                                if cell.get_is_check() { "cell_check" } else { "" },
                                                if cell.get_color() == Color::White { "cell cell_white" } else { "cell cell_black" }
                                            )
                                        } onclick={on_click}>
                                            if cell.get_piece().is_some() {
                                                <img src={cell.get_piece().unwrap().get_svg()} height="60px" />
                                            }
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
}
