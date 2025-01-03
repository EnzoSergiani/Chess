mod board;
mod piece;

use crate::board::Board;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let board: UseStateHandle<Board> = use_state(|| Board::new());

    html! {
        <div class={classes!("board-border")}>
            <div class={classes!("board")}>
                {for board.get_board().iter().enumerate().map(|(row_idx, row)| {
                    html! {
                        <div class="row">
                            {for row.iter().enumerate().map(|(col_idx, piece)| {
                                let is_white_cell = (row_idx + col_idx) % 2 == 0;
                                html! {
                                    <div class={if is_white_cell { "cell_white" } else { "cell_black" }}>
                                        {piece}
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

fn main() {
    yew::start_app::<App>();
}
