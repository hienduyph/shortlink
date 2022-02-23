use yew::prelude::*;

#[function_component(App)]
fn main() -> Html {
    html! {
        <h1>{ "This is the shortlink system" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
