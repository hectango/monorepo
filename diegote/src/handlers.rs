use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::AppContext;

#[derive(Deserialize, Debug)]
pub struct GetVideosParams {
    pub query: String,
}

#[derive(sqlx::FromRow)]
struct VideoRow {
    video_id: i64,
    hash: String,
    length: i32,
    title: String,
    creator_address: String,
    url: String,
    description: String,
    state: String,
}

#[tracing::instrument]
pub async fn get_videos(
    State(context): State<AppContext>,
    Query(params): Query<GetVideosParams>,
) -> Result<Json<Vec<GetVideosResponse>>, StatusCode> {
    let query = format!("%{}%", params.query);
    let rows: Vec<GetVideosResponse> = sqlx::query_as::<_, VideoRow>(
        "select
            video_id,
            hash,
            length,
            title,
            creator_address,
            url,
            description,
            state
        from
            hectango.video
        where
            state = 'ready' and
            title like ?",
    )
    .bind(query)
    .fetch(&context.db_pool)
    .map_ok(GetVideosResponse::from)
    .try_collect()
    .await
    .map_err(|err| {
        error!("video query: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(rows))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVideosResponse {
    pub video_id: String,
    pub owner_address: String,
    pub video_url: String,
    pub title: String,
    pub description: String,
}

impl From<VideoRow> for GetVideosResponse {
    fn from(value: VideoRow) -> Self {
        Self {
            video_id: value.hash,
            owner_address: value.creator_address,
            video_url: value.url,
            title: value.title,
            description: value.description,
        }
    }
}
