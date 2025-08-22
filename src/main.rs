use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Copy)]
struct GlobalState {
    dollars_per_year: Signal<f32>,
    global_view: Signal<String>,
}

#[component]
fn App() -> Element {
    let app_state = use_context_provider(|| GlobalState {
        dollars_per_year: Signal::new(0.0),
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

#[component]
pub fn Results() -> Element {
    rsx! {
        h1 {"Results"}
    }
}

#[component]
pub fn Inputs() -> Element {
    //Context
    let mut global_view = use_context::<GlobalState>().global_view;
    let mut dollars_per_year = use_context::<GlobalState>().dollars_per_year;

    // State
    let mut component_view = use_signal(|| "hourly".to_string());
    let mut hourly_rate = use_signal(|| 0.0);
    let mut hours_per_week = use_signal(|| 40.0);

    let mut handle_paid_click = move |event: Event<MouseData>, label: &str| {
        component_view.set(label.to_string());
    };

    rsx! {
       h2 { "How are you paid?" }
       div { class: "button-wrapper",
       button { id: "hourly-button", onclick: move |event| handle_paid_click(event, "hourly"), "Hourly" }
       button { id: "yearly-button", onclick: move |event| handle_paid_click(event, "yearly"), "Yearly" }
       }

       p { "Yearly Income: ${dollars_per_year}" }
       
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
