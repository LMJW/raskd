use chrono::{DateTime, Local};
use r2d2;
use r2d2_sqlite;
use raskd::models::{Incoming, Outgoing};
use rusqlite::{params, NO_PARAMS};

/// define type alias to make less verbose
pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
type QueryResults = Result<Vec<Outgoing>, rusqlite::Error>;
type QueryResult = Result<Outgoing, rusqlite::Error>;

fn query_task(conn: Connection, id: i64) -> QueryResult {
    conn.query_row(
        "SELECT id, name, tasktype, start, stop FROM tasks WHERE id=?1",
        params![id],
        |row| {
            let (start, stop): (String, String) = (row.get(4)?, row.get(5)?);
            let (dt_start, dt_stop) = (
                DateTime::parse_from_rfc2822(&start).unwrap(),
                DateTime::parse_from_rfc2822(&stop).unwrap(),
            );
            let duration = dt_stop - dt_start;
            Ok(Outgoing::Success {
                id: row.get(0)?,
                name: row.get(1)?,
                tasktype: row.get(2)?,
                start_at: start,
                duration: format!(
                    "{:02}:{:02}:{:02}",
                    &duration.num_hours(),
                    &duration.num_minutes(),
                    &duration.num_seconds()
                ),
            })
        },
    )
}

pub fn query_tasks(conn: Connection) -> QueryResults {
    let mut stmt = conn
        .prepare("SELECT id, name, tasktype, start FROM tasks")
        .unwrap();
    stmt.query_map(NO_PARAMS, |row| {
        let start: String = row.get(3)?;
        let dt = DateTime::parse_from_rfc2822(&start).unwrap();
        let duration = Local::now().time() - dt.time();
        Ok(Outgoing::Success {
            id: row.get(0)?,
            name: row.get(1)?,
            tasktype: row.get(2)?,
            start_at: start,
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

pub fn add_task(conn: Connection, task: Incoming) -> QueryResult {
    match task {
        Incoming::Create { name, tasktype, .. } => {
            let dt = Local::now();
            let ts = dt.to_rfc2822();

            conn.execute(
                "INSERT into tasks (name, tasktype, start) VALUES (?1, ?2, ?3)",
                params![&name, &tasktype, &ts],
            )?;

            let last_id: i64 = conn.last_insert_rowid();

            Ok(Outgoing::Success {
                id: last_id,
                name,
                tasktype,
                duration: "00:00:00".to_string(),
                start_at: ts,
            })
        }
        _ => Ok(Outgoing::Error {
            msg: "unknow input".to_string(),
        }),
    }
}

pub fn update_task(conn: Connection, task: Incoming) -> QueryResult {
    let curt = Local::now();
    match task {
        Incoming::Update { id, name, .. } => match (id, name) {
            (Some(id), _) => {
                conn.execute(
                    "UPDATE tasks SET stop=?1 WHERE id=?2",
                    params![curt.to_rfc2822(), id],
                )?;
                query_task(conn, id)
            }
            (None, Some(name)) => {
                let id = conn.query_row(
                    "SELECT id FROM tasks WHERE name LIKE '%?1%'",
                    params![&name],
                    |row| {
                        let id: i64 = row.get(0)?;
                        Ok(id)
                    },
                )?;
                conn.execute(
                    "UPDATE tasks SET stop=?1 WHERE id=?2",
                    params![curt.to_rfc2822(), id],
                )?;
                query_task(conn, id)
            }
            (None, None) => Ok(Outgoing::Error {
                msg: "invalid input".to_string(),
            }),
        },
        _ => Ok(Outgoing::Error {
            msg: "unknown input".to_string(),
        }),
    }
}
