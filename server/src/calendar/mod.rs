//! Business logic layer.
use chrono::{NaiveDate, Datelike};
use sqlx::{Pool, Postgres};

use mcalendar_shared::*;

mod holidays;
use holidays::get_holidays;

mod data_access;
use data_access::{read_events, write_event};


fn get_weekday_name(i: chrono::Weekday) -> String {
    use chrono::Weekday::*;
    match i {
        Mon => String::from("Pondělí"),
        Tue => String::from("Úterý"),
        Wed => String::from("Středa"),
        Thu => String::from("Čtvrtek"),
        Fri => String::from("Pátek"),
        Sat => String::from("Sobota"),
        Sun => String::from("Neděle")
    }
}

fn get_month_name(m: u32) -> String {
    match m {
         1 => String::from("Leden"),
         2 => String::from("Únor"),
         3 => String::from("Březen"),
         4 => String::from("Duben"),
         5 => String::from("Květen"),
         6 => String::from("Červen"),
         7 => String::from("Červenec"),
         8 => String::from("Srpen"),
         9 => String::from("Zaří"),
        10 => String::from("Říjen"),
        11 => String::from("Listopad"),
        12 => String::from("Prosinec"),
         _ => String::from(""),
    }
}

fn get_month_days(month: u32, year: u32) -> Vec<NaiveDate> {    
    let mut days = Vec::<NaiveDate>::new();    
    let mut dt = NaiveDate::from_ymd(year as i32, month, 1);

    loop {
        days.push(dt);
        dt = dt.succ();
        if dt.month() != month {
            break;
        }
    }
    
    days
}

pub async fn read_month(pool: &Pool<Postgres>, month: u32, year: u32) -> Month {

    //let start = std::time::Instant::now();

    let (mut events, holidays) = tokio::join!(
        read_events(pool, month, year),
        get_holidays(month, year)
    );

    let name = get_month_name(month);

    let days: Vec<NaiveDate> = get_month_days(month, year);

    let mut weeks: Vec<Week> = Vec::with_capacity(5);

    let mut week: Vec<Day> = Vec::with_capacity(7);

    for day in days { 

        let weekday = day.weekday();

        let mut non_workday =
            weekday == chrono::Weekday::Sat ||
            weekday == chrono::Weekday::Sun;

        let weekday: String = get_weekday_name(weekday);

        let day: u32 = day.day();

        let mut event = events.remove(&day).unwrap_or("".to_owned());

        if holidays.contains_key(&day) {
            let holiday = holidays.get(&day);

            if let Some(holiday) = holiday {
                non_workday = true;

                if event == "" {
                    event = holiday.to_string();
                }
            }
        }
        
        let entry = Day {
            day,
            weekday: weekday.clone(),
            event,
            is_non_workday: non_workday
        };

        week.push(entry);

        if weekday == get_weekday_name(chrono::Weekday::Sun) {
            weeks.push(Week { days: week });
            week = Vec::with_capacity(7);
        }
    }

    if !week.is_empty() {
        weeks.push(Week { days: week });
    }    

    //let elapsed = start.elapsed();
    //println!("read_month: {:?}", elapsed);

    Month { month, year, name, weeks }
}

/// Write events into database.
pub async fn write_month(pool: &Pool<Postgres>, month: Month) {
    for week in &month.weeks {
        for day in &week.days {
            if &day.event != "" {
                let event = day.event.clone();
                write_event(&pool, day.day, month.month, month.year, event).await;
            }
        }
    }
}