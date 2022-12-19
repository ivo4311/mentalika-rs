use uuid::Uuid;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::model::{Assignments, MultiplicationAssignment, MultiplicationTask};

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
    let (state, dispatch) = use_store::<Assignments>();

    let hoverable = if *active {
        classes!("w3-hoverable")
    } else {
        classes!()
    };
    html! {
        <ul class={classes!("w3-ul", "w3-margin-left", hoverable)}>
        {
            for assignments.iter().map(|id| {
                let assignment = state.get(id.clone());
                html!{
                    if let Some(assignment) = assignment {
                        <li class="w3-row">
                            <div class="w3-cell w3-cell-middle w3-round w3-teal w3-padding">
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

// #[derive(PartialEq, Properties)]
// pub struct AssignmentDetailsProps {
//     pub assignment: Uuid,
// }

// #[function_component]
// pub fn AssignmentDetails(AssignmentDetailsProps { assignment }: &AssignmentDetailsProps) -> Html {
//     let (state, dispatch) = use_store::<State>();

//     let on_submit = dispatch.reduce_mut_callback_with(move |s, e: KeyboardEvent| {
//         if e.key() == "Enter" {
//             let input: web_sys::HtmlInputElement = e.target_unchecked_into();
//             let answer = match input.value().parse::<i32>() {
//                 Ok(answer) => Some(answer),
//                 Err(_) => None,
//             };
//             input.set_value("");
//         };
//     });

//     let task_view = if let Some(ref assignment) = assignment {
//         assignment.next().as_ref().map(|task| {
//             html! {
// <div class="w3-container w3-text-theme w3-center">
//     <p class="w3-xxxlarge"><b> { task } </b></p>
//     <p><input placeholder="What is your answer?" class="w3-input" type="text" onkeypress={on_submit}/></p>
// </div>
//             }
//         })
//     } else {
//         None
//     };

//     html! {
//         <div class="w3-container w3-card w3-white w3-margin-bottom w3-padding">
//             <h2 class="w3-text-grey w3-padding-16">
//                 <i class="fa fa-suitcase fa-fw w3-margin-right w3-xxlarge w3-text-teal"></i>
//                 if let Some(ref assignment) = assignment {{assignment.title()}}
//             </h2>
//             <p>if let Some(ref assignment) = assignment {{assignment.description()}}</p>

//             {for task_view}

//             <hr/>
//             if let Some(ref assignment) = assignment{<TaskList tasks={assignment.tasks.clone()} />}
//         </div>
//     }
// }

// #[derive(Properties, PartialEq)]
// pub struct TaskListProps {
//     tasks: Vec<MultiplicationTask>,
// }

// #[function_component]
// pub fn TaskList(TaskListProps { tasks }: &TaskListProps) -> Html {
//     html! {
// <ul class="w3-ul">
//     { for tasks.iter().rev().map(|task| {
//         html! {
//             <li class="w3-bar">
//                 <i class={classes!("w3-bar-item", "w3-round-large", task.state())}></i>
//                 <div class="w3-bar-item w3-center">{ task }</div>
//             </li>
//         }
//     })}
// </ul>
//     }
// }
