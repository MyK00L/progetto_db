use std::fmt::Display;

pub fn get_sql<I>(row: &postgres::row::Row, idx: I, sql_type: &str) -> String
where
    I: postgres::row::RowIndex + Display,
{
    match sql_type {
        "BOOL" => {
            let x: bool = row.get(idx);
            match x {
                true => "V",
                false => "F",
            }
            .to_string()
        }
        "INT" => {
            let x: i32 = row.get(idx);
            format!("{}", x)
        }
        "REAL" => {
            let x: f32 = row.get(idx);
            format!("{}", x)
        }
        "TEXT" => row.get(idx),
        _ => panic!(), //Default::default(),
    }
}
