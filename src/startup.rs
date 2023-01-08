use crate::configuration::Settings;
use crate::handlers::about::about_handler;
use crate::handlers::composer::composer_handler;
use crate::handlers::error::error_handler;
use crate::handlers::helpers::handle_static_asset_error;
use crate::handlers::index::index_handler;
use crate::handlers::not_found::not_found_handler;
use crate::handlers::search::search_handler;
use crate::handlers::work::work_handler;
use crate::repositories::database::{get_connection_pool, Database};
use axum::body::BoxBody;
use axum::http::{header, HeaderValue};
use axum::response::Response;
use axum::routing::{get, get_service, IntoMakeService};
use axum::{Extension, Router, Server};
use hyper::server::conn::AddrIncoming;
use std::convert::Infallible;
use std::sync::Arc;
use tera::Tera;
use tower::util::AndThenLayer;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;

/// Application data for rendering in html templates.
pub struct AppData {
    pub static_assets_url: String,
    pub umami_id: String,
}

impl AppData {
    /// Creates new application data package.
    pub fn new(static_assets_url: &str, umami_id: &str) -> Self {
        Self {
            static_assets_url: static_assets_url.to_string(),
            umami_id: umami_id.to_string(),
        }
    }
}

/// Initialises tera html templates.
fn init_templates() -> Tera {
    match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    }
}

/// Returns default headers to be applied to all served resources.
fn add_cache_headers() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::if_not_present(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=604800"),
    )
}

fn add_no_cache_headers() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::if_not_present(
        header::CACHE_CONTROL,
        HeaderValue::from_static("private, max-age=0"),
    )
}

async fn add_security_headers(mut res: Response<BoxBody>) -> Result<Response<BoxBody>, Infallible> {
    let headers = res.headers_mut();
    headers.append(
        header::REFERRER_POLICY,
        HeaderValue::from_static("no-referrer"),
    );
    headers.append(
        header::STRICT_TRANSPORT_SECURITY,
        HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
    );
    headers.append(
        "permissions-policy",
        HeaderValue::from_static("geolocation=(), microphone=()"),
    );
    headers.append(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static("default-src 'none'; manifest-src 'self'; connect-src 'self' https://analytics.umami.is; script-src 'self' https://analytics.umami.is; style-src 'self'; img-src 'self' https://static.zunh.dev"),
    );
    headers.append(
        header::X_XSS_PROTECTION,
        HeaderValue::from_static("1; mode=block"),
    );
    headers.append(
        header::X_FRAME_OPTIONS,
        HeaderValue::from_static("sameorigin"),
    );
    headers.append(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.append(
        "X-Permitted-Cross-Domain-Policies",
        HeaderValue::from_static("none"),
    );
    Ok(res)
}

/// Builds web server.
pub async fn build_app(
    configuration: Settings,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, anyhow::Error> {
    let address = format!("0.0.0.0:{}", configuration.application.port);
    //let listener = TcpListener::bind(&address)?;
    let database = Arc::new(Database {
        pg_pool: get_connection_pool(&configuration.database),
    });
    let templates = init_templates();
    let app_data = AppData::new(&configuration.static_assets_url, &configuration.umami_id);
    let serve_dir = get_service(ServeDir::new("static"))
        .handle_error(handle_static_asset_error)
        .layer(add_cache_headers());
    let router = Router::new()
        .route("/about", get(about_handler))
        .route("/", get(index_handler))
        .route("/api/search", get(search_handler))
        .route("/composer/:slug", get(composer_handler))
        .route("/error", get(error_handler))
        .route("/composer/:slug/work/:id", get(work_handler))
        .nest_service("/static", serve_dir)
        .fallback_service(get(not_found_handler))
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new())
                .layer(add_no_cache_headers())
                .layer(AndThenLayer::new(add_security_headers))
                .layer(Extension(database))
                .layer(Extension(Arc::new(templates)))
                .layer(Extension(Arc::new(app_data))),
        );

    let server = axum::Server::bind(&address.parse().unwrap()).serve(router.into_make_service());
    Ok(server)
}
