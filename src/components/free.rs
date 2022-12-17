use yew::prelude::*;

#[function_component]
pub fn FreePlayCard() -> Html {
    html! {
        <div class="w3-container w3-card w3-white">
            <h2 class="w3-text-grey w3-padding-16">
                <i class="fa fa-solid fa-gamepad fa-fw w3-margin-right w3-xxlarge w3-text-teal"></i>{"Свободна Игра (TODO)"}
            </h2>
            <div class="w3-container">
                <button class="w3-button w3-teal w3-round">{"Нова Игра"}</button>
                <hr />
            </div>
        </div>
    }
}
