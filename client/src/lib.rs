use seed::{prelude::*, *};
use mcalendar_shared::Month;

struct Model {
    month: Option<Month>
}

async fn fetch_month(month: u32, year: u32) -> Msg {
    let url = format!("/api/{}/{}", year, month);
    let response = fetch(url).await.expect("HTTP request failed");

    let month = response
        .check_status() // ensure we've got 2xx status
        .expect("status check failed")
        .json::<Month>()
        .await
        .expect("deserialization failed");

    Msg::Received(month)
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    // TODO: server side endpoint to get data for current date
    orders.perform_cmd(fetch_month(3, 2021));
    Model { month: None }
}

#[derive(Clone)]
enum Msg {
    FetchNextMonth,
    FetchPreviousMonth,
    Received(Month),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchNextMonth => {
            match &model.month {
                Some(month) => {
                    let next = month.next();
                    orders.skip();
                    orders.perform_cmd(fetch_month(next.0, next.1));
                }
                None => { }
            }
        }

        Msg::FetchPreviousMonth => {
            match &model.month {
                Some(month) => {
                    let previous = month.previous();
                    orders.skip();
                    orders.perform_cmd(fetch_month(previous.0, previous.1));
                }
                None => { }
            }
        }

        Msg::Received(received) => {
            model.month = Some(received);
        }
    }
}

fn heading(month_name: &str, year: u32) -> Node<Msg> {
    div![
        C!["heading"],
        span![
            C!["arrow"],
            id!["arrow_previous"],
            ev(Ev::Click, |_| Msg::FetchPreviousMonth),
            "◀"
        ],
        span![
            C!["heading_label"],
            format!("{} {}", month_name, year)
        ],
        span![
            C!["arrow"],
            id!["arrow_next"],
            ev(Ev::Click, |_| Msg::FetchNextMonth),
            "▶"
        ],
        hr![],
    ]
}

fn body(month: Month) -> Node<Msg> {
    div![
        C!["calendar"],

        /*
        <td class="col_date">{{day}}. {{../../month}}.</td>
        <td class="col_weekday {{~#if is_non_workday}} non_working_day {{~/if~}}">{{weekday}}</td>
        <td class="col_event">
            <input type="text" data-day="{{day}}" value="{{event}}"/>
        </td>
        */

        month.weeks.iter().enumerate().map(|(_week_id, week)| {
            table![
                week.days.iter().enumerate().map(|(_day_id, day)| {
                    div![
                        td![
                            C!["col_date"], 
                            format!("{}. {}.", day.day, month.month),
                        ],
                        td![
                            C!["col_weekday"],
                            format!("{}", day.weekday),
                        ],
                        td![
                            C!["col_event"],
                            format!("{}", day.event),
                        ],
                    ]
                })
            ]
        }),
        hr![],
    ]
}

fn view(model: &Model) -> Node<Msg> {
    div![
        if let Some(month) = model.month.clone() {
            div![
                heading(&month.name, month.year),
                body(month)
            ]
        }
        else {
            div![]
        }
    ]
}

#[wasm_bindgen(start)]
pub async fn start() {
    App::start("app", init, update, view);
}
