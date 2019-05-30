extern crate chrono;
extern crate clap;
extern crate once_cell;
extern crate serde;
extern crate handlebars;
extern crate reqwest;
extern crate toml;
extern crate postgres;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rouille;

use std::io::Read;
use std::collections::HashMap;
use std::fs::File;

use once_cell::sync::OnceCell;
use chrono::prelude::*;
use chrono::NaiveDate;
use clap::{App, Arg};
use serde_json::Value;
use handlebars::Handlebars;
use rouille::Response;    

use postgres::{Connection, TlsMode};

mod data_access;
mod model;

use data_access::*;

use model::*;



/*
TODO: 
- REMOVE UNWRAPS, add error handling
- use human_panic crate
- logovani
- split into multiple files
- replace Sled with postgres

error handling v mcalendar:

- chyba v mainu, nonrecoverable, pred spustenim serveru ->
panic se smysluplnou chybovou hlaskou

- chyba v request handleru -> return http error

- pomocne funkce -> vracet option nebo result typy

*/

static HBS: OnceCell<Handlebars> = OnceCell::INIT;

static CONN_STR: OnceCell<String> = OnceCell::INIT;


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


// Calls external web service to obtain list of
//  public holidays for given month and year.
fn get_holidays(month: u32, year: u32) -> HashMap<u32, String> {

    let mut dict = HashMap::new();

    let addr: &str = &format!(
        "https://kayaposoft.com/enrico/json/v2.0/?action=getHolidaysForMonth&month={0}&year={1}&country=cz&holidayType=public_holiday",
        month, year);

    // TODO: Create and use single Reqwest client for whole app

    let mut body = match reqwest::get(addr) {
        Ok(val) => val,
        Err(msg) => {
            println!("WARNING: Failed to request holidays for month {} and year {}.
                Message: {}", month, year, msg);
            return dict;
        }
    };

    let body = match body.text() {
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

    // Rewrite using iterator or something?

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

fn read_month(month: u32, year: u32) -> Month {

    let conn_str =  CONN_STR.get().unwrap();
    let conn: Connection = Connection::connect(conn_str.clone(), TlsMode::None).unwrap();


    let month_name = get_month_name(month);

    let days: Vec<NaiveDate> = get_month_days(month, year);
    
    let mut weeks: Vec<Week> = Vec::new();

    let mut week: Vec<Day> = Vec::new();

    let holidays = get_holidays(month, year);

    for day in days { 

        let weekday = day.weekday();

        let mut non_workday =
            weekday == Weekday::Sat ||
            weekday == Weekday::Sun;

        let weekday: String = get_weekday_name(weekday);

        let day = day.day();

        let mut event: String =
            read_event(&conn, day, month, year);

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

        if weekday == get_weekday_name(Weekday::Sun) {
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

fn now() -> NaiveDate {
    let d = Local::now().date();
    NaiveDate::from_ymd(d.year(), d.month(), d.day())
}

fn index_handler() -> Response {
    let now = now();
    index_month_handler(now.year() as u32, now.month())
}

fn index_month_handler(year: u32, month: u32) -> Response {    
    let month = read_month(month, year);
    let json_value: Value = json!(month);
    let handlebars = HBS.get().unwrap();
    let res = handlebars.render("month", &json_value).unwrap();
    rouille::Response::html(res)
}

fn read_month_handler(year: u32, month: u32) -> Response {
    let entries = read_month(month, year);
    let res = serde_json::to_string(&entries).unwrap();
    rouille::Response::html(res)
}

fn write_event_handler(
    year: u32,
    month: u32,
    day: u32,
    request: &rouille::Request)
{
    let mut event: String = "".to_string();
    request.data().unwrap().read_to_string(&mut event).unwrap();

    let conn_str =  CONN_STR.get().unwrap();
    let conn: Connection = Connection::connect(conn_str.clone(), TlsMode::None).unwrap();

    write_event(&conn, day, month, year, event);
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


    // Read configuration file

    let filename = options.value_of("file")
        .expect("Path to configuration file is required parameter. Aborting.");
    
    let mut file = File::open(filename)
        .expect("Couldn't open configuration file. Aborting.");

    let mut contents = String::new();        

    file.read_to_string(&mut contents)
        .expect("Couldn't read configuration file. Aborting.");

    let config: Config = toml::from_str(&contents)
        .expect("Couldn't parse configuration file. Make sure it is valid TOML. Aborting.");

    // Create owned copies of configuration parameters,
    //  so we can pass them to different functions.

    let address = config.address.to_owned();
    let port = config.port.to_owned();
    let connection_string = config.conn_string.to_owned();
    let templates_location = config.templates.to_owned();
    let wwwroot_location = config.wwwroot.to_owned();


    CONN_STR.set(connection_string)
        .expect("Failed to register connection string. Aborting.");

    // Setup Handlebars

    let handlebars = {

        let mut handlebars = Handlebars::new();

        let index = templates_location + "//index.hbs";

        handlebars.register_template_file("month", index)
            .expect("Failed to register template to Handlebars registry. Aborting.");

        handlebars
    };

    HBS.set(handlebars)
        .expect("Couldn't set Handlebars registry to OnceCell, it was already used. Aborting.");



    // Start server

    let addr = address + ":" + &port.to_string();

    println!("Started server on {}", addr);

    rouille::start_server(addr, move |request| {
    
        let response = rouille::match_assets(&request, &wwwroot_location);

        if response.is_success() {
            return response;
        }

        router!(request,
            (GET) (/) => { index_handler() },

            (GET) (/{year: u32}/{month: u32}) => {
                index_month_handler(year, month)
            },

            (GET) (/read-month/{year: u32}/{month: u32}) => {
                read_month_handler(year, month)
            },

            (POST) (/write-event/{year: u32}/{month: u32}/{day: u32}) => {
                write_event_handler(year, month, day, &request);                
                rouille::Response::empty_204()
            },

            _ => rouille::Response::empty_404()
        )
    });
}
