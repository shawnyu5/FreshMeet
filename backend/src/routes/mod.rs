pub mod meetup;

use self::meetup::meetups_today_handler;
use axum::Router;
use common_axum::axum::{
    __path_app_version, app_version, attach_tracing_cors_middleware,
    generate_open_api_spec_from_open_api,
};
use meetup::{
    __path_meetups_today_handler, __path_recommended_meetups_handler, __path_search_handler,
};
use meetup::{recommended_meetups_handler, search_handler};
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn app() -> Router {
    let (router, mut api_spec) = OpenApiRouter::new()
        .routes(routes!(app_version))
        .routes(routes!(meetups_today_handler))
        .routes(routes!(recommended_meetups_handler))
        .routes(routes!(search_handler))
        .split_for_parts();

    api_spec.info.title = "freshmeet backend".to_string();
    api_spec.info.description = None;
    api_spec.info.contact = None;
    api_spec.info.license = None;

    generate_open_api_spec_from_open_api(api_spec, "open_api_spec.json")
        .expect("Failed to generate open API spec");
    info!("Generated open api spec");
    return attach_tracing_cors_middleware(router);
}
