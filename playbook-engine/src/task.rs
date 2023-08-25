use std::collections::BTreeMap;
use std::fmt::Debug;

/// The status of task execution.
pub enum TaskStatus {
    Success {
        outputs: BTreeMap<String, String>,
    },
    Failed {
        message: String,
    },
    /// The task is an external task, requires confirmation from VC/Centralized.
    External {
        step_seq: u32,
        action: String,
        params: BTreeMap<String, String>,
    },
}

/// The possible task types.
#[derive(Debug, Clone)]
pub enum Task {
    External(ExternalTask),
    #[cfg(test)]
    /// Tests-only task that always fails.
    Fail,
    #[cfg(test)]
    /// Tests-only task that is always succeeds.
    Success,
}

impl Task {
    pub fn run(&self, step_seq: u32, params: BTreeMap<String, String>) -> TaskStatus {
        match self {
            Task::External(task) => task.run(step_seq, params),
            #[cfg(test)]
            Task::Fail => TaskStatus::Failed {
                message: "i am failed".to_string(),
            },
            #[cfg(test)]
            Task::Success => TaskStatus::Success { outputs: params },
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Task::External(task) => task.name(),
            #[cfg(test)]
            Task::Fail => "failure",
            #[cfg(test)]
            Task::Success => "success",
        }
    }

    pub fn required_params(&self) -> &[String] {
        match self {
            Task::External(task) => &task.required_params,
            #[cfg(test)]
            Task::Fail | Task::Success => &[],
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExternalTask {
    name: String,
    required_params: Vec<String>,
}

impl ExternalTask {
    pub fn new<'a>(name: impl Into<String>, required_params: &'a [&'a str]) -> Self {
        Self {
            name: name.into(),
            required_params: required_params.iter().map(|p| p.to_string()).collect(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn run(&self, step_seq: u32, params: BTreeMap<String, String>) -> TaskStatus {
        for required_param in &self.required_params {
            if !params.contains_key(required_param) {
                return TaskStatus::Failed {
                    message: format!("missing parameter: {}", required_param),
                };
            }
        }

        for passed_param in params.keys() {
            if !self.required_params.contains(passed_param) {
                return TaskStatus::Failed {
                    message: format!("unknown parameter: {}", passed_param),
                };
            }
        }

        TaskStatus::External {
            step_seq,
            action: self.name.clone(),
            params,
        }
    }
}
