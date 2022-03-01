// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Int8,
        shorten -> Varchar,
        link_type -> Int4,
        url -> Text,
        created_by -> Nullable<Int8>,
        updated_by -> Nullable<Int8>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        updated_by -> Nullable<Int8>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        status -> Nullable<Int4>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(links, users,);
