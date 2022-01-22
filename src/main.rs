#[macro_use]
extern crate rocket;
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::postgres::Client;
mod db;

#[database("database")]
struct DbConn(Client);

#[get("/insert/<tablename>")]
async fn insert_item(tablename: String, db: DbConn) -> Template {
    let mut context = vec![("table.name".to_owned(), tablename.to_owned())];
    let cols = db
        .run(move |conn| {
            conn
        .query(
            "SELECT column_name, data_type FROM information_schema.columns WHERE table_name = $1",
            &[&tablename],
        ).unwrap()
        })
        .await;
    for col in cols {
        let column_name: String = col.get("column_name");
        let column_type: String = col.get("column_name");
        let is_nullable: String = col.get("column_name");
        context.push((
            format!("table.fields[{}].name", &column_name),
            column_name.clone(),
        ));
        context.push((format!("table.fields[{}].type", &column_name), column_type));
        context.push((
            format!("table.fields[{}].is_requeired", &column_name),
            if is_nullable == "NO" { "true" } else { "false" }.to_owned(),
        ));
    }
    Template::render("insert_item", &context)
}

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
            std::env::var("DB_USER").unwrap(),
            std::env::var("DB_PASSWORD").unwrap(),
            std::env::var("DB_HOST").unwrap(),
            std::env::var("DB_NAME").unwrap(),
        ),
    );
    rocket::build()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .mount("/", routes![insert_item])
}
