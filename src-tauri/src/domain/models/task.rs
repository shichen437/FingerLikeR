use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS, PartialEq, Default)]
#[ts(export)]
pub enum TaskStatus {
    #[default]
    Idle,
    Running,
    Cancelled,
    Finished,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, Default)]
#[ts(export)]
pub struct TaskProgress {
    pub status: TaskStatus,
    pub progress: u32,
    pub total: u32,
}
