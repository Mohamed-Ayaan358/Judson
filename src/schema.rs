// @generated automatically by Diesel CLI.

diesel::table! {
    jud (id) {
        id -> Integer,
        #[max_length = 100]
        abspath -> Varchar,
        #[max_length = 100]
        zip -> Varchar,
    }
}
