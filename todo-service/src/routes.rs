use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
   cfg.route("/health", web::get().to(health_check_handler));
}

pub fn todo_routes(cfg: &mut web::ServiceConfig) {
   cfg.service(
       web::scope("/todos")
           .route("/", web::post().to(post_new_todo))
           .route("/{owner_id}", web::get().to(get_todos_for_owner))
           .route("/{owner_id}/{todo_id}", web::get().to(get_todo_details)),
   );
}