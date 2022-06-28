use crate::utils;
use regex::Regex;
use rocket_dyn_templates::Template;
use serde::Serialize;

#[get("/insert/<tablename>")]
pub async fn insert_item(tablename: String, db: crate::DbConn) -> Option<Template> {
    #[derive(Debug, Serialize)]
    struct Column {
        name: String,
        r#type: String,
        is_required: bool,
        options: Option<Vec<String>>,
        enum_options: Option<Vec<String>>,
    }
    #[derive(Debug, Serialize)]
    struct Table {
        name: String,
        cols: Vec<Column>,
    }
    let tname0 = tablename.clone();
    let table_type: String = db
        .run(move |conn| {
            conn.query(
                "SELECT table_type FROM information_schema.tables WHERE table_name like $1",
                &[&(tname0)],
            )
            .unwrap()
        })
        .await
        .get(0)
        .map(|x| x.get("table_type"))
        .unwrap_or_else(|| String::from("N"));
    if table_type != "BASE TABLE" {
        return None;
    }
    let tname1 = tablename.clone();
    let cols = db
        .run(move |conn| {
            conn
        .query(
            "SELECT column_name, data_type, udt_name FROM information_schema.columns WHERE table_name like $1",
            &[&tname1],
        ).unwrap()
        })
        .await;
    let tname2 = tablename.clone();
    let fks = db
        .run(move |conn| {
            conn
        .query(
            &format!("SELECT pg_catalog.pg_get_constraintdef(r.oid, true) as condef FROM pg_catalog.pg_constraint r WHERE r.conrelid = '{}'::regclass AND r.contype = 'f' ORDER BY 1;",tname2),
            &[],
        ).unwrap()
        })
        .await;

    let re = Regex::new(r"^FOREIGN KEY \((\w+)\) REFERENCES (\w+)\((\w+)\)$").unwrap();
    let mut completion = std::collections::HashMap::<String, Vec<String>>::new();
    for i in fks.iter() {
        let text: String = i.get(0);
        let cap = re.captures(&text).unwrap();
        let col = String::from(&cap[1]);
        let table = String::from(&cap[2]);
        let id = String::from(&cap[3]);
        let options: Vec<String> = db
            .run(move |conn| {
                conn.query(&format!("SELECT {} FROM {};", id, table), &[])
                    .unwrap()
            })
            .await
            .iter()
            .map(|x| utils::get_sql(x, 0))
            .collect();
        completion.insert(col, options);
    }
    let mut context = Table {
        name: tablename,
        cols: cols
            .iter()
            .map(|col| {
                let column_name: String = col.get("column_name");
                let column_type: String = col.get("data_type");
                let is_nullable: String = "YES".to_owned(); //col.get("is_nullable");
                Column {
                    options: completion.get(&column_name).map(|x| x.to_owned()),
                    enum_options: None,
                    name: column_name,
                    r#type: column_type, // not
                    is_required: is_nullable == "NO",
                }
            })
            .collect(),
    };
    for (i, col) in cols.iter().enumerate() {
        if context.cols[i].r#type == "USER-DEFINED" {
            let enum_name: String = col.get("udt_name");
            let options: Vec<String> = db
                .run(move |conn| {
                    conn.query(
                        &format!(
                            "SELECT unnest(enum_range(NULL::{}))::text AS values",
                            enum_name
                        ),
                        &[],
                    )
                    .unwrap()
                })
                .await
                .iter()
                .map(|x| x.get("values"))
                .collect();
            context.cols[i].enum_options = Some(options);
        }
    }
    Some(Template::render("insert_item", &context))
}
