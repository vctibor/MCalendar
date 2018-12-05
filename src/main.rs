extern crate sled;
extern crate chrono;
extern crate clap;
extern crate once_cell;
extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rouille;
extern crate handlebars;

use once_cell::sync::OnceCell;
use sled::{Tree, ConfigBuilder};
use chrono::prelude::*;
use chrono::NaiveDate;
use clap::{App, Arg};
use serde_json::Value;
use handlebars::Handlebars;
use std::io::Read;

// TODO: Split into multiple files

// Default path to file persistenting key-value store.
const PATH: &str = "./mcal_db.sled";

// Format in which is date used as key in key-value store.
const FORMAT: &str = "%Y-%m-%d";

static TREE: OnceCell<sled::Tree> = OnceCell::INIT;

static HBS: OnceCell<Handlebars> = OnceCell::INIT;

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

        let mut event = String::from("");

        if let Ok(Some(x)) = TREE.get().unwrap().get(&*key) {
            event = String::from(std::str::from_utf8(&x).unwrap());
        };

        let entry = Day {
            day: day.day(),
            weekday: weekday,
            event: event,
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

fn write_event(day: u32, month: u32, year: u32, event: String) -> bool {
    
    let day = NaiveDate::from_ymd(year as i32, month, day);
    let key = day.format(FORMAT).to_string();
    let value = event.into_bytes();

    match TREE.get().unwrap().set(key, value) {
        Ok(_) => true,
        Err(_) => false
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

fn now() -> NaiveDate {
    let d = Local::now().date();
    NaiveDate::from_ymd(d.year(), d.month(), d.day())
}

fn index_handler() -> String {
    let now = now();
    index_month_handler(now.year() as u32, now.month())
}

fn index_month_handler(year: u32, month: u32) -> String {    
    let month = read_month(month, year);
    let json_value: Value = json!(month);
    let handlebars = HBS.get().unwrap();
    let res = handlebars.render("month", &json_value).unwrap();
    res
}

fn read_month_handler(year: u32, month: u32) -> String {
    let entries = read_month(month, year);
    let res = serde_json::to_string(&entries).unwrap();
    res
}

fn write_event_handler(year: u32, month: u32, day: u32, request: &rouille::Request) -> () {
    let mut event: String = "".to_string();
    request.data().unwrap().read_to_string(&mut event).unwrap();
    write_event(day, month, year, event);
}

fn main() {

    let options = App::new("Calendar")
        /*
        .arg(Arg::with_name("port")
            .long("port")
            .value_name("PORT")
            .help("HTTP port to listen on")
            .required(true)
            .takes_value(true))
        */
        .arg(Arg::with_name("file")
            .long("file")
            .value_name("FILE")
            .help("Sled database file")
            .required(false)
            .takes_value(true))
        .get_matches();

    // Setup Sled

    let sled_file = options.value_of("file").unwrap_or(PATH);

    let sled_config = ConfigBuilder::new()
        .temporary(false)
        .path(sled_file)
        .build();

    let tree = Some(Tree::start(sled_config).unwrap());
    
    TREE.set(tree.unwrap()).unwrap();


    // Setup Handlebars

    let mut handlebars: handlebars::Handlebars = Handlebars::new();
    handlebars.register_template_file("month", "./templates//index.hbs").unwrap();

    HBS.set(handlebars).unwrap();

    rouille::start_server("127.0.0.1:8000", move |request| {
    
        let response = rouille::match_assets(&request, "./static/");

        if response.is_success() {
            return response;
        }

        router!(request,
            (GET) (/) => {
                let res = index_handler();
                rouille::Response::html(res)
            },

            (GET) (/{year: u32}/{month: u32}) => {
                let res = index_month_handler(year, month);
                rouille::Response::html(res)
            },

            (GET) (/read-month/{year: u32}/{month: u32}) => {
                let res = read_month_handler(year, month);
                rouille::Response::html(res)
            },

            (POST) (/write-event/{year: u32}/{month: u32}/{day: u32}) => {
                write_event_handler(year, month, day, &request);                
                rouille::Response::empty_204()
            },

            _ => rouille::Response::empty_404()
        )
    });
}
