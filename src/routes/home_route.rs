use rocket_dyn_templates::Template;

#[get("/")]
pub async fn home() -> Template {
    let ctx: std::collections::HashMap<u8, u8> = Default::default();
    Template::render("home", ctx)
}
