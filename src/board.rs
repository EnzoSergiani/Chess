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

    pub fn display(&self) {
        for row in &self.board {
            for piece in row {
                print!("{}", piece);
            }
            println!();
        }
    }

    pub fn get_board(&self) -> &Vec<Vec<Piece>> {
        &self.board
    }
}
