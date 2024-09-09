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
#[diesel(table_name = crate::schema::comment_reactions)]
#[diesel(check_for_backend(Pg))]
pub struct CommentReaction {
    id: i32,
    user_id: i32,
    comment_id: i32,
    created_at: NaiveDateTime,
    reaction_id: i32,
    reaction_name: String,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::comment_reactions)]
#[serde(rename_all = "camelCase")]
pub struct InsertCommentReactionRequest {
    // pub user_id: i32,
    pub comment_id: i32,
    pub reaction_name: String,
}
#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::comment_reactions)]
pub struct InsertCommentReaction {
    pub user_id: i32,
    pub comment_id: i32,
    pub reaction_name: String,
}
#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::comment_reactions)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCommentReactionRequest {
    pub comment_id: i32,
    pub reaction_name: String,
}
#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::comment_reactions)]
pub struct DeleteCommentReaction {
    pub user_id: i32,
    pub comment_id: i32,
    pub reaction_name: String,
}

impl CommentReaction {
    pub fn insert_comment_reaction(
        pool: &DbPool,
        insert_commentreaction: InsertCommentReaction,
    ) -> Result<Data<CommentReaction>, diesel::result::Error> {
        use crate::schema::comment_reactions::dsl;
        use diesel::pg::upsert::excluded;
        use diesel::prelude::*;

        let mut conn = establish_pg_connection(pool).unwrap();

        let data = diesel::insert_into(dsl::comment_reactions)
            .values(&insert_commentreaction)
            .on_conflict((dsl::user_id, dsl::comment_id, dsl::reaction_name))
            .do_update()
            .set((
                dsl::reaction_id.eq(excluded(dsl::reaction_id)),
                dsl::created_at.eq(excluded(dsl::created_at)), // 更新时间戳为当前时间
            ))
            .returning(CommentReaction::as_returning())
            .get_result(&mut conn)?;

        Ok(Data::new(data, None))
    }

    pub fn delete_comment_reaction(
        pool: &DbPool,
        delete_commentreaction: DeleteCommentReaction,
    ) -> Result<Data<CommentReaction>, diesel::result::Error> {
        use crate::schema::comment_reactions::dsl;
        use diesel::prelude::*;
        let mut conn = establish_pg_connection(pool).unwrap();
        let data = diesel::delete(dsl::comment_reactions)
            .filter(dsl::user_id.eq(delete_commentreaction.user_id))
            .filter(dsl::comment_id.eq(delete_commentreaction.comment_id))
            .filter(dsl::reaction_name.eq(delete_commentreaction.reaction_name))
            .get_result::<CommentReaction>(&mut conn)?;
        let body = Data::new(data, None);
        Ok(body)
    }

    /// 查询某个用户对某个Comment的所有CommentReaction.
    pub fn get_comment_comment_reactions(
        pool: &DbPool,
        user_id: i32,
        comment_id: i32,
    ) -> Result<Data<Vec<CommentReaction>>, diesel::result::Error> {
        use crate::schema::comment_reactions::dsl;
        use diesel::prelude::*;
        let mut conn = establish_pg_connection(pool).unwrap();
        let data = dsl::comment_reactions
            .filter(dsl::user_id.eq(user_id))
            .filter(dsl::comment_id.eq(comment_id))
            .load(&mut conn)?;
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
    fn test_insert_commentreaction() {
        // 建立数据库连接
        let pool = establish_pool();
        let mut conn = establish_pg_connection(&pool).unwrap();

        // 启动事务，以便测试完成后回滚
        conn.test_transaction::<_, diesel::result::Error, _>(|_conn| {
            // 创建测试数据
            let insert_commentreaction = InsertCommentReaction {
                user_id: 2,                        // 假设存在的 user_id
                comment_id: 2,                     // 假设存在的 comment_id
                reaction_name: "like".to_string(), // 测试的 reaction_name
            };

            // 调用 insert_commentreaction 方法
            let result = CommentReaction::insert_comment_reaction(&pool, insert_commentreaction);

            // 验证结果
            assert!(result.is_ok()); // 检查操作是否成功
            let data = result.unwrap().data;
            assert_eq!(data.user_id, 2); // 检查返回的数据是否与插入的数据匹配
            assert_eq!(data.comment_id, 2);
            assert_eq!(data.reaction_name, "like");

            Ok(())
        });
    }

    #[test]
    fn test_delete_commentreaction() {
        use diesel::prelude::*;
        // 建立数据库连接
        let pool = establish_pool();
        let mut conn = establish_pg_connection(&pool).unwrap();

        // 启动事务，以便测试完成后回滚
        conn.test_transaction::<_, diesel::result::Error, _>(|_conn| {
            // 先插入一个反应记录
            let insert_commentreaction = InsertCommentReaction {
                user_id: 3,                        // 假设存在的 user_id
                comment_id: 3,                     // 假设存在的 comment_id
                reaction_name: "love".to_string(), // 测试的 reaction_name
            };

            let insert_result =
                CommentReaction::insert_comment_reaction(&pool, insert_commentreaction);
            assert!(insert_result.is_ok()); // 确保插入操作成功

            // 创建删除请求
            let delete_commentreaction = DeleteCommentReaction {
                user_id: 3,
                comment_id: 3,
                reaction_name: "love".to_string(),
            };

            // 调用 delete_commentreaction 方法
            let delete_result =
                CommentReaction::delete_comment_reaction(&pool, delete_commentreaction);

            // 验证删除结果
            assert!(delete_result.is_ok()); // 检查操作是否成功
            let deleted_data = delete_result.unwrap().data;
            assert_eq!(deleted_data.user_id, 3); // 验证返回的删除数据是否匹配
            assert_eq!(deleted_data.comment_id, 3);
            assert_eq!(deleted_data.reaction_name, "love");
            Ok(())
        });
    }
}
