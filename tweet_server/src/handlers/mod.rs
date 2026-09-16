//! HTTP handlers. Each function: resolve the session, validate input, call
//! `db`, shape the response. No SQL lives here.

pub mod auth;
pub mod comment;
pub mod file;
pub mod follow;
pub mod notification;
pub mod post;
pub mod reaction;
pub mod upload;
pub mod user;
