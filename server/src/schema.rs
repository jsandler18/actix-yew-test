table! {
    todos (id) {
        id -> Int4,
        created_on -> Timestamp,
        item -> Text,
        due -> Nullable<Timestamp>,
        completed_on -> Nullable<Timestamp>,
        owner -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password_hash -> Text,
    }
}

joinable!(todos -> users (owner));

allow_tables_to_appear_in_same_query!(
    todos,
    users,
);
