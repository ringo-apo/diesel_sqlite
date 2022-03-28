use diesel::prelude::*;

// use diesel_sample::*;
use sample_proj::*;

use std::env::args;

fn main() {
//    use diesel_sample::schema::posts::dsl::*;
    use sample_proj::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}

