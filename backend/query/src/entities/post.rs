use chrono::NaiveDateTime;
use diesel::dsl::count_star;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use serde_json::Value;
use shared::lib::data::Data;
use shared::request::request::ListRequest;
use shared::response::pagination::ResponsePagination;

use crate::filter::PostFilter;
use crate::sort::PostSort;
use crate::{establish_pg_connection, DbPool};

#[derive(Serialize, Deserialize, Debug, Default, Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[serde(rename_all = "camelCase")]
#[diesel(check_for_backend(Pg))]
pub struct Post {
    pub id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: i32,
    pub reply_to: Option<i32>,
    pub user_name: String,
    pub reactions: Option<Value>, // Add serde_json feature.
}

#[derive(Serialize, Deserialize, Debug, Default, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::posts)]
#[serde(rename_all = "camelCase")]
pub struct InsertPost {
    pub content: String,
    pub user_id: i32,
    pub reply_to: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Default, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::posts)]
#[serde(rename_all = "camelCase")]
pub struct InsertPostRequest {
    pub content: String,
    pub reply_to: Option<i32>,
}

impl Post {
    pub fn list_post(
        pool: &DbPool,
        list_request: ListRequest<PostFilter, PostSort>,
    ) -> Result<Data<Vec<Post>>, diesel::result::Error> {
        use crate::schema::posts::dsl::*;
        use crate::sort::PostSortBy as SortBy;
        use crate::sort::SortOrder;
        let mut conn = establish_pg_connection(pool).unwrap();
        let mut query = posts.into_boxed();

        let filter = list_request.filters.unwrap_or_default();
        let pagination = list_request.pagination.unwrap_or_default();
        query = query.order_by(id.desc());
        // 找到对应用户的post
        if let Some(requested_user_id) = list_request.user_id {
            query = query.filter(user_id.eq(requested_user_id));
        }
        // Apply filters
        if let Some(filter_id) = filter.id {
            query = query.filter(id.eq(filter_id));
        }

        if let Some(filter_content) = filter.content {
            query = query.filter(content.eq(filter_content));
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
        if let Some(filter_user_id) = filter.user_id {
            query = query.filter(user_id.eq(filter_user_id));
        }

        if let Some(filter_reply_to) = filter.reply_to {
            query = query.filter(reply_to.eq(filter_reply_to));
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
                        SortBy::CreatedAt => query.order_by(created_at.asc()),
                        SortBy::UpdatedAt => query.order_by(updated_at.asc()),
                        SortBy::UserId => query.order_by(user_id.asc()),
                        SortBy::ReplyTo => query.order_by(reply_to.asc()),
                    };
                } else {
                    query = match sort_by {
                        SortBy::Id => query.order_by(id.desc()),
                        SortBy::CreatedAt => query.order_by(created_at.desc()),
                        SortBy::UpdatedAt => query.order_by(updated_at.desc()),
                        SortBy::UserId => query.order_by(user_id.desc()),
                        SortBy::ReplyTo => query.order_by(reply_to.desc()),
                    };
                }
            }
        }

        // Apply pagination
        query = query
            .limit(pagination.limit.unwrap() as i64)
            .offset(pagination.offset.unwrap() as i64);

        let data = query.load::<Post>(&mut conn)?;

        // Response pagination.
        let page = (pagination.offset.unwrap() / pagination.limit.unwrap()) + 1;
        let per_page = pagination.limit.unwrap();
        // 获取总记录数
        // let total_count = users.count().get_result::<i64>(&mut conn)? as i32;
        let total_count = posts.select(count_star()).first::<i64>(&mut conn).unwrap() as i32;
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

    pub fn insert_post(
        pool: &DbPool,
        insert_post: InsertPost,
    ) -> Result<Data<Post>, diesel::result::Error> {
        use crate::schema::posts::dsl;
        let mut conn = establish_pg_connection(&pool).unwrap();
        let data = diesel::insert_into(dsl::posts)
            .values(insert_post)
            .returning(Post::as_returning())
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    pub fn get_post(pool: &DbPool, post_id: i32) -> Result<Data<Post>, diesel::result::Error> {
        use crate::schema::posts::dsl;
        let mut conn = establish_pg_connection(&pool).unwrap();
        let data = dsl::posts.filter(dsl::id.eq(post_id)).first(&mut conn)?;
        let body = Data::new(data, None);
        Ok(body)
    }
}

#[cfg(test)]
mod test {
    use shared::request::{pagination::RequestPagination, request::ListRequest};

    use crate::{
        entities::post::Post,
        establish_pool,
        filter::PostFilter,
        sort::{PostSort, SortOrder},
    };

    #[test]
    fn test_insert_post() {
        use crate::entities::post::{InsertPost, Post};
        let pool = establish_pool(); // 假设你有一个用于获取连接池的函数

        // 创建一个 InsertPost 示例
        let new_post = InsertPost {
            content: String::from("This is a test post."),
            user_id: 1,
            reply_to: None, // or Some(reply_post_id) if you want to set a reply
        };

        // 调用 insert_post 函数
        let result = Post::insert_post(&pool, new_post);

        // 检查插入是否成功
        assert!(result.is_ok());

        let inserted_post = result.unwrap().data;

        // 验证插入的内容是否与预期一致
        assert_eq!(inserted_post.content, "This is a test post.");
        assert_eq!(inserted_post.user_id, 1);
        assert_eq!(inserted_post.reply_to, None);
    }

    #[test]
    fn test_get_post_list() {
        use crate::sort::PostSortBy as SortBy;
        let pool = establish_pool(); // 假设你有一个用于获取连接池的函数

        // 创建一个 ListRequest 示例
        let filters = PostFilter {
            id: Some(1),
            content: None,
            created_at_min: None,
            created_at_max: None,
            updated_at_min: None,
            updated_at_max: None,
            user_id: None,
            reply_to: None,
        };

        let sort = PostSort {
            sort_by: Some(SortBy::Id),
            order: Some(SortOrder::Asc),
        };

        let list_request = ListRequest {
            user_id: Some(1),
            filters: Some(filters),
            sort: Some(sort),
            pagination: Some(RequestPagination {
                limit: Some(10),
                offset: Some(0),
            }),
        };

        // 调用 get_post_list 函数
        let result = Post::list_post(&pool, list_request);
        assert!(result.is_ok());

        let data = result.unwrap().data;
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].content, "Content of post 1");
    }
}
