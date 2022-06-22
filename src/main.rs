#[macro_use]
extern crate rocket;
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::postgres::Client;
use serde::Serialize;
mod db;
mod routes;

#[database("database")]
pub struct DbConn(Client);

#[get("/train_status/<train_number>")]
async fn train_status(train_number: i32, db: DbConn) -> Template {
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
async fn insert_item(tablename: String, db: DbConn) -> Template {
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
    let cacca = tablename.clone();
    let cols = db
        .run(move |conn| {
            conn
        .query(
            "SELECT column_name, data_type FROM information_schema.columns WHERE table_name like $1",
            &[&cacca],
        ).unwrap()
        })
        .await;
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
        .mount(
            "/",
            routes![insert_item, train_status, routes::station_timetable],
        )
}
