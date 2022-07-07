use chrono::NaiveDate;
use rocket_dyn_templates::Template;
use serde::Serialize;

#[get("/train_composition/<id_treno>/<num_days_from_ce>")]
pub async fn train_composition(
    id_treno: i32,
    num_days_from_ce: i32,
    db: crate::DbConn,
) -> Option<Template> {
    #[derive(Debug, Serialize)]
    struct Coach {
        class: i32,
        seats: i32,
    }
    #[derive(Debug, Serialize)]
    struct Context {
        coaches: Vec<Coach>,
        class_1_seats: i32,
        class_2_seats: i32,
    }
    let date = NaiveDate::from_num_days_from_ce(num_days_from_ce);
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
            class: x.get("carrozza.classe"),
            seats: x.get("carrozza.posti"),
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
        coaches,
        class_1_seats,
        class_2_seats,
    };
    Some(Template::render("train_composition", &context))
}
