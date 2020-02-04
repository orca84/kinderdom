use crate::Db;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    pub name: String,
    pub items: Vec<&'static str>,
}

#[get("/events")]
pub fn list(_conn: Db) -> Template {
    let context = TemplateContext {
        name: "events list".to_string(),
        items: vec!["one", "two"],
    };
    Template::render("admin/events/list", &context)
}

#[get("/events/<id>")]
pub fn show(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "event details"
}

#[post("/events")]
pub fn create(_conn: Db) -> &'static str {
    "create event"
}

#[put("/events/<id>")]
pub fn update(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "update event"
}

#[delete("/events/<id>")]
pub fn delete(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "delete event"
}
