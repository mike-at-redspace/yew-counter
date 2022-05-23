use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| 0);

    let incr_counter = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };

    let decr_counter = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state - 1))
    };

    html!(
        <div class="card">
            <h3>{"åCount: "} {*state}</h3>
            <div class="button-list">
                <button onclick={incr_counter}>{"⬆ Up"}</button>
                <button onclick={decr_counter}>{"Down ⬇"}</button>
            </div>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
