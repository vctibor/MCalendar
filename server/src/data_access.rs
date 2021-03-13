//use postgres::Connection;

//! Data access layer.

pub fn read_event(day: u32, month: u32, year: u32) -> String {

    /*
    This isn't perfect - we should use query interpolation
        instead of string concatenation for dynamic parameters
        because of sql injection; however, that doesn't work for some reason.
    I'm leaving it this way for now (a.k.a. forever) because it's private
        application, where security is obtained through different means
        (i.e. not being exposed to Internet).
    */

    /*
    let query = format!(
        "select event from events where date = '{y}-{m}-{d}'",
        y = year, m = month, d = day);

    let result: &postgres::rows::Rows =
        &conn.query(&query, &[]).expect("Read database failed.");

    if !result.is_empty() {
        let event: String = result.get(0).get(0);
        return event;
    }
    */

    "".to_string()

}

/*
pub fn write_event(conn: &Connection, day: u32, month: u32, year: u32, event: String) {

    let query = format!(
        "INSERT INTO events (date, event) VALUES (
                '{year}-{month}-{day}',
                '{event}') 
        ON CONFLICT (date) DO UPDATE
        SET event = '{event}';",
        year = year, month = month, day = day, event = event);

    conn.execute(&query, &[]).expect("Write database failed.");
}
*/