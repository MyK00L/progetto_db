use std::fmt::Display;

pub fn get_sql<I>(row: &postgres::row::Row, idx: I) -> String
where
    I: postgres::row::RowIndex + Display,
{
    if let Ok(x) = row.try_get::<_, String>(idx) {
        return x;
    }
    if let Ok(x) = row.try_get::<_, i32>(idx) {
        return format!("{}", x);
    }
    if let Ok(x) = row.try_get::<_, f32>(idx) {
        return format!("{}", x);
    }
    if let Ok(x) = row.try_get::<_, bool>(idx) {
        return match x {
            true => "V",
            false => "F",
        }
        .to_string();
    }
    panic!();
}
