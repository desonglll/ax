use crate::DbPool;
use crate::{establish_pg_connection, schema::users};
use diesel::prelude::*;
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};
use shared::data::Data;
use shared::hash::Hash;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Queryable, Selectable, Default)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
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

impl User {
    pub fn new(
        id: i32,
        user_name: String,
        email: String,
        password: String,
        full_name: Option<String>,
        phone: Option<String>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
        last_login: Option<chrono::NaiveDateTime>,
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

        if Hash::verify_password(password, result.unwrap().password_hash).unwrap() {
            Ok(true)
        } else {
            Ok(false)
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

    pub fn get_user_list(pool: &DbPool) -> Result<Data<Vec<User>>, diesel::result::Error> {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    // pub id: i32,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    // pub created_at: Option<chrono::NaiveDateTime>,
    // pub updated_at: Option<chrono::NaiveDateTime>,
    // pub last_login: Option<chrono::NaiveDateTime>,
    pub is_active: bool,
    pub is_admin: bool,
    pub profile_picture: Option<Uuid>,
}

impl CreateUserRequest {
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
    // pub created_at: Option<chrono::NaiveDateTime>,
    // pub updated_at: Option<chrono::NaiveDateTime>,
    // pub last_login: Option<chrono::NaiveDateTime>,
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

impl From<CreateUserRequest> for InsertUser {
    fn from(request: CreateUserRequest) -> Self {
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
#[cfg(test)]
mod test {
    use diesel::{ExpressionMethods, RunQueryDsl};
    use shared::hash::Hash;
    use uuid::Uuid;

    use crate::{establish_pool, schema::users, user::User};

    #[test]
    fn test_user_creation() {
        let id = 1;
        let user_name = "johndoe".to_string();
        let email = "john.doe@example.com".to_string();
        let password = "super_secure_password".to_string();
        let full_name = Some("John Doe".to_string());
        let phone = Some("1234567890".to_string());
        let created_at = Some(chrono::Utc::now().naive_utc());
        let updated_at = Some(chrono::Utc::now().naive_utc());
        let last_login = None;
        let is_active = true;
        let is_admin = false;
        let profile_picture = None;

        let user = User::new(
            id,
            user_name,
            email,
            password.clone(),
            full_name,
            phone,
            created_at,
            updated_at,
            last_login,
            is_active,
            is_admin,
            profile_picture,
        );

        // 验证 User 实例的字段
        assert_eq!(user.id, id);
        assert_eq!(user.user_name, "johndoe");
        assert_eq!(user.email, "john.doe@example.com");
        assert_eq!(user.is_active, true);
        assert_eq!(user.is_admin, false);

        // 验证密码哈希是否正确生成
        let is_valid = Hash::verify_password(password, user.password_hash).unwrap();
        assert!(is_valid);
    }
    #[test]
    fn test_check_password_correct() {
        // let binding = establish_pool();
        // let pool = web::Data::from(Arc::new(binding));
        let pool = establish_pool();

        // 准备测试数据
        let user_name = "testuser".to_string();
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
            User::check_password_correct(&pool, user_name, "wrong_password".to_string()).unwrap();
        assert!(!is_wrong_password);
    }
}
