use rocket_dyn_templates::Template;
use serde::Serialize;

#[get("/train_status/<train_number>")]
pub async fn train_status(train_number: i32, db: crate::DbConn) -> Template {
    #[derive(Debug, Serialize)]
    struct Item {
        name: String,
        scheduled_arrival: Option<chrono::NaiveTime>,
        scheduled_departure: Option<chrono::NaiveTime>,
        arrival: Option<chrono::NaiveDateTime>,
        departure: Option<chrono::NaiveDateTime>,
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
    let (ultimo_pdp_nome, ultimo_pdp_orario) = db
        .run(move |conn| {
            conn.query("SELECT rpdp.data AS orario, pdps.nome FROM RitardoPdP rpdp LEFT JOIN PdPStazione pdps on rpdp.idpdp = pdps.idpdp WHERE rpdp.numero = $1 ORDER BY orario DESC;", &[&train_number])
                .unwrap()
        })
        .await.iter().map(|x| (x.get("nome"),x.get::<_,Option<chrono::NaiveDateTime>>("orario"))).find(|x| (x.1).is_some()).unwrap_or_default();
    let ritardo = db
        .run(move |conn| {
            conn.query(
                "select ritardo from ritardotreno where numero = $1",
                &[&train_number],
            )
            .unwrap()
        })
        .await
        .iter()
        .map(|x| x.get::<_, f64>(0) as i16)
        .next()
        .unwrap_or_default();

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
            "SELECT OrarioArrivo, OrarioPartenza, DataArrivo, DataPartenza, Nome FROM PdPStazione, RitardoPdP WHERE RitardoPdP.numero = $1 AND RitardoPdP.idpdp = PdPStazione.IDPdP ORDER BY OrarioArrivo",
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
            .map(|col| Item {
                name: col.get("Nome"),
                scheduled_arrival: col.get("OrarioArrivo"),
                scheduled_departure: col.get("OrarioPartenza"),
                arrival: col.get("DataArrivo"),
                departure: col.get("DataPartenza"),
            })
            .collect(),
    };
    Template::render("train_status", &context)
}
