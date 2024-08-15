// @generated automatically by Diesel CLI.

diesel::table! {
    comment_reactions (id) {
        id -> Int4,
        user_id -> Int4,
        comment_id -> Int4,
        created_at -> Timestamptz,
        reaction_id -> Int4,
        reaction_name -> Varchar,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        content -> Text,
        reply_to -> Int4,
        user_id -> Int4,
        user_name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        reactions -> Nullable<Jsonb>,
        reply_to_type -> Varchar,
    }
}

diesel::table! {
    files (id) {
        id -> Uuid,
        name -> Varchar,
        path -> Varchar,
        size -> Int8,
        content_type -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        user_id -> Int4,
        description -> Nullable<Text>,
        checksum -> Varchar,
        is_deleted -> Bool,
    }
}

diesel::table! {
    post_actions (id, name) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
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
        user_name -> Varchar,
        reactions -> Nullable<Jsonb>,
    }
}

diesel::table! {
    reactions (id) {
        id -> Int4,
        user_id -> Int4,
        post_id -> Int4,
        created_at -> Timestamptz,
        reaction_id -> Int4,
        reaction_name -> Varchar,
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
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        last_login -> Nullable<Timestamptz>,
        is_active -> Bool,
        is_admin -> Bool,
        profile_picture -> Nullable<Uuid>,
    }
}

diesel::joinable!(comments -> users (user_id));
diesel::joinable!(posts -> users (user_id));
diesel::joinable!(reactions -> posts (post_id));
diesel::joinable!(reactions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comment_reactions,
    comments,
    files,
    post_actions,
    posts,
    reactions,
    users,
);
