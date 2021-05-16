//! Data access layer, sqlx, postgres.

use std::collections::HashMap;

use sqlx::types::time::Date;
use sqlx::{Pool, Postgres};

pub async fn read_events(pool: &Pool<Postgres>, month: u32, year: u32) -> HashMap<u32, String> {

    let now = std::time::Instant::now();

    #[allow(unused_imports)]
    use futures::stream::StreamExt;

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

    let events = rows.collect::<Vec<sqlx::Result<Row>>>().await
        .into_iter()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .filter(|x| x.day.is_some())
        .map(|x| (x.day.unwrap() as u32, x.event))
        .collect();

    println!("read_events: {:?}", now.elapsed());

    events
}

pub async fn write_events(pool: &Pool<Postgres>, events: Vec<(Date, String)>) -> bool {
    
    let dates: Vec<Date> = events.clone().into_iter().map(|x| x.0).collect();
    let events: Vec<String> = events.into_iter().map(|x| x.1).collect();

    sqlx::query!(
        "insert into events (date, event)
        select * from
        unnest($1::date[], $2::text[])
        on conflict (date) do update
        set event = excluded.event;",
        &dates, &events).execute(pool).await.is_ok()
}