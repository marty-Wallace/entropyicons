
use diesel::prelude::*;
use diesel::pg::PgConnection;
use models::ImageSet;
use schema::image_set::dsl::*;
use dotenv::dotenv;
use std::env;

/// Returns a connection object to the Postgres Database.
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

/// Returns all of the current active imagesets in the database.
///
/// # Arguments
/// * connection - A connection object to the Postgres Database
///
pub fn active_imagesets(connection: &PgConnection) -> Vec<ImageSet> {
    image_set.filter(active.eq(true))
        .load::<ImageSet>(connection)
        .expect("Error loading image sets")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection() {
        establish_connection();
    }

    #[test]
    fn test_active_imagesets_query() {
        let conn = establish_connection();
        let imagesets = active_imagesets(&conn);

        println!("Current active imagesets:");
        for im in imagesets {
            println!("  {}", im.name);
        }
    }

}
