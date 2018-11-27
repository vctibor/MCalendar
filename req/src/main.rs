extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::{Value, Error};

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


fn main() {

    let year = 2018;

    for month in 1 .. 12 {

        let addr: &str = &format!(
            "https://kayaposoft.com/enrico/json/v2.0/?action=getHolidaysForMonth&month={0}&year={1}&country=cz&holidayType=public_holiday",
            month, year);

        let body = reqwest::get(addr).unwrap().text().unwrap();

        //println!("{:?}", body);

        let holidays: Result<Vec<Holiday>, Error> = serde_json::from_str(&body);

        if let Err(_) = holidays {
            continue;
        }

        let holidays = holidays.unwrap();

        for h in holidays {
            println!("{}. {}. {} - {}", h.date.day, h.date.month, year, h.name[0].text);
        }

        //println!("{}", holidays.len());

        //println!("\n");
    }
}
