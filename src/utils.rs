use std::fmt::Display;

pub fn get_sql<I>(row: &postgres::row::Row, idx: I, sql_type: &str) -> String
where
    I: postgres::row::RowIndex + Display,
{
    match sql_type {
        "bool" => {
            let x: bool = row.get(idx);
            match x {
                true => "V",
                false => "F",
            }
            .to_string()
        }
        "integer" | "INT" => {
            let x: i32 = row.get(idx);
            format!("{}", x)
        }
        "real" => {
            let x: f32 = row.get(idx);
            format!("{}", x)
        }
        "time without time zone" => {
            let x: chrono::NaiveTime = row.get(idx);
            x.format("%H:%M").to_string()
        }
        "text" => row.get(idx),
        _ => panic!("{}, {}", idx, sql_type), //Default::default(),
    }
}
