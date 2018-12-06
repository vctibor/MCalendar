extern crate sled;
extern crate chrono;
extern crate clap;
extern crate once_cell;
extern crate serde;
extern crate handlebars;
extern crate reqwest;
extern crate toml;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rouille;

use std::io::Read;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use once_cell::sync::OnceCell;
use sled::{Tree, ConfigBuilder};
use chrono::prelude::*;
use chrono::NaiveDate;
use clap::{App, Arg};
use serde_json::{Value, Error};
use handlebars::Handlebars;

/*
TODO: 
- REMOVE UNWRAPS, add error handling
- logovani
- systemd unit
- split into multiple files

Konfigurace:
- sled file lcoation
- templates file location
- static files folder location
- ip address
- port
*/

/*
// Default path to file persistenting key-value store.
const PATH: &str = "./mcal_db.sled";
*/

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

/* -- holiday request -- */
#[derive(Serialize, Deserialize)]
struct Date {
    day: u32,
    month: u32,
    year: u32,
    dayOfWeek: u32
}

#[derive(Serialize, Deserialize)]
struct LocalizedText {
    lang: String,
    text: String
}

#[derive(Serialize, Deserialize)]
struct Holiday {
    date: Date,
    name: Vec<LocalizedText>,
    note: Option<Vec<LocalizedText>>,
    flags: Option<Vec<String>>,
    holidayType: String
}
/* ---- */


/* -- configuration -- */
#[derive(Debug, Deserialize)]
struct Config {
    /* IP address */
    address: String,

    /* port to listen on */
    port: u64,

    /* sled file location */
    sled: String,

    /* templates (handlebars) location */
    templates: String,

    /* Static files location */
    wwwroot: String
}
/* ---- */

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

fn get_holidays(month: u32, year: u32) -> HashMap<u32, String> {

    let mut holidays_dict = HashMap::new();

    let addr: &str = &format!(
        "https://kayaposoft.com/enrico/json/v2.0/?action=getHolidaysForMonth&month={0}&year={1}&country=cz&holidayType=public_holiday",
        month, year);

    let body = reqwest::get(addr).unwrap().text().unwrap();

    let holidays: Result<Vec<Holiday>, Error> = serde_json::from_str(&body);

    if let Err(_) = holidays {
        return holidays_dict;
    }

    let mut holidays = holidays.unwrap();

    for i in 0..holidays.len() {
        let mut holiday = holidays.remove(0);
        let day = holiday.date.day;

        let mut holiday_name = "".to_string();

        for j in 0..holiday.name.len() {
            let name = holiday.name.remove(0);
            let lang = name.lang;
            
            if lang == "cs" {
                holiday_name = name.text;
            }
        }

        holidays_dict.insert(
            day, holiday_name
        );
    }

    holidays_dict
}

fn read_month(month: u32, year: u32) -> Month {

    let month_name = get_month_name(month);

    let days: Vec<NaiveDate> = get_month_days(month, year);
    
    let mut weeks: Vec<Week> = Vec::new();

    let mut week: Vec<Day> = Vec::new();

    let holidays = get_holidays(month, year);

    for day in days {

        let key = day.format(FORMAT).to_string();    

        let weekday: String = get_weekday_name(day.weekday());

        let mut non_workday =
            day.weekday() == Weekday::Sat ||
            day.weekday() == Weekday::Sun;

        let mut event = String::from("");

        if let Ok(Some(x)) = TREE.get().unwrap().get(&*key) {
            event = String::from(std::str::from_utf8(&x).unwrap());
        };

        if holidays.contains_key(&day.day()) {
            let holiday = holidays.get(&day.day());

            if let Some(holiday) = holiday {
                non_workday = true;

                if event == "" {
                    event = holiday.to_string();
                }
            }
        }

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

    // Get options

    let options = App::new("Calendar")
        .arg(Arg::with_name("file")
            .index(1)
            .help("TOML config")
            .required(true)
            .takes_value(true))
        .get_matches();



    // Read configuration

    let filename = options.value_of("file").unwrap();
    
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let config: Config = toml::from_str(&contents).unwrap();

    let address = config.address.to_owned();
    let port = config.port.to_owned();
    let sled_location: String = config.sled.to_owned();
    let templates_location: String = config.templates.to_owned();
    let wwwroot_location: String = config.wwwroot.to_owned();



    // Setup Sled

    let sled_config = ConfigBuilder::new()
        .temporary(false)
        .path(sled_location)
        .build();

    let tree = Some(Tree::start(sled_config).unwrap());
    
    TREE.set(tree.unwrap()).unwrap();




    // Setup Handlebars

    let mut handlebars: handlebars::Handlebars = Handlebars::new();

    let index = templates_location + "//index.hbs";

    handlebars.register_template_file("month", index).unwrap();

    HBS.set(handlebars).unwrap();



    // Start server

    let addr = address + ":" + &port.to_string();

    println!("Started server on {}", addr);

    rouille::start_server(addr, move |request| {
    
        let response = rouille::match_assets(&request, &config.wwwroot);

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
