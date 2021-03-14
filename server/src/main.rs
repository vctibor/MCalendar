use actix_web::{get, post, web, App, HttpServer, Result};
use actix_web_static_files::ResourceFiles;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use once_cell::sync::OnceCell;
use chrono::{Local, Datelike};

use mcalendar_shared::*;

mod calendar;
use calendar::{read_month, write_month};

static CONN_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

/// Get events for current month.
#[get("/api/current")]
async fn get_current() -> Result<String> {
    let now = Local::now().date();
    let pool = CONN_POOL.get().unwrap();
    let month: Month = read_month(&pool, now.month(), now.year() as u32).await;
    Ok(month.to_json())
}

/// Get events for given month in given year.
#[get("/api/{year}/{month}")]
async fn get_events(path: web::Path<(u32, u32)>) -> Result<String> {
    let (year, month) = path.into_inner();
    assert!(month > 0 && month < 13, "Month must be between 1 and 12!");
    let pool: &Pool<Postgres> = CONN_POOL.get().unwrap();
    let month: Month = read_month(pool, month, year).await;
    Ok(month.to_json())
}

/// Write events for given month in given year.
#[post("/api/{year}/{month}")]
async fn write_events(path: web::Path<(u32, u32)>, month_events: web::Json<Month>)
    -> Result<String>
{
    let (year, month) = path.into_inner();

    assert!(month > 0 && month < 13, "Month must be between 1 and 12!");

    assert!(month_events.month == month && month_events.year == year,
        "Mismatch between values in URI and body!");

    let pool = CONN_POOL.get().unwrap();

    write_month(&pool, month_events.into_inner()).await;

    Ok("".to_owned())
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    let conn_string = std::env::var("DATABASE_URL").unwrap();

    let pg_pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&conn_string).await.unwrap();

    CONN_POOL.set(pg_pool).unwrap();

    let addr = "0.0.0.0:9000";

    println!("Serving on {}", addr);

    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .service(get_current)
            .service(get_events)
            .service(write_events)
            .service(ResourceFiles::new("/", generated))
    })
    .bind(addr)?
    .run()
    .await
}