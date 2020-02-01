use crate::Db;

#[get("/profiles")]
pub fn list(_conn: Db) -> &'static str {
    "all profiles"
}

#[get("/profiles/<id>")]
pub fn show(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "profile details"
}

#[post("/profiles")]
pub fn create(_conn: Db) -> &'static str {
    "create profile"
}

#[put("/profiles/<id>")]
pub fn update(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "update profile"
}

#[delete("/profiles/<id>")]
pub fn delete(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "delete profile"
}
