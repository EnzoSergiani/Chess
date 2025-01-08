mod board;
mod cell;
mod chess;
mod color;
mod kind;
mod piece;
mod position;
mod shift;

use yew::prelude::*;

use crate::chess::Chess;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Chess />
    }
}

fn main() {
    yew::start_app::<App>();
}
