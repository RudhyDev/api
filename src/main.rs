#[macro_use]
extern crate diesel;
extern crate diesel_codegen;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod models;


fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url).unwrap();

    let book = models::NewBook {
        title: String::from("Meu livro 01"),
        author: String::from("Rudhy Pereira"),
        published: true,
    };

    if models::Book::insert(book, &conn){
        println!("Success!");
    } else {
        println!("Failed!");
    }
}
