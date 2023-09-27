use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::model::{task::TaskBuilderMode, ui::Assignments};

#[function_component]
pub fn UserCard() -> Html {
    html! {
        <div class="w3-white w3-text-grey w3-card-4">
            <UserAvatar/>
            <div class="w3-container">
                <p><i class="fa fa-home fa-fw w3-margin-right w3-large w3-text-teal"></i>{"Център \"Менталика\", София, България"}</p>
                <hr />
                <UserSkills/>
                <UserAchievements/>
                <UserArchive/>
            </div>
        </div>
    }
}

#[function_component]
fn UserAvatar() -> Html {
    html! {
        <div class="w3-display-container">
            <img src="/img/mishe.jpg" style="width: 100%" alt="Avatar" />
            <div class="w3-display-bottomleft w3-container w3-text-black">
                <h2>{"Менталика мише?"}</h2>
            </div>
        </div>
    }
}

#[function_component]
fn UserSkills() -> Html {
    let (store, _d) = use_store::<Assignments>();

    let mut multiplication_count = 0;
    let mut multiplication_score = 0_f32;

    let mut addition_count = 0;
    let mut addition_score = 0_f32;
    store
        .assignments
        .iter()
        .filter(|a| a.is_done())
        .for_each(|a| {
            let progress = a.progress();
            match a.builder.mode {
                TaskBuilderMode::Multiplication => {
                    multiplication_score +=
                        progress.correct as f32 / (progress.correct + progress.wrong) as f32;
                    multiplication_count += 1;
                }
                TaskBuilderMode::AdditionAndSubtraction => {
                    addition_score +=
                        progress.correct as f32 / (progress.correct + progress.wrong) as f32;
                    addition_count += 1;
                }
            }
        });
    let multiplication_score = if multiplication_count == 0 {
        "N/A".to_owned()
    } else {
        format!(
            "{:.0}%",
            (multiplication_score / multiplication_count as f32) * 100_f32
        )
    };
    let addition_score = if addition_count == 0 {
        "N/A".to_owned()
    } else {
        format!("{:.0}%", (addition_score / addition_count as f32) * 100_f32)
    };

    html! {
        <>
        <div class="w3-display-container">
            <p class="w3-large">
                <b><i class="fa fa-asterisk fa-fw w3-margin-right w3-text-teal"></i>{"Умения"}</b>
            </p>
        <div class="w3-display-bottomright w3-tiny"><a href="#">{"see more"}</a></div>
            </div>
            <div class="w3-row w3-margin-left">
                <div class="w3-col s8"><i class="fa fa-solid fa-xmark w3-small w3-text-teal w3-margin-right"></i>{"Умножение"}</div>
                <div class="w3-col s4">
                    <div class="w3-light-grey w3-round-xlarge w3-small">
                        <div class="w3-container w3-center w3-round-xlarge w3-teal" style="width: 100%">{multiplication_score}</div>
                    </div>
                </div>
            </div>

            <div class="w3-row w3-margin-left w3-margin-top">
                <div class="w3-col s8"><i class="fa fa-solid fa-divide w3-small w3-text-teal w3-margin-right"></i>{"Деление"}</div>
                <div class="w3-col s4">
                    <div class="w3-light-grey w3-round-xlarge w3-small">
                        <div class="w3-container w3-center w3-round-xlarge w3-teal" style="width: 100%">{"N/A"}</div>
                    </div>
                </div>
            </div>

            <div class="w3-row w3-margin-left w3-margin-top">
                <div class="w3-col s8">
                    <i class="fa fa-solid fa-plus-minus w3-small w3-text-teal w3-margin-right"></i> {"Събиране и Изваждане"}
                </div>
                <div class="w3-col s4">
                    <div class="w3-light-grey w3-round-xlarge w3-small">
                        <div class="w3-container w3-center w3-round-xlarge w3-teal" style="width: 100%">{addition_score}</div>
                    </div>
                </div>
            </div>
            <hr />
        </>
    }
}

#[function_component]
fn UserAchievements() -> Html {
    html! {
        <>
        <div class="w3-display-container">
            <p class="w3-large">
                <b><i class="fa fa-solid fa-award fa-fw w3-margin-right w3-text-teal"></i>{"Заслуги (TODO)"}</b>
            </p>
            <div class="w3-display-bottomright w3-tiny"><a href="#">{"see more"}</a></div>
        </div>
        <hr />
        </>
    }
}

#[function_component]
fn UserArchive() -> Html {
    html! {
        <>
        <div class="w3-display-container">
            <p class="w3-large">
                <b><i class="fa fa-solid fa-book fa-fw w3-margin-right w3-text-teal"></i>{"Архив (TODO)"}</b>
            </p>
            <div class="w3-display-bottomright w3-tiny"><a href="#">{"see more"}</a></div>
        </div>
        <hr />
        </>
    }
}
