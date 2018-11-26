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
struct Month {
    month: u32,
    year: u32,
    name: String,
    weeks: Vec<Week>
}

#[derive(Serialize, Deserialize)]
struct Week {
    days: Vec<Day>
}

#[derive(Serialize, Deserialize)]
struct Day {
    day: u32,
    weekday: String,
    event: String,
    is_non_workday: bool
}

fn get_weekday_name(i: chrono::Weekday) -> String {
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

fn read_month(month: u32, year: u32) -> Month {

    let month_name = get_month_name(month);

    let days = get_month_days(month, year);
    
    let mut weeks: Vec<Week> = Vec::new();

    let mut week: Vec<Day> = Vec::new();
    
    for day in days {

        let key = day.format(FORMAT).to_string();    

        let weekday: String = get_weekday_name(day.weekday());

        let non_workday =
            day.weekday() == Weekday::Sat ||
            day.weekday() == Weekday::Sun;

        let entry = Day {
            day: day.day(),
            weekday: weekday,
            event: String::from(""),
            is_non_workday: non_workday
        };

        week.push(entry);

        if day.weekday() == Weekday::Sun {
            weeks.push(Week { days: week });
            week = Vec::new();
        }
    }

    if week.len() != 0 {
        weeks.push(Week { days: week });
    }
    
    Month {
        month: month,
        year: year,
        name: month_name,
        weeks: weeks
    }    
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