use chrono::{Days, NaiveDate, Utc};
use rand::{distributions::Uniform, Rng};
use serde_derive::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;
use yew::{classes, Classes};

use yewdux::prelude::*;

#[derive(PartialEq, Serialize, Deserialize, Store)]
// #[store(storage = "local")]
pub struct State {
    pub homework: Vec<Homework>,
}

impl Default for State {
    fn default() -> Self {
        let today: NaiveDate = Utc::now().naive_utc().date();
        let tomorrow = today.checked_add_days(Days::new(1)).unwrap();
        let yesterday = today.checked_sub_days(Days::new(1)).unwrap();

        let mut v = vec![
            Homework {
                id: Uuid::new_v4(),
                due_date: today,
                assignments: vec![MultiplicationAssignment::new()],
            },
            Homework {
                id: Uuid::new_v4(),
                due_date: tomorrow,
                assignments: vec![MultiplicationAssignment::new()],
            },
            Homework {
                id: Uuid::new_v4(),
                due_date: yesterday,
                assignments: vec![
                    MultiplicationAssignment::new(),
                    MultiplicationAssignment::new(),
                    MultiplicationAssignment::new(),
                ],
            },
        ];

        v.sort();

        Self { homework: v }
    }
}

pub enum HomeworkTag {
    DueToday,
    PastDue,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Homework {
    pub id: Uuid,
    pub due_date: NaiveDate,
    pub assignments: Vec<MultiplicationAssignment>,
}

impl PartialEq for Homework {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Homework {}

impl Ord for Homework {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.due_date.cmp(&other.due_date)
    }
}

impl Homework {
    pub fn tags(&self) -> Option<HomeworkTag> {
        match self.due_date.cmp(&Utc::now().date_naive()) {
            std::cmp::Ordering::Less => Some(HomeworkTag::PastDue),
            std::cmp::Ordering::Equal => Some(HomeworkTag::DueToday),
            std::cmp::Ordering::Greater => None,
        }
    }

    pub fn is_done(&self) -> bool {
        self.assignments.iter().all(|a| a.is_done())
    }
}

impl PartialOrd for Homework {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.due_date.partial_cmp(&other.due_date)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct MultiplicationTask {
    pub x: i32,
    pub y: i32,
    pub answer: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct MultiplicationTaskBuilder {
    xrange: Uniform<i32>,
    yrange: Uniform<i32>,
}

impl MultiplicationTaskBuilder {
    pub fn build(&self) -> MultiplicationTask {
        let mut rng = rand::thread_rng();
        MultiplicationTask {
            x: rng.sample(self.xrange),
            y: rng.sample(self.yrange),
            answer: None,
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

impl MultiplicationTask {
    pub fn correct(&self) -> bool {
        match self.answer {
            Some(answer) => answer == self.x * self.y,
            None => false,
        }
    }

    pub fn state(&self) -> TaskState {
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

pub enum TaskState {
    Correct,
    Wrong,
    Skipped,
}

impl Into<Classes> for TaskState {
    fn into(self) -> Classes {
        match self {
            TaskState::Correct => classes!("fa", "fa-solid", "fa-check", "w3-teal"),
            TaskState::Wrong => classes!("fa", "fa-solid", "fa-times", "w3-red"),
            TaskState::Skipped => classes!("fa", "fa-solid", "fa-share", "w3-grey"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct MultiplicationAssignment {
    id: Uuid,
    title: String,
    description: String,
    num_tasks: i32,
    pub tasks: Vec<MultiplicationTask>,
    next: Option<MultiplicationTask>,
    builder: MultiplicationTaskBuilder,
}

impl MultiplicationAssignment {
    pub fn new_sd_sd(num_tasks: i32) -> Self {
        let builder = MultiplicationTaskBuilder {
            xrange: Uniform::new(1, 10),
            yrange: Uniform::new(1, 10),
        };
        Self {
            id: Uuid::new_v4(),
            title: "Умножение".to_owned(),
            description: "едноцифрено по едноцифрено".to_owned(),
            num_tasks,
            tasks: Vec::new(),
            next: Some(builder.build()),
            builder: builder,
        }
    }

    // pub fn new_sd_sd_wo1(num_tasks: i32) -> Self {
    //     let builder = MultiplicationTaskBuilder {
    //         xrange: Uniform::new(2, 10),
    //         yrange: Uniform::new(2, 10),
    //     };
    //     Self {
    //         title: "Умножение".to_owned(),
    //         description: "едноцифрено по едноцифрено без 1".to_owned(),
    //         num_tasks,
    //         tasks: Vec::new(),
    //         next: Some(builder.build()),
    //         builder: builder,
    //     }
    // }

    pub fn new() -> Self {
        Self::new_sd_sd(10)
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn description(&self) -> String {
        let (correct, wrong) = self.score();
        format!(
            "{} - [ {} ✓ ] [ {} ✗ ] / [ {} ]",
            self.description, correct, wrong, self.num_tasks
        )
    }

    pub fn submit(&mut self, answer: Option<i32>) {
        if let Some(task) = self.next.as_mut() {
            task.answer = answer;
            self.tasks.push(task.clone());
            self.next = if self.is_done() {
                None
            } else {
                Some(self.builder.build())
            }
        }
    }

    pub fn next(&self) -> Option<MultiplicationTask> {
        self.next
    }

    pub fn is_done(&self) -> bool {
        self.num_tasks == (self.tasks.iter().filter(|t| t.correct()).count() as i32)
    }

    pub fn score(&self) -> (i32, i32) {
        let correct = self.tasks.iter().filter(|t| t.correct()).count() as i32;
        let wrong = self.tasks.iter().filter(|t| !t.correct()).count() as i32;
        (correct, wrong)
    }
}
