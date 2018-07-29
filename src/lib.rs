#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;


pub mod schema;
pub mod models;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Post, NewPost};

pub fn create_post<'a>(conn: &SqliteConnection, _title: &'a str, _body: &'a str) -> QueryResult<Post> {
    use schema::posts::dsl::*;

    let new_post = NewPost {
        title: _title,
        body: _body,
    };
    
    conn.transaction::<_, diesel::result::Error, _> (
        || {
            diesel::insert_into(posts)
            .values(&new_post)
            .execute(conn)?;

            posts.order(id.desc()).first::<Post>(conn)
        }
    )
}