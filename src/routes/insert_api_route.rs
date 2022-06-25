use rocket::form::Form;
use rocket::form::Strict;
use std::collections::HashMap;

#[derive(FromForm, Debug)]
pub struct InsertItem {
    table: String,
    columns: HashMap<String, String>,
}
#[post("/api/insert", data = "<stuff>")]
pub async fn insert_api(stuff: Form<Strict<InsertItem>>, db: crate::DbConn) -> String {
    let cols = stuff
        .columns
        .iter()
        .map(|x| x.0.clone())
        .collect::<Vec<String>>()
        .join(", ");
    let vals = stuff
        .columns
        .iter()
        .map(|x| format!("'{}'", x.1))
        .collect::<Vec<String>>()
        .join(", ");
    let query = format!("insert into {} ({}) values ({});", stuff.table, cols, vals);
    eprintln!("{}", query);
    let ans = db.run(move |conn| conn.query(&query, &[]).unwrap()).await;
    eprintln!("{:?}", ans);
    String::from("halp")
}
