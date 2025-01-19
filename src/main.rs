use yew::prelude::*;

use chess::chess::Chess;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Chess />
    }
}

fn main() {
    yew::start_app::<App>();
}
