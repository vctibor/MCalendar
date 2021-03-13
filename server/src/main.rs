// #[macro_use] extern crate serde_json;
// #[macro_use] extern crate serde_derive;
// #[macro_use] extern crate rouille;
// 
// use std::io::Read;
// 
// use std::fs::File;
// 
// use once_cell::sync::OnceCell;
// 
// 
// use clap::{App, Arg};
use serde_json;
// use handlebars::Handlebars;
// use rouille::Response;    
// 
// use postgres::{Connection, TlsMode};
// 
mod data_access;
// mod model;
// 
//use data_access::*;
// use model::*;

use std::collections::HashMap;

use actix_web::{get, post, web, App, HttpServer, Result};
use actix_web_static_files::ResourceFiles;

use chrono::{NaiveDate, Datelike, Local};

use mcalendar_shared::*;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

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

/// Calls external web service to obtain list of public holidays for given month and year.
async fn get_holidays(month: u32, year: u32) -> HashMap<u32, String> {

    let mut dict = HashMap::new();

    let addr: &str = &format!(
        "http://kayaposoft.com/enrico/json/v2.0/?action=getHolidaysForMonth&month={0}&year={1}&country=cz",
        month, year);

    let response = reqwest::get(addr).await;

    let body = match response {
        Ok(val) => val,
        Err(msg) => {
            println!("WARNING: Failed to request holidays for month {} and year {}.
                Message: {}", month, year, msg);
            return dict;
        }
    };

    let body = match body.text().await {
        Ok(val) => val,
        Err(msg) => {
            println!("WARNING: Failed to request holidays for month {} and year {}.
                Message: {}", month, year, msg);
            return dict;
        }
    };

    let mut holidays: Vec<Holiday> = match serde_json::from_str(&body) {
        Ok(val) => val,
        Err(msg) => {
            println!("WARNING: Failed to parse holiday response JSON for month {} and year {}.
                Message: {}", month, year, msg);
            return dict;
        }
    };

    for _ in 0..holidays.len() {

        // Pop top element from vector
        let mut holiday = holidays.remove(0);

        // Get day number of holiday
        let day = holiday.date.day;

        let mut holiday_name = "".to_string();

        // Holiday name is actually dictionary for multiple languages.
        // We attempt to find 'cs' variant.
        // Perhaps could be rewritted to simpler form. 
        for _ in 0..holiday.name.len() {

            // Pop top element
            let name = holiday.name.remove(0);
            
            if name.lang == "cs" {
                holiday_name = name.text;
                break;
            }
        }

        dict.insert(
            day, holiday_name
        );
    }

    dict
}

async fn read_month(month: u32, year: u32) -> Month {

    //let conn_str = CONN_STR.get().unwrap();
    //let conn: Connection = Connection::connect(conn_str.clone(), TlsMode::None).unwrap();


    let month_name = get_month_name(month);

    let days: Vec<NaiveDate> = get_month_days(month, year);
    
    let mut weeks: Vec<Week> = Vec::new();

    let mut week: Vec<Day> = Vec::new();

    let holidays = get_holidays(month, year).await;

    for day in days { 

        let weekday = day.weekday();

        let mut non_workday =
            weekday == chrono::Weekday::Sat ||
            weekday == chrono::Weekday::Sun;

        let weekday: String = get_weekday_name(weekday);

        let day = day.day();

        let mut event = data_access::read_event(day, month, year);
        
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
            week = Vec::new();
        }
    }

    if !week.is_empty() {
        weeks.push(Week { days: week });
    }
    
    Month {
        month,
        year,
        name: month_name,
        weeks
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

/// Get events for given month in given year.
#[get("/api/{year}/{month}")]
async fn get_events(path: web::Path<(u32, u32)>) -> Result<String>
{
    let (year, month) = path.into_inner();
    Ok(read_month(month, year).await.to_json())
}

/// Write events for given month in given year.
#[post("/api/{year}/{month}")]
async fn write_events(path: web::Path<(u32, u32)>) -> Result<String>
{
    let (year, month) = path.into_inner();

    /*
    let month = read_month(month, year);
    let json_value: Value = json!(month);
    let handlebars = HBS.get().unwrap();
    let res = handlebars.render("month", &json_value).unwrap();
    */
    

    Ok(format!("Welcome {}, user_id {}!", year, month))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    let addr = "127.0.0.1:9000";

    println!("Serving on {}", addr);

    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .service(get_events)
            .service(write_events)
            .service(ResourceFiles::new("/", generated))
    })
    .bind(addr)?
    .run()
    .await
}