use std::fs;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::time::Date;
use string_builder::Builder;

#[derive(Debug)]
struct Row {
    date: Date,
    event: String
}

#[actix_web::main]
async fn main() {
    let conn_string = std::env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&conn_string).await.unwrap();

    let rows: Vec<Row> = sqlx::query_as!(Row,
        "select * from events order by date")
        .fetch_all(&pool)
        .await
        .unwrap()
        .into_iter()
        .filter(|row| !row.event.trim().is_empty())
        .collect();

    let mut builder = Builder::default();
    for row in rows {
        builder.append(format!("{} {}", row.date, row.event));
        builder.append("\n")
    }

    fs::write("data.txt", builder.string().unwrap()).unwrap()
}