use std::{error::Error, net::SocketAddr, pin::Pin, sync::Arc};
use tokio::sync::Mutex;

use axum::{extract::State, http::StatusCode, routing::post, Json, Router, Server};
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

use crate::proxy;

#[derive(Default)]
struct ProxyState {
    handle: Option<Pin<Box<JoinHandle<()>>>>,
}

type SharedProxyState = Arc<Mutex<ProxyState>>;

pub async fn start_server(port: u16) -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let controller = Arc::new(Mutex::new(ProxyState::default()));

    let app = Router::new()
        .route("/proxy", post(start_proxy))
        .with_state(controller);

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct StartProxy {
    port: u16,
}

async fn start_proxy(
    State(proxy_status): State<SharedProxyState>,
    Json(input): Json<StartProxy>,
) -> Result<Json<StartProxy>, StatusCode> {
    let mut status = proxy_status.lock().await;

    if status.handle.is_none() {
        let new_handle = proxy::start(proxy::Arguments { port: input.port })
            .await
            .map_err(|_| StatusCode::PRECONDITION_FAILED)?;
        status.handle = Some(Box::pin(new_handle));
    }

    Ok(Json(StartProxy { port: input.port }))
}
