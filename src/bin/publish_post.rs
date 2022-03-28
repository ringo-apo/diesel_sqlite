use diesel::prelude::*;

// use diesel_sample::*;
use sample_proj::*;

use std::env::args;

fn main() {
//     use diesel_sample::schema::posts::dsl::{posts, published};
    use sample_proj::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let _ = diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(&connection)
        .unwrap();

    let post: models::Post = posts
        .find(id)
        .first(&connection)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));

    println!("Published post {}", post.title);
}

