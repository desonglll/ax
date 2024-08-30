use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: i32,
    pub reply_to: Option<i32>,
    pub user_name: String,
    pub reactions: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatePost {
    pub content: String,
    pub user_id: i32,
    pub reply_to: Option<i32>,
    pub user_name: String,
    pub reactions: Option<Value>,
}

impl From<web::Json<CreatePost>> for CreatePost {
    fn from(value: web::Json<CreatePost>) -> Self {
        CreatePost {
            content: value.content.clone(),
            user_id: value.user_id,
            reply_to: value.reply_to,
            user_name: value.user_name.clone(),
            reactions: value.reactions.clone(),
        }
    }
}

/*
-- Your SQL goes here
CREATE TABLE public.posts (
    "id" SERIAL NOT NULL UNIQUE,
    "content" TEXT NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "user_id" INTEGER NOT NULL,
    "reply_to" INTEGER,
    "user_name" VARCHAR NOT NULL,
    "reactions" JSONB,
    PRIMARY KEY("id")
);
 */
