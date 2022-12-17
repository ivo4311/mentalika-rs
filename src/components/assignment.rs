use yew::prelude::*;

use crate::model::MultiplicationAssignment;

#[derive(Properties, PartialEq)]
pub struct AssignmentListProps {
    pub active: bool,
    pub assignments: Vec<MultiplicationAssignment>,
}

#[function_component(AssignmentList)]
pub fn assignment_list(
    AssignmentListProps {
        active,
        assignments,
    }: &AssignmentListProps,
) -> Html {
    let hoverable = if *active {
        classes!("w3-hoverable")
    } else {
        classes!()
    };
    html! {
        <ul class={classes!("w3-ul", "w3-margin-left", hoverable)}>
        {
            for assignments.iter().map(|a| {
                html!{
                    <li class="w3-row">
                        <div class="w3-cell w3-cell-middle w3-round w3-teal w3-padding">
                            <i class="fa fa-solid fa-calculator w3-xlarge"></i>
                        </div>
                        <div class="w3-cell w3-padding">
                            <span>{a.title()}</span><br />
                            <span class="w3-small">{a.description()}</span>
                        </div>
                    </li>
                }
            })
        }
        </ul>
    }
}
