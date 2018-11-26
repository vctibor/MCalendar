extern crate handlebars;
extern crate chrono;
extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use handlebars::Handlebars;
use chrono::prelude::*;
use chrono::NaiveDate;
use serde_json::Value;

const FORMAT: &str = "%Y-%m-%d";

#[derive(Serialize, Deserialize)]
struct Entry {
    rows: Vec<Row>
}

#[derive(Serialize, Deserialize)]
struct Row {
    year: u32,
    month: u32,
    day: u32,
    weekday: String,
    event: String
}

fn weekday(i: chrono::Weekday) -> String {
    match i {
        Weekday::Mon => String::from("Pondělí"),
        Weekday::Tue => String::from("Úterý"),
        Weekday::Wed => String::from("Středa"),
        Weekday::Thu => String::from("Čtvrtek"),
        Weekday::Fri => String::from("Pátek"),
        Weekday::Sat => String::from("Sobota"),
        Weekday::Sun => String::from("Neděle")
    }
}

fn read_month(month: u32, year: u32) -> Entry {
    
    let mut result: Vec<Row> = Vec::new();

    let days = get_month_days(month, year);
    
    for day in days {

        let key = day.format(FORMAT).to_string();    

        let weekday: String = weekday(day.weekday());

        let entry = Row {
            year: day.year() as u32,
            month: day.month(),
            day: day.day(),
            weekday: weekday,
            event: String::from("")
        };

        result.push(entry);
    }
    
    let result = result;
    return Entry { rows: result };
}

fn get_month_days(month: u32, year: u32) -> Vec<NaiveDate> {    
    let mut days = Vec::<NaiveDate>::new();    
    let mut dt = NaiveDate::from_ymd(year as i32, month, 01);

    loop {
        days.push(dt.clone());
        dt = dt.succ();
        if dt.month() != month {
            break;
        }
    }

    let days = days;
    return days;
}


fn main() {

    let month = read_month(11, 2018);

    let json_value: Value = json!(month);

    println!("{}", json_value);

    let mut handlebars = Handlebars::new();

    let template = handlebars.register_template_file("month", "./templates_input/index.hbs").unwrap();

    let res = handlebars.render("month", &json_value).unwrap();

    let mut file = File::create(&"./templates_output/index.html").unwrap();
    file.write_all(res.as_bytes()).unwrap();
}