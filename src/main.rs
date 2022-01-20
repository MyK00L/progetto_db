use rocket::*;
use rocket_dyn_templates::Template;
use postgres::{Client, NoTls};

#[get("/insert/<tablename>")]
fn insert_item(tablename: &str) -> Template {
    let mut context = Vec::<(String,String)>::new();
    context.push(("table.name".to_owned(), tablename.to_owned()));
    let mut client = Client::connect(&format!("host={} user={}",DBHOST,DBUSER), NoTls).unwrap();
    let cols = client.query("SELECT column_name, data_type FROM information_schema.columns WHERE table_name = $1",&[&tablename]).unwrap();
    for col in cols {
        let column_name: String = col.get("column_name");
        let column_type: String = col.get("column_name");
        let is_nullable: String = col.get("column_name");
        context.push((format!("table.fields[{}].name",&column_name), column_name.clone()));
        context.push((format!("table.fields[{}].type",&column_name), column_type));
        context.push((format!("table.fields[{}].is_requeired",&column_name), if is_nullable=="NO" { "true" } else { "false" }.to_owned()));
    }
    Template::render("insert_item",&context)
}

const DBHOST: &str = "localhost";
const DBUSER: &str = "user";

// main
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                insert_item,
            ],
        )
        .attach(Template::fairing())
}

