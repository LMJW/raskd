use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Model {
    Task {
        id: i64,
        name: String,
        tasktype: String,
    },
    Error {
        msg: String,
    },
}
