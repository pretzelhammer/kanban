table! {
    use diesel::sql_types::*;
    use crate::models::Status_enum;

    boards (id) {
        id -> Int8,
        name -> Text,
        created_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::Status_enum;

    cards (id) {
        id -> Int8,
        board_id -> Int8,
        description -> Text,
        status -> Status_enum,
        created_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::Status_enum;

    tokens (id) {
        id -> Text,
        expired_at -> Timestamptz,
    }
}

joinable!(cards -> boards (board_id));

allow_tables_to_appear_in_same_query!(
    boards,
    cards,
    tokens,
);
