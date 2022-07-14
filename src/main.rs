#[macro_use]
extern crate rocket;
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::postgres::Client;
mod db;
mod routes;
mod utils;

#[database("database")]
pub struct DbConn(Client);

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    std::thread::spawn(|| {
        db::create_db().expect("Failed to create DB");
    });
    std::env::set_var(
        "ROCKET_DATABASES",
        format!(
            "{{database = {{ url = \"postgres://{}:{}@{}/{}\" }}}}",
            std::env::var("DB_USER").expect("DB_USER not found in environment"),
            std::env::var("DB_PASSWORD").expect("DB_PASSWORD not found in environment"),
            std::env::var("DB_HOST").expect("DB_HOST not found in environment"),
            std::env::var("DB_NAME").expect("DB_NAME not found in environment"),
        ),
    );
    rocket::build()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                routes::insert_item,
                routes::insert_api,
                routes::train_status,
                routes::list_table,
                routes::station_timetable,
                routes::home,
                routes::train_composition,
            ],
        )
}
