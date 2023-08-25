use crate::task::{ExternalTask, Task};
pub use condition::*;
pub use engine::*;
pub use playbook::*;
use std::sync::OnceLock;

mod condition;
mod engine;
pub mod error;
mod playbook;
mod task;

static TASK_REGISTRY: OnceLock<Vec<Task>> = OnceLock::new();

pub fn task_registry() -> &'static [Task] {
    TASK_REGISTRY.get_or_init(|| {
        vec![
            #[cfg(test)]
            Task::Fail,
            #[cfg(test)]
            Task::Success,
            Task::External(ExternalTask::new("dummy@1", &["foo"])),
        ]
    })
}
