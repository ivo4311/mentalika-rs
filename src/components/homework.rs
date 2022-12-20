use std::cmp::Ordering;

use chrono::Utc;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{components::assignment::AssignmentList, model::Homework};

#[function_component]
pub fn HomeworkCard() -> Html {
    html! {
        <div class="w3-container w3-card w3-white w3-margin-bottom w3-padding">
            <h2 class="w3-text-grey w3-padding-16">
                <i class="fa fa-suitcase fa-fw w3-margin-right w3-xxlarge w3-text-teal"></i>{"Домашна Работа"}
            </h2>
            <HomeworkList/>
        </div>
    }
}

#[function_component]
fn HomeworkList() -> Html {
    let (state, _dispatch) = use_store::<Homework>();
    html! {
        <div class="w3-container">
        {
            for state.homework.iter().enumerate().map(|(i, (due_date, assignments))| {
                let today = Utc::now().date_naive();
                let active = i == 0 && due_date <= &today;
                let effects = if active {classes!()} else {classes!("w3-grayscale-max", "w3-opacity")};
                let tag = match today.cmp(due_date) {
                    Ordering::Less => html!{}, // No tags for upcoming assignments
                    Ordering::Equal => html!{<span class="w3-tag w3-teal w3-round w3-margin-right">{"За Днес"}</span>},
                    Ordering::Greater => html!{<span class="w3-tag w3-red w3-round w3-margin-right">{"За Наваксване!"}</span>},
                };
                let assignments = assignments.clone();
                html!{
                    <div class={classes!("w3-margin-left", effects)}>
                        <h6 class="w3-text-teal">
                            <i class="fa fa-calendar fa-fw w3-margin-right"></i>
                            {tag}
                            {due_date}
                        </h6>
                        <AssignmentList {active} {assignments} />
                        <hr />
                    </div>
                }
            })
        }
        </div>
    }
}
