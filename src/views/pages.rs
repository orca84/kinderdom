use crate::Db;
use rocket::Request;
// use rocket::State;
use crate::models::profile::Profile;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[derive(Serialize)]
struct TemplateContext {
    pub name: String,
    pub items: Vec<&'static str>,
}

#[get("/")]
// pub fn index(config: State<Config>) -> Template {
pub fn index(connection: Db) -> Template {
    // let secret = &config.secret;
    // println!("{}", secret);
    // -----------------------------------------------------------

    let ps = Profile::all(&connection);
    for p in ps {
        println!("profile: {:?}", p);
    }

    // -----------------------------------------------------------

    let name = "Denis".to_string();
    let context = TemplateContext {
        name,
        items: vec!["One", "Two", "Three"],
    };
    Template::render("pages/index", &context)
}

#[get("/articles")]
pub fn articles() -> Template {
    let context = TemplateContext {
        name: "".to_string(),
        items: vec![],
    };
    Template::render("pages/articles", &context)
}

#[get("/profiles")]
pub fn profiles() -> Template {
    let context = TemplateContext {
        name: "".to_string(),
        items: vec![],
    };
    Template::render("pages/profiles", &context)
}

#[get("/projects")]
pub fn projects() -> Template {
    let context = TemplateContext {
        name: "".to_string(),
        items: vec![],
    };
    Template::render("pages/projects", &context)
}

#[get("/events")]
pub fn events() -> Template {
    let context = TemplateContext {
        name: "".to_string(),
        items: vec![],
    };
    Template::render("pages/events", &context)
}

#[get("/about")]
pub fn about() -> Template {
    let context = TemplateContext {
        name: "".to_string(),
        items: vec![],
    };
    Template::render("pages/about", &context)
}

#[get("/docs")]
pub fn docs() -> Template {
    let context = TemplateContext {
        name: "".to_string(),
        items: vec![],
    };
    Template::render("pages/docs", &context)
}

#[get("/help")]
pub fn help() -> Template {
    let context = TemplateContext {
        name: "Name".to_string(),
        items: vec!["one", "two"],
    };
    Template::render("pages/help", &context)
}

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}
