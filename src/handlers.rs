use crate::db::{add_task, query_tasks, update_task, Pool};
use actix_web::{get, patch, post, web, HttpResponse, Responder};
use raskd::models::{Incoming, Outgoing, QueryParams};

#[post("/task")]
async fn post_tasks(db: web::Data<Pool>, data: web::Json<Incoming>) -> impl Responder {
    let conn = db.clone().get().unwrap();

    let res = add_task(conn, data.into_inner());
    match res {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => HttpResponse::Ok().json(Outgoing::Error {
            msg: format!("{}", e),
        }),
    }
}

#[get("/task")]
async fn get_tasks(db: web::Data<Pool>, param: web::Query<QueryParams>) -> impl Responder {
    let conn = db.clone().get().unwrap();

    match query_tasks(conn, param.into_inner()) {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => HttpResponse::Ok().json(Outgoing::Error {
            msg: format!("{}", e),
        }),
    }
}

#[patch("/task")]
async fn patch_task(db: web::Data<Pool>, data: web::Json<Incoming>) -> impl Responder {
    let conn = db.clone().get().unwrap();

    match update_task(conn, data.into_inner()) {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => HttpResponse::Ok().json(Outgoing::Error {
            msg: format!("{}", e),
        }),
    }
}
