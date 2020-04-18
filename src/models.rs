use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Model {
    Task {
        id: i64,
        name: String,
        tasktype: String,
        #[serde(skip_serializing)]
        duration: String,
        #[serde(skip_serializing)]
        start_at: String,
    },
    Error {
        msg: String,
    },
}
