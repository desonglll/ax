//! Ax backend: a small micro-blogging API built on Actix-web and PostgreSQL.
//!
//! Layout:
//! - `routes`    — URL table (`/api/...`).
//! - `handlers`  — request handlers: auth checks, validation, response shaping.
//! - `db`        — all SQL, one module per table.
//! - `models`    — request/response and row types.
//! - `services`  — background work (AI title generation).

pub mod auth;
pub mod config;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod hash;
pub mod models;
pub mod response;
pub mod routes;
pub mod services;
pub mod state;
