use actix_web::{middleware, App, HttpServer};
use clap;
use r2d2;
use r2d2_sqlite;
mod db;
mod handlers;
use std::fs;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let cmds = clap::App::new("Raskd server")
        .version("0.1.0")
        .author("LMJW")
        .about("command line tool to do task management")
        .subcommand(clap::App::new("init").about("setup the sqlite3 database to store the data"))
        .subcommand(clap::App::new("start").about("start the raskd server"))
        .get_matches();

    match cmds.subcommand() {
        ("init", Some(_)) => {
            let sql = fs::read_to_string("./src/sql/0001.sql")?;
            let conn = rusqlite::Connection::open("raskd.db").unwrap();
            match conn.execute_batch(&sql) {
                Ok(_) => {
                    eprintln!("Successfully generated sqlite3 database `raskd.db`.");
                    eprintln!("You can now run `raskd start` to start the server.");
                    Ok(())
                }
                Err(e) => {
                    eprintln!("{}", e);
                    let err = std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e));
                    Err(err)
                }
            }
        }
        _ => {
            // std::env::set_var("RUST_LOG", "actix_web=info");
            // env_logger::init();
            // let sql = fs::read_to_string("./src/sql/0001.sql")?;
            if let Err(e) = fs::read("raskd.db") {
                eprintln!("database does not exist. use `raskd init` to create database");
                return Err(e);
            };
            let conn = r2d2_sqlite::SqliteConnectionManager::file("raskd.db");
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
