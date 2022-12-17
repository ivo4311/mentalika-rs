use crate::{
    components::assignment::AssignmentList,
    model::{Homework, HomeworkTag, State},
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn HomeworkCard() -> Html {
    html! {
        <div class="w3-container w3-card w3-white w3-margin-bottom w3-padding">
            <h2 class="w3-text-grey w3-padding-16">
                <i class="fa fa-suitcase fa-fw w3-margin-right w3-xxlarge w3-text-teal"></i>{"Домашна Работа"}
            </h2>
            <div class="w3-container">
                <HomeworkList/>
            </div>
        </div>
    }
}

#[function_component]
fn HomeworkList() -> Html {
    let (state, _dispatch) = use_store::<State>();
    html! {
        <>
        {
            for state.homework.iter().enumerate().map(|(i, h)| {
                let active = i == 0;
                let homework = h.clone();
                html!{<HomeworkListItem {homework} {active}/>}
            })
        }
        </>
    }
}

#[derive(PartialEq, Properties)]
struct HomeworkListItemProps {
    homework: Homework,
    active: bool,
}

#[function_component]
fn HomeworkListItem(HomeworkListItemProps { homework, active }: &HomeworkListItemProps) -> Html {
    let tag = homework.tags();
    let tag = match tag {
        Some(HomeworkTag::DueToday) => {
            html! {<span class="w3-tag w3-teal w3-round w3-margin-right">{"За Днес"}</span>}
        }
        Some(HomeworkTag::PastDue) => {
            html! {<span class="w3-tag w3-red w3-round w3-margin-right">{"За Наваксване!"}</span>}
        }
        None => html! {},
    };
    let effects = if *active {
        classes!()
    } else {
        classes!("w3-grayscale-max", "w3-opacity")
    };
    html! {
        <div class={classes!("w3-margin-left", effects)}>
            <h6 class="w3-text-teal">
                <i class="fa fa-calendar fa-fw w3-margin-right"></i>
                {tag}
                {homework.due_date}
            </h6>
            <AssignmentList {active} assignments={homework.assignments.clone()} />
            <hr />
        </div>
    }
}
