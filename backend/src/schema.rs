// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    rants (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        username -> Text,
        //dateposted -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    rants,
);
