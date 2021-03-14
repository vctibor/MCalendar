//! Data access layer, sqlx, postgres.

use std::collections::HashMap;

use sqlx::types::time::Date;
use sqlx::{Pool, Postgres};

use futures::stream::StreamExt;


pub async fn read_events(pool: &Pool<Postgres>, month: u32, year: u32) -> HashMap<u32, String> {
    
    struct Row {
        day: Option<f64>,
        event: String
    }

    let month: f64 = month as f64;
    let year: f64 = year as f64;

    let rows = sqlx::query_as!(Row, 
        "select extract(day from date) as day, event from events
        where extract(month from date) = $1
        and extract(year from date) = $2;", month, year)
        .fetch(pool);

    rows.collect::<Vec<sqlx::Result<Row>>>().await
        .into_iter()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .filter(|x| x.day.is_some())
        .map(|x| (x.day.unwrap() as u32, x.event))
        .collect()
}

pub async fn write_event(pool: &Pool<Postgres>, day: u32, month: u32, year: u32, event: String) {
    let date = Date::try_from_ymd(year as i32, month as u8, day as u8).unwrap();

    sqlx::query!(
        "insert into events (date, event) values ($1, $2) 
        on conflict (date) do update
        set event = $2;",
        date, event).execute(pool).await.unwrap();
}