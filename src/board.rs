use yew::prelude::*;

use crate::piece::{Color, Kind, Piece};

static SIZE: usize = 8;

pub struct Board {
    board: Vec<Vec<Piece>>,
}

impl Board {
    pub fn new() -> Board {
        let mut board: Vec<Vec<Piece>> = Vec::new();

        for row_index in 0..SIZE {
            let mut row: Vec<Piece> = Vec::new();

            for col_index in 0..SIZE {
                if row_index == 1 {
                    row.push(Piece::create(Kind::Pawn, Color::Black));
                } else if row_index == 6 {
                    row.push(Piece::create(Kind::Pawn, Color::White));
                } else if row_index == 0 || row_index == 7 {
                    match col_index {
                        0 | 7 => row.push(Piece::create(
                            Kind::Rook,
                            if row_index == 0 {
                                Color::Black
                            } else {
                                Color::White
                            },
                        )),
                        1 | 6 => row.push(Piece::create(
                            Kind::Knight,
                            if row_index == 0 {
                                Color::Black
                            } else {
                                Color::White
                            },
                        )),
                        2 | 5 => row.push(Piece::create(
                            Kind::Bishop,
                            if row_index == 0 {
                                Color::Black
                            } else {
                                Color::White
                            },
                        )),
                        3 => row.push(Piece::create(
                            Kind::Queen,
                            if row_index == 0 {
                                Color::Black
                            } else {
                                Color::White
                            },
                        )),
                        4 => row.push(Piece::create(
                            Kind::King,
                            if row_index == 0 {
                                Color::Black
                            } else {
                                Color::White
                            },
                        )),
                        _ => {}
                    }
                } else {
                    row.push(Piece::create(Kind::None, Color::White));
                }
            }

            board.push(row);
        }

        Board { board }
    }

    pub fn render(&self) -> Html {
        html! {
            <div class={classes!("board-border")}>
                <div class={classes!("board")}>
                    {for self.board.iter().enumerate().map(|(row_idx, row)| {
                        html! {
                            <div class="row">
                                {for row.iter().enumerate().map(|(col_idx, piece)| {
                                    let is_white_cell = (row_idx + col_idx) % 2 == 0;
                                    html! {
                                        <div class={if is_white_cell { "cell_white" } else { "cell_black" }}>
                                            <img src={piece.get_svg()} height="60px"  />
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
