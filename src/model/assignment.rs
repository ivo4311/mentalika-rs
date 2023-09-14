use chrono::NaiveDate;
use rand::distributions::Uniform;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use super::task::{Task, TaskBuilder, TaskBuilderMode};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Assignment {
    pub id: Uuid,
    pub due_date: NaiveDate,
    pub timed: bool,
    pub title: String,
    pub description: String,
    pub num_tasks: i32,
    pub tasks: Vec<Task>,
    pub next: Option<Task>,
    pub builder: TaskBuilder,
}

impl Assignment {
    pub fn new_sd_sd_multiplication(num_tasks: i32, due_date: NaiveDate) -> Self {
        let builder = TaskBuilder {
            mode: TaskBuilderMode::Multiplication,
            xrange: Uniform::new(1, 10),
            yrange: Uniform::new(1, 10),
        };
        Self {
            id: Uuid::new_v4(),
            due_date,
            timed: true,
            title: "Умножение".to_owned(),
            description: "едноцифрено по едноцифрено".to_owned(),
            num_tasks,
            tasks: Vec::new(),
            next: Some(builder.build()),
            builder: builder,
        }
    }

    pub fn new_sd_sd_addition(num_tasks: i32, due_date: NaiveDate) -> Self {
        let builder = TaskBuilder {
            mode: TaskBuilderMode::AdditionAndSubtraction,
            xrange: Uniform::new(1, 10),
            yrange: Uniform::new(1, 10),
        };
        Self {
            id: Uuid::new_v4(),
            due_date,
            timed: true,
            title: "Събиране и Изваждане".to_owned(),
            description: "едноцифрено с едноцифрено".to_owned(),
            num_tasks,
            tasks: Vec::new(),
            next: Some(builder.build()),
            builder: builder,
        }
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

    pub fn next(&self) -> Option<Task> {
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

    pub fn progress(&self) -> Progress {
        let mut correct = 0;
        let mut wrong = 0;
        let mut skipped = 0;

        self.tasks.iter().for_each(|t| {
            match t.answer {
                Some(_) => {
                    if t.correct() {
                        correct += 1;
                    } else {
                        wrong += 1;
                    }
                }
                None => {
                    skipped += 1;
                }
            };
        });
        let percent_done = (correct as f32 * 100.0 / self.num_tasks as f32).round() as i32;
        Progress {
            total: self.num_tasks,
            correct,
            wrong,
            skipped,
            percent_done,
        }
    }
}

#[derive(PartialEq)]
pub struct Progress {
    pub total: i32,
    pub correct: i32,
    pub wrong: i32,
    pub skipped: i32,
    pub percent_done: i32,
}