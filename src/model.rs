use chrono::{Days, NaiveDate, Utc};
use rand::{distributions::Uniform, Rng};
use serde_derive::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display, ops::Index};
use uuid::Uuid;
use yew::{classes, Classes};

use yewdux::prelude::*;

use crate::components::assignment;

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Store)]
// #[store(storage = "local")]
pub struct Homework {
    pub homework: BTreeMap<NaiveDate, Vec<Uuid>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
// #[store(storage = "local")]
pub struct Assignments {
    assignments: Vec<MultiplicationAssignment>,
}

impl Store for Assignments {
    fn new() -> Self {
        init_listener(AssignmentsListener);
        Self {
            assignments: Vec::new(),
        }
    }

    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

impl Assignments {
    pub fn get(&self, id: Uuid) -> Option<&MultiplicationAssignment> {
        return self.assignments.iter().find(|a| a.id == id);
    }

    pub fn push(&mut self, a: MultiplicationAssignment) {
        self.assignments.push(a);
    }

    pub fn remove(&mut self, id: Uuid) {
        let pos = self.assignments.iter().position(|a| a.id == id);
        if let Some(pos) = pos {
            self.assignments.remove(pos);
        }
    }

    pub fn submit(&mut self, id: Uuid, answer: Option<i32>) {
        let assignment = self.assignments.iter_mut().find(|a| a.id == id);
        if let Some(assignment) = assignment {
            assignment.submit(answer);
        }
    }

    pub fn empty(&self) -> bool {
        self.assignments.is_empty()
    }

    pub fn init(&mut self) {
        let today: NaiveDate = Utc::now().naive_utc().date();
        let tomorrow = today.checked_add_days(Days::new(1)).unwrap();
        let yesterday = today.checked_sub_days(Days::new(1)).unwrap();

        let v = vec![
            MultiplicationAssignment::new_sd_sd(10, today.clone()),
            MultiplicationAssignment::new_sd_sd(10, yesterday.clone()),
            MultiplicationAssignment::new_sd_sd(11, tomorrow.clone()),
            MultiplicationAssignment::new_sd_sd(11, yesterday.clone()),
            MultiplicationAssignment::new_sd_sd(10, tomorrow.clone()),
            MultiplicationAssignment::new_sd_sd(13, yesterday.clone()),
        ];

        self.assignments = v;
    }
}

struct AssignmentsListener;
impl Listener for AssignmentsListener {
    type Store = Assignments;

    fn on_change(&mut self, state: std::rc::Rc<Self::Store>) {
        let mut map: BTreeMap<NaiveDate, Vec<Uuid>> = BTreeMap::new();
        state.assignments.iter().for_each(|a| {
            let due_date = a.due_date.clone();
            let id = a.id.clone();
            map.entry(due_date).or_default().push(id);
        });
        Dispatch::new().set(Homework { homework: map })
    }
}

impl Default for Assignments {
    fn default() -> Self {
        let today: NaiveDate = Utc::now().naive_utc().date();
        let tomorrow = today.checked_add_days(Days::new(1)).unwrap();
        let yesterday = today.checked_sub_days(Days::new(1)).unwrap();

        let v = vec![
            MultiplicationAssignment::new_sd_sd(10, today.clone()),
            MultiplicationAssignment::new_sd_sd(10, yesterday.clone()),
            MultiplicationAssignment::new_sd_sd(11, tomorrow.clone()),
            MultiplicationAssignment::new_sd_sd(11, yesterday.clone()),
            MultiplicationAssignment::new_sd_sd(10, tomorrow.clone()),
        ];

        Self { assignments: v }
    }
}

pub enum HomeworkTag {
    DueToday,
    PastDue,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct MultiplicationTask {
    pub x: i32,
    pub y: i32,
    pub answer: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct MultiplicationAssignment {
    pub id: Uuid,
    pub due_date: NaiveDate,
    title: String,
    description: String,
    pub num_tasks: i32,
    pub tasks: Vec<MultiplicationTask>,
    next: Option<MultiplicationTask>,
    builder: MultiplicationTaskBuilder,
}

impl MultiplicationAssignment {
    pub fn new_sd_sd(num_tasks: i32, due_date: NaiveDate) -> Self {
        let builder = MultiplicationTaskBuilder {
            xrange: Uniform::new(1, 10),
            yrange: Uniform::new(1, 10),
        };
        Self {
            id: Uuid::new_v4(),
            due_date,
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
        Self::new_sd_sd(10, Utc::now().date_naive())
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