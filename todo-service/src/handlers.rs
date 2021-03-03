use super::db_access::*;
use super::models::Todo;
use super::state::AppState;
use std::convert::TryFrom;

use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!(
        " {} - by now, u did it {} times",
        health_check_response, visit_count
    );
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn get_todos_for_owner(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,)>,
) -> HttpResponse {
    let tuple = params.0;
    let owner_id: i32 = i32::try_from(tuple.0).unwrap();
    let todos = get_todos_for_owner_db(&app_state.db, owner_id).await;

    HttpResponse::Ok().json(todos)
}

pub async fn get_todo_details(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    let tuple = params.0;
    let owner_id: i32 = i32::try_from(tuple.0).unwrap();
    let todo_id: i32 = i32::try_from(tuple.1).unwrap();
    let todo = get_todo_details_db(&app_state.db, owner_id, todo_id).await;

    HttpResponse::Ok().json(todo)
}

pub async fn post_new_todo(
    new_todo: web::Json<Todo>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let todo = post_new_todo_db(&app_state.db, new_todo.into()).await;

    HttpResponse::Ok().json(todo)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::NaiveDate;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_todos_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::new(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let owner_id: web::Path<(usize,)> = web::Path::from((1,));
        let resp = get_todos_for_owner(app_state, owner_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_todo_detail_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::new(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let params: web::Path<(usize, usize)> = web::Path::from((1, 2));
        let resp = get_todo_details(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn post_todo_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::new(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let new_todo_msg = Todo {
            todo_id: 1,
            owner_id: 1,
            todo_name: "This is the next todo".into(),
            posted_time: Some(NaiveDate::from_ymd(2020, 9, 17).and_hms(14, 01, 11)),
        };
        let todo_param = web::Json(new_todo_msg);
        let resp = post_new_todo(todo_param, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
