//! The live web server.

use std::{net::SocketAddr, path::Path, sync::Arc};

use axum::{
    extract::{Path as UrlPath, State},
    http::{
        header::{CACHE_CONTROL, REFERRER_POLICY, X_CONTENT_TYPE_OPTIONS},
        HeaderValue, StatusCode,
    },
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, services::ServeDir, set_header::SetResponseHeaderLayer,
};

use crate::{content, site::Site, views};

/// Builds the application: pages, static files and response headers.
pub fn router(site: Arc<Site>, assets_dir: &Path) -> Router {
    // Browsers may keep assets but must check they are still current before
    // using them. An unchanged file costs one small "not modified" reply, and
    // an edited stylesheet shows up on the next reload.
    let assets = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_static("public, no-cache"),
        ))
        .service(ServeDir::new(assets_dir));

    Router::new()
        .route("/", get(home))
        .route("/work/{slug}", get(project_without_slash))
        .route("/work/{slug}/", get(project))
        .route("/healthz", get(|| async { "ok" }))
        .nest_service("/assets", assets)
        .fallback(not_found)
        .layer(SetResponseHeaderLayer::if_not_present(
            X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            REFERRER_POLICY,
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(CompressionLayer::new())
        .with_state(site)
}

/// Serves the site until Ctrl+C is pressed.
pub async fn serve(site: Arc<Site>, assets_dir: &Path, addr: SocketAddr) -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Serving the portfolio at http://{}", listener.local_addr()?);
    println!("Press Ctrl+C to stop.");

    axum::serve(listener, router(site, assets_dir))
        .with_graceful_shutdown(async {
            // If the signal handler cannot be installed, keep serving.
            if tokio::signal::ctrl_c().await.is_err() {
                std::future::pending::<()>().await;
            }
        })
        .await
}

async fn home(State(site): State<Arc<Site>>) -> Html<String> {
    Html(views::home(&site).into_string())
}

async fn project(State(site): State<Arc<Site>>, UrlPath(slug): UrlPath<String>) -> Response {
    match content::find_project(&slug) {
        Some(project) => Html(views::project(&site, project).into_string()).into_response(),
        None => not_found(State(site)).await.into_response(),
    }
}

/// Project pages live at `/work/<slug>/`, matching the exported folders.
async fn project_without_slash(
    State(site): State<Arc<Site>>,
    UrlPath(slug): UrlPath<String>,
) -> Response {
    match content::find_project(&slug) {
        Some(project) => Redirect::permanent(&site.url(&project.path())).into_response(),
        None => not_found(State(site)).await.into_response(),
    }
}

async fn not_found(State(site): State<Arc<Site>>) -> (StatusCode, Html<String>) {
    (
        StatusCode::NOT_FOUND,
        Html(views::not_found(&site).into_string()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    fn app() -> Router {
        let assets = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
        router(Arc::new(Site::new("", &assets)), &assets)
    }

    async fn status(path: &str) -> StatusCode {
        let request = Request::builder().uri(path).body(Body::empty()).unwrap();
        app().oneshot(request).await.unwrap().status()
    }

    #[tokio::test]
    async fn pages_respond() {
        assert_eq!(status("/").await, StatusCode::OK);
        assert_eq!(status("/healthz").await, StatusCode::OK);
        for project in content::PROJECTS {
            assert_eq!(status(&project.path()).await, StatusCode::OK);
        }
    }

    #[tokio::test]
    async fn assets_are_served() {
        assert_eq!(status("/assets/css/site.css").await, StatusCode::OK);
        assert_eq!(status("/assets/js/site.js").await, StatusCode::OK);
        assert_eq!(status("/assets/nope.png").await, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn unknown_addresses_are_404() {
        assert_eq!(status("/nope").await, StatusCode::NOT_FOUND);
        assert_eq!(status("/work/nope/").await, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn missing_slash_redirects() {
        assert_eq!(
            status("/work/lookout").await,
            StatusCode::PERMANENT_REDIRECT
        );
    }
}
