use dioxus::prelude::*;

use thousands::Separable;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Copy)]
struct GlobalState {
    dollars_per_year: Signal<f32>,
    hourly_rate: Signal<f32>,
    hours_per_week: Signal<f32>,
    global_view: Signal<String>,
}

#[component]
fn App() -> Element {
    let app_state = use_context_provider(|| GlobalState {
        dollars_per_year: Signal::new(0.0),
        hourly_rate: Signal::new(0.0),
        hours_per_week: Signal::new(40.0),
        global_view: Signal::new("inputs".to_string()),
    });

    rsx!(
        document::Link { rel: "icon", href: FAVICON },
        document::Link { rel: "stylesheet", href: MAIN_CSS },
        Home {}
    )
}

#[component]
pub fn Home() -> Element {
    let mut global_view = use_context::<GlobalState>().global_view;

    rsx! {
        if global_view() == "inputs" { Inputs {} } else { Results {} }
    }
}

fn cs(num: f32) -> String {
    let rounded = (num * 100.0).round() / 100.0;
    rounded.separate_with_commas()
}

#[component]
pub fn Results() -> Element {
    // Context
    let dollars_per_year = use_context::<GlobalState>().dollars_per_year;
    let hourly_rate = use_context::<GlobalState>().hourly_rate;
    let hours_per_week = use_context::<GlobalState>().hours_per_week;

    // Constants
    let bn = 1_000_000_000.0;
    let seconds_per_year = 31_536_000.0;

    // Maths
    let percent_of_a_bn = (dollars_per_year() / bn) * 100.0;
    let hours_to_a_bn = cs(bn / hourly_rate());
    let hours_in_a_year = cs((24 * 365) as f32);
    let bn_hourly = cs(bn / 40.0 / 52.0);
    let bn_pay_per_second = bn / seconds_per_year;
    let bn_makes_your_salary_minutes = cs(dollars_per_year() / bn_pay_per_second / 60.0);

    rsx! {
        p { "Your yearly income is ${cs(dollars_per_year())} which is {percent_of_a_bn}% of a billion dollars." }
        p { "At your current rate of pay, you would need to work {hours_to_a_bn} hours per year to reach a billion dollars. There are only {hours_in_a_year} hours in a year." }
        p { "A billionaire makes $480,769.20 an hour if they worked 40 hours a week, but they rarely ever work." }
        p { "A billionaire makes your yearly salary after {bn_makes_your_salary_minutes} minutes on the first day of the year." }
    }
}

#[component]
pub fn Inputs() -> Element {
    //Context
    let mut global_view = use_context::<GlobalState>().global_view;
    let mut dollars_per_year = use_context::<GlobalState>().dollars_per_year;
    let mut hourly_rate = use_context::<GlobalState>().hourly_rate;
    let mut hours_per_week = use_context::<GlobalState>().hours_per_week;

    // State
    let mut component_view = use_signal(|| "hourly".to_string());

    let mut handle_paid_click = move |event: Event<MouseData>, label: &str| {
        component_view.set(label.to_string());
    };

    rsx! {
       h2 { "How are you paid?" }
       div { class: "button-wrapper",
       button { id: "hourly-button", onclick: move |event| handle_paid_click(event, "hourly"), "Hourly" }
       button { id: "yearly-button", onclick: move |event| handle_paid_click(event, "yearly"), "Yearly" }
       }

       p { "Yearly Income: ${cs(dollars_per_year())}" }
       
       if component_view() == "hourly" {
            label {
                "Hourly Rate"

                input {
                    type: "number",
                    value: hourly_rate,
                    oninput: move |event| {
                        if let Ok(value) = event.value().parse::<f32>() {
                            hourly_rate.set(value);
                            dollars_per_year.set(52.0 * value * hours_per_week());
                        }
                    }
                }
            }
            label {
                "Hours Per Week"

                input {
                    type: "number",
                    value: hours_per_week,
                    oninput: move |event| {
                        if let Ok(value) = event.value().parse::<f32>() {
                            hours_per_week.set(value);
                            dollars_per_year.set(52.0 * value * hourly_rate());
                        }
                    }
                }
            }
       } else {
            label {
                "Yearly Salary"

                input {
                    type: "number",
                    value: dollars_per_year,
                    oninput: move |event| {
                        if let Ok(value) = event.value().parse::<f32>() {
                            dollars_per_year.set(value);
                        }
                    }
                }
            }
       }

       button {
           onclick: move |_| {
               if dollars_per_year() > 0.0 {
                   global_view.set("results".to_string())
                }
            },
            "Go"
       }
    }
}
