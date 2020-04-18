mod db;
mod handlers;
mod models;

use actix_web::{middleware, web, App, HttpServer};
use r2d2;
use r2d2_sqlite;
use rusqlite::OpenFlags;
use std::fs;
use std::path::Path;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sql = fs::read_to_string("./src/sql/0001.sql")?;
    let conn =
        r2d2_sqlite::SqliteConnectionManager::memory().with_init(move |c| c.execute_batch(&sql));
    let pool = r2d2::Pool::new(conn).unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .service(handlers::index)
            .service(handlers::tasks)
    })
    .bind("127.0.0.1:22022")?
    .run()
    .await
}
