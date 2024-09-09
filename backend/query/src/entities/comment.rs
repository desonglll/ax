use chrono::NaiveDateTime;
use diesel::{pg::Pg, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::lib::data::Data;

use crate::{establish_pg_connection, DbPool};

#[derive(Serialize, Deserialize, Debug, Default, Selectable, Queryable, Insertable)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(Pg))]
pub struct Comment {
    id: i32,
    content: String,
    reply_to: i32,
    user_id: i32,
    user_name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    reactions: Option<Value>,
    reply_to_type: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Insertable)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(Pg))]
pub struct InsertComment {
    pub content: String,
    pub reply_to: i32,
    pub user_id: i32,
    pub reply_to_type: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct InsertCommentRequest {
    pub content: String,
    pub reply_to: i32,
    pub reply_to_type: String,
}
#[derive(Serialize, Deserialize)]
pub struct DeleteCommentRequest {
    pub id: i32,
}
impl Comment {
    pub fn insert_comment(
        pool: &DbPool,
        insert_comment: InsertComment,
    ) -> Result<Data<Comment>, diesel::result::Error> {
        use crate::schema::comments::dsl;
        use diesel::prelude::*;
        let mut conn = establish_pg_connection(pool).unwrap();
        let data = diesel::insert_into(dsl::comments)
            .values(insert_comment)
            .returning(Comment::as_returning())
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    pub fn delete_comment(
        pool: &DbPool,
        delete_comment_id: i32,
    ) -> Result<Data<Comment>, diesel::result::Error> {
        use crate::schema::comments::dsl;
        use diesel::prelude::*;
        let mut conn = establish_pg_connection(pool).unwrap();
        let data = diesel::delete(dsl::comments)
            .filter(dsl::id.eq(delete_comment_id))
            .get_result::<Comment>(&mut conn)?;
        let body = Data::new(data, None);
        Ok(body)
    }

    pub fn get_comments_by_reply_to_id(
        pool: &DbPool,
        post_id: i32,
        reply_to_type: String,
    ) -> Result<Data<Vec<Comment>>, diesel::result::Error> {
        use crate::schema::comments::dsl;
        use diesel::prelude::*;
        let mut conn = establish_pg_connection(pool).unwrap();
        let data = dsl::comments
            .filter(dsl::reply_to_type.eq(reply_to_type))
            .filter(dsl::reply_to.eq(post_id))
            .load::<Comment>(&mut conn)?;
        let body = Data::new(data, None);
        Ok(body)
    }
}
#[cfg(test)]
mod test {
    use crate::establish_pool;

    use super::*;
    use diesel::prelude::*;
    use diesel::result::Error;

    #[test]
    fn test_insert_comment() {
        let pool = establish_pool();

        let insert_comment = InsertComment {
            content: "Test comment".to_string(),
            reply_to: 1, // 需要确保这个帖子 ID 在数据库中存在
            user_id: 1,  // 需要确保这个用户 ID 在数据库中存在
            reply_to_type: "post".to_string(),
        };

        let result = Comment::insert_comment(&pool, insert_comment);

        assert!(result.is_ok());
        if let Ok(data) = result {
            let comment = data.data;
            assert_eq!(comment.content, "Test comment");
            assert_eq!(comment.reply_to, 1);
            assert_eq!(comment.user_id, 1);
            assert!(comment.id > 0); // 确保 ID 被正确生成
        }
    }

    #[test]
    fn test_delete_comment() {
        let pool = establish_pool();

        // 先插入一条评论，方便后续删除测试
        let insert_comment = InsertComment {
            content: "Comment to be deleted".to_string(),
            reply_to: 1,
            user_id: 1,
            reply_to_type: "post".to_string(),
        };

        let inserted = Comment::insert_comment(&pool, insert_comment).unwrap();
        let comment_id = inserted.data.id;

        // 删除插入的评论
        let result = Comment::delete_comment(&pool, comment_id);
        assert!(result.is_ok());

        // 确保删除的评论与插入的评论相同
        if let Ok(data) = result {
            let comment = data.data;
            assert_eq!(comment.id, comment_id);
            assert_eq!(comment.content, "Comment to be deleted");
        }

        // 确保评论确实被删除
        let mut connection = establish_pg_connection(&pool).unwrap();
        let find_comment: Result<Comment, Error> = crate::schema::comments::dsl::comments
            .filter(crate::schema::comments::dsl::id.eq(comment_id))
            .first(&mut connection);

        assert!(find_comment.is_err()); // 如果找不到，证明删除成功
    }
}
