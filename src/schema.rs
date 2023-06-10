// @generated automatically by Diesel CLI.

diesel::table! {
    jud (id) {
        #[max_length = 100]
        id -> Varchar,
        #[max_length = 100]
        abspath -> Varchar,
        #[max_length = 100]
        zipmethod -> Varchar,
    }
}
