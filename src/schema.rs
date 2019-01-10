table! {
    users (id) {
        id -> Int4,
        email -> Text,
        username -> Text,
        password -> Text,
        is_banned -> Bool,
        is_deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
