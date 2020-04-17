mod db;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use r2d2;
use r2d2_sqlite;
use rusqlite::OpenFlags;
use std::fs;
use std::path::Path;

// fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
//     use actix_web::error::JsonPayloadError;

//     let detail = err.to_string();
//     let resp = match &err {
//         JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body(detail),
//         JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
//             HttpResponse::UnprocessableEntity().body(detail)
//         }
//         _ => HttpResponse::BadRequest().body(detail),
//     };
//     error::InternalError::from_response(err, resp).into()
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let sql = fs::read_to_string("./src/sql/0001.sql")?;
    let conn =
        r2d2_sqlite::SqliteConnectionManager::memory().with_init(move |c| c.execute_batch(&sql));
    // TODO not sure using with init is a good idea or not
    // might need to change later
    let pool = r2d2::Pool::new(conn).unwrap();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(handlers::index)
            .service(handlers::tasks)
        // .app_data(web::Json::<models::Model>::configure(|cfg| {
        //     cfg.error_handler(json_error_handler)
        // }))
    })
    .bind("127.0.0.1:22022")?
    .run()
    .await
}
