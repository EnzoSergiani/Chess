mod board;
mod piece;

use crate::board::Board;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let mut board = Board::new();
    board.initialize();

    html! {
        <div>
            {board.render()}
        </div>
    }
}
fn main() {
    yew::start_app::<App>();
}
