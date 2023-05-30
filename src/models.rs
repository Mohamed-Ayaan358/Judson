use diesel::prelude::*;

#[derive(Debug, PartialEq, Queryable, Selectable)]
#[diesel(table_name = crate::schema::jud)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Jud {
    pub id: i32,
    pub abspath: String,
    pub zip: String,
}
