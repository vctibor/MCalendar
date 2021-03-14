use seed::{prelude::*, *};
use mcalendar_shared::{Day, Month};

const ENTER_KEY: &str = "Enter";

struct Model {
    month: Month
}

fn color(day: &Day) -> String {
    if day.is_current_day {
        if day.is_non_workday {
            "#ffa600".to_owned()
        } else {
            "#999".to_owned()
        }
    } else {
        if day.is_non_workday {
            "#a86600".to_owned()
        } else {
            "#555".to_owned()
        }
    }
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
    orders.send_msg(Msg::FetchCurrentMonth);
    Model { month: Month::empty() }
}

#[derive(Clone)]
enum Msg {
    FetchCurrentMonth,
    FetchNextMonth,
    FetchPreviousMonth,
    Received(Month),

    ChangeDay(usize, usize, String),
    SubmitChanges
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchCurrentMonth => {
            orders.skip();
            orders.perform_cmd(async {
                let url = format!("/api/current");
                let response = fetch(url).await.expect("HTTP request failed");
            
                let month = response
                    .check_status() // ensure we've got 2xx status
                    .expect("status check failed")
                    .json::<Month>()
                    .await
                    .expect("deserialization failed");
            
                Msg::Received(month)
            });
        }

        Msg::FetchNextMonth => {
            let next = model.month.next();
            orders.skip();
            orders.perform_cmd(fetch_month(next.0, next.1));
        }

        Msg::FetchPreviousMonth => {
            let previous = model.month.previous();
            orders.skip();
            orders.perform_cmd(fetch_month(previous.0, previous.1));
        }

        Msg::Received(received) => {
            model.month = received;
        }

        Msg::ChangeDay(week_id, day_id, text) => {
            model.month.weeks[week_id].days[day_id].event = text;
        }

        Msg::SubmitChanges => {
            let url = format!("/api/{}/{}", model.month.year, model.month.month);

            let request = Request::new(url)
                .method(Method::Post)
                .json(&model.month)
                .expect("Serialization failed");

            orders.perform_cmd(async {
                let _response = fetch(request).await.expect("HTTP request failed");
                Msg::FetchCurrentMonth
            });
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
        month.weeks.iter().enumerate().map(|(week_id, week)| {
            div![
                table![
                    week.days.iter().enumerate().map(|(day_id, day)| {
                        div![
                            td![
                                C!["col_date"],
                                attrs![At::Style => format!("color:{}", color(&day))],
                                format!("{}. {}.", day.day, month.month),
                            ],
                            td![
                                C!["col_weekday"],
                                attrs![At::Style => format!("color:{}", color(&day))],
                                format!("{}", day.weekday),
                            ],
                            td![
                                C!["col_event"],
                                input![
                                    attrs! {
                                        At::Value => day.event.to_owned()
                                    },
                                    
                                    input_ev(Ev::Input, move |input| {
                                        Msg::ChangeDay(week_id, day_id, input)
                                    }),
                                    
                                    keyboard_ev(Ev::KeyDown, |keyboard_event| {
                                        IF!(keyboard_event.key() == ENTER_KEY => Msg::SubmitChanges)
                                    }),
                                ]
                            ],
                        ]
                    })
                ],
                br![],
            ]
        }),
        hr![],
    ]
}

fn view(model: &Model) -> Node<Msg> {
    div![
        heading(&model.month.name, model.month.year),
        body(model.month.clone())
    ]
}

#[wasm_bindgen(start)]
pub async fn start() {
    App::start("app", init, update, view);
}
