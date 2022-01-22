use rocket_dyn_templates::Template;
use serde::Serialize;

#[get("/stazione/<name>")]
pub async fn station_timetable(name: String, db: crate::DbConn) -> Template {
    #[derive(Serialize)]
    struct Ctx {
        categoria: String,
        numero_treno: i32,
        destinazione: String,
        ora_arrivo_destinazione: String,
        orario: String,
        ritardo: i32,
        binario: i8,
    }
    let cols = db
        .run(move |conn| {
            conn
        .query(
            "SELECT Categoria, RitardoPdP.Numero, Orario, RitardoTreno.Ritardo, PdPStazione.* FROM RitardoPdP JOIN RitardoTreno ON RitardoTreno.Numero = RitardoPdP.Numero JOIN PdPStazione ON PdPStazione.IDPdP = RitardoPdP.IDPdP WHERE PdPStazione.Nome = $1 AND data IS NULL;",
            &[&name],
        )
        .unwrap()
        })
        .await;
    Template::render(
        "station_timetable",
        &cols
            .iter()
            .map(|x| Ctx {
                categoria: x.get("categoria"),
                numero_treno: x.get("numero"),
                destinazione: "todo!()".to_owned(),
                ora_arrivo_destinazione: "todo!()".to_owned(),
                orario: x.get("orario"),
                ritardo: x.get("ritardo"),
                binario: x.get("binario"),
            })
            .collect::<Vec<_>>(),
    )
}
