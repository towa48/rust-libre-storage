table! {
    files (id) {
        id -> Integer,
        name -> Text,
        folder_id -> Integer,
        owner_id -> Integer,
        date_created -> Timestamp,
        date_validated -> Nullable<Timestamp>,
        sha1_hash -> Text,
    }
}

table! {
    folders (id) {
        id -> Integer,
        name -> Text,
        owner_id -> Integer,
        date_created -> Timestamp,
        date_validated -> Nullable<Timestamp>,
        depth -> Integer,
        lft -> Integer,
        rgt -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        date_created -> Timestamp,
        salt -> Text,
        password_hash -> Text,
        is_admin -> Bool,
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
