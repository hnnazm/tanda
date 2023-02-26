use axum::{
    routing::get,
    Router,
    extract::State,
    http::{StatusCode},
};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[tokio::main]
async fn main() {
    const DATABASE_URL: &str = "postgres://tanda:password@db/tanda";

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await
        .expect("Failed to connect to database.");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/database-healthcheck", get(db_healthcheck))
        .with_state(pool);

    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn db_healthcheck(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
