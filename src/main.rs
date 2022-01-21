#[macro_use]
extern crate rocket;
mod db;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    std::thread::spawn(|| {
        db::create_db().expect("Failed to create DB");
    });
    rocket::build().mount("/", routes![index])
}
