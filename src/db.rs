use crate::models::Model;
use r2d2;
use r2d2_sqlite;
use rusqlite::{Statement, NO_PARAMS};

/// define type alias to make less verbose
pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
type QueryResult = Result<Vec<Model>, rusqlite::Error>;

pub fn get_tasks(conn: Connection) -> QueryResult {
    let mut stmt = conn
        .prepare("SELECT id, name, tasktype FROM tasks")
        .unwrap();
    stmt.query_map(NO_PARAMS, |row| {
        Ok(Model::Task {
            id: row.get(0)?,
            name: row.get(1)?,
            tasktype: row.get(2)?,
        })
    })
    .and_then(Iterator::collect)
}

pub fn add_task(conn: Connection, task: Model) -> Result<Model, rusqlite::Error> {
    match task {
        Model::Task { name, tasktype, .. } => {
            conn.execute(
                "INSERT into tasks (name, tasktype) VALUES (?1, ?2)",
                &[&name, &tasktype],
            )?;
            let last_id: i64 = conn.last_insert_rowid();
            Ok(Model::Task {
                id: last_id,
                name,
                tasktype,
            })
        }
        _ => Ok(Model::Error {
            msg: "unknow input".to_string(),
        }),
    }
}
