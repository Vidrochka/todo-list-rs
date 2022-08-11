mod utils;
mod handlers;
mod models;
mod middlewares;
mod db;

use actix_web::{
    middleware::Logger,
    get,
    web,
    App,
    HttpServer,
    Responder,
};
use std::{
    env,
    net::Ipv4Addr
};
use slog;

use crate::handlers::*;

#[get("/ping")]
async fn ping(logger: web::Data<slog::Logger>) -> impl Responder {
    slog::info!(logger, "pong");
    format!("pong")
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    dotenv::dotenv().ok();

    let (logger, _scope_guard) = utils::logging::create_logger();

    slog::info!(logger, "Logger created");

    let db_pool = utils::db::prepare_db(&logger).await?;

    let (ip,port) = get_address();
    slog::info!(logger, "Starting server on:[{ip}:{port}] ...");
    
    let actix_logger = logger.clone();
    let actix_db_pool = db_pool.clone();
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(actix_logger.clone()))
            .app_data(web::Data::new(actix_db_pool.clone()))
            .wrap(Logger::default())
            .service(ping)
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/user")
                            .service(
                                web::resource("/register")
                                    .route(web::post().to(register))
                            )
                            .service(
                                web::resource("/login")
                                    .route(web::post().to(login))
                            )
                    )
                    .service(
                        web::scope("/list")
                            .service(
                                web::resource("")
                                    .route(web::get().to(get_list))
                                    .route(web::post().to(new_list))
                                    .route(web::delete().to(delete_list))
                                    .route(web::patch().to(update_list))
                            )
                    )
                    .service(
                        web::scope("/task")
                            .service(
                                web::resource("")
                                    .route(web::get().to(get_tasks))
                                    .route(web::post().to(new_task))
                            )
                            .service(
                                web::resource("range")
                                    .route(web::get().to(get_tasks_range))
                            )
                            .service(
                                web::scope("/{task_id}")
                                    .service(
                                        web::resource("")
                                            .route(web::delete().to(delete_tasks))
                                            .route(web::patch().to(update_task))
                                    )
                                    .service(
                                        web::resource("/move")
                                            .route(web::post().to(move_task))
                                    )
                            )
                    )
            )
    })
    .bind((ip.to_string(), port))?
    .run()
    .await?;

    db_pool.close().await;

    Ok(())
}

const TODO_SERVICE_PORT_ENV: &str = "TODO_SERVICE_PORT";
const TODO_SERVICE_IP_ENV: &str = "TODO_SERVICE_IP";

fn get_address() -> (Ipv4Addr, u16) {
    let ip = env::var(TODO_SERVICE_IP_ENV)
        .expect(&*format!("Env {TODO_SERVICE_IP_ENV} not found"))
        .parse::<Ipv4Addr>()
        .expect(&*format!("Env {TODO_SERVICE_IP_ENV} must be valid ipv4"));

    let port = env::var(TODO_SERVICE_PORT_ENV)
        .expect(&*format!("Env {TODO_SERVICE_PORT_ENV} not found"))
        .parse::<u16>()
        .expect(&*format!("Env {TODO_SERVICE_PORT_ENV} must be valid u32"));

    (ip, port)
}