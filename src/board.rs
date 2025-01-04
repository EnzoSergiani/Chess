use yew::prelude::*;

use crate::piece::{Color, Kind, Piece};

static SIZE: usize = 8;

#[derive(Clone)]
struct Cell {
    color: Color,
    piece: Piece,
}

#[derive(Clone)]
pub struct Board {
    board: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new() -> Board {
        let mut board: Vec<Vec<Cell>> = Vec::new();

        for row_idx in 0..SIZE {
            let mut row: Vec<Cell> = Vec::new();

            for col_idx in 0..SIZE {
                let color: Color = if (row_idx + col_idx) % 2 == 0 {
                    Color::White
                } else {
                    Color::Black
                };

                row.push(Cell {
                    color,
                    piece: Piece::none(),
                });
            }

            board.push(row);
        }

        Board { board }
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
        self.load_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        self
    }

    pub fn render(&self, on_click: Callback<(usize, usize)>) -> Html {
        html! {
            <div class={classes!("board-border")}>
                <div class={classes!("board")}>
                    {for self.board.iter().map(|row| {
                        html! {
                            <div class="row">
                                {for row.iter().map(|cell| {
                                    html! {
                                        <div class={if cell.color == Color::White { "cell_white" } else { "cell_black" }}>
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
}
