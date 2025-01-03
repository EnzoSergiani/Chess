mod board;
mod piece;

use crate::board::Board;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let board: Board = Board::new();
    board.render()
}

fn main() {
    yew::start_app::<App>();
}
