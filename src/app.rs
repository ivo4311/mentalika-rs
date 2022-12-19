// use crate::components::assignment::AssignmentDetails;
// use crate::components::free::FreePlayCard;
// use crate::components::homework::HomeworkCard;
use crate::{
    components::{free::FreePlayCard, homework::HomeworkCard, user::UserCard},
    model::Assignments,
};

use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn App() -> Html {
    let (a, d) = use_store::<Assignments>();
    if a.empty() {
        d.reduce_mut(|a| a.init());
    }

    html! {
        <>
        <div class="w3-content w3-margin-top" style="max-width: 1400px">
            <div class="w3-row-padding">
                <div class="w3-third">
                    <UserCard/>
                </div>
                <div class="w3-twothird">
                    <HomeworkCard/>
                    <FreePlayCard/>
                </div>
            </div>
        </div>
        <footer class="w3-container w3-teal w3-center w3-margin-top">
            <p>{"Powered by "}<a href="https://www.w3schools.com/w3css/default.asp" target="_blank">{"w3.css"}</a></p>
        </footer>
        </>
    }
}
