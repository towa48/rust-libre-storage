table! {
    files (id) {
        id -> BigInt,
        name -> Text,
        folder_id -> BigInt,
        owner_id -> BigInt,
        date_created -> Timestamp,
        date_validated -> Nullable<Timestamp>,
        sha1_hash -> Text,
    }
}

table! {
    folders (id) {
        id -> BigInt,
        name -> Text,
        owner_id -> BigInt,
        date_created -> Timestamp,
        date_validated -> Nullable<Timestamp>,
        depth -> Integer,
        lft -> Integer,
        rgt -> Integer,
    }
}

table! {
    users (id) {
        id -> BigInt,
        username -> Text,
        date_created -> Timestamp,
        salt -> Text,
        password_hash -> Text,
        is_admin -> Bool,
        should_initialize -> Bool,
    }
}

joinable!(files -> folders (folder_id));
joinable!(files -> users (owner_id));
joinable!(folders -> users (owner_id));

allow_tables_to_appear_in_same_query!(
    files,
    folders,
    users,
);
