//! Data access layer, sqlx, postgres.

use sqlx::types::time::Date;
use sqlx::{Pool, Postgres};

pub async fn read_event(pool: &Pool<Postgres>, day: u32, month: u32, year: u32) -> String {
    struct Row {
        event: String
    }

    let date = Date::try_from_ymd(year as i32, month as u8, day as u8).unwrap();

    let row = sqlx::query_as!(Row,
        "select event from events where date = $1", date)
        .fetch_optional(pool)
        .await.unwrap();

    if let Some(row) = row {
        return row.event;
    }

    "".to_owned()
}

pub async fn write_event(pool: &Pool<Postgres>, day: u32, month: u32, year: u32, event: String) {
    let date = Date::try_from_ymd(year as i32, month as u8, day as u8).unwrap();

    sqlx::query!(
        "insert into events (date, event) values ($1, $2) 
        on conflict (date) do update
        set event = $2;",
        date, event).execute(pool).await;
}