use std::fmt::Display;

pub fn get_sql<I>(row: &postgres::row::Row, idx: I) -> String
where
    I: postgres::row::RowIndex + Display,
{
    if let Ok(x) = row.try_get::<_, String>(&idx) {
        return x;
    }
    if let Ok(x) = row.try_get::<_, i32>(&idx) {
        return format!("{}", x);
    }
    if let Ok(x) = row.try_get::<_, f32>(&idx) {
        return format!("{}", x);
    }
    if let Ok(x) = row.try_get::<_, chrono::NaiveTime>(&idx) {
        return x.format("%H:%M").to_string();
    }
    if let Ok(x) = row.try_get::<_, chrono::NaiveDate>(&idx) {
        return x.format("%d/%M/%Y").to_string();
    }
    if let Ok(x) = row.try_get::<_, chrono::NaiveDateTime>(&idx) {
        return x.format("%d/%M/%Y %H:%M").to_string();
    }
    if let Ok(x) = row.try_get::<_, Option<chrono::NaiveTime>>(&idx) {
        return x.map(|y| y.format("%H:%M").to_string()).unwrap_or_default();
    }
    if let Ok(x) = row.try_get::<_, Option<chrono::NaiveDate>>(&idx) {
        return x
            .map(|y| y.format("%d/%M/%Y").to_string())
            .unwrap_or_default();
    }
    if let Ok(x) = row.try_get::<_, Option<chrono::NaiveDateTime>>(&idx) {
        return x
            .map(|y| y.format("%d/%M/%Y %H:%M").to_string())
            .unwrap_or_default();
    }
    if let Ok(x) = row.try_get::<_, bool>(&idx) {
        return match x {
            true => "1",
            false => "0",
        }
        .to_string();
    }
    panic!("Failed to convert `{}` to string", idx);
}
