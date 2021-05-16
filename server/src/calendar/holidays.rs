//! Interface to web service kayaposoft.com to get public holidays.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Holiday {
    date: Date,
    name: Vec<LocalizedText>,
    note: Option<Vec<LocalizedText>>,
    flags: Option<Vec<String>>,
    holidayType: String
}

#[allow(non_snake_case)]
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

/// Calls external web service to obtain list of public holidays for given month and year.
pub async fn get_holidays(month: u32, year: u32) -> HashMap<u32, String> {

    let now = std::time::Instant::now();

    let mut dict = HashMap::new();

    if year < 1900 {
        return dict;
    }

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

    println!("get_holidays: {:?}", now.elapsed());

    dict
}