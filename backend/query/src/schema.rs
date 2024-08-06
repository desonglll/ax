// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Uuid,
        name -> Nullable<Varchar>,
        path -> Nullable<Varchar>,
        size -> Nullable<Int8>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        user_id -> Nullable<Int8>,
        description -> Nullable<Text>,
        checksum -> Nullable<Varchar>,
        is_deleted -> Nullable<Bool>,
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

diesel::allow_tables_to_appear_in_same_query!(
    files,
    users,
);
