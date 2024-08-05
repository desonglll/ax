use diesel::Queryable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Queryable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: Uuid,
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub last_login: Option<chrono::NaiveDateTime>,
    pub is_active: bool,
    pub is_admin: bool,
    pub profile_picture: Option<Uuid>,
}

impl User {}