use axum::{
    body::{Body, Bytes},
    extract::Path,
    http::{self, HeaderValue, Method, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::Local;
use db::app_configure::AppConfigure;
use dotenv::dotenv;
use once_cell::sync::OnceCell;
use serde_json::json;
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
        .route("/all-app-configure", get(all_app_configure))
        .route("/query-one/:name", get(query_one))
        .route("/update-one", post(update_one))
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

async fn all_app_configure() -> impl IntoResponse {
    let rows = AppConfigure::all(&POOL.get().unwrap()).await;
    Json(json!({
        "code": 1,
        "result": rows.iter().map(|r| r.to_json()).collect::<serde_json::Value>()
    }))
}

async fn query_one(Path(name): Path<String>) -> impl IntoResponse {
    let row = AppConfigure::query_by_name(&POOL.get().unwrap(), name).await;
    Json(json!({
        "code": 1,
        "result": row
    }))
}

async fn update_one(Json(body): Json<serde_json::Value>) -> impl IntoResponse {
    println!("{:?}", body);

    let name = body.get("name");
    let field = body.get("field");
    let value = body.get("value");

    if name.is_none() {
        return Json(json!({"code": 0, "result": "未找到配置名"}));
    }

    if field.is_none() {
        return Json(json!({"code": 0, "result": "未设置要修改的字段名"}));
    }

    if value.is_none() {
        return Json(json!({"code": 0, "result": "未设置要修改的字段值"}));
    }

    let update_status = AppConfigure::update_field_value_with_name(
        &POOL.get().unwrap(),
        name.unwrap().as_str().unwrap(),
        field.unwrap().as_str().unwrap(),
        value.unwrap().as_str().unwrap(),
    )
    .await;

    match update_status {
        true => Json(json!({"code": 1, "result": "success"})),
        false => Json(json!({"code": 0, "result": "fail"})),
    }

    
}
