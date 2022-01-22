#[get("/stazione/<name>")]
pub async fn station_timetable(name: String, db: crate::DbConn) -> () {
    let mut context = Vec::<(String, String)>::new();
    //context.push(("table.name".to_owned(), tablename.to_owned()));
    let cols = db
        .run(move |conn| {
            conn
        .query(
            "SELECT Categoria, RitardoPdP.Numero, Orario, RitardoTreno.Ritardo, PdPStazione.* FROM RitardoPdP JOIN RitardoTreno ON RitardoTreno.Numero = RitardoPdP.Numero JOIN PdPStazione ON PdPStazione.IDPdP = RitardoPdP.IDPdP WHERE = PdPStazione.Nome = $1 AND data IS NULL;",
            &[&name],
        )
        .unwrap()
        })
        .await;
    dbg!("{:?}", cols);
    /*return;
    for col in cols {
        let column_name: String = col.get("column_name");
        let column_type: String = col.get("column_name");
        let is_nullable: String = col.get("column_name");
        context.push((
            format!("table.fields[{}].name", &column_name),
            column_name.clone(),
        ));
        context.push((format!("table.fields[{}].type", &column_name), column_type));
        context.push((
            format!("table.fields[{}].is_requeired", &column_name),
            if is_nullable == "NO" { "true" } else { "false" }.to_owned(),
        ));
    }
    Template::render("insert_item", &context)*/
}
