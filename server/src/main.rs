use axum::{
    body::{Body, Bytes},
    http::{self, HeaderValue, Method, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use chrono::Local;
use dotenv::dotenv;
use once_cell::sync::OnceCell;
use sqlx::postgres::{PgPoolOptions, Postgres};
use sqlx::Pool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

// use db::app_configure::AppConfigure;
static POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    POOL.set(pool).unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 65528));
    println!("Server listening on {}...", addr);

    let cors_layer = CorsLayer::new()
        .allow_origin("http://localhost:65528".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([http::header::CONTENT_TYPE]);

    let app = Router::new()
        .layer(cors_layer)
        .route("/", get(hello_world))
        .layer(middleware::from_fn(custom_middleware));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn custom_middleware(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print(format!("{} {}", parts.method, parts.uri).as_str(), body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));
    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("Response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<T>(direction: &str, body: T) -> Result<Bytes, (StatusCode, String)>
where
    T: axum::body::HttpBody<Data = Bytes>,
    T::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Failed to read {} body: {}", direction, err),
            ))
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        println!(
            "[{}] {} body = {:?}",
            Local::now().format("%Y年%m月%d日 %H:%M:%S"),
            direction,
            body
        );
        if direction == "Response" {
            print!("{}", "\n");
        }
    }
    Ok(bytes)
}

async fn hello_world() -> impl IntoResponse {
    Json("Hello World!")
}
