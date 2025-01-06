use yew::prelude::*;

use crate::piece::{Color, Kind, Piece};
use crate::r#move::get_possible_moves;

#[derive(Clone)]
pub struct Cell {
    color: Color,
    piece: Piece,
    is_selected: bool,
    is_threat: bool,
}

#[derive(Clone)]
pub struct Board {
    board: Vec<Vec<Cell>>,
    pub selected_piece: Option<(usize, usize)>,
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
                    is_threat: false,
                });
            }

            board.push(row);
        }

        Board {
            board,
            selected_piece: None,
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
        // self.load_from_fen("8/8/8/3q4/3PP3/8/8");
        self
    }

    pub fn render(&self, on_click: Callback<(usize, usize)>) -> Html {
        html! {
            <div class={classes!("board-border")}>
                <div class={classes!("board")}>
                    {for self.board.iter().enumerate().map(|(row_idx, row)| {
                        html! {
                            <div class="row">
                                {for row.iter().enumerate().map(|(col_idx, cell)| {
                                    let on_click = {
                                        let on_click = on_click.clone();
                                        Callback::from(move |_| on_click.emit((row_idx, col_idx)))
                                    };
                                    html! {
                                        <div class={
                                            classes!(
                                                if cell.is_selected { "cell_move" } else { "" },
                                                if cell.is_threat { "cell_threat" } else { "" },
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

    //TODO: need to refactor
    pub fn handle_click(&mut self, row: usize, col: usize) {
        let size: usize = self.board.len();
        // clear previous selections
        if let Some((selected_row, selected_col)) = self.selected_piece {
            if selected_row != row || selected_col != col {
                for r in 0..size {
                    for c in 0..size {
                        self.board[r][c].is_selected = false;
                    }
                }
            }
        }

        if let Some((selected_row, selected_col)) = self.selected_piece {
            let possible_moves: Vec<(usize, usize)> =
                get_possible_moves(&self.board, selected_row, selected_col);

            if possible_moves.contains(&(row, col)) {
                // Move the piece
                self.board[row][col].piece = self.board[selected_row][selected_col].piece;
                self.board[selected_row][selected_col].piece = Piece::none();
                self.selected_piece = None;
            } else {
                // Select a new piece
                if self.board[row][col].piece.get_kind() != Kind::None {
                    self.selected_piece = Some((row, col));

                    // displays possible movements
                    let possible_moves: Vec<(usize, usize)> =
                        get_possible_moves(&self.board, row, col);
                    for (r, c) in possible_moves.iter() {
                        self.board[*r][*c].is_selected = true;
                    }
                }
            }
        } else {
            // Select a new piece
            if self.board[row][col].piece.get_kind() != Kind::None {
                self.selected_piece = Some((row, col));
                // displays possible movements
                let possible_moves: Vec<(usize, usize)> = get_possible_moves(&self.board, row, col);
                for (r, c) in possible_moves.iter() {
                    self.board[*r][*c].is_selected = true;
                }
            }
        }
    }
}

impl Cell {
    pub fn get_piece(&self) -> Option<Piece> {
        if self.piece.get_kind() != Kind::None {
            Some(self.piece.clone())
        } else {
            None
        }
    }
}
