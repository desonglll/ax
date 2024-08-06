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
