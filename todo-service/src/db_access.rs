use super::models::Todo;
use sqlx::postgres::PgPool;


pub async fn get_todos_for_owner_db(pool: &PgPool, owner_id: i32) -> Vec<Todo> {
    // Prepare SQL statement
    let todo_rows = sqlx::query!(
        "SELECT owner_id, todo_id, todo_name, posted_time FROM todos where owner_id = $1",
        owner_id
    )
    .fetch_all(pool)
    .await
    .unwrap();
    // Extract result
    todo_rows
        .iter()
        .map(|todo_row| Todo {
            todo_id: todo_row.todo_id,
            owner_id: todo_row.owner_id,
            todo_name: todo_row.todo_name.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(todo_row.posted_time.unwrap())),
        })
        .collect()
 }

 pub async fn get_todo_details_db(pool: &PgPool, owner_id: i32, todo_id: i32) -> Todo {
    let todo_row = sqlx::query!(
        "SELECT owner_id, todo_id, todo_name, posted_time FROM todos where owner_id = $1 and todo_id = $2",
        owner_id, todo_id
    )
    .fetch_one(pool)
    .await
    .unwrap();
    // Execute query
    Todo {
        todo_id: todo_row.todo_id,
        owner_id: todo_row.owner_id,
        todo_name: todo_row.todo_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(todo_row.posted_time.unwrap())),
    }
 }

 pub async fn post_new_todo_db(pool: &PgPool, new_todo: Todo) -> Todo {

    let todo_row = sqlx::query!("insert into todos (todo_id,owner_id, todo_name) values ($1,$2,$3) returning owner_id, todo_id,todo_name, posted_time", new_todo.todo_id, new_todo.owner_id, new_todo.todo_name)
    .fetch_one(pool)
    .await.unwrap();

    Todo {
        todo_id: todo_row.todo_id,
        owner_id: todo_row.owner_id,
        todo_name: todo_row.todo_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(todo_row.posted_time.unwrap())),
    }
 }