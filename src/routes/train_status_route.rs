use rocket_dyn_templates::Template;
use serde::Serialize;

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
