use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(App)]
fn main() -> Html {
    html! {
        <h1>{ "This is the shortlink system" }</h1>
    }
}

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    Ok(())
}
