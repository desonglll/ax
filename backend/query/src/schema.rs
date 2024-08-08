// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Uuid,
        name -> Varchar,
        path -> Varchar,
        size -> Int8,
        content_type -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        user_id -> Int4,
        description -> Nullable<Text>,
        checksum -> Varchar,
        is_deleted -> Bool,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Int4,
        reply_to -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        user_name -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        full_name -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        last_login -> Nullable<Timestamp>,
        is_active -> Bool,
        is_admin -> Bool,
        profile_picture -> Nullable<Uuid>,
    }
}

diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    files,
    posts,
    users,
);
