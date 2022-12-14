use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::models::{MultiplicationAssignment, MultiplicationTask};

#[derive(Properties, PartialEq)]
pub struct AssignmentListProps {
    pub assignments: Vec<MultiplicationAssignment>,
    pub onclick: Callback<usize>,
}

#[function_component(AssignmentList)]
pub fn assignment_list(
    AssignmentListProps {
        assignments,
        onclick,
    }: &AssignmentListProps,
) -> Html {
    html! {
        <ul class="w3-ul w3-card-4 w3-hoverable">
        {
            for assignments.iter().enumerate().map(|(i, a)| {
                let on_assignment_select = {
                    let onclick = onclick.clone();
                    Callback::from(move |_| {
                        onclick.emit(i)
                    })

                };
                html!{
                    <li class="w3-row" onclick={on_assignment_select}>
                        <div class="w3-cell w3-cell-middle w3-round-large w3-green w3-padding w3-large"><b>{"2 x 2"}</b></div>
                        <div class="w3-cell w3-padding">
                            <span class="w3-large">{a.title()}</span><br/>
                            <span>{a.description()}</span>
                        </div>
                    </li>
                }
            })
        }
        </ul>
    }
}

#[derive(Properties, PartialEq)]
pub struct AssignmentDetailsProps {
    pub assignment: MultiplicationAssignment,
    pub onsubmit: Callback<Option<i32>>,
}

#[function_component(AssignmentDetailsFunc)]
pub fn assignment_details(
    AssignmentDetailsProps {
        assignment,
        onsubmit,
    }: &AssignmentDetailsProps,
) -> Html {
    let on_submit_task = {
        let onsubmit = onsubmit.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let answer = match input.value().parse::<i32>() {
                    Ok(answer) => Some(answer),
                    Err(_) => None,
                };
                input.set_value("");
                onsubmit.emit(answer);
            }
        })
    };

    let task_view = assignment.next().as_ref().map(|task| {
        html! {
            <div class="w3-container w3-text-theme w3-center">
                <p class="w3-xxxlarge"><b> { task } </b></p>
                <p><input placeholder="What is your answer?" class="w3-input" type="text" onkeypress={on_submit_task}/></p>
            </div>
        }
    });

    html! {
        <>
        <div class="w3-container w3-theme">
            <h1>{assignment.title().clone()}</h1>
        </div>
        <div class="w3-container w3-content">
            {for task_view}
        </div>
        <TaskList tasks={assignment.tasks.clone()}/>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct TaskListProps {
    tasks: Vec<MultiplicationTask>,
}

#[function_component(TaskList)]
pub fn task_list(TaskListProps { tasks }: &TaskListProps) -> Html {
    html! {
        <ul class="w3-ul">
            { for tasks.iter().rev().map(|task| {
                html! {
                    <li class="w3-bar">
                        <i class={classes!("w3-bar-item", "w3-round-large", task.state())}></i>
                        <div class="w3-bar-item w3-center">{ task }</div>
                    </li>
                }
            })}
        </ul>
    }
}
