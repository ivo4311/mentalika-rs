use std::rc::Rc;

use chrono::Utc;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    app::Route,
    model::{
        assignment::{Assignment, Progress},
        task::Task,
        ui::Assignments,
    },
};

#[derive(Properties, PartialEq)]
pub struct AssignmentListProps {
    pub active: bool,
    pub assignments: Vec<Uuid>,
}

#[function_component]
pub fn AssignmentList(
    AssignmentListProps {
        active,
        assignments,
    }: &AssignmentListProps,
) -> Html {
    let (state, _dispatch) = use_store::<Assignments>();
    let navigator = use_navigator().unwrap();

    let hoverable = if *active {
        classes!("w3-hoverable")
    } else {
        classes!()
    };
    html! {
        <ul class={classes!("w3-ul", "w3-margin-left", hoverable)}>
        {
            for assignments.iter().map(|id| {
                let id = id.clone();
                let assignment = state.get(id);
                let navigator = navigator.clone();
                let onclick = if *active {
                    Callback::from(move |_| {
                        navigator.push(&Route::Assignment { id: id });
                    })
                } else{
                    Callback::from(move |_| {})
                };
                html!{
                    if let Some(assignment) = assignment {
                        <li class="w3-row" {onclick}>
                            <div class="w3-cell w3-cell-middle w3-round w3-teal w3-padding" >
                                <i class="fa fa-solid fa-calculator w3-xlarge"></i>
                            </div>
                            <div class="w3-cell w3-padding">
                                <span>{assignment.title()}</span><br />
                                <span class="w3-small">{assignment.description()}</span>
                            </div>
                        </li>
                    }
                }
            })
        }
        </ul>
    }
}

pub enum AssignmentMessage {
    State(Rc<Assignments>),
}

#[derive(PartialEq, Properties)]
pub struct AssignmentCardProps {
    pub assignment: Uuid,
}

pub struct AssignmentCard {
    state: Rc<Assignments>,
    _dispatch: Dispatch<Assignments>,
    assignment: Option<Assignment>,
}

impl Component for AssignmentCard {
    type Message = AssignmentMessage;
    type Properties = AssignmentCardProps;

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch =
            Dispatch::<Assignments>::subscribe(ctx.link().callback(AssignmentMessage::State));
        let state = dispatch.get();
        let assignment = state.get(ctx.props().assignment).cloned();
        Self {
            state,
            _dispatch: dispatch,
            assignment,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AssignmentMessage::State(state) => {
                self.assignment = state.get(ctx.props().assignment).cloned();
                self.state = state;
            }
        };
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="w3-container w3-card w3-white w3-margin-bottom w3-padding">
                if let Some(ref assignment) = self.assignment {
                    <h2 class="w3-text-grey w3-padding-16">
                    <i class="fa fa-solid fa-calculator fa-fw w3-margin-right w3-xxlarge w3-text-teal"></i>
                        {assignment.title()}
                    </h2>

                    <div class="w3-container">
                        if !assignment.is_done() {
                            <div class="w3-display-container">
                                <ul class="w3-ul w3-display-topright">
                                    // ProgressView renders the number of correct and wrong tasks
                                    <ProgressView progress={assignment.progress()}/>
                                </ul>

                                // TaskView renders the current task
                                <TaskView assignment_id={assignment.id}/>
                            </div>
                        }
                        // TaskList renders the tasks that have been completed in the assignment
                        <TaskList tasks={assignment.tasks.clone()} show_time={assignment.timed}/>
                    </div>
                    <hr/>
                } else {
                    <h2 class="w3-text-grey w3-padding-16">
                    <i class="fa fa-solid fa-calculator fa-fw w3-margin-right w3-xxlarge w3-text-teal"></i>
                        {"Ooops!?"}
                    </h2>
                    <p>{"Такова домашно няма!"}</p>
                    <Link<Route> to={Route::Home}>{ "Назад" }</Link<Route>>
                }
            </div>
        }
    }
}

#[derive(PartialEq, Properties)]
struct ProgressViewProps {
    progress: Progress,
}

#[function_component]
fn ProgressView(ProgressViewProps { progress }: &ProgressViewProps) -> Html {
    html! {
        <li class="w3-card-4 w3-bar w3-light-gray w3-round-large w3-padding-small w3-margin-bottom">
            <div class="w3-bar-item w3-padding-small">
                <i class="fa fa-solid fa-circle-check w3-round w3-padding w3-teal">{format!(" {}", progress.correct)}</i>
            </div>
            <div class="w3-bar-item w3-padding-small">
                <i class="fa fa-solid fa-circle-xmark w3-round w3-padding w3-red">{format!(" {}", progress.wrong)}</i>
            </div>
        </li>
    }
}

#[derive(PartialEq, Properties)]
struct TaskViewProps {
    assignment_id: Uuid,
}

#[function_component]
fn TaskView(TaskViewProps { assignment_id }: &TaskViewProps) -> Html {
    let (assignments, dispatch) = use_store::<Assignments>();
    let next_task = assignments
        .get(assignment_id.clone())
        .expect("invalid assignment id")
        .task();
    let onkeypress = {
        let id = assignment_id.clone();
        dispatch.reduce_mut_callback_with(move |s, e: KeyboardEvent| {
            let mut task = next_task.clone();
            if e.key() == "Enter" {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let answer = input.value().parse::<i32>().ok();
                input.set_value("");
                task.answer = answer;
                task.t_finish = Some(Utc::now().timestamp_millis());
                s.submit_task(id, task);
            };
        })
    };

    html! {
        <div class="w3-container w3-text-teal w3-center w3-content w3-margin-right w3-margin-left">
            <p class="w3-jumbo"><b>{next_task}</b></p>
            <p><input placeholder="Колко получи?" class="w3-input" type="text" {onkeypress}/></p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TaskListProps {
    tasks: Vec<Task>,
    show_time: bool,
}

#[function_component]
pub fn TaskList(TaskListProps { tasks, show_time }: &TaskListProps) -> Html {
    html! {
        <ul class="w3-ul">
            { for tasks.iter().rev().enumerate().map(|(i, task)| {
                let effects = if i ==0 {
                    classes!("w3-animate-right")
                } else {
                    classes!()
                };
                html! {
                    <li class={classes!("w3-bar", effects)}>
                        {task.state().icon()}
                        <div class="w3-bar-item w3-center">{ task }</div>
                        if *show_time  {
                            if let (Some(start), Some(finish)) = (task.t_start, task.t_finish) {
                                <i class="w3-bar-item w3-round w3-right fa fa-solid fa-stopwatch w3-light-gray">{format!(" {:.2}", (finish - start) as f32/1000_f32)}</i>
                            }
                        }
                    </li>
                }
            })}
        </ul>
    }
}
