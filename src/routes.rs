use crate::utils;
use regex::Regex;
use rocket::form::Form;
use rocket::form::Strict;
use rocket_dyn_templates::Template;
use serde::Serialize;
use std::collections::HashMap;

#[get("/stazione/<name>")]
pub async fn station_timetable(name: String, db: crate::DbConn) -> Template {
    #[derive(Serialize)]
    struct Ctx {
        rows: Vec<CtxLine>,
        station: String,
    }
    #[derive(Serialize)]
    struct CtxLine {
        categoria: String,
        numero_treno: i32,
        destinazione: String,
        ora_arrivo_destinazione: String,
        orario: chrono::NaiveDateTime,
        ritardo: f64,
        binario: String,
    }
    let station = name.clone();
    let cols = db
        .run(move |conn| {
            conn
        .query(
            "SELECT Categoria, RitardoPdP.Numero, Orario, RitardoTreno.Ritardo, PdPStazione.*, dst.Nome AS destinazione FROM RitardoPdP JOIN RitardoTreno ON RitardoTreno.Numero = RitardoPdP.Numero JOIN PdPStazione ON PdPStazione.IDPdP = RitardoPdP.IDPdP JOIN DestinazioneTreno dst ON dst.numero = RitardoPdP.Numero WHERE PdPStazione.Nome = $1 AND data IS NULL;",
            &[&name],
        )
        .unwrap()
        })
        .await;
    Template::render(
        "station_timetable",
        &Ctx {
            station,
            rows: cols
                .iter()
                .map(|x| CtxLine {
                    categoria: x.get("categoria"),
                    numero_treno: x.get("numero"),
                    destinazione: x.get("destinazione"),
                    ora_arrivo_destinazione: "todo!()".to_owned(),
                    orario: x.get("orario"),
                    ritardo: x.get("ritardo"),
                    binario: x.get("binario"),
                })
                .collect::<Vec<_>>(),
        },
    )
}

#[get("/train_status/<train_number>")]
pub async fn train_status(train_number: i32, db: crate::DbConn) -> Template {
    #[derive(Debug, Serialize)]
    struct Item {
        name: String,
        scheduled_arrival: chrono::NaiveDateTime,
        arrival: Option<chrono::NaiveDateTime>,
    }
    #[derive(Debug, Serialize)]
    struct Context {
        numero: i32,
        categoria: String,
        ultimo_pdp_nome: Option<String>,
        ritardo: i16,
        ultimo_pdp_orario: Option<chrono::NaiveDateTime>,
        items: Vec<Item>,
    }
    let (ultimo_pdp_nome, ultimo_pdp_orario, ritardo) = db
        .run(move |conn| {
            conn.query("SELECT rpdp.ritardo AS r, rpdp.data AS orario, pdps.nome FROM RitardoPdP rpdp LEFT JOIN PdPStazione pdps on rpdp.idpdp = pdps.idpdp WHERE rpdp.numero = $1 ORDER BY orario DESC;", &[&train_number])
                .unwrap()
        })
        .await.iter().map(|x| (x.get("nome"),
        {
            let y: Option<chrono::NaiveDateTime> = x.get("orario");
            y
        }, {
            let a: f64 = x.get("r");
            a
        } as i16)).find(|x| (x.1).is_some()).unwrap_or_default();

    let categoria = db
        .run(move |conn| {
            conn.query(
                "SELECT Categoria FROM Treno WHERE Numero = $1;",
                &[&train_number],
            )
            .unwrap()
        })
        .await
        .iter()
        .map(|col| col.get("Categoria"))
        .next()
        .unwrap();
    let cols = db
        .run(move |conn| {
            conn
        .query(
            "SELECT orario, data, Nome FROM PdPStazione, RitardoPdP WHERE RitardoPdP.numero = $1 AND RitardoPdP.idpdp = PdPStazione.IDPdP ORDER BY orario",
            &[&train_number],
        ).unwrap()
        })
        .await;
    let context = Context {
        numero: train_number,
        categoria,
        ultimo_pdp_nome,
        ultimo_pdp_orario,
        ritardo,
        items: cols
            .iter()
            .map(|col| {
                let orario: chrono::NaiveDateTime = col.get("orario");
                let data: Option<chrono::NaiveDateTime> = col.get("data");
                let nome: String = col.get("Nome");
                Item {
                    name: nome,
                    scheduled_arrival: orario,
                    arrival: data,
                }
            })
            .collect(),
    };
    Template::render("train_status", &context)
}

#[get("/insert/<tablename>")]
pub async fn insert_item(tablename: String, db: crate::DbConn) -> Option<Template> {
    #[derive(Debug, Serialize)]
    struct Column {
        name: String,
        r#type: String,
        is_required: bool,
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
    eprintln!("{}", table_type);
    if table_type != "BASE TABLE" {
        return None; // TODO: proper error
    }
    let tname1 = tablename.clone();
    let cols = db
        .run(move |conn| {
            conn
        .query(
            "SELECT column_name, data_type FROM information_schema.columns WHERE table_name like $1",
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
    for i in fks.iter() {
        let text: String = i.get(0);
        let cap = re.captures(&text).unwrap();
        let col = String::from(&cap[1]);
        let table = String::from(&cap[2]);
        let id = String::from(&cap[3]);
        let shit: Vec<String> = db
            .run(move |conn| {
                conn.query(&format!("SELECT {} FROM {};", id, table), &[])
                    .unwrap()
            })
            .await
            .iter()
            .map(|x| utils::get_sql(x, 0))
            .collect();
        eprintln!("{:?}", shit);
    }
    let context = Table {
        name: tablename,
        cols: cols
            .iter()
            .map(|col| {
                let column_name: String = col.get("column_name");
                let column_type: String = col.get("data_type");
                let is_nullable: String = "YES".to_owned(); //col.get("is_nullable");
                Column {
                    name: column_name,
                    r#type: column_type, // not
                    is_required: is_nullable == "NO",
                }
            })
            .collect(),
    };
    eprintln!("{:?}", context);
    Some(Template::render("insert_item", &context))
}

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

#[get("/list/<tablename>?<q>")]
pub async fn list_table(
    tablename: String,
    q: Option<String>,
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
    eprintln!("{}", table_type);
    if table_type != "BASE TABLE" {
        return None; // TODO: proper error
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
            conn.query(&format!("SELECT * FROM {}", tname2), &[])
                .unwrap()
        })
        .await
        .iter()
        // FIXME This doesn't work because everything is **casted** to a string
        .map(|x| {
            cols.iter()
                .map(|y| crate::utils::get_sql(&x, y.name.as_str()))
                .collect()
        })
        .collect();

    let context = Table {
        name: tablename,
        cols,
        data,
    };
    eprintln!("{:?}", context);
    Some(Template::render("list", &dbg!(context)))
}
