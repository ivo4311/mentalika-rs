use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    app::Route,
    model::{Assignments, MultiplicationTask, Progress},
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

#[derive(PartialEq, Properties)]
pub struct AssignmentDetailsProps {
    pub assignment: Uuid,
}

#[function_component]
pub fn AssignmentCard(AssignmentDetailsProps { assignment }: &AssignmentDetailsProps) -> Html {
    let (state, _dispatch) = use_store::<Assignments>();
    let navigator = use_navigator().unwrap();
    let assignment = state.get(*assignment);
    html! {
        <div class="w3-container w3-card w3-white w3-margin-bottom w3-padding">
            if let Some(ref assignment) = assignment {
                <h2 class="w3-text-grey w3-padding-16">
                <i class="fa fa-solid fa-calculator fa-fw w3-margin-right w3-xxlarge w3-text-teal"></i>
                    {assignment.title()}
                </h2>

                <div class="w3-container">
                    // TaskView renders the current task if the assignment is not completed
                    if let Some(task) = assignment.next() {
                        <TaskView assignment_id={assignment.id} {task}/>
                    }
                    // ProgressView Renders the progress bar for the assignment.
                    <ProgressView progress={assignment.progress()}/>
                    // TaskList renders the tasks that have been completed in the assignment
                    <TaskList tasks={assignment.tasks.clone()}/>
                </div>
                <hr/>
            } else {
                <h2 class="w3-text-grey w3-padding-16">
                <i class="fa fa-solid fa-calculator fa-fw w3-margin-right w3-xxlarge w3-text-teal"></i>
                    {"Ooops!?"}
                </h2>
                <p>{"Такова домашно няма!"}</p>
                <button class="w3-button w3-round w3-teal" onclick={Callback::from(move|_|navigator.push(&Route::Home))}>{"Go Back"}</button>
            }
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct ProgressViewProps {
    progress: Progress,
}

#[function_component]
fn ProgressView(ProgressViewProps { progress }: &ProgressViewProps) -> Html {
    if progress.correct < progress.total {
        html! {
            <div class="w3-grey w3-text-white w3-round w3-display-container" style="height: 20px">
                <div class="w3-display-middle">{format!("{}/{}", progress.correct, progress.total)}</div>
                <div class="w3-round w3-teal" style={format!{"height: 20px; width: {}%", progress.percent_done}}></div>
            </div>
        }
    } else {
        html! {}
    }
}

#[derive(PartialEq, Properties)]
struct TaskViewProps {
    assignment_id: Uuid,
    task: MultiplicationTask,
}

#[function_component]
fn TaskView(
    TaskViewProps {
        assignment_id,
        task,
    }: &TaskViewProps,
) -> Html {
    let (_, dispatch) = use_store::<Assignments>();
    let onkeypress = {
        let id = assignment_id.clone();
        dispatch.reduce_mut_callback_with(move |s, e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let answer = match input.value().parse::<i32>() {
                    Ok(answer) => Some(answer),
                    Err(_) => None,
                };
                input.set_value("");
                s.submit(id, answer);
            };
        })
    };

    html! {
        <div class="w3-container w3-text-teal w3-center w3-content w3-margin-right w3-margin-left">
            <p class="w3-jumbo"><b>{task}</b></p>
            <p><input placeholder="Колко получи?" class="w3-input" type="text" {onkeypress}/></p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TaskListProps {
    tasks: Vec<MultiplicationTask>,
}

#[function_component]
pub fn TaskList(TaskListProps { tasks }: &TaskListProps) -> Html {
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
                    </li>
                }
            })}
        </ul>
    }
}
