// @generated automatically by Diesel CLI.

diesel::table! {
    jud (id) {
        id -> Integer,
        abspath -> Nullable<Varchar>,
        zip -> Nullable<Varchar>,
    }
}
