use chrono::NaiveDateTime;
use diesel::{
    pg::Pg,
    prelude::{Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};
use shared::lib::data::Data;

use crate::{establish_pg_connection, DbPool};

#[derive(Serialize, Deserialize, Debug, Default, Selectable, Queryable, Insertable)]
#[diesel(table_name = crate::schema::reactions)]
#[diesel(check_for_backend(Pg))]
pub struct Reaction {
    id: i32,
    user_id: i32,
    post_id: i32,
    created_at: NaiveDateTime,
    reaction_id: i32,
    reaction_name: String,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::reactions)]
pub struct InsertReactionRequest {
    // pub user_id: i32,
    pub post_id: i32,
    pub reaction_name: String,
}
#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::reactions)]
pub struct InsertReaction {
    pub user_id: i32,
    pub post_id: i32,
    pub reaction_name: String,
}
#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::reactions)]
pub struct DeleteReactionRequest {
    pub post_id: i32,
    pub reaction_name: String,
}
#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::reactions)]
pub struct DeleteReaction {
    pub user_id: i32,
    pub post_id: i32,
    pub reaction_name: String,
}

impl Reaction {
    pub fn insert_reaction(
        pool: &DbPool,
        insert_reaction: InsertReaction,
    ) -> Result<Data<Reaction>, diesel::result::Error> {
        use crate::schema::reactions::dsl;
        use diesel::pg::upsert::excluded;
        use diesel::prelude::*;

        let mut conn = establish_pg_connection(&pool).unwrap();

        let data = diesel::insert_into(dsl::reactions)
            .values(&insert_reaction)
            .on_conflict((dsl::user_id, dsl::post_id, dsl::reaction_name))
            .do_update()
            .set((
                dsl::reaction_id.eq(excluded(dsl::reaction_id)),
                dsl::created_at.eq(excluded(dsl::created_at)), // 更新时间戳为当前时间
            ))
            .returning(Reaction::as_returning())
            .get_result(&mut conn)?;

        Ok(Data::new(data, None))
    }

    pub fn delete_reaction(
        pool: &DbPool,
        delete_reaction: DeleteReaction,
    ) -> Result<Data<Reaction>, diesel::result::Error> {
        use crate::schema::reactions::dsl;
        use diesel::prelude::*;
        let mut conn = establish_pg_connection(&pool).unwrap();
        let data = diesel::delete(dsl::reactions)
            .filter(dsl::user_id.eq(delete_reaction.user_id))
            .filter(dsl::post_id.eq(delete_reaction.post_id))
            .filter(dsl::reaction_name.eq(delete_reaction.reaction_name))
            .get_result::<Reaction>(&mut conn)?;
        let body = Data::new(data, None);
        Ok(body)
    }
}
#[cfg(test)]
mod tests {
    use crate::establish_pool;

    use super::*;
    use diesel::Connection;

    #[test]
    fn test_insert_reaction() {
        // 建立数据库连接
        let pool = establish_pool();
        let mut conn = establish_pg_connection(&pool).unwrap();

        // 启动事务，以便测试完成后回滚
        conn.test_transaction::<_, diesel::result::Error, _>(|_conn| {
            // 创建测试数据
            let insert_reaction = InsertReaction {
                user_id: 2,                        // 假设存在的 user_id
                post_id: 2,                        // 假设存在的 post_id
                reaction_name: "like".to_string(), // 测试的 reaction_name
            };

            // 调用 insert_reaction 方法
            let result = Reaction::insert_reaction(&pool, insert_reaction);

            // 验证结果
            assert!(result.is_ok()); // 检查操作是否成功
            let data = result.unwrap().data;
            assert_eq!(data.user_id, 2); // 检查返回的数据是否与插入的数据匹配
            assert_eq!(data.post_id, 2);
            assert_eq!(data.reaction_name, "like");

            Ok(())
        });
    }

    #[test]
    fn test_delete_reaction() {
        use diesel::prelude::*;
        // 建立数据库连接
        let pool = establish_pool();
        let mut conn = establish_pg_connection(&pool).unwrap();

        // 启动事务，以便测试完成后回滚
        conn.test_transaction::<_, diesel::result::Error, _>(|_conn| {
            // 先插入一个反应记录
            let insert_reaction = InsertReaction {
                user_id: 3,                        // 假设存在的 user_id
                post_id: 3,                        // 假设存在的 post_id
                reaction_name: "love".to_string(), // 测试的 reaction_name
            };

            let insert_result = Reaction::insert_reaction(&pool, insert_reaction);
            assert!(insert_result.is_ok()); // 确保插入操作成功

            // 创建删除请求
            let delete_reaction = DeleteReaction {
                user_id: 3,
                post_id: 3,
                reaction_name: "love".to_string(),
            };

            // 调用 delete_reaction 方法
            let delete_result = Reaction::delete_reaction(&pool, delete_reaction);

            // 验证删除结果
            assert!(delete_result.is_ok()); // 检查操作是否成功
            let deleted_data = delete_result.unwrap().data;
            assert_eq!(deleted_data.user_id, 3); // 验证返回的删除数据是否匹配
            assert_eq!(deleted_data.post_id, 3);
            assert_eq!(deleted_data.reaction_name, "love");
            Ok(())
        });
    }
}
