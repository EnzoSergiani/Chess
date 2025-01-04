mod board;
mod piece;
mod shift;

use crate::board::Board;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let board: UseStateHandle<Board> = use_state(|| Board::new());

    {
        let board: UseStateHandle<Board> = board.clone();
        use_effect_with_deps(
            move |_| {
                board.set(Board::new().initialize());
                || ()
            },
            (),
        );
    }

    let on_click: Callback<(usize, usize)> = {
        let board: UseStateHandle<Board> = board.clone();
        Callback::from(move |(row, col): (usize, usize)| {
            let mut new_board = (*board).clone();
            new_board.handle_click(row, col);
            board.set(new_board);
        })
    };

    html! {
        <div>
            {board.render(on_click)}
        </div>
    }
}
fn main() {
    yew::start_app::<App>();
}
