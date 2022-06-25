use rocket_dyn_templates::Template;
use serde::Serialize;

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
            "SELECT Categoria, RitardoPdP.Numero, Orario, RitardoTreno.Ritardo, PdPStazione.*, dst.Nome AS destinazione FROM RitardoPdP JOIN RitardoTreno ON RitardoTreno.Numero = RitardoPdP.Numero JOIN PdPStazione ON PdPStazione.IDPdP = RitardoPdP.IDPdP JOIN DestinazioneTreno dst ON dst.numero = RitardoPdP.Numero WHERE LOWER(PdPStazione.Nome) LIKE $1 AND data IS NULL;",
            &[&format!("%{}%", name.to_lowercase())],
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
