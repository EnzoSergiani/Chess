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
    /// Indicates whether the game has ended.
    is_end: bool,
    /// A vector of strings representing the move notations.
    notations: Vec<String>,
    /// Points scored by the white player.
    white_score: u8,
    /// Points scored by the black player.
    black_score: u8,
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
            is_end: false,
            notations: Vec::new(),
            white_score: 0,
            black_score: 0,
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
        if !self.is_end {
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
        } else {
            self.handle_selection(cell);
        }
    }

    /// Advances to the next turn.
    fn next_turn(&mut self) -> () {
        if !self.is_end {
            self.color_turn = !self.color_turn;
            web_sys::console::log_1(&format!("Next turn").into());
        }
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
        self.add_notation(self.get_chess_notation(from, to));
        let piece: Piece = self.board[from.get_row()][from.get_col()]
            .get_piece()
            .unwrap()
            .clone();

        if let Some(piece_captured) = self.board[to.get_row()][to.get_col()].get_piece() {
            self.update_points(piece_captured);
        }

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

    /// Checks the status of the king and updates the board if the king is in check.
    fn check_king_status(&mut self) -> () {
        let opposant_color: Color = !self.color_turn;
        let position_king: Option<Position> = self.get_position_king(opposant_color);

        if let Some(position_king) = position_king {
            if self.is_king_in_check(position_king) {
                self.display_king_in_check(position_king);

                if self.is_king_in_check_mate(position_king) {
                    web_sys::console::log_1(&"King is in check mate".into());
                    self.win();
                    return;
                }

                web_sys::console::log_1(&"King is in check".into());
            }
        }
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
        self.clone()
            .shift
            .is_in_check(self, position, !self.color_turn)
    }

    /// Checks if the king is in checkmate at the given position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the king to check.
    ///
    /// # Returns
    ///
    /// `true` if the king is in checkmate, `false` otherwise.
    fn is_king_in_check_mate(&self, position: Position) -> bool {
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

        self.is_king_in_check(position)
            && king_moves.iter().all(|(dx, dy)| {
                let new_row: isize = position.get_row() as isize + dx;
                let new_col: isize = position.get_col() as isize + dy;
                if new_row >= 0
                    && new_row < self.size as isize
                    && new_col >= 0
                    && new_col < self.size as isize
                {
                    let new_position: Position = Position::new(new_row as usize, new_col as usize);
                    self.is_valid_move(position, new_position)
                } else {
                    true
                }
            })
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

    /// Converts a move from one position to another into standard chess notation.
    ///
    /// # Arguments
    ///
    /// * `from` - The starting position of the piece.
    /// * `to` - The ending position of the piece.
    ///
    /// # Returns
    ///
    /// A `String` representing the move in standard chess notation.
    fn get_chess_notation(&self, from: Position, to: Position) -> String {
        let is_attack: bool = self.get_cell(to).get_piece().is_some();
        let piece_symbol: char = self.get_cell(from).get_piece().unwrap().get_symbol();
        let position_king: Option<Position> = self.get_position_king(!self.color_turn);
        let (row, col) = self.convert_index_to_notation(to);
        let (is_check, is_check_mate) = if let Some(position_king) = position_king {
            let is_check: bool = self.is_king_in_check(position_king);
            let is_check_mate: bool = self.is_king_in_check_mate(position_king);
            (is_check, is_check_mate)
        } else {
            (false, false)
        };

        let symbol: char = if is_check_mate {
            '#'
        } else if is_check {
            '+'
        } else if is_attack {
            'x'
        } else {
            ' '
        };
        web_sys::console::log_1(&format!("{}{}{}{}", symbol, piece_symbol, row, col).into());
        format!("{}{}{}{}", symbol, piece_symbol, row, col)
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
    fn convert_index_to_notation(&self, position: Position) -> (char, usize) {
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
            0 => 8,
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

    /// Adds a move notation to the list of notations.
    ///
    /// # Arguments
    ///
    /// * `notation` - The notation of the move to add.
    pub fn add_notation(&mut self, notation: String) -> () {
        self.notations.push(notation);
    }

    /// Updates the points for the player based on the captured piece.
    ///
    /// # Arguments
    ///
    /// * `piece` - The piece that was captured.
    fn update_points(&mut self, piece: Piece) {
        match piece.get_color() {
            Color::White => {
                if let Some(value) = piece.get_value() {
                    self.black_score += value
                }
            }
            Color::Black => {
                if let Some(value) = piece.get_value() {
                    self.white_score += value
                }
            }
        }
    }

    /// Ends the game and logs the end of the game.
    fn win(&mut self) -> () {
        self.is_end = true;
        web_sys::console::log_1(&"End of the game".into());
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
            <div class={classes!("container")}>
                {self.render_win_screen()}
                <div class={classes!("container-board")}>
                    {self.render_board(on_click.clone())}
                </div>
                <div class={classes!("container-data")}>
                    {self.render_score(self.black_score, "score")}
                    {self.render_notation()}
                    {self.render_score(self.white_score, "score")}
                </div>
            </div>
        }
    }

    /// Renders the win screen.
    ///
    /// # Returns
    ///
    /// An `Html` representation of the win screen.
    fn render_win_screen(&self) -> Html {
        html! {
            <div class={classes!(if self.is_end { "win-screen" } else { "win-screen hidden" })}>
                {match self.color_turn {
                    Color::White => html! {
                        <>
                            <p class={classes!("win-screen-text", "win-screen-text-white")}>{"WHITE WON"}</p>
                            <div class={classes!("win-screen-container", "win-screen-container-white")}></div>
                        </>
                    },
                    Color::Black => html! {
                        <>
                            <p class={classes!("win-screen-text", "win-screen-text-black")}>{"BLACK WON"}</p>
                            <div class={classes!("win-screen-container", "win-screen-container-black")}></div>
                        </>
                    },
                }}
            </div>
        }
    }

    /// Renders the board.
    ///
    /// # Arguments
    ///
    /// * `on_click` - A callback function to handle click events on the board cells.
    ///
    /// # Returns
    ///
    /// An `Html` representation of the board.
    fn render_board(&self, on_click: Callback<Position>) -> Html {
        html! {
            <div class={classes!("board")}>
                {for self.board.iter().enumerate().map(|(row_idx, row)| {
                    html! {
                        <div class="row">
                            {for row.iter().enumerate().map(|(col_idx, cell)| {
                                let on_click = {
                                    let on_click = on_click.clone();
                                    Callback::from(move |_| on_click.emit(Position::new(row_idx, col_idx)))
                                };
                                let cell_classes = classes!(
                                    if cell.get_is_selected() { "cell-move" } else { "" },
                                    if cell.get_is_check() { "cell-check" } else { "" },
                                    if cell.get_color() == Color::White { "cell cell-white" } else { "cell cell-black" }
                                );
                                html! {
                                    <div class={cell_classes} onclick={on_click}>
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
        }
    }

    /// Renders the score.
    ///
    /// # Arguments
    ///
    /// * `score` - The score to display.
    ///
    /// # Returns
    ///
    /// An `Html` representation of the score.
    fn render_score(&self, score: u8, label: &str) -> Html {
        html! {
            <div class={classes!("score")}>
                {format!("{} : {}", label, score)}
            </div>
        }
    }

    /// Renders the notation.
    ///
    /// # Returns
    ///
    /// An `Html` representation of the notation.
    fn render_notation(&self) -> Html {
        html! {
            <div class={classes!("notation")}>
                {for self.notations.chunks(2).enumerate().map(|(index, chunk)| {
                    let white_move = chunk.get(0).unwrap_or(&String::new()).clone();
                    let black_move = chunk.get(1).unwrap_or(&String::new()).clone();
                    let color_line  = classes!(if index % 2 == 0 { "notation-line notation-line-white" } else { "notation-line notation-line-black" });
                    html! {
                        <div class={color_line}>
                            <div class={classes!("notation-column")}>{format!("{}",index+1)}</div>
                            <div class={classes!("notation-column")}>{format!("{}",white_move)}</div>
                            <div class={classes!("notation-column")}>{format!("{}",black_move)}</div>
                        </div>
                    }
                })}
            </div>
        }
    }
}
