use rand::{distributions::Uniform, Rng};
use std::fmt::Display;
use web_sys::HtmlInputElement;
use yew::prelude::*;

fn main() {
    yew::start_app::<MultiplicationTable>();
}

enum MultiplicationTableMsg {
    Start,
    Submit(Option<i32>),
}

#[derive(Debug)]
struct MultiplicationTask {
    x: i32,
    y: i32,
    answer: Option<i32>,
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

    fn state(&self) -> &'static str {
        if let Some(answer) = self.answer {
            if answer == self.x * self.y {
                "correct"
            } else {
                "wrong"
            }
        } else {
            "skipped"
        }
    }
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

struct MultiplicationTable {
    started: bool,
    num_tasks: i32,
    correct: i32,
    wrong: i32,
    completed_tasks: Vec<MultiplicationTask>,
    current_task: Option<MultiplicationTask>,
}

impl Component for MultiplicationTable {
    type Message = MultiplicationTableMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            started: false,
            num_tasks: 10,
            correct: 0,
            wrong: 0,
            completed_tasks: Vec::new(),
            current_task: None,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MultiplicationTableMsg::Start => {
                self.started = true;
                self.correct = 0;
                self.wrong = 0;
                self.completed_tasks.clear();
                self.current_task = Some(MultiplicationTask::random());
                true
            }
            MultiplicationTableMsg::Submit(answer) => {
                if let Some(MultiplicationTask { x, y, answer: _ }) = self.current_task {
                    let task = MultiplicationTask { x, y, answer };

                    if task.correct() {
                        self.correct += 1;
                    } else {
                        self.wrong += 1;
                    }

                    self.completed_tasks.push(task);

                    if self.correct == self.num_tasks {
                        self.started = false;
                        self.current_task = None;
                    } else {
                        self.current_task = Some(MultiplicationTask::random());
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
            <>
                <h3>{ format!("Multiplication: [ {} ✓ ] [ {} ✗ ] / [ {} ]", self.correct, self.wrong, self.num_tasks) }</h3>

                {if let Some(MultiplicationTask{x, y, answer:_}) = self.current_task {
                    html!{
                        <h2> { format!("{} x {} = ?", x, y) } </h2>
                    }
                } else {
                    html!{}
                }
                }


                <input onkeypress={onpress}/>
                { if !self.started {
                    html! {
                        <button onclick={link.callback(|_| MultiplicationTableMsg::Start )}> { "Start" } </button>
                    }
                    } else {
                        html!{}
                    }
                }
                <ul>
                { for self.completed_tasks.iter().map(|task| {
                    html! {
                        <li class={task.state()}> { task } </li>
                    }
                })}
                </ul>

            </>
        }
    }
}
