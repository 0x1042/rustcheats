use std::{convert::Infallible, path::PathBuf, time::Duration};

use async_stream::try_stream;
use axum::{
    extract::{Path, State, TypedHeader},
    response::{
        sse::{Event, KeepAlive, Sse},
        Html,
    },
    routing::get,
    Router,
};
use futures::stream::{self, Stream};
use tokio::sync::{broadcast, broadcast::error::SendError, mpsc};
use tokio_stream::StreamExt as _;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::trace;

use crate::infra::{AppState, ServerState};

pub async fn sse() -> Router {
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let static_files_service = ServeDir::new(assets_dir).append_index_html_on_directories(true);

    let (tx, mut rx) = mpsc::channel(16);

    let sstate = ServerState { event_stream: tx };
    Router::new()
        .fallback_service(static_files_service)
        .route("/sse", get(sse_handler))
        .route("/sse2", get(event_stream))
        .route("/sender/:msg", get(mock_send))
        .layer(TraceLayer::new_for_http())
        .with_state(sstate)
}

async fn mock_send(State(state): State<ServerState>, Path(msg): Path<String>) -> Html<String> {
    match state.event_stream.send(msg) {
        Ok(size) => Html(format!("size:{}", size)),
        Err(err) => Html(format!("err:{}", err)),
    }
}

async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    trace!("`{}` connected", user_agent.as_str());

    // A `Stream` that repeats an event every second
    let stream = stream::repeat_with(|| {
        Event::default().data(format!("hi, now is {}", chrono::Local::now()))
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

/// https://github.com/tokio-rs/axum/discussions/1670
async fn event_stream(
    State(state): State<ServerState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut receiver = state.event_stream.subscribe();

    Sse::new(try_stream! {
        loop {
            match receiver.recv().await {
                Ok(i) => {
                    let event = Event::default()
                        .data(i);

                    yield event;
                },

                Err(e) => {
                    tracing::error!(error = ?e, "Failed to get");
                }
            }
        }
    })
    .keep_alive(KeepAlive::default())
}
