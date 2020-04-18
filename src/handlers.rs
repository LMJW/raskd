use crate::db::{add_task, Pool};
use crate::models::Model;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use failure::Error;
use futures::{Future, TryFutureExt};
use std::time::SystemTime;

#[get("/{id}/{name}")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    let data = Model::Task {
        id: 1,
        name: "haha".to_string(),
        tasktype: "abc".to_string(),
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    format!("Hello {}! id:{}", info.1, info.0)
}

#[post("/task")]
async fn tasks(db: web::Data<Pool>, data: web::Json<Model>) -> impl Responder {
    let conn = db.clone().get().unwrap();

    let res = add_task(conn, data.into_inner());
    match res {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => HttpResponse::Ok().json(Model::Error {
            msg: format!("{}", e),
        }),
    }
}
