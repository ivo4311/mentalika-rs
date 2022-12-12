use rand::{distributions::Uniform, Rng};
use std::fmt::Display;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct MultiplicationTable {
    done: bool,
    num_tasks: i32,
    correct: i32,
    wrong: i32,
    completed_tasks: Vec<MultiplicationTask>,
    current_task: MultiplicationTask,
}

pub enum MultiplicationTableMsg {
    Submit(Option<i32>),
}

#[derive(Debug)]
struct MultiplicationTask {
    x: i32,
    y: i32,
    answer: Option<i32>,
}

enum TaskState {
    Correct,
    Wrong,
    Skipped,
}

impl Into<Classes> for TaskState {
    fn into(self) -> Classes {
        match self {
            TaskState::Correct => {
                classes!("fa", "fa-solid", "fa-check", "w3-green")
            }
            TaskState::Wrong => classes!("fa", "fa-solid", "fa-times", "w3-red"),
            TaskState::Skipped => {
                classes!("fa", "fa-solid", "fa-share", "w3-grey")
            }
        }
    }
    // fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

    // }
}

impl MultiplicationTask {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(1, 10);

        Self {
            x: rng.sample(range),
            y: rng.sample(range),
            answer: None,
        }
    }

    fn correct(&self) -> bool {
        match self.answer {
            Some(answer) => answer == self.x * self.y,
            None => false,
        }
    }

    fn state(&self) -> TaskState {
        if let Some(answer) = self.answer {
            if answer == self.x * self.y {
                TaskState::Correct
            } else {
                TaskState::Wrong
            }
        } else {
            TaskState::Skipped
        }
    }
}

trait Task: Display {
    fn state(&self) -> &'static str;
}

impl Display for MultiplicationTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.answer {
            Some(answer) => {
                write!(f, "{} x {} = {}", self.x, self.y, answer)
            }
            None => {
                write!(f, "{} x {} = ?", self.x, self.y)
            }
        }
    }
}

impl Component for MultiplicationTable {
    type Message = MultiplicationTableMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            done: false,
            num_tasks: 10,
            correct: 0,
            wrong: 0,
            completed_tasks: Vec::new(),
            current_task: MultiplicationTask::random(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MultiplicationTableMsg::Submit(answer) => {
                if !self.done {
                    let MultiplicationTask { x, y, .. } = self.current_task;
                    let task = MultiplicationTask { x, y, answer };

                    if task.correct() {
                        self.correct += 1;
                    } else {
                        self.wrong += 1;
                    }

                    self.completed_tasks.push(task);

                    if self.correct == self.num_tasks {
                        self.done = true;
                    } else {
                        self.current_task = MultiplicationTask::random();
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let onsubmit = link.callback(|value| MultiplicationTableMsg::Submit(value));
        let onpress = move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let answer = match input.value().parse::<i32>() {
                    Ok(answer) => Some(answer),
                    Err(_) => None,
                };
                input.set_value("");
                onsubmit.emit(answer)
            }
        };
        html! {
            <div class="w3-container w3-content">
                <header class="w3-container w3-theme w3-padding w3-center">
                    <h2>{ format!("Multiplication Table: [ {} ✓ ] [ {} ✗ ] / [ {} ]", self.correct, self.wrong, self.num_tasks) }</h2>
                </header>

                <div class="w3-row-padding w3-margin">
                    <div class="w3-card-4 w3-container">
                        {
                            if  !self.done {
                                let MultiplicationTask{x, y, ..} = self.current_task;
                                html!{
                                    <div class="w3-container w3-text-theme w3-center">
                                        <p class="w3-xxxlarge"><b> { format!("{} x {} = ?", x, y) } </b></p>
                                        <p><input placeholder="What is your answer?" class="w3-input" type="text" onkeypress={onpress}/></p>
                                    </div>
                                }
                            } else {
                                html!{}
                            }
                        }
                        <ul class="w3-ul">
                            { for self.completed_tasks.iter().rev().map(|task| {
                                html! {
                                    <li class="w3-bar">
                                        <i class={classes!("w3-bar-item", "w3-round-large", task.state())}></i>
                                        <div class="w3-bar-item w3-center">{ task }</div>
                                    </li>
                                }
                            })}
                        </ul>
                    </div>
                </div>
            </div>
        }
    }
}
