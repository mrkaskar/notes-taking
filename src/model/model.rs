use crate::schema::{notes, users};
use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
struct User {
    id: i32,
    username: String,
    email: String,
    password_hash: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Queryable, Debug)]
struct Note {
    id: i32,
    title: String,
    content: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    user_id: i32,
}

#[derive(Insertable)]
#[table_name = "notes"]
struct NewNote {
    pub title: String,
    pub content: String,
    pub user_id: i32,
}
