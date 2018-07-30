extern crate diesel_demo_sqlite as demo_sqlite;
extern crate diesel;

use self::diesel::prelude::*;
use self::demo_sqlite::*;
use std::env::args;

fn main() {
    use demo_sqlite::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let conn = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
    .execute(&conn)
    .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}