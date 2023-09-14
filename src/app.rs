use crate::{
    components::{
        assignment::AssignmentCard, free::FreePlayCard, homework::HomeworkCard, user::UserCard,
    },
    model::ui::Assignments,
};

use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[not_found]
    #[at("/")]
    Home,
    #[at("/assignment/:id")]
    Assignment { id: Uuid },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <>
                <HomeworkCard/>
                <FreePlayCard/>
            </>
        },
        Route::Assignment { id } => html! {
            <AssignmentCard assignment={id}/>
        },
    }
}

#[function_component]
pub fn App() -> Html {
    let (_, d) = use_store::<Assignments>();
    d.reduce_mut(|a| a.fill());
    html! {
        <BrowserRouter>
        <div class="w3-content w3-margin-top" style="max-width: 1600px">
            <div class="w3-row-padding">
                <div class="w3-third">
                    <UserCard/>
                </div>
                <div class="w3-twothird">
                    <Switch<Route> render={switch} />
                </div>
            </div>
        </div>
        <footer class="w3-container w3-teal w3-center w3-margin-top">
            <p>{"Powered by "}<a href="https://www.w3schools.com/w3css/default.asp" target="_blank">{"w3.css"}</a></p>
        </footer>
        </BrowserRouter>
    }
}
