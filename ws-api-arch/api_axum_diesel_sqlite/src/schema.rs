// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        age -> Nullable<Integer>,
        dob -> Nullable<Date>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
