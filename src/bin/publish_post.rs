extern crate diesel_demo_sqlite as demo_sqlite;
extern crate diesel;

use self::diesel::prelude::*;
use self::demo_sqlite::*;
use self::models::Post;
use std::env::args;

fn main() {
    use demo_sqlite::schema::posts::dsl::{posts, published, id};

    let _id = args().nth(1).expect("publish_post pequired a post id")
    .parse::<i32>().expect("Invalid ID");
    let conn = establish_connection();
    
    let post = conn.transaction::<_, diesel::result::Error, _>(
        || {
            diesel::update(posts.find(_id))
            .set(published.eq(true))
            .execute(&conn)?;

            posts.filter(id.eq(_id))
            .first::<Post>(&conn)
        }
    );

    match post {
        Ok(save_post) => println!("Published post {}", save_post.title),
        Err(_) => println!("Unable to find post {}", _id),
    }
}