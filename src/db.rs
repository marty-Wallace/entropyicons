
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

use std::env;
use super::models::{Post, NewPost};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))

}

pub fn create_post(conn: &PgConnection, title: &str, body: &str) -> Post {
    use super::schema::posts;

    let new_post = NewPost {
        title,
        body,
    };

    diesel::insert(&new_post).into(posts::table)
        .get_result(conn)
        .expect("Error saving new post")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection() {
        establish_connection();
    }

    #[test]
    fn test_insert() {
        let conn = establish_connection();
        let title = "A test post";
        let body = "The body of the test post";
        create_post(&conn, title, body);
    }

}