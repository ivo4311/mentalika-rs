use uuid::Uuid;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    components::assignment::AssignmentList,
    model::{assignment::Assignment, ui::Assignments},
};

struct FreePlayCardState {
    show_modal: bool,
}

#[function_component]
pub fn FreePlayCard() -> Html {
    let (store, dispatch) = use_store::<Assignments>();
    let assignments: Vec<Uuid> = store
        .assignments
        .iter()
        .filter(|a| !a.is_done() && a.due_date.is_none())
        .map(|a| a.id)
        .collect();

    let add_multiplication = dispatch.reduce_mut_callback(|assignments| {
        assignments.push(Assignment::new_sd_sd_multiplication(100, None));
    });

    let add_addition = dispatch.reduce_mut_callback(|assignments| {
        assignments.push(Assignment::new_sd_sd_addition(100, None));
    });

    let state = use_state(|| FreePlayCardState { show_modal: false });
    let show_modal = {
        let state = state.clone();
        Callback::from(move |_| state.set(FreePlayCardState { show_modal: true }))
    };
    let hide_modal = {
        let state = state.clone();
        Callback::from(move |_| state.set(FreePlayCardState { show_modal: false }))
    };
    let modal_style = if state.show_modal {
        "display: block;"
    } else {
        "display: none;"
    };
    html! {
        <>
        // <!-- Modal window for adding new free games -->
        <div class="w3-modal" style={modal_style}>
            <div class="w3-modal-content">
                <header class="w3-container w3-teal">
                    <h4>{"Добави Игра"}</h4>
                </header>
                <div class="w3-container w3-padding-16">
                    <div class="w3-row-padding">
                        <div class="w3-half w3-margin-bottom">
                            <ul class="w3-ul w3-border w3-center w3-hover-shadow">
                                <li class="w3-large w3-padding-16 w3-text-grey"><i
                                        class="fa fa-calculator fa-fw w3-margin-right w3-xlarge w3-text-teal"></i>{"Умножение"}
                                </li>
                                <li class="w3-padding-16 w3-small">{"едноцифрено по едноцифрено"}</li>
                                <li class="w3-padding-16 w3-small"><b>{"100"}</b> {"примера"} </li>
                                <li class="w3-light-grey w3-padding-16">
                                    <button onclick={add_multiplication} class="w3-button w3-teal w3-round w3-padding">{"Добави"}</button>
                                </li>
                            </ul>
                        </div>

                        <div class="w3-half w3-margin-bottom">
                            <ul class="w3-ul w3-border w3-center w3-hover-shadow">
                                <li class="w3-large w3-padding-16 w3-text-grey"><i
                                        class="fa fa-plus-minus fa-fw w3-margin-right w3-xlarge w3-text-teal"></i>{"Събиране и Изваждане"}</li>
                                <li class="w3-padding-16 w3-small">{"едноцифрено с едноцифрено"}</li>
                                <li class="w3-padding-16 w3-small"><b>{"100"}</b> {"примера"} </li>
                                <li class="w3-light-grey w3-padding-16">
                                    <button onclick={add_addition} class="w3-button w3-teal w3-round w3-padding">{"Добави"}</button>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
                <footer class="w3-container w3-teal">
                    <div class="w3-bar">
                        <button onclick={hide_modal}
                            class="w3-bar-item w3-right w3-button w3-round w3-green w3-margin">{"Готово"}</button>
                    </div>
                </footer>
            </div>
        </div>
        // <!-- The free games card -->
        <div class="w3-container w3-card w3-white w3-display-container">
            <div class="w3-container w3-display-topright w3-margin">
                <button onclick={show_modal} class="w3-button w3-teal w3-round">{"+"}</button>
            </div>
            <h2 class="w3-text-grey w3-padding-16">
                <i class="fa fa-solid fa-gamepad fa-fw w3-margin-right w3-xxlarge w3-text-teal"></i>{"Свободна Игра"}
            </h2>
            <div class="w3-container ">
                <AssignmentList active={true} {assignments}/>
                <hr />
            </div>
        </div>
        </>
    }
}
