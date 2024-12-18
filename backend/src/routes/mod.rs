pub mod meetup;

use axum::{
    http::{self, Method},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use hyper::StatusCode;
use meetup::{recommended_meetups_handler, search_handler};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use utoipa::{OpenApi, ToSchema};

use self::meetup::meetups_today_handler;
use meetup::{
    __path_meetups_today_handler, __path_recommended_meetups_handler, __path_search_handler,
};

pub fn app() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST]);

    let tracing = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    return Router::new()
        .route("/", get(app_version))
        .route("/today", get(meetups_today_handler))
        .route("/recommended", get(recommended_meetups_handler))
        .route("/search", post(search_handler))
        .layer(tracing)
        .layer(cors);
}

#[derive(OpenApi)]
#[openapi(paths(
    recommended_meetups_handler,
    meetups_today_handler,
    search_handler,
    app_version
))]
pub struct APIDoc;

#[derive(Debug)]
pub struct AppError(pub anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}
// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Serialize, Deserialize, Clone, JsonSchema, ToSchema)]
pub struct HomeResponse {
    pub version: String,
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Version of the server", body = HomeResponse)
    )
)]
pub async fn app_version() -> Result<Json<HomeResponse>, AppError> {
    /// Simplified `Cargo.toml` structure
    #[derive(Deserialize)]
    struct CargoToml {
        pub package: PackageKeys,
    }

    #[derive(Deserialize)]
    struct PackageKeys {
        // pub name: String,
        pub version: String,
    }

    let mut file = File::open("Cargo.toml").await?;
    let mut file_contents: String = Default::default();
    file.read_to_string(&mut file_contents).await?;
    let cargo_toml = toml::from_str::<CargoToml>(file_contents.as_str())?;

    return Ok(Json(HomeResponse {
        version: cargo_toml.package.version,
    }));
}
