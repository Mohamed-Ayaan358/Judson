use crate::schema::jud;
use diesel::prelude::*;

#[derive(Debug, PartialEq, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::jud)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Jud {
    pub id: String,
    pub abspath: String,
    pub zipmethod: String,
}

// #[derive(Insertable)]
// #[diesel(table_name = crate::schema::jud)]
// pub struct NewRecord {
//     pub
// }
