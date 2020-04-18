use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Start,
    Stop,
    /// TODO : allow pause functionality
    /// Current pause is treated as stop
    Pause,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Incoming {
    Create {
        name: String,
        tasktype: String,
    },
    Update {
        id: Option<i64>,
        name: Option<String>,
        action: Status,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Outgoing {
    Success {
        id: i64,
        name: String,
        tasktype: String,
        duration: String,
        start_at: String,
    },
    Error {
        msg: String,
    },
}
