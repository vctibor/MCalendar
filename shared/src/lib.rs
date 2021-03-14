use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Month {
    pub month: u32,
    pub year: u32,
    pub name: String,
    pub weeks: Vec<Week>
}

impl Month {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Get previous month as tuple (month, year).
    pub fn previous(&self) -> (u32, u32) {
        if self.month == 1 {
            (12, self.year - 1)
        } else {
            (self.month - 1, self.year)
        }
    }

    /// Get next month as tuple (month, year).
    pub fn next(&self) -> (u32, u32) {
        if self.month == 12 {
            (1, self.year + 1)
        } else {
            (self.month + 1, self.year)
        }
    }

    pub fn empty() -> Month {
        Month {
            month: 0,
            year: 0,
            name: "".to_owned(),
            weeks: vec!()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Week {
    pub days: Vec<Day>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Day {
    pub day: u32,
    pub weekday: String,
    pub event: String,
    pub is_non_workday: bool,
    pub is_current_day: bool,
}