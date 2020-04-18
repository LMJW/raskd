use actix_web::{middleware, App, HttpServer};
use r2d2;
use r2d2_sqlite;
use std::fs;
mod db;
mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "actix_web=info");
    // env_logger::init();
    let sql = fs::read_to_string("./src/sql/0001.sql")?;
    let conn =
        r2d2_sqlite::SqliteConnectionManager::memory().with_init(move |c| c.execute_batch(&sql));
    let pool = r2d2::Pool::new(conn).unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .service(handlers::post_tasks)
            .service(handlers::get_tasks)
    })
    .bind("127.0.0.1:22022")?
    .run()
    .await
}
