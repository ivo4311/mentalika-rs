use crate::assignment::MultiplicationTable;
use yew::prelude::*;

pub struct App {}

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <MultiplicationTable/>
        }
    }
}
