use chrono::NaiveDate;
use rocket_dyn_templates::Template;
use serde::Serialize;

/*
SELECT * FROM Esercizio JOIN Locomotiva ON Esercizio.idLocomotiva = Locomotiva.ID WHERE Esercizio.idTreno = $1 AND data = $2;
 * */

#[get("/train_composition/<id_treno>/<num_days_from_ce>")]
pub async fn train_composition(
    id_treno: i32,
    num_days_from_ce: i32,
    db: crate::DbConn,
) -> Option<Template> {
    #[derive(Debug, Serialize)]
    struct Locomotive {
        speed: i32,
        power: String,
    }
    #[derive(Debug, Serialize)]
    struct Coach {
        class: i32,
        seats: i32,
    }
    #[derive(Debug, Serialize)]
    struct Context {
        locomotive: Locomotive,
        coaches: Vec<Coach>,
        class_1_seats: i32,
        class_2_seats: i32,
    }
    let date = NaiveDate::from_num_days_from_ce(num_days_from_ce);
    let locomotive: Locomotive = db.run(move |conn| {
        conn.query("SELECT * FROM Esercizio JOIN Locomotiva ON Esercizio.idLocomotiva = Locomotiva.ID WHERE Esercizio.idTreno = $1 AND data = $2", &[&id_treno, &date]).unwrap()
    }).await.iter().map(|x| Locomotive {speed: x.get("velocita"), power: x.get("tensione"),}).next()?;
    let coaches: Vec<Coach> = db
        .run(move |conn| {
            conn.query(
                "select * from composizione where idtreno = $1 and data = $2",
                &[&id_treno, &date],
            )
            .unwrap()
        })
        .await
        .iter()
        .map(|x| Coach {
            class: x.get("classe"),
            seats: x.get("posti"),
        })
        .collect();
    let class_1_seats: i32 = coaches
        .iter()
        .filter(|x| x.class == 1)
        .map(|x| x.seats)
        .sum();
    let class_2_seats: i32 = coaches
        .iter()
        .filter(|x| x.class == 2)
        .map(|x| x.seats)
        .sum();
    let context = Context {
        locomotive,
        coaches,
        class_1_seats,
        class_2_seats,
    };
    Some(Template::render("train_composition", &context))
}
