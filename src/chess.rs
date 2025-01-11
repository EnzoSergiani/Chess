use crate::{board::Board, position::Position};
use yew::prelude::*;

#[function_component(Chess)]
pub fn game() -> Html {
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

    let on_click: Callback<Position> = {
        let board: UseStateHandle<Board> = board.clone();
        Callback::from(move |pos: Position| {
            let mut new_board: Board = (*board).clone();
            new_board.handle_click(*board.get_cell(pos));
            board.set(new_board);
        })
    };

    html! {
        <div>
            {board.render(on_click)}
        </div>
    }
}
