use diesel::Queryable;

#[derive(Queryable)]
pub struct Jud {
    pub id: i32,
    pub abspath: String,
    pub zip: String,
}
