use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::{delete, get, post, put},
    Form, Json, Router,
};
use tracing::{error, info};

use crate::{
    entity::todo::Model,
    infra::{db::init_pool, orm::init_conn, AppState},
    service::todo as service,
};

pub async fn todorouter() -> Router {
    info!("init todorouter ");
    let db = init_pool().await;
    let conn = init_conn().await;
    let state = AppState { db, conn };

    Router::new()
        .route("/info/:id", get(info))
        .route("/update/:id", put(update))
        .route("/del/:id", delete(del))
        .route("/add", post(add))
        .route("/list", get(list))
        .with_state(state)
}

async fn del(state: State<AppState>, id: Path<i32>) -> Html<String> {
    let rsp = service::del(&state.conn, id).await;
    match rsp {
        Ok(dr) => Html(format!("delete success: {}", dr.rows_affected)),
        Err(err) => Html(format!("delete error: {}", err.to_string())),
    }
}

async fn add(state: State<AppState>, form: Form<Model>) -> Html<String> {
    let rsp = service::create(&state.conn, &form).await;
    match rsp {
        Ok(_) => Html("create success".to_string()),
        Err(err) => Html(format!("create error:{}", err)),
    }
}

async fn list(state: State<AppState>) -> Result<Json<Vec<Model>>, StatusCode> {
    let rsp = service::list(&state.conn).await;
    match rsp {
        Ok(models) => Ok(Json(models)),
        Err(err) => {
            error!("error: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn info(state: State<AppState>, id: Path<i32>) -> Result<Json<Model>, StatusCode> {
    let rsp = service::info(&state.conn, id).await;

    match rsp {
        Ok(model) => Ok(Json(model)),
        Err(err) => {
            error!("error: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn update(state: State<AppState>, form: Form<Model>) -> Result<Json<Model>, StatusCode> {
    let rsp = service::update(&state.conn, form.0).await;

    match rsp {
        Ok(model) => Ok(Json(model)),
        Err(err) => {
            error!("error: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
