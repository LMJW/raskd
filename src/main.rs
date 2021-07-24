use actix_web::{middleware, App, HttpServer};
use clap;
use r2d2;
use r2d2_sqlite;
mod db;
mod handlers;
use std::env;
use std::fs;
use std::path::PathBuf;
use tracing::debug;
use tracing_subscriber;

fn create_database(path: &PathBuf) -> Result<(), std::io::Error> {
    const SQL: &str = r#"
    CREATE TABLE tasks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        tasktype TEXT NOT NULL,
        start TEXT NOT NULL,
        stop TEXT
    );
    "#;
    eprintln!("Your sqlite db is located at:");
    eprintln!("`{:?}`", path.as_os_str());
    let conn = rusqlite::Connection::open(path).expect("unable to open sqlite connection");
    match conn.execute_batch(&SQL) {
        Ok(_) => {
            eprintln!("Successfully generated sqlite3 database `raskd.db`.");
            Ok(())
        }
        Err(e) => {
            debug!("{}", e);
            let err = std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e));
            Err(err)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    let cmds = clap::App::new("Raskd server")
        .version("0.1.0")
        .author("LMJW")
        .about("command line tool to do task management")
        .subcommand(clap::App::new("init").about("setup the sqlite3 database to store the data"))
        .subcommand(clap::App::new("start").about("start the raskd server"))
        .get_matches();

    let mut path = env::current_exe()?;
    path.pop();
    path.push("raskd.db");
    match cmds.subcommand() {
        ("init", Some(_)) => create_database(&path),
        _ => {
            if let Err(e) = fs::read(&path) {
                debug!("{}", e);
                eprintln!("database does not exist. creating the database...");
                create_database(&path)?;
            };
            let conn = r2d2_sqlite::SqliteConnectionManager::file(&path);
            let pool = r2d2::Pool::new(conn).unwrap();
            HttpServer::new(move || {
                App::new()
                    .wrap(middleware::Logger::default())
                    .data(pool.clone())
                    .service(handlers::post_tasks)
                    .service(handlers::get_tasks)
                    .service(handlers::patch_task)
            })
            .bind("127.0.0.1:22022")?
            .run()
            .await
        }
    }
}
