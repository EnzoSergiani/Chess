use std::vec;

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

    pub fn initialize(&mut self) {
        for (row_idx, row) in self.board.iter_mut().enumerate() {
            for (col_idx, cell) in row.iter_mut().enumerate() {
                cell.piece = if row_idx == 1 {
                    Piece::create(Kind::Pawn, Color::Black)
                } else if row_idx == 6 {
                    Piece::create(Kind::Pawn, Color::White)
                } else if row_idx == 0 || row_idx == 7 {
                    match col_idx {
                        0 | 7 => Piece::create(
                            Kind::Rook,
                            if row_idx == 0 {
                                Color::Black
                            } else {
                                Color::White
                            },
                        ),
                        1 | 6 => Piece::create(
                            Kind::Knight,
                            if row_idx == 0 {
                                Color::Black
                            } else {
                                Color::White
                            },
                        ),
                        2 | 5 => Piece::create(
                            Kind::Bishop,
                            if row_idx == 0 {
                                Color::Black
                            } else {
                                Color::White
                            },
                        ),
                        3 => Piece::create(
                            Kind::Queen,
                            if row_idx == 0 {
                                Color::Black
                            } else {
                                Color::White
                            },
                        ),
                        4 => Piece::create(
                            Kind::King,
                            if row_idx == 0 {
                                Color::Black
                            } else {
                                Color::White
                            },
                        ),
                        _ => Piece::create(Kind::None, Color::White),
                    }
                } else {
                    Piece::none()
                };
            }
        }
    }

    pub fn render(&self) -> Html {
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
