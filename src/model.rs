#[derive(Serialize, Deserialize)]
pub struct Month {
    pub month: u32,
    pub year: u32,
    pub name: String,
    pub weeks: Vec<Week>
}

#[derive(Serialize, Deserialize)]
pub struct Week {
    pub days: Vec<Day>
}

#[derive(Serialize, Deserialize)]
pub struct Day {
    pub day: u32,
    pub weekday: String,
    pub event: String,
    pub is_non_workday: bool
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: u32,
    pub dayOfWeek: u32
}

#[derive(Serialize, Deserialize)]
pub struct LocalizedText {
    pub lang: String,
    pub text: String
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Holiday {
    pub date: Date,
    pub name: Vec<LocalizedText>,
    pub note: Option<Vec<LocalizedText>>,
    pub flags: Option<Vec<String>>,
    pub holidayType: String
}

/* -- configuration -- */
#[derive(Debug, Deserialize)]
pub struct Config {
    /* IP address */
    pub address: String,

    /* port to listen on */
    pub port: u64,

    /* database connection string */
    pub conn_string: String,

    /* templates (handlebars) location */
    pub templates: String,

    /* Static files location */
    pub wwwroot: String
}