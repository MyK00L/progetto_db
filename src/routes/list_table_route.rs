use rocket_dyn_templates::Template;
use serde::Serialize;

#[get("/list/<tablename>?<q>&<c>")]
pub async fn list_table(
    tablename: String,
    q: Option<String>,
    c: Option<String>,
    db: crate::DbConn,
) -> Option<Template> {
    #[derive(Debug, Serialize)]
    struct Column {
        name: String,
        sql_type: String,
    }
    #[derive(Debug, Serialize)]
    struct Table {
        name: String,
        cols: Vec<Column>,
        data: Vec<Vec<String>>,
    }

    let tname1 = tablename.clone();
    let tname2 = tablename.clone();
    let cols: Vec<Column> = db
        .run(move |conn| {
            conn.query(
                "SELECT column_name, data_type FROM information_schema.columns WHERE table_name like $1",
                &[&tname1],
            )
            .unwrap()
        })
        .await
        .iter()
        .map(|col| {
            let column_name: String = col.get("column_name");
            Column {name:column_name, sql_type: col.get("data_type") }
        })
        .collect();

    let data = db
        .run(move |conn| {
            if let Some((q, c)) = q.zip(c) {
                conn.query(
                    &format!(
                        "SELECT * FROM {} WHERE LOWER({}::varchar(255)) LIKE $1",
                        tname2, c
                    ),
                    &[&format!("%{}%", q.to_lowercase())],
                )
                .unwrap()
            } else {
                conn.query(&format!("SELECT * FROM {}", tname2), &[])
                    .unwrap()
            }
        })
        .await
        .iter()
        .map(|x| {
            cols.iter()
                .map(|y| crate::utils::get_sql(x, y.name.as_str()))
                .collect()
        })
        .collect();

    let context = Table {
        name: tablename,
        cols,
        data,
    };
    Some(Template::render("list", context))
}
