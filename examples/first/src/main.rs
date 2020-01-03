#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_model_macros;

use diesel::prelude::*;
use diesel_model::Model;
use std::env;

mod models;
mod schema;

pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn create_post(connection: &MysqlConnection) -> QueryResult<usize> {
    use crate::schema::posts::dsl::*;

    diesel::insert_into(posts)
        .values((
            title.eq("Test Title"),
            body.eq("Test Body"),
            published.eq(true),
        ))
        .execute(connection)
}

fn load_all_published_posts(connection: &MysqlConnection) -> QueryResult<Vec<models::Post>> {
    use crate::models::Post;
    use crate::schema::posts::dsl::*;

    models::Post::all()
        .filter(published.eq(true))
        .load::<Post>(connection)
}

fn main() {
    let connection = &establish_connection();

    let created_count = create_post(connection).expect("Error creating posts");
    println!("Created {} posts", created_count);

    let posts = load_all_published_posts(connection).expect("Error loading posts");
    println!("Displaying {} posts", posts.len());
}
