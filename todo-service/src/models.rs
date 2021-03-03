use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Todo {
   pub todo_id: i32,
   pub owner_id: i32,
   pub todo_name: String,
   pub posted_time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Todo {
   fn from(todo: web::Json<Todo>) -> Self {
       Course {
           course_id: todo.todo_id,
           owner_id: todo.owner_id,
           todo_name: todo.todo_name.clone(),
           posted_time: todo.posted_time,
       }
   }
}