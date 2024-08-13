use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared::lib::data::Data;
use shared::lib::hash::Hash;
use shared::request::request::ListRequest;
use shared::response::pagination::ResponsePagination;

use crate::filter::UserFilter;
use crate::sort::UserSort;
use crate::DbPool;
use crate::{establish_pg_connection, schema::users};

#[derive(Deserialize, Serialize, Debug, Queryable, Selectable, Default)]
#[diesel(table_name = crate::schema::users)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub last_login: Option<NaiveDateTime>,
    pub is_active: bool,
    pub is_admin: bool,
    pub profile_picture: Option<Uuid>,
}

impl User {
    pub fn new(
        id: i32,
        user_name: String,
        email: String,
        password: String,
        full_name: Option<String>,
        phone: Option<String>,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
        last_login: Option<NaiveDateTime>,
        is_active: bool,
        is_admin: bool,
        profile_picture: Option<Uuid>,
    ) -> Self {
        let pw_hash = Hash::create_password_hash(password).unwrap();
        Self {
            id,
            user_name,
            email,
            password_hash: pw_hash,
            full_name,
            phone,
            created_at,
            updated_at,
            last_login,
            is_active,
            is_admin,
            profile_picture,
        }
    }

    pub fn check_password_correct(
        pool: &DbPool,
        user_name: String,
        password: String,
    ) -> Result<bool, diesel::result::Error> {
        let mut conn = establish_pg_connection(&pool).unwrap();
        let result = users::dsl::users
            .filter(users::user_name.eq(user_name))
            .first::<User>(&mut conn);
        match result {
            Ok(result) => match Hash::verify_password(password, result.password_hash) {
                Ok(result) => Ok(result),
                Err(_) => Ok(false),
            },
            Err(e) => Err(e),
        }
    }

    pub fn insert_user(
        pool: &DbPool,
        insert_user: InsertUser,
    ) -> Result<Data<User>, diesel::result::Error> {
        let mut conn = establish_pg_connection(&pool).unwrap();
        let data = diesel::insert_into(users::dsl::users)
            .values(insert_user)
            .returning(User::as_returning())
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    pub fn list_user(
        pool: &DbPool,
        list_request: ListRequest<UserFilter, UserSort>,
    ) -> Result<Data<Vec<User>>, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        use crate::sort::SortOrder;
        use crate::sort::UserSortBy as SortBy;
        let mut conn = establish_pg_connection(pool).unwrap();
        let mut query = users.into_boxed();

        let filter = list_request.filters.unwrap_or_default();
        let pagination = list_request.pagination.unwrap_or_default();
        query = query.order_by(id.asc());
        // Apply filters
        if let Some(filter_id) = filter.id {
            query = query.filter(id.eq(filter_id));
        }

        if let Some(filter_user_name) = filter.user_name {
            query = query.filter(user_name.like(format!("%{}%", filter_user_name)));
        }

        if let Some(filter_email) = filter.email {
            query = query.filter(email.like(format!("%{}%", filter_email)));
        }

        if let Some(filter_full_name) = filter.full_name {
            query = query.filter(full_name.like(format!("%{}%", filter_full_name)));
        }

        if let Some(filter_phone) = filter.phone {
            query = query.filter(phone.eq(filter_phone));
        }

        if let Some(filter_created_at_min) = filter.created_at_min {
            query = query.filter(created_at.ge(filter_created_at_min));
        }

        if let Some(filter_created_at_max) = filter.created_at_max {
            query = query.filter(created_at.le(filter_created_at_max));
        }

        if let Some(filter_updated_at_min) = filter.updated_at_min {
            query = query.filter(updated_at.ge(filter_updated_at_min));
        }

        if let Some(filter_updated_at_max) = filter.updated_at_max {
            query = query.filter(updated_at.le(filter_updated_at_max));
        }

        if let Some(filter_last_login_min) = filter.last_login_min {
            query = query.filter(last_login.ge(filter_last_login_min));
        }

        if let Some(filter_last_login_max) = filter.last_login_max {
            query = query.filter(last_login.le(filter_last_login_max));
        }

        if let Some(filter_is_active) = filter.is_active {
            query = query.filter(is_active.eq(filter_is_active));
        }

        if let Some(filter_is_admin) = filter.is_admin {
            query = query.filter(is_admin.eq(filter_is_admin));
        }

        if let Some(filter_profile_picture) = filter.profile_picture {
            query = query.filter(profile_picture.eq(filter_profile_picture));
        }
        let sort = list_request.sort;
        // Apply sorting
        if let Some(sort) = sort {
            if let Some(sort_by) = sort.sort_by {
                let order = match sort.order.unwrap_or(SortOrder::Asc) {
                    SortOrder::Asc => true,
                    SortOrder::Desc => false,
                };
                if order {
                    query = match sort_by {
                        SortBy::Id => query.order_by(id.asc()),
                        SortBy::UserName => query.order_by(user_name.asc()),
                        SortBy::Email => query.order_by(email.asc()),
                        SortBy::FullName => query.order_by(full_name.asc()),
                        SortBy::Phone => query.order_by(phone.asc()),
                        SortBy::CreatedAt => query.order_by(created_at.asc()),
                        SortBy::UpdatedAt => query.order_by(updated_at.asc()),
                        SortBy::LastLogin => query.order_by(last_login.asc()),
                        SortBy::IsActive => query.order_by(is_active.asc()),
                        SortBy::IsAdmin => query.order_by(is_admin.asc()),
                    };
                } else {
                    query = match sort_by {
                        SortBy::Id => query.order_by(id.desc()),
                        SortBy::UserName => query.order_by(user_name.desc()),
                        SortBy::Email => query.order_by(email.desc()),
                        SortBy::FullName => query.order_by(full_name.desc()),
                        SortBy::Phone => query.order_by(phone.desc()),
                        SortBy::CreatedAt => query.order_by(created_at.desc()),
                        SortBy::UpdatedAt => query.order_by(updated_at.desc()),
                        SortBy::LastLogin => query.order_by(last_login.desc()),
                        SortBy::IsActive => query.order_by(is_active.desc()),
                        SortBy::IsAdmin => query.order_by(is_admin.desc()),
                    };
                }
            }
        }

        // Apply pagination
        query = query
            .limit(pagination.limit.unwrap() as i64)
            .offset(pagination.offset.unwrap() as i64);

        let data = query.load::<User>(&mut conn)?;

        // Response pagination.
        let page = (pagination.offset.unwrap() / pagination.limit.unwrap()) + 1;
        let per_page = pagination.limit.unwrap();
        // 获取总记录数
        // let total_count = users.count().get_result::<i64>(&mut conn)? as i32;
        let total_count: i32 = data.len() as i32;
        // println!("total_count: {total_count}");
        // if total_count < pagination.offset.unwrap() {
        //     return Err(diesel::NotFound);
        // }

        // 计算总页数
        let total_pages = (total_count + per_page - 1) / per_page;

        let previous_page_offset = (page - 2) * per_page;
        let next_page_offset = page * per_page;
        let pagination = ResponsePagination::new(
            page,
            per_page,
            total_pages,
            total_count,
            Some(format!("?limit={}&offset={}", per_page, next_page_offset)),
            Some(format!(
                "?limit={}&offset={}",
                per_page, previous_page_offset
            )),
        );

        let body = Data::new(data, Some(pagination));
        Ok(body)
    }

    pub fn delete_user(
        pool: &DbPool,
        user_name: String,
    ) -> Result<Data<User>, Box<dyn std::error::Error>> {
        use crate::schema::users::dsl;
        let mut conn = establish_pg_connection(&pool).unwrap();
        let data = diesel::delete(dsl::users)
            .filter(dsl::user_name.eq(user_name))
            .get_result::<User>(&mut conn)?;
        let body = Data::new(data, None);
        Ok(body)
    }
    pub fn get_user(
        pool: &DbPool,
        user_name: String,
    ) -> Result<Data<User>, Box<dyn std::error::Error>> {
        use crate::schema::users::dsl;
        let mut conn = establish_pg_connection(&pool).unwrap();
        let data = dsl::users
            .filter(dsl::user_name.eq(user_name))
            .first(&mut conn)?;
        let body = Data::new(data, None);
        Ok(body)
    }
    pub fn update_user(
        pool: &DbPool,
        updated_user: UpdateUser,
    ) -> Result<Data<User>, diesel::result::Error> {
        use crate::schema::users::dsl;
        use diesel::prelude::*;
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data: User = diesel::update(dsl::users.filter(dsl::id.eq(updated_user.id)))
            .set((
                users::user_name.eq(updated_user.user_name),
                users::password_hash.eq(updated_user.password_hash),
                users::email.eq(updated_user.email),
                users::full_name.eq(updated_user.full_name),
                users::is_active.eq(updated_user.is_active),
                users::is_admin.eq(updated_user.is_admin),
                users::updated_at.eq(Some(chrono::Utc::now().naive_utc())),
                users::phone.eq(updated_user.phone),
                users::profile_picture.eq(updated_user.profile_picture),
            ))
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }
    // pub fn update_user(
    //     pool: &DbPool,
    //     updated_user: UpdateUser,
    // ) -> Result<Data<User>, diesel::result::Error> {
    //     use crate::schema::users::dsl;
    //     use diesel::prelude::*;

    //     let mut conn = establish_pg_connection(pool).expect("Failed to connect to the database");

    //     // Start building the update query
    //     let target = dsl::users.filter(dsl::id.eq(updated_user.id));
    //     let _ = diesel::update(target).set((
    //         users::user_name.eq(updated_user.user_name),
    //         users::email.eq(updated_user.email),
    //         users::full_name.eq(updated_user.full_name),
    //         users::updated_at.eq(Some(chrono::Utc::now().naive_utc())),
    //         users::phone.eq(updated_user.phone),
    //     ));

    //     // Conditionally update password_hash if provided
    //     if let Some(password_hash) = updated_user.password_hash {
    //         let _ = diesel::update(target).set(users::password_hash.eq(password_hash));
    //     }

    //     let data = dsl::users
    //         .filter(dsl::id.eq(updated_user.id))
    //         .first(&mut conn)?;
    //     Ok(Data::new(data, None))
    // }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InsertUserRequest {
    // pub id: i32,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    // pub created_at: Option<NaiveDateTime>,
    // pub updated_at: Option<NaiveDateTime>,
    // pub last_login: Option<NaiveDateTime>,
    pub is_active: bool,
    pub is_admin: bool,
    pub profile_picture: Option<Uuid>,
}

impl InsertUserRequest {
    pub fn new(
        user_name: String,
        email: String,
        password: String,
        full_name: Option<String>,
        phone: Option<String>,
        is_active: bool,
        is_admin: bool,
        profile_picture: Option<Uuid>,
    ) -> Self {
        Self {
            user_name,
            email,
            password,
            full_name,
            phone,
            is_active,
            is_admin,
            profile_picture,
        }
    }
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct InsertUser {
    // pub id: i32,
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    // pub created_at: Option<NaiveDateTime>,
    // pub updated_at: Option<NaiveDateTime>,
    // pub last_login: Option<NaiveDateTime>,
    pub is_active: bool,
    pub is_admin: bool,
    pub profile_picture: Option<Uuid>,
}

impl InsertUser {
    pub fn new(
        user_name: String,
        email: String,
        password_hash: String,
        full_name: Option<String>,
        phone: Option<String>,
        is_active: bool,
        is_admin: bool,
        profile_picture: Option<Uuid>,
    ) -> Self {
        Self {
            user_name,
            email,
            password_hash,
            full_name,
            phone,
            is_active,
            is_admin,
            profile_picture,
        }
    }
}

impl From<InsertUserRequest> for InsertUser {
    fn from(request: InsertUserRequest) -> Self {
        let pw_hash = Hash::create_password_hash(request.password).unwrap();
        InsertUser::new(
            request.user_name,
            request.email,
            pw_hash,
            request.full_name,
            request.phone,
            request.is_active,
            request.is_admin,
            request.profile_picture,
        )
    }
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    pub id: i32,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub is_active: bool,
    pub is_admin: bool,
    pub profile_picture: Option<Uuid>,
}

impl UpdateUserRequest {
    pub fn new(
        id: i32,
        user_name: String,
        email: String,
        password: String,
        full_name: Option<String>,
        phone: Option<String>,
        is_active: bool,
        is_admin: bool,
        profile_picture: Option<Uuid>,
    ) -> Self {
        Self {
            id,
            user_name,
            email,
            password,
            full_name,
            phone,
            is_active,
            is_admin,
            profile_picture,
        }
    }
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateUser {
    pub id: i32,
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub is_active: bool,
    pub is_admin: bool,
    pub profile_picture: Option<Uuid>,
}

impl UpdateUser {
    pub fn new(
        id: i32,
        user_name: String,
        email: String,
        password_hash: String,
        full_name: Option<String>,
        phone: Option<String>,
        is_active: bool,
        is_admin: bool,
        profile_picture: Option<Uuid>,
    ) -> Self {
        Self {
            id,
            user_name,
            email,
            password_hash,
            full_name,
            phone,
            is_active,
            is_admin,
            profile_picture,
        }
    }
}

impl From<UpdateUserRequest> for UpdateUser {
    fn from(value: UpdateUserRequest) -> Self {
        let pw_hash = Hash::create_password_hash(value.password).unwrap();
        UpdateUser::new(
            value.id,
            value.user_name,
            value.email,
            pw_hash,
            value.full_name,
            value.phone,
            value.is_active,
            value.is_admin,
            value.profile_picture,
        )
    }
}

#[cfg(test)]
mod test {
    use diesel::{ExpressionMethods, RunQueryDsl};
    use uuid::Uuid;

    use shared::lib::hash::Hash;
    use shared::request::{pagination::RequestPagination, request::ListRequest};

    use crate::entities::user::{InsertUserRequest, User};
    use crate::{
        establish_pool,
        filter::UserFilter,
        schema::users,
        sort::{SortOrder, UserSort},
    };

    #[test]
    fn test_insert_user() {
        let pool = establish_pool();
        let request_user = InsertUserRequest::new(
            "test_insert_user".to_string(),
            "lindesong666@gmail.com".to_string(),
            "070011".to_string(),
            None,
            None,
            true,
            true,
            None,
        );
        let result = User::insert_user(&pool, request_user.into()).unwrap();

        assert_eq!(result.data.user_name, "test_insert_user");
        assert_eq!(result.data.email, "lindesong666@gmail.com");
        // 可以添加更多的断言来验证插入的数据

        let _ = User::delete_user(&pool, "test_insert_user".to_string());
    }

    #[test]
    fn test_check_password_correct() {
        // let binding = establish_pool();
        // let pool = web::Data::from(Arc::new(binding));
        let pool = establish_pool();

        // 准备测试数据
        let user_name = "test_user".to_string();
        let email = "testuser@example.com".to_string();
        let password = "test_password".to_string();
        let pw_hash = Hash::create_password_hash(password.clone()).unwrap();
        let full_name = Some("Test User".to_string());
        let phone = Some("1234567890".to_string());
        let created_at = Some(chrono::Utc::now().naive_utc());
        let updated_at = Some(chrono::Utc::now().naive_utc());
        let last_login = Some(chrono::Utc::now().naive_utc());
        let is_active = true;
        let is_admin = false;
        let profile_picture: Option<Uuid> = None;

        let conn = &mut pool.get().unwrap();
        diesel::insert_into(users::table)
            .values((
                users::user_name.eq(user_name.clone()),
                users::email.eq(email),
                users::password_hash.eq(pw_hash),
                users::full_name.eq(full_name),
                users::phone.eq(phone),
                users::created_at.eq(created_at),
                users::updated_at.eq(updated_at),
                users::last_login.eq(last_login),
                users::is_active.eq(is_active),
                users::is_admin.eq(is_admin),
                users::profile_picture.eq(profile_picture),
            ))
            .execute(conn)
            .unwrap();

        // 调用 check_password_correct
        let is_correct = User::check_password_correct(&pool, user_name.clone(), password).unwrap();
        assert!(is_correct);

        let is_wrong_password =
            User::check_password_correct(&pool, user_name.clone(), "wrong_password".to_string())
                .unwrap();
        assert!(!is_wrong_password);
        // 删除测试用户
        let _ = User::delete_user(&pool, user_name.clone());
    }

    #[test]
    fn test_get_user_list() {
        use crate::sort::UserSortBy as SortBy;
        let pool = establish_pool(); // 假设你有一个用于获取连接池的函数

        // 创建一个 ListRequest 示例
        let filters = UserFilter {
            id: None,
            user_name: Some("mike".to_string()),
            email: None,
            full_name: None,
            phone: None,
            created_at_min: None,
            created_at_max: None,
            updated_at_min: None,
            updated_at_max: None,
            last_login_min: None,
            last_login_max: None,
            is_active: Some(true),
            is_admin: None,
            profile_picture: None,
        };

        let sort = UserSort {
            sort_by: Some(SortBy::UserName),
            order: Some(SortOrder::Asc),
        };

        let list_request = ListRequest {
            filters: Some(filters),
            user_id: None,
            sort: Some(sort),
            pagination: Some(RequestPagination {
                limit: Some(10),
                offset: Some(0),
            }),
        };

        // 调用 get_user_list 函数
        let result = User::list_user(&pool, list_request);
        assert!(result.is_ok());

        let data = result.unwrap().data;
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].user_name, "mike");
    }

    #[test]
    fn test_delete_user() {
        let pool = establish_pool();
        let user_name = String::from("test_delete_user");

        // Creating a user for testing
        let new_user = InsertUserRequest {
            user_name: user_name.clone(),
            email: "test@example.com".to_string(),
            password: "password".to_string(),
            full_name: Some("Test User".to_string()),
            phone: Some("1234567890".to_string()),
            is_active: true,
            is_admin: false,
            profile_picture: None,
        };

        // Inserting the user
        let _ = User::insert_user(&pool, new_user.into());

        // Deleting the user
        let result = User::delete_user(&pool, user_name.clone());

        // Verifying the user was deleted
        match result {
            Ok(data) => {
                assert_eq!(data.data.user_name, user_name);
                // Further assertions based on the `User` struct can be added here
            }
            Err(e) => panic!("Error deleting user: {:?}", e),
        }
    }
}
