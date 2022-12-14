use crate::assignment::*;
use crate::models::MultiplicationAssignment;
use yew::prelude::*;

use gloo::storage::{LocalStorage, Storage};

const KEY: &str = "ivo4311.mentalika-rs.assignments";

pub enum AppMsg {
    Add,
    Select(usize),
    Submit(Option<i32>),
    Clear,
}
pub struct App {
    assignments: Vec<MultiplicationAssignment>,
    selected: Option<usize>,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let assignments: Vec<MultiplicationAssignment> =
            LocalStorage::get(KEY).unwrap_or_else(|_| vec![MultiplicationAssignment::new()]);

        Self {
            assignments,
            selected: Some(0),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Add => {
                self.assignments
                    .push(MultiplicationAssignment::new_sd_sd(10));
                LocalStorage::set(KEY, self.assignments.clone())
                    .expect("failed to set assignments");
            }
            AppMsg::Select(assignment) => {
                self.selected = Some(assignment);
            }
            AppMsg::Submit(answer) => {
                if let Some(assignment) = self.selected {
                    self.assignments
                        .iter_mut()
                        .nth(assignment)
                        .unwrap()
                        .submit(answer.clone());
                    LocalStorage::set(KEY, self.assignments.clone())
                        .expect("Failed to store assignments!");
                }
            }
            AppMsg::Clear => {
                self.assignments.clear();
                LocalStorage::set(KEY, self.assignments.clone())
                    .expect("Failed to store assignments!");
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_select_assignment = ctx.link().callback(move |a| AppMsg::Select(a));
        let on_add_assignment = ctx.link().callback(|_| AppMsg::Add);
        let on_clear_assignment = ctx.link().callback(|_| AppMsg::Clear);

        let details = self.selected.and(self.assignments.iter().nth(self.selected.unwrap()).cloned().map(|assignment| {
            html! {
                <AssignmentDetailsFunc assignment={assignment} onsubmit={ctx.link().callback(move |a| AppMsg::Submit(a))} />
            }
        }));

        html! {
            <>
            <div class="w3-sidebar" style="width:33%">
                <div class="w3-display-container w3-container w3-theme">
                    <h1>{"My Assignments"}</h1>
                    <div class="w3-bar w3-container w3-display-right">
                    <button class="w3-button w3-round-large w3-black w3-margin-right w3-right" onclick={on_clear_assignment}><i class="fa fa-solid fa-trash"></i></button>
                    <button class="w3-button w3-round-large w3-black w3-margin-right w3-right" onclick={on_add_assignment}><i class="fa fa-solid fa-plus"></i></button>
                    </div>
                </div>
               <AssignmentList
                    assignments={self.assignments.clone()}
                    onclick={on_select_assignment}/>
            </div>

            <div class="w3-main" style="margin-left:33%">
                {for details}
            </div>
        </>
        }
    }
}
