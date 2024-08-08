use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UserSort {
    pub sort_by: Option<UserSortBy>,
    pub order: Option<SortOrder>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum UserSortBy {
    Id,
    UserName,
    Email,
    FullName,
    Phone,
    CreatedAt,
    UpdatedAt,
    LastLogin,
    IsActive,
    IsAdmin,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct PostSort {
    pub sort_by: Option<PostSortBy>,
    pub order: Option<SortOrder>,
}
#[derive(Deserialize, Serialize, Debug)]
pub enum PostSortBy {
    Id,
    CreatedAt,
    UpdatedAt,
    UserId,
    ReplyTo,
}
