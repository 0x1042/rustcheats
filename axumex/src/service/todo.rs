use axum::{extract::Path, Form};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbConn, DbErr, DeleteResult, EntityTrait};
use tracing::info;

use crate::entity::{prelude::Todo, todo, todo::Model};

pub async fn create(db: &DbConn, form: &Form<Model>) -> Result<todo::ActiveModel, DbErr> {
    let id = uuid::Uuid::new_v4();
    info!("create {}:{:?}", &id.to_string(), &form);
    let am = todo::ActiveModel {
        done: Set(Some(0)),
        context: Set(form.context.clone()),
        uid: Set(form.uid.clone()),
        ..Default::default()
    };
    info!("sam is {:?}", &am);
    am.save(db).await
}

pub async fn info(db: &DbConn, id: Path<i32>) -> Result<Model, DbErr> {
    let todo: Model = Todo::find_by_id(id.0)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find post.".to_owned()))?;
    Ok(todo)
}

pub async fn list(db: &DbConn) -> Result<Vec<Model>, DbErr> {
    let todo = Todo::find().all(db).await?;
    Ok(todo)
}

pub async fn del(db: &DbConn, id: Path<i32>) -> Result<DeleteResult, DbErr> {
    let post: todo::ActiveModel = Todo::find_by_id(id.0)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find todo.".to_owned()))
        .map(Into::into)?;

    post.delete(db).await
}

pub async fn clean(db: &DbConn) -> Result<DeleteResult, DbErr> {
    Todo::delete_many().exec(db).await
}

pub async fn update(db: &DbConn, form: Model) -> Result<Model, DbErr> {
    let id = form.id;

    let todo: todo::ActiveModel = Todo::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find todo.".to_owned()))
        .map(Into::into)?;

    todo::ActiveModel {
        id: Set(id),
        done: Set(Some(0)),
        context: Set(form.context.clone()),
        uid: Set(form.uid.clone()),
        ..todo
    }
    .update(db)
    .await
}
