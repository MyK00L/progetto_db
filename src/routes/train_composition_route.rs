use rocket_dyn_templates::Template;
use serde::Serialize;

#[get("/train_composition/<id_treno>/<date>")]
pub async fn train_composition(id_treno: i32, date: String, db: crate::DbConn) -> Option<Template> {
    let coaches: Vec<i32> = db
        .run(move |conn| {
            conn.query(
                "select * from composizione where idtreno = $1 and data = $2",
                &[&id_treno, &date],
            )
            .unwrap()
        })
        .await
        .iter()
        .map(|x| {
            let id_carrozza: i32 = x.get("carrozza.id");
            id_carrozza
        })
        .collect();
    eprintln!("{:?}", coaches);
    todo!();
}
