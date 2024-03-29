use chrono::{Days, NaiveDate, Utc};
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

use yewdux::{prelude::*, storage};

use super::{assignment::Assignment, task::Task};

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Homework {
    pub homework: BTreeMap<NaiveDate, Vec<Uuid>>,
}

impl Store for Homework {
    fn new() -> Self {
        let assignments = match storage::load::<Assignments>(storage::Area::Local) {
            Ok(assignments) => assignments.unwrap_or_default(),
            Err(err) => {
                log::info!("failed to load Assignments {}", err);
                Default::default()
            }
        };
        assignments.assignments.clone().into()
    }

    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

impl From<Vec<Assignment>> for Homework {
    fn from(assignments: Vec<Assignment>) -> Self {
        let mut map: BTreeMap<NaiveDate, Vec<Uuid>> = BTreeMap::new();
        assignments
            .iter()
            .filter(|a| !a.is_done() && a.due_date.is_some())
            .for_each(|a| {
                let due_date = a.due_date.unwrap().clone();
                let id = a.id.clone();
                map.entry(due_date).or_default().push(id);
            });
        Self { homework: map }
    }
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assignments {
    pub assignments: Vec<Assignment>,
}

impl Store for Assignments {
    fn new() -> Self {
        init_listener(AssignmentsListener);
        match storage::load::<Assignments>(storage::Area::Local) {
            Ok(assignments) => assignments.unwrap_or_default(),
            Err(err) => {
                log::error!("failed to load Assignments {}", err);
                Default::default()
            }
        }
    }

    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

impl Assignments {
    pub fn get(&self, id: Uuid) -> Option<&Assignment> {
        return self.assignments.iter().find(|a| a.id == id);
    }

    pub fn push(&mut self, a: Assignment) {
        self.assignments.push(a);
    }

    pub fn remove(&mut self, id: Uuid) {
        let index = self
            .assignments
            .iter()
            .position(|a| a.id == id && a.due_date.is_none());
        if let Some(index) = index {
            self.assignments.remove(index);
        }
    }

    pub fn submit_task(&mut self, id: Uuid, task: Task) {
        let assignment = self.assignments.iter_mut().find(|a| a.id == id);
        if let Some(assignment) = assignment {
            assignment.submit_task(task);
        }
    }

    pub fn fill(&mut self) {
        let today: NaiveDate = Utc::now().naive_utc().date();
        let latest = self
            .assignments
            .iter()
            .filter(|a| !a.is_done() && a.due_date.is_some())
            .max_by(|a1, a2| a1.due_date.cmp(&a2.due_date));

        if let Some(assignment) = latest {
            let mut next = assignment
                .due_date
                .unwrap()
                .checked_add_days(Days::new(1))
                .unwrap();
            while next <= today {
                self.push(Assignment::new_sd_sd_multiplication(100, Some(next)));
                next = next.checked_add_days(Days::new(1)).unwrap();
            }
        } else {
            self.push(Assignment::new_sd_sd_multiplication(100, Some(today)));
        }
    }
}

struct AssignmentsListener;
impl Listener for AssignmentsListener {
    type Store = Assignments;

    fn on_change(&mut self, state: std::rc::Rc<Self::Store>) {
        if let Err(err) = storage::save(state.as_ref(), storage::Area::Local) {
            log::error!("failed to save Assignments to local storage: {}", err);
        }
        let homework: Homework = state.assignments.clone().into();
        Dispatch::new().set(homework);
    }
}
