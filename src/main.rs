// src/main.rs
#[macro_use] extern crate rocket;

mod models;
mod schema;
mod routes;
use rocket_sync_db_pools::database;



#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![routes::create_user, routes::create_post,routes::list_posts])
}
