use crate::models::Model;
use chrono::{DateTime, Local};
use r2d2;
use r2d2_sqlite;
use rusqlite::{params, Statement, NO_PARAMS};

/// define type alias to make less verbose
pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
type QueryResult = Result<Vec<Model>, rusqlite::Error>;

pub fn get_tasks(conn: Connection) -> QueryResult {
    let mut stmt = conn
        .prepare("SELECT id, name, tasktype, start FROM tasks")
        .unwrap();
    stmt.query_map(NO_PARAMS, |row| {
        let start: String = row.get(3)?;
        let dt = DateTime::parse_from_rfc2822(&start).unwrap();
        let duration = Local::now().time() - dt.time();
        Ok(Model::Task {
            id: row.get(0)?,
            name: row.get(1)?,
            tasktype: row.get(2)?,
            start_at: dt.time().format("%H%M%S").to_string(),
            duration: format!(
                "{:02}:{:02}:{:02}",
                &duration.num_hours(),
                &duration.num_minutes(),
                &duration.num_seconds()
            ),
        })
    })
    .and_then(Iterator::collect)
}

pub fn add_task(conn: Connection, task: Model) -> Result<Model, rusqlite::Error> {
    match task {
        Model::Task { name, tasktype, .. } => {
            let dt = Local::now();
            let ts = dt.to_rfc2822();

            conn.execute(
                "INSERT into tasks (name, tasktype, start, parts) VALUES (?1, ?2, ?3, ?4)",
                params![&name, &tasktype, &ts, 1],
            )?;

            let last_id: i64 = conn.last_insert_rowid();

            Ok(Model::Task {
                id: last_id,
                name,
                tasktype,
                duration: "00:00:00".to_string(),
                start_at: dt.format("%H:%M:%S").to_string(),
            })
        }
        _ => Ok(Model::Error {
            msg: "unknow input".to_string(),
        }),
    }
}
