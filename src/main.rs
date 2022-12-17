mod app;
mod components;
mod model;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
