use std::ptr::null;
use ::entity::{m_account, posts, posts::Entity as Post};
use sea_orm::*;


pub struct Mutation;

impl Mutation {
    pub async fn create_post(
        db: &DbConn,
        form_data: posts::Model,
    ) -> Result<posts::ActiveModel, DbErr> {
        posts::ActiveModel {
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_post_by_id(
        db: &DbConn,
        id: i32,
        form_data: posts::Model,
    ) -> Result<posts::Model, DbErr> {
        let post: posts::ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        posts::ActiveModel {
            id: post.id,
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
            ..Default::default()
        }
        .update(db)
        .await
    }
    pub async fn insert_account(
        db: &DbConn,
        form_data: m_account::Model,
    ) -> Result<m_account::ActiveModel, DbErr> {

        m_account::ActiveModel {
            account_name: Set(form_data.account_name.to_owned()),
            password: Set(form_data.password.to_owned()),
            ..Default::default()
        }
            .save(db)
            .await
    }
    pub async fn delete_post(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let post: posts::ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }

    pub async fn delete_all_posts(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Post::delete_many().exec(db).await
    }
}
