use chrono::Utc;
use rand::{distributions::Uniform, Rng};
use serde_derive::{Deserialize, Serialize};
use std::fmt::Display;
use yew::{html, Html};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Task {
    pub x: i32,
    pub y: i32,
    pub op: Operation,
    pub answer: Option<i32>,
    pub t_start: Option<i64>,
    pub t_finish: Option<i64>,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self.op {
            Operation::Addition => "+",
            Operation::Subtraction => "-",
            Operation::Multiplication => "x",
        };
        match self.answer {
            Some(answer) => {
                write!(f, "{} {} {} = {}", self.x, op, self.y, answer)
            }
            None => {
                write!(f, "{} {} {} = ?", self.x, op, self.y)
            }
        }
    }
}

impl Task {
    pub fn correct(&self) -> bool {
        match self.answer {
            Some(answer) => answer == self.op.apply(self.x, self.y),
            None => false,
        }
    }

    pub fn state(&self) -> TaskState {
        if let Some(answer) = self.answer {
            if answer == self.op.apply(self.x, self.y) {
                TaskState::Correct
            } else {
                TaskState::Wrong
            }
        } else {
            TaskState::Skipped
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
}

impl Operation {
    pub fn apply(&self, x: i32, y: i32) -> i32 {
        match self {
            Operation::Addition => x + y,
            Operation::Subtraction => x - y,
            Operation::Multiplication => x * y,
        }
    }
}
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskBuilderMode {
    Multiplication,
    AdditionAndSubtraction,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskBuilder {
    pub mode: TaskBuilderMode,
    pub xrange: Uniform<i32>,
    pub yrange: Uniform<i32>,
}

impl TaskBuilder {
    pub fn build(&self) -> Task {
        match self.mode {
            TaskBuilderMode::Multiplication => self.new_multiplication_task(),
            TaskBuilderMode::AdditionAndSubtraction => self.new_addsub_task(),
        }
    }

    fn new_multiplication_task(&self) -> Task {
        let mut rng = rand::thread_rng();
        Task {
            x: rng.sample(self.xrange),
            y: rng.sample(self.yrange),
            op: Operation::Multiplication,
            answer: None,
            t_start: Some(Utc::now().timestamp_millis()),
            t_finish: None,
        }
    }

    fn new_addsub_task(&self) -> Task {
        let mut rng = rand::thread_rng();
        let op = if rng.gen_bool(0.5) {
            Operation::Addition
        } else {
            Operation::Subtraction
        };

        let x = rng.sample(self.xrange);
        let y = rng.sample(self.yrange);
        let (x, y) = if op == Operation::Subtraction && x < y {
            (y, x)
        } else {
            (x, y)
        };
        Task {
            x,
            y,
            op,
            answer: None,
            t_start: Some(Utc::now().timestamp_millis()),
            t_finish: None,
        }
    }
}

pub enum TaskState {
    Correct,
    Wrong,
    Skipped,
}

impl TaskState {
    pub fn icon(&self) -> Html {
        match self {
            TaskState::Correct => {
                html! {<i class="w3-bar-item w3-round fa fa-solid fa-circle-check w3-teal"></i>}
            }
            TaskState::Wrong => {
                html! {<i class="w3-bar-item w3-round fa fa-solid fa-circle-xmark w3-red"></i>}
            }
            TaskState::Skipped => {
                html! {<i class="w3-bar-item w3-round fa fa-solid fa-share w3-gray"></i>}
            }
        }
    }
}
