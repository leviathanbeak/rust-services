use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

mod db_access;
mod handlers;
mod models;
mod routes;
mod state;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::new(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "Thanks for checking me out".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(todo_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
