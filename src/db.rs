use once_cell::sync::Lazy;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};

pub type ConnManager = r2d2_postgres::PostgresConnectionManager<NoTls>;
pub static POOL: Lazy<r2d2::Pool<ConnManager>> = Lazy::new(|| {
    let manager = PostgresConnectionManager::new(
        format!(
            "host={} user={} password={} dbname={}",
            std::env::var("DB_HOST").expect("DB_HOST undefined"),
            std::env::var("DB_USER").expect("DB_USER undefined"),
            std::env::var("DB_PASSWORD").expect("DB_PASSWORD undefined"),
            std::env::var("DB_NAME").expect("DB_NAME undefined"),
        )
        .parse()
        .unwrap(),
        NoTls,
    );
    r2d2::Pool::new(manager).unwrap()
});
pub fn create_db() -> Result<(), String> {
    let pool = POOL.clone();
    let mut client = pool.get().map_err(|x| format!("{:?}", x))?;
    let queries = include_str!("../queries/create_db.sql")
        .split(';')
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .map(|x| x.replace('§', ";"));
    for query in queries {
        client
            .execute(&format!("{};", query), &[])
            .map_err(|x| format!("{:?}", x))?;
    }
    Ok(())
}
