use std::usize;

use crate::board::Cell;
use crate::piece::{Color, Kind, Piece};

pub fn get_possible_moves(board: &Vec<Vec<Cell>>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let piece: Piece = board[row][col].get_piece().unwrap();
    let kind: Kind = piece.get_kind();
    let color: Color = piece.get_color();
    match kind {
        Kind::Pawn => get_pawn_possible_moves(board, row, col),
        Kind::Knight => get_knight_possible_moves(board, row, col),
        Kind::Bishop => get_bishop_possible_moves(board, row, col),
        Kind::Rook => get_rook_possible_moves(board, row, col),
        Kind::Queen => get_queen_possible_moves(board, row, col),
        Kind::King => get_king_possible_moves(board, row, col),
        Kind::None => Vec::new(),
    }
}

fn is_piece_there(board: &Vec<Vec<Cell>>, row: usize, col: usize, color: Color) -> bool {
    board[row][col].get_piece().is_some()
        && board[row][col].get_piece().unwrap().get_color() == color
}

fn is_illegal_move(board: &Vec<Vec<Cell>>, row: usize, col: usize, color: Color) -> bool {
    let size: usize = board.len();
    for (r, c) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    {
        let new_row: isize = row as isize + r;
        let new_col: isize = col as isize + c;
        if new_row >= 0 && new_col >= 0 && new_row < size as isize && new_col < size as isize {
            let new_row: usize = new_row as usize;
            let new_col: usize = new_col as usize;
            if let Some(piece) = board[new_row][new_col].get_piece() {
                if piece.get_color() != color {
                    match piece.get_kind() {
                        Kind::Pawn => {
                            if (color == Color::White && r == &-1 && (c == &-1 || c == &1))
                                || (color == Color::Black && r == &1 && (c == &-1 || c == &1))
                            {
                                return true;
                            }
                        }
                        Kind::Knight => {
                            if [
                                (-2, -1),
                                (-1, -2),
                                (1, -2),
                                (2, -1),
                                (2, 1),
                                (1, 2),
                                (-1, 2),
                                (-2, 1),
                            ]
                            .contains(&(*r, *c))
                            {
                                return true;
                            }
                        }
                        Kind::Bishop => {
                            if r.abs() == c.abs() {
                                return true;
                            }
                        }
                        Kind::Rook => {
                            if r == &0 || c == &0 {
                                return true;
                            }
                        }
                        Kind::Queen => {
                            if r.abs() == c.abs() || r == &0 || c == &0 {
                                return true;
                            }
                        }
                        Kind::King => {
                            return true;
                        }
                        Kind::None => {}
                    }
                }
            }
        }
    }

    false
}

// TODO: add en passant
fn get_pawn_possible_moves(board: &Vec<Vec<Cell>>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut possible_moves: Vec<(usize, usize)> = Vec::new();
    let color: Color = board[row][col].get_piece().unwrap().get_color();
    let size: usize = board.len();

    match color {
        Color::White => {
            // first move
            if row == 6
                && col < size
                && !is_piece_there(board, row - 1, col, Color::White)
                && !is_piece_there(board, row - 2, col, Color::White)
                && !is_piece_there(board, row - 1, col, Color::Black)
                && !is_piece_there(board, row - 2, col, Color::Black)
            {
                possible_moves.push((row - 2, col));
            }
            // forward
            if row > 0
                && !is_piece_there(board, row - 1, col, Color::White)
                && !is_piece_there(board, row - 1, col, Color::Black)
            {
                possible_moves.push((row - 1, col));
            }
            // attack left
            if row > 0
                && col > 0
                && !is_piece_there(board, row - 1, col - 1, Color::White)
                && board[row - 1][col - 1].get_piece().is_some()
                && board[row - 1][col - 1].get_piece().unwrap().get_color() == Color::Black
            {
                possible_moves.push((row - 1, col - 1));
            }
            // attack right
            if row > 0
                && col + 1 < size
                && !is_piece_there(board, row - 1, col + 1, Color::White)
                && board[row - 1][col + 1].get_piece().is_some()
                && board[row - 1][col + 1].get_piece().unwrap().get_color() == Color::Black
            {
                possible_moves.push((row - 1, col + 1));
            }
        }
        Color::Black => {
            // first move
            if row == 1
                && !is_piece_there(board, row + 1, col, Color::White)
                && !is_piece_there(board, row + 1, col, Color::Black)
                && !is_piece_there(board, row + 2, col, Color::White)
                && !is_piece_there(board, row + 2, col, Color::Black)
            {
                possible_moves.push((row + 2, col));
            }
            // forward
            if row + 1 < size
                && !is_piece_there(board, row + 1, col, Color::White)
                && !is_piece_there(board, row + 1, col, Color::Black)
            {
                possible_moves.push((row + 1, col));
            }
            // attack left
            if row > 0
                && col > 0
                && !is_piece_there(board, row + 1, col - 1, Color::Black)
                && board[row + 1][col - 1].get_piece().is_some()
                && board[row + 1][col - 1].get_piece().unwrap().get_color() == Color::White
            {
                possible_moves.push((row + 1, col - 1));
            }
            // attack right
            if row > 0
                && col + 1 < size
                && !is_piece_there(board, row + 1, col + 1, Color::Black)
                && board[row + 1][col + 1].get_piece().is_some()
                && board[row + 1][col + 1].get_piece().unwrap().get_color() == Color::White
            {
                possible_moves.push((row + 1, col + 1));
            }
        }
    }

    possible_moves
}

fn get_knight_possible_moves(
    board: &Vec<Vec<Cell>>,
    row: usize,
    col: usize,
) -> Vec<(usize, usize)> {
    let mut possible_moves: Vec<(usize, usize)> = Vec::new();
    let size: usize = board.len();
    let color: Color = board[row][col].get_piece().unwrap().get_color();

    for (r, c) in [
        (-1, -2),
        (1, -2),
        (-1, 2),
        (1, 2),
        (-2, -1),
        (2, -1),
        (-2, 1),
        (2, 1),
    ]
    .iter()
    {
        let new_row: isize = row as isize + *r;
        let new_col: isize = col as isize + *c;
        if new_row >= 0
            && new_row < size as isize
            && new_col >= 0
            && new_col < size as isize
            && !is_piece_there(board, new_row as usize, new_col as usize, color)
        {
            possible_moves.push((new_row as usize, new_col as usize));
        }
    }

    possible_moves
}

fn get_bishop_possible_moves(
    board: &Vec<Vec<Cell>>,
    row: usize,
    col: usize,
) -> Vec<(usize, usize)> {
    let mut possible_moves: Vec<(usize, usize)> = Vec::new();
    let size: usize = board.len();
    let color: Color = board[row][col].get_piece().unwrap().get_color();

    // top-left
    for i in 1..=row.min(col) {
        if is_piece_there(board, row - i, col - i, color) {
            break;
        } else {
            possible_moves.push((row - i, col - i));
            if board[row - i][col - i].get_piece().is_some() {
                break;
            }
        }
    }

    // top-right
    for i in 1..=row.min(size - 1 - col) {
        if is_piece_there(board, row - i, col + i, color) {
            break;
        } else {
            possible_moves.push((row - i, col + i));
            if board[row - i][col + i].get_piece().is_some() {
                break;
            }
        }
    }

    // bottom-left
    for i in 1..=(size - 1 - row).min(col) {
        if is_piece_there(board, row + i, col - i, color) {
            break;
        } else {
            possible_moves.push((row + i, col - i));
            if board[row + i][col - i].get_piece().is_some() {
                break;
            }
        }
    }

    // bottom-right
    for i in 1..=(size - 1 - row).min(size - 1 - col) {
        if is_piece_there(board, row + i, col + i, color) {
            break;
        } else {
            possible_moves.push((row + i, col + i));
            if board[row + i][col + i].get_piece().is_some() {
                break;
            }
        }
    }

    possible_moves
}

fn get_rook_possible_moves(board: &Vec<Vec<Cell>>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut possible_moves: Vec<(usize, usize)> = Vec::new();
    let size: usize = board.len();
    let color: Color = board[row][col].get_piece().unwrap().get_color();

    // up
    for r in (0..row).rev() {
        if is_piece_there(board, r, col, color) {
            break;
        } else {
            possible_moves.push((r, col));
            if board[r][col].get_piece().is_some() {
                break;
            }
        }
    }

    // down
    for r in row + 1..size {
        if is_piece_there(board, r, col, color) {
            break;
        } else {
            possible_moves.push((r, col));
            if board[r][col].get_piece().is_some() {
                break;
            }
        }
    }

    // left
    for c in (0..col).rev() {
        if is_piece_there(board, row, c, color) {
            break;
        } else {
            possible_moves.push((row, c));
            if board[row][c].get_piece().is_some() {
                break;
            }
        }
    }

    // right
    for c in col + 1..size {
        if is_piece_there(board, row, c, color) {
            break;
        } else {
            possible_moves.push((row, c));
            if board[row][c].get_piece().is_some() {
                break;
            }
        }
    }

    possible_moves
}

fn get_queen_possible_moves(board: &Vec<Vec<Cell>>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut possible_moves: Vec<(usize, usize)> = Vec::new();

    for (r, c) in get_bishop_possible_moves(board, row, col).iter() {
        possible_moves.push((*r, *c));
    }

    for (r, c) in get_rook_possible_moves(board, row, col).iter() {
        possible_moves.push((*r, *c));
    }

    possible_moves
}

// TODO: add castling
fn get_king_possible_moves(board: &Vec<Vec<Cell>>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut possible_moves: Vec<(usize, usize)> = Vec::new();
    let size: usize = board.len();
    let color: Color = board[row][col].get_piece().unwrap().get_color();

    for (r, c) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    {
        let new_row: isize = row as isize + *r;
        let new_col: isize = col as isize + *c;
        if new_row >= 0
            && new_row < size as isize
            && new_col >= 0
            && new_col < size as isize
            && !is_piece_there(board, new_row as usize, new_col as usize, color)
            && !is_illegal_move(board, new_row as usize, new_col as usize, color)
        {
            possible_moves.push((new_row as usize, new_col as usize));
        }
    }

    possible_moves
}

// TODO: add promoting
fn promoting(board: &Vec<Vec<Cell>>, row: usize, col: usize) -> Piece {
    Piece::none()
}
