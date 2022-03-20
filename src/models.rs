use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use schema::books;
use schema::books::dsl::books as all_books;

#[derive(Queryable)]
pub struct Book {
  pub id: i32,
  pub title: String,
  pub author: String,
  pub published: bool,
}

#[derive(Insertable)]
#[table_name="books"]
pub struct NewBook {
  pub title: String,
  pub author: String,
  pub published: bool,
}